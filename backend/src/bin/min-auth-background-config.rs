use getopts::Options;
use min_auth_backend::config::Config;
use min_auth_common::error::Error;
use min_auth_common::Result;
use std::env;

fn main() -> Result<()> {
    env_logger::init();

    // Parses command line arguments.
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optopt("c", "config", "path to config file", "CONFIG");
    opts.optflag("n", "workers", "number of workers");
    opts.optflag("r", "requests", "requests directory");
    opts.optflag("u", "users", "users directory");
    opts.optflag("w", "workspace", "workspace directory");

    let matches = opts.parse(&args[1..])?;
    let config_path = matches
        .opt_str("c")
        .ok_or(Error::from("No config path was specified."))?;

    // Loads a configuration file.
    let config = Config::load(config_path)?;

    if matches.opt_present("n") {
        println!("{}", config.backend.workers);
    } else if matches.opt_present("r") {
        println!("{}", config.file_system.requests);
    } else if matches.opt_present("u") {
        println!("{}", config.file_system.users);
    } else if matches.opt_present("w") {
        println!("{}", config.file_system.workspace);
    } else {
        return Err(Error::new("No flag was specified."));
    }

    Ok(())
}
