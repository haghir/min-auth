use std::env;
use getopts::Options;
use min_auth_common::config::BackgroundConfig;
use min_auth_common::error::Error;
use min_auth_common::Result;

fn main() -> Result<()> {
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
    let config = <&String as TryInto<BackgroundConfig>>::try_into(&config)?;

    print!("{}", config.workers);

    Ok(())
}
