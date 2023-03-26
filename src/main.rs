use actix_web::{web::Data, get, App, HttpServer, HttpResponse, Result};
use actix_web_httpauth::extractors::basic::BasicAuth;
use mysql_async::{prelude::*, Opts, OptsBuilder, Pool};
use sha2::{Sha256, Digest};
use std::{env, sync::Mutex};
use log::{info, debug};

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

async fn verify(pool: &mut Box<Pool>, secret: &String, user_id: &str, password: &str) -> bool {
    let mut conn = pool.as_mut().get_conn().await.unwrap();

    let password = format!("{}{}", secret, password);
    let pwhash = get_hash(password.as_str()).to_lowercase();
    debug!("Authenticating '{}' with {}.", user_id, pwhash);

    let selected =
        "SELECT 1 FROM credentials WHERE id = :id AND pwhash = :pwhash"
        .with(params! {
            "id" => user_id,
            "pwhash" => pwhash,
        })
        .map(&mut conn, |x: u8| x)
        .await
        .unwrap();

    for _ in selected {
        debug!("Succeeded.");
        return true;
    }

    debug!("Failed.");
    false
}

#[get("/auth")]
async fn auth(
    pool: Data<Mutex<Option<Box<Pool>>>>,
    secret: Data<String>,
    auth: BasicAuth
) -> Result<HttpResponse> {
    let mut pool = pool.lock().unwrap();
    {
        let pool = pool.as_mut().unwrap();

        if let Some(password) = auth.password() {
            if verify(pool, &secret, auth.user_id(), password).await {
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
    let pool = Data::new(Mutex::new(Some(Box::new(Pool::new::<Opts>(opts)))));

    let ret = {
        let pool = Data::clone(&pool);
        let secret = Data::new(get_enval("PASSWORD_SECRET", ""));

        HttpServer::new(move || {
            App::new()
                .app_data(Data::clone(&pool))
                .app_data(Data::clone(&secret))
                .service(auth)
        })
            .bind("0.0.0.0:3000")?
            .run()
            .await
    };

    let mut pool = pool.lock().unwrap();
    let pool = std::mem::replace(&mut *pool, None);
    pool.unwrap().disconnect().await.unwrap();
    info!("The connection pool was disconnected.");

    ret
}
