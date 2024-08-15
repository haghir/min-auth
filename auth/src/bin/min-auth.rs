use actix_web::{web::Data, get, App, HttpServer, HttpResponse, Result as WebResult};
use actix_web_httpauth::extractors::basic::BasicAuth;
use getopts::Options;
use redis::{Client as RedisClient, AsyncCommands, RedisResult};
use std::{env, sync::Mutex};
use log::{error, info, debug};

use min_auth_common::config::MinAuthConfig;
use min_auth_common::credentials::Credential;
use min_auth_common::error::Error;
use min_auth_common::Result;

struct WebContext {
    // Redis client
    redis: Option<Box<RedisClient>>,
}

fn new_redis_client(config: &MinAuthConfig) -> RedisResult<Box<RedisClient>> {
    Ok(Box::new(RedisClient::open(config.redis.uri.as_str())?))
}

async fn verify(
    web_ctx: &mut WebContext,
    config: &MinAuthConfig,
    user_id: &str,
    password: &str
) -> Result<bool> {
    // Retrieves an async connection of Redis.
    let redconn = web_ctx.redis.as_mut()
        .ok_or(Error::new("No redis client was found."))?;
    let mut redconn = redconn.get_multiplexed_async_connection().await?;

    if let Ok(cached) = redconn.get::<&str, String>(user_id).await {
        debug!("{}", cached);

        if let Ok(cred) = <&String as TryInto<Credential>>::try_into(&cached) {
            if cred.verify(&config.password_secret, password) {
                info!("{} was authenticated.", user_id);
                Ok(true)
            } else {
                info!("Failed to authenticate {}.", user_id);
                Ok(false)
            }
        } else {
            Err("Failed to convert the serialized string into Credential.".into())
        }
    } else {
        info!("{}'s credential information is not found.", user_id);
        Ok(false)
    }
}

#[get("/auth")]
async fn auth(
    web_ctx: Data<Mutex<WebContext>>,
    config: Data<MinAuthConfig>,
    auth: BasicAuth
) -> WebResult<HttpResponse> {
    let mut web_ctx = match web_ctx.lock() {
        Ok(v) => v,
        Err(e) => {
            error!("{}", e);
            return Ok(HttpResponse::InternalServerError().finish());
        },
    };

    if let Some(password) = auth.password() {
        if let Ok(ret) = verify(&mut web_ctx, &(**config), auth.user_id(), password).await {
            if ret {
                Ok(HttpResponse::Ok().finish())
            } else {
                Ok(HttpResponse::Forbidden().finish())
            }
        } else {
            Ok(HttpResponse::InternalServerError().finish())
        }
    } else {
        Ok(HttpResponse::Forbidden().finish())
    }
}

#[actix_web::main]
async fn main() -> Result<()> {
    env_logger::init();

    // Parses command line arguments.
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optopt("c", "config", "path to a config file", "CONFIG");
    opts.optopt("p", "port", "port number", "PORT");
    let matches = opts.parse(&args[1..])?;
    let config_path = matches.opt_str("c")
        .ok_or(Error::from("No config path was specified."))?;
    let port = matches.opt_str("p")
        .ok_or(Error::from("No port was specified."))?;

    // Loads a configuration file.
    let config: String = std::fs::read_to_string(config_path)?;
    let config: MinAuthConfig = (&config).try_into()?;
    let config = Data::new(config);
    let expose = format!("{}:{}", config.hostname, port);

    // Creates a web context.
    let web_ctx = Data::new(Mutex::new(WebContext {
        redis: Some(new_redis_client(&config)?),
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
        .await?;

    Ok(())
}
