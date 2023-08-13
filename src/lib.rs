use serde::{Serialize, Deserialize};
use serde_json::json;
use sha2::{Sha256, Digest};
use toml::{de::from_str, ser::to_string};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MinAuthConfig {
    pub expose: String,
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
    pub lifetime: usize,
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

#[derive(Serialize, Deserialize)]
pub struct Credential {
    pub id: String,
    pub salt: String,
    pub pwhash: String,
}

impl Credential {
    pub fn verify(&self, secret: &String, password: &str) -> bool {
        let plain = format!("{}{}{}", secret, self.salt, password);
        let pwhash = get_hash(plain.as_str());
        plain.eq_ignore_ascii_case(&pwhash)
    }
}

impl TryFrom<&str> for Credential {
    type Error = serde_json::error::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        serde_json::from_str(value)
    }
}

impl TryFrom<&String> for Credential {
    type Error = serde_json::error::Error;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        value.as_str().try_into()
    }
}

impl From<&Credential> for String {
    fn from(value: &Credential) -> Self {
        json!(value).to_string()
    }
}

pub fn get_hash(password: &str) -> String {
    let encoded = password.as_bytes();
    let mut hasher = Sha256::new();
    hasher.update(encoded);
    hex::encode(hasher.finalize()).to_lowercase()
}
