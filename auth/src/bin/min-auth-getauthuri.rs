use getopts::Options;
use std::env;

use min_auth_common::config::MinAuthConfig;
use min_auth_common::error::Error;
use min_auth_common::Result;

fn main() -> Result<()> {
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
        .ok_or(Error::from("No port number was specified."))?;

    // Laods a configuration file.
    let config: String = std::fs::read_to_string(config_path)?;
    let config: MinAuthConfig = (&config).try_into()?;
    let expose = format!("{}:{}", config.hostname, port);

    // Displays the service URI.
    print!("http://{}/auth", expose);

    Ok(())
}
