use std::env;
use getopts::Options;
use min_auth::config::{Config, MinAuthConfig, MySQLConfig, RedisConfig};
use min_auth::error::Error;
use min_auth::Result;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optopt("d", "prefix", "Prefix of the application", "PREFIX");
    let matches = opts.parse(&args[1..])?;
    let prefix = matches.opt_str("d")
        .ok_or(Error::from("No prefix was specified."))?;

    let config = Config {
        minauth: MinAuthConfig {
            hostname: "127.0.0.1".to_string(),
            password_secret: "PASSWORD SECRET".to_string(),
            tickets_dir: format!("{}/var/tickets", prefix),
            requests_dir: format!("{}/var/requests", prefix),
        },
        mysql: MySQLConfig {
            hostname: "localhost".to_string(),
            port: 3306,
            username: "minauth".to_string(),
            password: "minauth".to_string(),
            database: "minauth".to_string(),
        },
        redis: RedisConfig {
            uri: "redis://127.0.0.1:6379/0".to_string(),
            lifetime: 600,
        },
    };

    let toml: String = match <&Config as TryInto<String>>::try_into(&config) {
        Ok(v) => v,
        Err(e) => return Err(e.to_string().into())
    };
    print!("{}", toml);

    Ok(())
}
