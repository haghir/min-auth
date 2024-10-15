pub mod config;
pub mod data;
pub mod error;
pub mod utils;

pub type DynError = Box<dyn std::error::Error + Send + Sync>;
