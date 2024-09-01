use std::convert::TryFrom;
use serde::{Serialize, Deserialize};
use toml::{de::from_str, ser::to_string};

// ===================================================================
// Authentication Service
// ===================================================================

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MinAuthConfig {
    pub hostname: String,
    pub password_secret: String,
    pub redis: RedisConfig,
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

// ===================================================================
// Web Service
// ===================================================================

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WebConfig {
    pub requests_dir: String,
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

// ===================================================================
// Background Service
// ===================================================================

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BackgroundConfig {
    pub workers: u32,
    pub requests_dir: String,
    pub workspace_dir: String,
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

// ===================================================================
// Redis
// ===================================================================

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RedisConfig {
    pub uri: String,
    pub lifetime: u64,
}
