use std::path::Path;

use anyhow::{anyhow, Context, Ok, Result};
use reqwest::Client;
use tokio::{
    fs::{self, File},
    io::AsyncWriteExt,
};

use crate::modules::config::service::{AppConfig, ConfigService};

use super::repository::PaperRepository;

pub struct PaperService;

pub struct DownloadConfig {
    pub version: String,
    pub build: u32,
}

impl PaperService {
    pub async fn generate_eula(dir: &Path) -> Result<()> {
        let path = dir.join("eula.txt");
        if !path.exists() {
            let mut file = fs::File::create(path).await?;
            file.write_all(b"eula=true").await?;
        }

        Ok(())
    }

    pub async fn download(
        config: Option<DownloadConfig>,
        dir: &Path,
        client: &Client,
    ) -> Result<()> {
        if dir.join("paper.jar").exists() {
            return Err(anyhow!("`paper.jar` is exists."));
        }

        let config = match config {
            Some(config) => config,
            None => {
                let version = PaperRepository::get_versions(client)
                    .await?
                    .versions
                    .last()
                    .context("Can't get versions.")?
                    .to_owned();
                DownloadConfig {
                    version: version.to_owned(),
                    build: *PaperRepository::get_builds(&version, client)
                        .await?
                        .builds
                        .last()
                        .context("Can't get builds.")?,
                }
            }
        };

        let file_name = format!("paper-{}-{}.jar", config.version, config.build);

        let mut res = client
            .get(format!(
                "https://api.papermc.io/v2/projects/paper/versions/{0}/builds/{1}/downloads/{2}",
                config.version, config.build, file_name
            ))
            .send()
            .await?;
        let mut file = tokio::fs::File::create(dir.join("paper.jar")).await?;
        while let Some(chunk) = res.chunk().await? {
            file.write_all(&chunk).await?;
        }

        ConfigService::save(
            dir,
            &AppConfig {
                paper_version: config.version,
                paper_build: config.build,
                jvm_args: vec![],
            },
        )
        .await?;

        Ok(())
    }
}
