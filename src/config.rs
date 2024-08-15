use std::convert::TryFrom;
use mysql_async::{Opts, OptsBuilder};
use serde::{Serialize, Deserialize};
use toml::{de::from_str, ser::to_string};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub minauth: MinAuthConfig,
    pub mysql: MySQLConfig,
    pub redis: RedisConfig,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MinAuthConfig {
    pub hostname: String,
    pub password_secret: String,
    pub tickets_dir: String,
    pub requests_dir: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MySQLConfig {
    pub hostname: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RedisConfig {
    pub uri: String,
    pub lifetime: u64,
}

impl TryFrom<&String> for Config {
    type Error = toml::de::Error;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        from_str(value.as_str())
    }
}

impl TryFrom<&Config> for String {
    type Error = toml::ser::Error;

    fn try_from(value: &Config) -> Result<Self, Self::Error> {
        to_string(value)
    }
}

impl From<&Config> for Opts {
    fn from(value: &Config) -> Self {
        OptsBuilder::default()
            .ip_or_hostname(&value.mysql.hostname)
            .tcp_port(value.mysql.port)
            .user(Some(&value.mysql.username))
            .pass(Some(&value.mysql.password))
            .db_name(Some(&value.mysql.database))
            .into()
    }
}
