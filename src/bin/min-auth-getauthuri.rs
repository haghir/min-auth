use getopts::Options;
use std::env;

use min_auth::Config;

fn main() {
    env_logger::init();

    // Parses command line arguments.
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optopt("c", "config", "path to a config file", "CONFIG");
    let matches = opts.parse(&args[1..]).unwrap();
    let config_path = matches.opt_str("c").unwrap();

    // Laods a configuration file.
    let config: String = std::fs::read_to_string(config_path).unwrap();
    let config: Config = (&config).try_into().unwrap();

    // Displays the service URI.
    print!("http://{}/auth", config.minauth.expose);
}
