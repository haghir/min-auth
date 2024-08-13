use actix_web::{web::Data, get, App, HttpServer, HttpResponse, Result};
use actix_web_httpauth::extractors::basic::BasicAuth;
use getopts::Options;
use redis::{Client as RedisClient, AsyncCommands};
use std::{env, sync::Mutex};
use log::{info, debug};

use min_auth_auth::{Config, Credential};

struct WebContext {
    // Redis client
    redis: Option<Box<RedisClient>>,
}

fn new_redis_client(config: &Config) -> Box<RedisClient> {
    Box::new(RedisClient::open(config.redis.uri.as_str()).unwrap())
}

async fn verify(
    web_ctx: &mut WebContext,
    config: &Config,
    user_id: &str,
    password: &str
) -> bool {
    // Retrieves an async connection of Redis.
    let redconn = web_ctx.redis.as_mut().unwrap();
    let mut redconn = redconn.get_async_connection().await.unwrap();

    if let Ok(cached) = redconn.get::<&str, String>(user_id).await {
        debug!("{}", cached);

        let cred: Credential = (&cached).try_into().unwrap();
        if cred.verify(&config.minauth.password_secret, password) {
            info!("{} was authenticated.", user_id);
            true
        } else {
            info!("Failed to authenticate {}.", user_id);
            false
        }
    } else {
        info!("{}'s credential information is not found.", user_id);
        false
    }
}

#[get("/auth")]
async fn auth(
    web_ctx: Data<Mutex<WebContext>>,
    config: Data<Config>,
    auth: BasicAuth
) -> Result<HttpResponse> {
    let mut web_ctx = web_ctx.lock().unwrap();

    if let Some(password) = auth.password() {
        if verify(&mut web_ctx, &(**config), auth.user_id(), password).await {
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

    // Parses command line arguments.
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optopt("c", "config", "path to a config file", "CONFIG");
    opts.optopt("p", "port", "port number", "PORT");
    let matches = opts.parse(&args[1..]).unwrap();
    let config_path = matches.opt_str("c").unwrap();
    let port = matches.opt_str("p").unwrap();

    // Laods a configuration file.
    let config: String = std::fs::read_to_string(config_path).unwrap();
    let config: Config = (&config).try_into().unwrap();
    let config = Data::new(config);
    let expose = format!("{}:{}", config.minauth.hostname, port);

    // Creates a web context.
    let web_ctx = Data::new(Mutex::new(WebContext {
        redis: Some(new_redis_client(&config)),
    }));

    // Starts a web server.
    HttpServer::new(move || {
        App::new()
            .app_data(Data::clone(&config))
            .app_data(Data::clone(&web_ctx))
            .service(auth)
    })
        .bind(expose.as_str())?
        .run()
        .await
}
