pub mod base35;
pub mod config;
pub mod credentials;
pub mod error;
pub mod genid;
pub mod requests;
pub mod threadid;
pub mod users;

pub type Result<T> = std::result::Result<T, error::Error>;
