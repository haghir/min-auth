use getopts::Options;
use min_auth_auth::config::{Config, FsConfig, SecurityConfig, WebConfig};
use min_auth_common::error::Error;
use min_auth_common::Result;
use std::env;
use std::path::PathBuf;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optopt("d", "prefix", "Prefix of the application", "PREFIX");
    let matches = opts.parse(&args[1..])?;
    let prefix = matches
        .opt_str("d")
        .ok_or(Error::from("No prefix was specified."))?;

    let users_dir: PathBuf = [prefix.as_str(), "var", "min-auth", "users"]
        .iter()
        .collect();

    let config = Config {
        expose: vec![
            WebConfig {
                hostname: "127.0.0.1".to_string(),
                port: 50080,
            },
            WebConfig {
                hostname: "127.0.0.1".to_string(),
                port: 50081,
            },
            WebConfig {
                hostname: "127.0.0.1".to_string(),
                port: 50082,
            },
            WebConfig {
                hostname: "127.0.0.1".to_string(),
                port: 50083,
            },
        ],
        security: SecurityConfig {
            password_salt: "".to_string(),
        },
        file_system: FsConfig {
            users_dir: users_dir.into_os_string().into_string()?,
        },
    };
    let config = config;

    let toml: String = match <&Config as TryInto<String>>::try_into(&config) {
        Ok(v) => v,
        Err(e) => return Err(e.to_string().into()),
    };
    print!("{}", toml);

    Ok(())
}
