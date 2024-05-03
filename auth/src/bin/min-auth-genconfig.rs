use min_auth::{Config, MinAuthConfig, MySQLConfig, RedisConfig};

fn main() {
    let config = Config {
        minauth: MinAuthConfig {
            hostname: "127.0.0.1".to_string(),
            password_secret: "PASSWORD SECRET".to_string(),
        },
        mysql: MySQLConfig {
            hostname: "localhost".to_string(),
            port: 3306,
            username: "minauth".to_string(),
            password: "minauth".to_string(),
            database: "minauth".to_string(),
        },
        redis: RedisConfig {
            uri: "redis://127.0.0.1:6379/0".to_string(),
            lifetime: 600,
        },
    };

    let toml: String = (&config).try_into().unwrap();
    print!("{}", toml);
}