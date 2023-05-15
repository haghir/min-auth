use actix_web::{web::Data, get, App, HttpServer, HttpResponse, Result};
use actix_web_httpauth::extractors::basic::BasicAuth;
use mysql_async::{prelude::*, Opts, OptsBuilder, Pool as MySQLPool};
use redis::{Client as RedisClient, AsyncCommands};
use sha2::{Sha256, Digest};
use std::{env, sync::Mutex};
use log::{info, debug};

struct CredContext {
    // MySQL connection pool
    mysql: Option<Box<MySQLPool>>,

    // Redis client
    redis: Option<Box<RedisClient>>,
}

impl CredContext {
    async fn close(&mut self) {
        if let Some(mysql_pool) = std::mem::replace(&mut self.mysql, None) {
            mysql_pool.disconnect().await.unwrap();
            info!("MySQL connection pool was disconnected.");
        }
    }
}

fn get_enval(name: &str, default: &str) -> String {
    match env::var(name) {
        Ok(val) => val,
        Err(_) => default.to_string(),
    }
}

fn get_enval_usize(name: &str, default: &str) -> usize {
    get_enval(name, default).parse().unwrap()
}

fn new_mysql_pool() -> Box<MySQLPool> {
    let mysql_host = get_enval("MYSQL_HOST", "127.0.0.1");
    let mysql_port = get_enval("MYSQL_PORT", "3306").parse::<u16>().unwrap();
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
    Box::new(MySQLPool::new::<Opts>(mysql_opts))
}

fn new_redis_client() -> Box<RedisClient> {
    let redis_uri = get_enval("REDIS_URI", "redis://127.0.0.1/");
    Box::new(RedisClient::open(redis_uri.as_str()).unwrap())
}

fn get_hash(password: &str) -> String {
    let encoded = password.as_bytes();
    let mut hasher = Sha256::new();
    hasher.update(encoded);
    hex::encode(hasher.finalize())
}

async fn verify(
    cred_ctx: &mut CredContext,
    redis_lifetime: usize,
    secret: &String,
    user_id: &str,
    password: &str
) -> bool {
    // Retrieve a MySQL connection from the connection pool.
    let myconn = cred_ctx.mysql.as_mut().unwrap();
    let mut myconn = myconn.get_conn().await.unwrap();

    // Retrieve an async connection of Redis.
    let redconn = cred_ctx.redis.as_mut().unwrap();
    let mut redconn = redconn.get_async_connection().await.unwrap();

    // If the credential information for the user is cached,
    // it is not necessary to check the record in MySQL.
    let cred = if let Ok(cached) = redconn.get::<&str, (String, String)>(user_id).await {
        cached
    } else {
        // Retrieve the credential information from the MySQL database.
        let cred: Option<(String, String)> = "SELECT salt, pwhash FROM credentials WHERE id = :id"
            .with(params! { "id" => user_id })
            .first(&mut myconn)
            .await
            .unwrap();

        // For performance, the authenticated credentials will be stored in the Redis database.
        if let Some(cred) = cred {
            let _: () = redconn.set_ex(user_id, &cred, redis_lifetime).await.unwrap();
            cred
        } else {
            info!("User ID \"{}\" is not found.", user_id);
            return false;
        }
    };

    // Calculate a SHA256 digest of a string concatenated the secret and the password.
    let password = format!("{}{}{}", secret, cred.0, password);
    let target = get_hash(password.as_str()).to_lowercase();

    if target.eq(&cred.1) {
        info!("Succeeded to authenticate \"{}\".", user_id);
        debug!("Credential: {}", target);
        true
    } else {
        info!("Failed to authenticate \"{}\".", user_id);
        debug!("Credential: {}", target);
        false
    }
}

#[get("/auth")]
async fn auth(
    cred_ctx: Data<Mutex<CredContext>>,
    redis_lifetime: Data<usize>,
    secret: Data<String>,
    auth: BasicAuth
) -> Result<HttpResponse> {
    let mut cred_ctx = cred_ctx.lock().unwrap();

    if let Some(password) = auth.password() {
        if verify(&mut cred_ctx, **redis_lifetime, &secret, auth.user_id(), password).await {
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

    let cred_ctx = Data::new(Mutex::new(CredContext {
        mysql: Some(new_mysql_pool()),
        redis: Some(new_redis_client()),
    }));

    let ret = {
        let cred_ctx = Data::clone(&cred_ctx);
        let redis_lifetime = Data::new(get_enval_usize("REDIS_LIFETIME", "3600"));
        let secret = Data::new(get_enval("PASSWORD_SECRET", ""));

        HttpServer::new(move || {
            App::new()
                .app_data(Data::clone(&cred_ctx))
                .app_data(Data::clone(&redis_lifetime))
                .app_data(Data::clone(&secret))
                .service(auth)
        })
            .bind("0.0.0.0:3000")?
            .run()
            .await
    };

    let mut cred_ctx = cred_ctx.lock().unwrap();
    cred_ctx.close().await;

    ret
}
