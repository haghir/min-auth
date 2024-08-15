use std::convert::TryFrom;
use mysql_async::{Opts, OptsBuilder};
use serde::{Serialize, Deserialize};
use toml::{de::from_str, ser::to_string};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MinAuthConfig {
    pub hostname: String,
    pub password_secret: String,
    pub mysql: MySQLConfig,
    pub redis: RedisConfig,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WebConfig {
    pub requests_dir: String,
    pub mysql: MySQLConfig,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BackgroundConfig {
    pub workers: u64,
    pub requests_dir: String,
    pub workspace_dir: String,
    pub mysql: MySQLConfig,
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

impl TryFrom<&String> for MinAuthConfig {
    type Error = toml::de::Error;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        from_str(value.as_str())
    }
}

impl TryFrom<&MinAuthConfig> for String {
    type Error = toml::ser::Error;

    fn try_from(value: &MinAuthConfig) -> Result<Self, Self::Error> {
        to_string(value)
    }
}

impl TryFrom<&String> for WebConfig {
    type Error = toml::de::Error;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        from_str(value.as_str())
    }
}

impl TryFrom<&WebConfig> for String {
    type Error = toml::ser::Error;

    fn try_from(value: &WebConfig) -> Result<Self, Self::Error> {
        to_string(value)
    }
}

impl TryFrom<&String> for BackgroundConfig {
    type Error = toml::de::Error;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        from_str(value.as_str())
    }
}

impl TryFrom<&BackgroundConfig> for String {
    type Error = toml::ser::Error;

    fn try_from(value: &BackgroundConfig) -> Result<Self, Self::Error> {
        to_string(value)
    }
}

impl From<&MySQLConfig> for Opts {
    fn from(value: &MySQLConfig) -> Self {
        OptsBuilder::default()
            .ip_or_hostname(&value.hostname)
            .tcp_port(value.port)
            .user(Some(&value.username))
            .pass(Some(&value.password))
            .db_name(Some(&value.database))
            .into()
    }
}
