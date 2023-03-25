use actix_web::{web::Data, get, App, HttpServer, HttpResponse, Result};
use actix_web_httpauth::extractors::basic::BasicAuth;
use mysql_async::{prelude::*, Opts, OptsBuilder, Pool};
use sha2::{Sha256, Digest};
use std::{env, sync::Mutex};
use log::info;

struct DbPool {
    pool: Mutex<Option<Box<Pool>>>,
}

impl DbPool {
    fn new(opts: Opts) -> Self {
        DbPool {
            pool: Mutex::new(Some(Box::new(Pool::new::<Opts>(opts)))),
        }
    }

    async fn disconnect(&self) {
        let mut pool = self.pool.lock().unwrap();
        let pool = std::mem::replace(&mut *pool, None);
        pool.unwrap().disconnect().await.unwrap();

        info!("The connection pool was disconnected.");
    }
}

fn get_enval(name: &str, default: &str) -> String {
    match env::var(name) {
        Ok(val) => val,
        Err(_) => default.to_string(),
    }
}

fn get_hash(password: &str) -> String {
    let encoded = password.as_bytes();
    let mut hasher = Sha256::new();
    hasher.update(encoded);
    hex::encode(hasher.finalize())
}

async fn verify(pool: &mut Box<Pool>, user_id: &str, password: &str) -> bool {
    let mut conn = pool.as_mut().get_conn().await.unwrap();
    let hash = get_hash(password);

    let selected = "SELECT pwhash FROM credentials WHERE id = :id"
        .with(params! { "id" => user_id })
        .map(&mut conn, |pwhash: String| pwhash)
        .await
        .unwrap();

    for pwhash in selected {
        if pwhash.eq(&hash) {
            return true;
        }
    }

    false
}

#[get("/auth")]
async fn auth(pool: Data<DbPool>, auth: BasicAuth) -> Result<HttpResponse> {
    let mut pool = pool.pool.lock().unwrap();
    {
        let pool = pool.as_mut().unwrap();

        if let Some(password) = auth.password() {
            if verify(pool, auth.user_id(), password).await {
                Ok(HttpResponse::Ok().finish())
            } else {
                Ok(HttpResponse::Forbidden().finish())
            }
        } else {
            Ok(HttpResponse::Forbidden().finish())
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let host = get_enval("MYSQL_HOST", "127.0.0.1");
    let port = get_enval("MYSQL_HOST", "3306").parse::<u16>().unwrap();
    let username = get_enval("MYSQL_USERNAME", "minauth"); // default for debug
    let password = get_enval("MYSQL_PASSWORD", "minauth"); // default for debug
    let dbname = get_enval("MYSQL_DBNAME", "minauth");

    let opts = OptsBuilder::default()
        .ip_or_hostname(host)
        .tcp_port(port)
        .user(Some(username))
        .pass(Some(password))
        .db_name(Some(dbname))
        .into();

    let pool = Data::new(DbPool::new(opts));

    let ret = {
        let pool = Data::clone(&pool);

        HttpServer::new(move || {
            App::new()
                .app_data(Data::clone(&pool))
                .service(auth)
        })
            .bind("0.0.0.0:3000")?
            .run()
            .await
    };

    pool.disconnect().await;

    ret
}
