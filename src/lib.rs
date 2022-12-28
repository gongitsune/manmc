#![feature(let_chains)]

use std::path::Path;

use anyhow::{Context, Ok, Result};
use clap::Parser;
use cli::{Cli, SubCommands};
use modules::{
    config::service::ConfigService,
    paper::service::{DownloadConfig, PaperService},
    process::service::ProcessService,
};

mod cli;
mod modules;

pub async fn cli() -> Result<()> {
    let cli = Cli::parse();
    let dir = &cli.dir.unwrap_or(Path::new("./").to_path_buf());
    let config = ConfigService::load(dir).await?;

    let client = reqwest::Client::new();

    match cli.sub_command {
        SubCommands::Setup { version, build } => {
            let config = {
                if let Some(version) = version && let Some(build) = build {
                    Some(DownloadConfig {
                        version,
                        build,
                    })
                } else {
                    None
                }
            };

            println!("Downloading paper files...");
            PaperService::download(config, dir, &client).await?;
            println!("Download complete!!");
            PaperService::generate_eula(dir).await?;
        }
        SubCommands::Start {} => {
            ProcessService::start_server(
                dir,
                config.context(format!(
                    "{}\n{}",
                    "This directory has not been initialized.",
                    "Please run the setup command first."
                ))?,
            )
            .await?;
        }
    }

    Ok(())
}
