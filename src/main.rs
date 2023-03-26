use actix_web::{web::Data, get, App, HttpServer, HttpResponse, Result};
use actix_web_httpauth::extractors::basic::BasicAuth;
use mysql_async::{prelude::*, Opts, OptsBuilder, Pool};
use redis::{Client, AsyncCommands};
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

async fn verify(
    mysql_pool: &mut Box<Pool>,
    redis_cli: &Client,
    redis_lifetime: usize,
    secret: &String,
    user_id: &str,
    password: &str
) -> bool {
    let mut myconn = mysql_pool.as_mut().get_conn().await.unwrap();
    let mut redconn = redis_cli.get_async_connection().await.unwrap();

    let password = format!("{}{}", secret, password);
    let pwhash = get_hash(password.as_str()).to_lowercase();
    debug!("Authenticating '{}' with {}.", user_id, pwhash);

    // If the credential information for the user is cached,
    // it is not necessary to check the record in MySQL.
    if let Ok(cached) = redconn.get::<&str, String>(user_id).await {
        debug!("A cached credential was found.");
        return if pwhash.eq(&cached) {
            debug!("Succeeded.");
            true
        } else {
            debug!("Failed.");
            false
        }
    }

    let selected = {
        let pwhash = pwhash.clone();

        "SELECT 1 FROM credentials WHERE id = :id AND pwhash = :pwhash"
            .with(params! {
            "id" => user_id,
            "pwhash" => pwhash,
        })
            .map(&mut myconn, |x: u8| x)
            .await
            .unwrap()
    };

    for _ in selected {
        debug!("Succeeded.");
        let _: () = redconn.set_ex(
            user_id, pwhash.as_str(), redis_lifetime).await.unwrap();
        return true;
    }

    debug!("Failed.");
    false
}

#[get("/auth")]
async fn auth(
    mysql_pool: Data<Mutex<Option<Box<Pool>>>>,
    redis_cli: Data<Mutex<Client>>,
    redis_lifetime: Data<usize>,
    secret: Data<String>,
    auth: BasicAuth
) -> Result<HttpResponse> {
    let mut mysql_pool = mysql_pool.lock().unwrap();
    let mysql_pool = mysql_pool.as_mut().unwrap();

    let redis_cli = redis_cli.lock().unwrap();

    if let Some(password) = auth.password() {
        if verify(mysql_pool, &redis_cli, **redis_lifetime, &secret, auth.user_id(), password).await {
            Ok(HttpResponse::Ok().finish())
        } else {
            Ok(HttpResponse::Forbidden().finish())
        }
    } else {
        Ok(HttpResponse::Forbidden().finish())
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let redis_uri = get_enval("REDIS_URI", "redis://127.0.0.1/");
    let redis_lifetime = get_enval("REDIS_LIFETIME", "3600");

    let mysql_host = get_enval("MYSQL_HOST", "127.0.0.1");
    let mysql_port = get_enval("MYSQL_HOST", "3306").parse::<u16>().unwrap();
    let mysql_username = get_enval("MYSQL_USERNAME", "minauth"); // default for debug
    let mysql_password = get_enval("MYSQL_PASSWORD", "minauth"); // default for debug
    let mysql_dbname = get_enval("MYSQL_DBNAME", "minauth");
    let mysql_opts = OptsBuilder::default()
        .ip_or_hostname(mysql_host)
        .tcp_port(mysql_port)
        .user(Some(mysql_username))
        .pass(Some(mysql_password))
        .db_name(Some(mysql_dbname))
        .into();
    let mysql_pool = Data::new(Mutex::new(Some(Box::new(Pool::new::<Opts>(mysql_opts)))));

    let ret = {
        let mysql_pool = Data::clone(&mysql_pool);
        let redis_cli = Data::new(Mutex::new(redis::Client::open(redis_uri.as_str()).unwrap()));
        let redis_lifetime = Data::new(redis_lifetime.parse::<usize>().unwrap());
        let secret = Data::new(get_enval("PASSWORD_SECRET", ""));

        HttpServer::new(move || {
            App::new()
                .app_data(Data::clone(&mysql_pool))
                .app_data(Data::clone(&redis_cli))
                .app_data(Data::clone(&redis_lifetime))
                .app_data(Data::clone(&secret))
                .service(auth)
        })
            .bind("0.0.0.0:3000")?
            .run()
            .await
    };

    let mut mysql_pool = mysql_pool.lock().unwrap();
    let mysql_pool = std::mem::replace(&mut *mysql_pool, None);
    mysql_pool.unwrap().disconnect().await.unwrap();
    info!("The connection pool was disconnected.");

    ret
}
