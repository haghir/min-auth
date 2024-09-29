use super::parts::{ExposeConfig, FsConfig};
use serde::{Deserialize, Serialize};
use std::{fs::File, io::BufReader, path::Path};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BackendConfig {
    pub expose: ExposeConfig,
    pub file_system: FsConfig,
}

impl BackendConfig {
    pub fn load<P>(path: P) -> Result<Self, std::io::Error>
    where
        P: AsRef<Path>,
    {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        Ok(serde_json::from_reader(reader)?)
    }
}
