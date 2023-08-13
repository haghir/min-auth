use min_auth::{Config, MinAuthConfig, MySQLConfig, RedisConfig};

fn main() {
    let config = Config {
        minauth: MinAuthConfig {
            expose: "127.0.0.1:3000".to_string(),
            password_secret: "PASSWORD SECRET".to_string(),
        },
        mysql: MySQLConfig {
            hostname: "localhost".to_string(),
            port: 3306,
            username: "foo".to_string(),
            password: "foofoo".to_string(),
            database: "foodb".to_string(),
        },
        redis: RedisConfig {
            uri: "redis://127.0.0.1:6379/0".to_string(),
            lifetime: 300,
        },
    };

    let toml: String = (&config).try_into().unwrap();
    println!("{}", toml);
}