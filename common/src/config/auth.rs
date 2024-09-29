use super::parts::{ExposeConfig, SecurityConfig};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use toml::{de::from_str, ser::to_string};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AuthConfig {
    pub expose: ExposeConfig,
    pub backend: ExposeConfig,
    pub security: SecurityConfig,
}

impl TryFrom<&String> for AuthConfig {
    type Error = toml::de::Error;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        from_str(value.as_str())
    }
}

impl TryFrom<&AuthConfig> for String {
    type Error = toml::ser::Error;

    fn try_from(value: &AuthConfig) -> Result<Self, Self::Error> {
        to_string(value)
    }
}
