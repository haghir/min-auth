use std::env;
use getopts::Options;
use min_auth_common::config::{BackgroundConfig, MySQLConfig};
use min_auth_common::error::Error;
use min_auth_common::Result;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optopt("d", "prefix", "Prefix of the application", "PREFIX");
    let matches = opts.parse(&args[1..])?;
    let prefix = matches.opt_str("d")
        .ok_or(Error::from("No prefix was specified."))?;

    let config = BackgroundConfig {
        requests_dir: format!("{}/var/min-auth/requests", prefix),
        workspace_dir: format!("{}/var/min-auth/workspace", prefix),
        mysql: MySQLConfig {
            hostname: "localhost".to_string(),
            port: 3306,
            username: "minauth".to_string(),
            password: "minauth".to_string(),
            database: "minauth".to_string(),
        },
    };

    let toml: String = match <&BackgroundConfig as TryInto<String>>::try_into(&config) {
        Ok(v) => v,
        Err(e) => return Err(e.to_string().into())
    };
    print!("{}", toml);

    Ok(())
}
