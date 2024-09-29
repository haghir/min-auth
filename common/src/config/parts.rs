use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ExposeConfig {
    addrs: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SecurityConfig {
    pub password_salt: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FsConfig {
    pub users: String,
    pub requests: String,
}
