use std::convert::TryFrom;
use serde::{Serialize, Deserialize};
use toml::{de::from_str, ser::to_string};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MinAuthConfig {
    pub hostname: String,
    pub password_secret: String,
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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub minauth: MinAuthConfig,
    pub mysql: MySQLConfig,
    pub redis: RedisConfig,
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
