use std::env;
use getopts::Options;
use mysql_async::{FromRowError, Row};
use mysql_async::prelude::FromValue;
use crate::config::Config;
use crate::error::Error;

pub fn get_from_row<T: FromValue>(row: &Row, idx: usize) -> Result<T, FromRowError> {
    match row.get(idx) {
        Some(v) => Ok(v),
        None => Err(FromRowError(row.clone()))
    }
}

pub fn load_config_from_args() -> Result<Config, Error> {
    // Parses command line arguments.
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optopt("c", "config", "path to config file", "CONFIG");
    let matches = opts.parse(&args[1..])?;

    if let Some(config_path) = matches.opt_str("c") {
        // Loads a configuration file.
        let config: String = std::fs::read_to_string(config_path)?;
        Ok((&config).try_into()?)
    } else {
        Err(Error::new("No config path was specified."))
    }
}
