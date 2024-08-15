use std::env;
use futures_util::StreamExt;
use getopts::Options;
use mysql_async::{prelude::*, Opts, Conn};
use redis::{Client as RedisClient, Commands};
use log::{info, debug};

use min_auth_common::config::MinAuthConfig;
use min_auth_common::credentials::Credential;
use min_auth_common::error::Error;
use min_auth_common::Result;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    // Parses command line arguments.
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optopt("c", "config", "path to config file", "CONFIG");
    let matches = opts.parse(&args[1..])?;
    let config_path = matches.opt_str("c")
        .ok_or(Error::from("No config path was specified."))?;

    // Loads a configuration file.
    let config: String = std::fs::read_to_string(config_path)?;
    let config = <&String as TryInto<MinAuthConfig>>::try_into(&config)?;

    // Initializes a MySQL client.
    let mysql_opts: Opts = (&config.mysql).into();
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
