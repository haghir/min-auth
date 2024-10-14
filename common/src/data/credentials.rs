use super::users::{AccessControl, AccessControlKind};
use crate::utils::get_hash;
use log::debug;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fmt::Display;

#[derive(Serialize, Deserialize)]
pub struct Credentials {
    pub id: String,
    pub salt: String,
    pub pwhash: String,
    pub acl: Vec<AccessControl>,
}

impl Credentials {
    pub fn verify<S, P>(&self, secret: S, password: P) -> bool
    where
        S: Display,
        P: Display,
    {
        let plain = format!("{}{}{}", secret, self.salt, password);
        let pwhash = get_hash(plain.as_str());
        debug!("registered: {}", self.pwhash);
        debug!("calculated: {}", pwhash);
        self.pwhash.eq_ignore_ascii_case(&pwhash)
    }

    pub fn allowed<S>(&self, service: S) -> bool
    where
        S: Display,
    {
        let service = format!("{}", service);
        for ref access in &self.acl {
            match access.control {
                AccessControlKind::Allow => {
                    if access.service == "*" || access.service == service {
                        return true;
                    }
                }
                AccessControlKind::Deny => {
                    if access.service == "*" || access.service == service {
                        return false;
                    }
                }
            }
        }
        false
    }
}

impl TryFrom<&str> for Credentials {
    type Error = serde_json::error::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        serde_json::from_str(value)
    }
}

impl TryFrom<&String> for Credentials {
    type Error = serde_json::error::Error;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        value.as_str().try_into()
    }
}

impl From<&Credentials> for String {
    fn from(value: &Credentials) -> Self {
        json!(value).to_string()
    }
}
