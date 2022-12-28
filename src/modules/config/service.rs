use std::path::Path;

use anyhow::{Ok, Result};
use serde::{Deserialize, Serialize};
use tokio::{fs, io::AsyncWriteExt};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub paper_version: String,
    pub paper_build: u32,
    pub jvm_args: Vec<String>,
}

pub struct ConfigService;

impl ConfigService {
    pub async fn load(dir: &Path) -> Result<Option<AppConfig>> {
        let path = dir.join("Config.toml");
        let config = if !path.exists() {
            None
        } else {
            let file_str = &fs::read_to_string(path).await?;
            let config = toml::from_str(file_str)?;
            Some(config)
        };
        Ok(config)
    }

    pub async fn save(dir: &Path, config: &AppConfig) -> Result<()> {
        let path = dir.join("Config.toml");
        let mut file = if path.exists() {
            fs::File::open(path).await?
        } else {
            fs::File::create(path).await?
        };
        let toml = toml::to_string_pretty(config)?;
        file.write_all(toml.as_bytes()).await?;

        Ok(())
    }
}
