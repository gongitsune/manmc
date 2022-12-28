use anyhow::{Ok, Result};
use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ProjectControllerResponse {
    pub versions: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct VersionControllerResponse {
    pub builds: Vec<u32>,
}

pub struct PaperRepository;

impl PaperRepository {
    pub async fn get_versions(client: &Client) -> Result<ProjectControllerResponse> {
        let res = client
            .get("https://api.papermc.io/v2/projects/paper")
            .send()
            .await?
            .json::<ProjectControllerResponse>()
            .await?;
        Ok(res)
    }

    pub async fn get_builds(version: &str, client: &Client) -> Result<VersionControllerResponse> {
        let res = client
            .get(format!(
                "https://api.papermc.io/v2/projects/paper/versions/{}",
                version
            ))
            .send()
            .await?
            .json::<VersionControllerResponse>()
            .await?;
        Ok(res)
    }
}
