use sha2::{Digest, Sha256};

pub mod base35;
pub mod genid;
pub mod threadid;

pub fn get_hash(password: &str) -> String {
    let encoded = password.as_bytes();
    let mut hasher = Sha256::new();
    hasher.update(encoded);
    hex::encode(hasher.finalize()).to_lowercase()
}
