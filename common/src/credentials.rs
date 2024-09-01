use serde::{Serialize, Deserialize};
use serde_json::json;
use sha2::{Sha256, Digest};
use log::debug;

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
        debug!("registered: {}", self.pwhash);
        debug!("calculated: {}", pwhash);
        self.pwhash.eq_ignore_ascii_case(&pwhash)
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
