use crate::DynError;
use serde::{Deserialize, Serialize};
use std::{
    fs::{read_to_string, File},
    io::Write,
    path::Path,
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AuthConfig {
    pub expose: ExposeConfig,
    pub security: SecurityConfig,
    pub redis: RedisConfig,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ExposeConfig {
    pub sockets: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SecurityConfig {
    pub password_secret: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RedisConfig {
    pub uri: String,
}

impl AuthConfig {
    pub fn load<P>(path: P) -> Result<Self, DynError>
    where
        P: AsRef<Path>,
    {
        let content = read_to_string(path)?;
        Ok(content.as_str().try_into()?)
    }

    pub fn save<P>(&self, path: P) -> Result<(), DynError>
    where
        P: AsRef<Path>,
    {
        let config: String = self.try_into()?;
        let mut file = File::create(path)?;
        file.write_all(config.as_bytes())?;
        Ok(())
    }
}

impl TryFrom<&str> for AuthConfig {
    type Error = toml::de::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        toml::de::from_str(value)
    }
}

impl TryFrom<&AuthConfig> for String {
    type Error = toml::ser::Error;

    fn try_from(value: &AuthConfig) -> Result<Self, Self::Error> {
        toml::ser::to_string(value)
    }
}
