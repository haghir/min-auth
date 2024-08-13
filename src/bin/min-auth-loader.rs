use getopts::Options;
use futures_util::StreamExt;
use mysql_async::{prelude::*, Opts, OptsBuilder, Conn};
use redis::{Client as RedisClient, Commands};
use std::env;
use log::{info, debug};

use min_auth::{Config, Credential};

#[tokio::main]
async fn main() {
    env_logger::init();

    // Parses command line arguments.
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optopt("c", "config", "path to config file", "CONFIG");
    let matches = opts.parse(&args[1..]).unwrap();
    let config_path = matches.opt_str("c").unwrap();

    // Laods a configuration file.
    let config: String = std::fs::read_to_string(config_path).unwrap();
    let config: Config = (&config).try_into().unwrap();

    // Initializes a MySQL client.
    let mysql_opts: Opts = OptsBuilder::default()
        .ip_or_hostname(config.mysql.hostname)
        .tcp_port(config.mysql.port)
        .user(Some(config.mysql.username))
        .pass(Some(config.mysql.password))
        .db_name(Some(config.mysql.database))
        .into();
    let mut mysql_conn = Conn::new(mysql_opts).await.unwrap();

    // Initializes a Redis client.
    let mut redis_conn = RedisClient::open(config.redis.uri.as_str()).unwrap();

    // Copy all credentials in the MySQL database to the Redis server.
    let sql = "SELECT id, salt, pwhash FROM credentials";
    let mut result = mysql_conn.query_iter(sql).await.unwrap();
    let mut stream = result.stream::<Credential>().await.unwrap().unwrap();
    while let Some(cred) = stream.next().await {
        let cred = cred.unwrap();
        let id: &str = cred.id.as_str();
        let json: String = (&cred).into();
        let lt = config.redis.lifetime;

        let _: () = redis_conn.set_ex(id, json.as_str(), lt).unwrap();

        info!("Prepare a credential record for {}.", id);
        debug!("  {}", json);
    }
}
