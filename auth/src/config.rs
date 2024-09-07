use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use toml::{de::from_str, ser::to_string};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub expose: Vec<WebConfig>,
    pub security: SecurityConfig,
    pub file_system: FsConfig,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WebConfig {
    pub hostname: String,
    pub port: u16,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SecurityConfig {
    pub password_salt: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FsConfig {
    pub users: String,
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
