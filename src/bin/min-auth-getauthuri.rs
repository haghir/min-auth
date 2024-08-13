use getopts::Options;
use std::env;

use min_auth_auth::Config;

fn main() {
    env_logger::init();

    // Parses command line arguments.
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optopt("c", "config", "path to a config file", "CONFIG");
    opts.optopt("p", "port", "port number", "PORT");
    let matches = opts.parse(&args[1..]).unwrap();
    let config_path = matches.opt_str("c").unwrap();
    let port = matches.opt_str("p").unwrap();

    // Laods a configuration file.
    let config: String = std::fs::read_to_string(config_path).unwrap();
    let config: Config = (&config).try_into().unwrap();
    let expose = format!("{}:{}", config.minauth.hostname, port);

    // Displays the service URI.
    print!("http://{}/auth", expose);
}
