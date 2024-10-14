use crate::DynError;
use log::error;
use sha2::{Digest, Sha256};
use std::fmt::Display;

pub fn emsg<T>(message: T) -> DynError
where
    T: Display,
{
    error!("{}", message);
    format!("{}", message).into()
}

pub fn get_hash(password: &str) -> String {
    let encoded = password.as_bytes();
    let mut hasher = Sha256::new();
    hasher.update(encoded);
    hex::encode(hasher.finalize()).to_lowercase()
}
