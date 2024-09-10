use min_auth_common::Result;
use serde::{Deserialize, Serialize};
use std::{fs::File, io::BufReader, path::Path};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub background: BgConfig,
    pub file_system: FsConfig,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DispatcherConfig {
    pub expose: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BgConfig {
    pub workers: usize,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FsConfig {
    pub users: String,
    pub requests: String,
    pub workspace: String,
}

impl Config {
    pub fn load<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        Ok(serde_json::from_reader(reader)?)
    }
}
