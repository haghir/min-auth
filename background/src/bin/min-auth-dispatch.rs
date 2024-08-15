use std::env;
use getopts::Options;
use mysql_async::{Conn, Opts};
use min_auth_common::config::BackgroundConfig;
use min_auth_common::error::Error;
use min_auth_common::Result;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    // Parses command line arguments.
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optopt("c", "config", "path to config file", "CONFIG");
    opts.optopt("i", "id", "request ID", "ID");
    opts.optopt("n", "num", "worker number", "NUMBER");
    opts.optopt("w", "workers", "the number of workers", "WORKER");

    let matches = opts.parse(&args[1..])?;
    let config_path = matches.opt_str("c")
        .ok_or(Error::from("No config path was specified."))?;
    let id = matches.opt_str("i")
        .ok_or(Error::from("No ID was specified."))?;
    let num = matches.opt_str("n")
        .ok_or(Error::from("No number was specified."))?
        .parse::<usize>()?;
    let workers = matches.opt_str("w")
        .ok_or(Error::from("No number of workers was specified."))?
        .parse::<usize>()?;

    // Loads a configuration file.
    let config: String = std::fs::read_to_string(config_path)?;
    let config = <&String as TryInto<BackgroundConfig>>::try_into(&config)?;

    // Initializes a MySQL client.
    let mysql_opts: Opts = (&config.mysql).into();
    let mut mysql_conn = Conn::new(mysql_opts).await?;

    Ok(())
}