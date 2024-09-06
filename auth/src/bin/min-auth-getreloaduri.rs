use getopts::Options;
use std::env;

use min_auth_auth::config::Config;
use min_auth_common::error::Error;
use min_auth_common::Result;

fn main() -> Result<()> {
    env_logger::init();

    // Parses command line arguments.
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optopt("c", "config", "path to a config file", "CONFIG");
    opts.optopt("i", "index", "process index", "PROCIDX");
    let matches = opts.parse(&args[1..])?;
    let config_path = matches
        .opt_str("c")
        .ok_or(Error::from("No config path was specified."))?;
    let index: usize = matches
        .opt_str("n")
        .ok_or(Error::from("No process index was specified."))?
        .parse::<usize>()?;

    // Laods a configuration file.
    let config: String = std::fs::read_to_string(config_path)?;
    let config: Config = (&config).try_into()?;
    let expose = &config.expose[index];

    // Displays the service URI.
    println!("http://{}:{}/reload", expose.hostname, expose.port);

    Ok(())
}
