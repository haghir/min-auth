use futures_util::StreamExt;
use mysql_async::{prelude::*, Opts, OptsBuilder, Conn};
use redis::{Client as RedisClient, Commands};
use log::{info, debug};

use min_auth::config::Config;
use min_auth::credentials::Credential;
use min_auth::error::Error;
use min_auth::utils::load_config_from_args;
use min_auth::Result;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    // Loads a configuration file.
    let config: Config = load_config_from_args()?;

    // Initializes a MySQL client.
    let mysql_opts: Opts = OptsBuilder::default()
        .ip_or_hostname(config.mysql.hostname)
        .tcp_port(config.mysql.port)
        .user(Some(config.mysql.username))
        .pass(Some(config.mysql.password))
        .db_name(Some(config.mysql.database))
        .into();
    let mut mysql_conn = Conn::new(mysql_opts).await?;

    // Initializes a Redis client.
    let mut redis_conn = RedisClient::open(config.redis.uri.as_str())?;

    // Copy all credentials in the MySQL database to the Redis server.
    let sql = "SELECT id, salt, pwhash FROM credentials";
    let mut result = mysql_conn.query_iter(sql).await?;
    let mut stream = result.stream::<Credential>().await?
        .ok_or(Error::from("Failed to get a stream from the result."))?;
    while let Some(cred) = stream.next().await {
        let cred = cred?;
        let id: &str = cred.id.as_str();
        let json: String = (&cred).into();
        let lt = config.redis.lifetime;

        let _: () = redis_conn.set_ex(id, json.as_str(), lt)?;

        info!("Prepare a credential record for {}.", id);
        debug!("  {}", json);
    }
    Ok(())
}
