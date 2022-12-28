use futures::prelude::*;
use std::{path::Path, process::Stdio};

use anyhow::{Context, Ok, Result};
use tokio::process::Command;
use tokio_util::codec::{FramedRead, LinesCodec};

use crate::modules::config::service::AppConfig;

pub struct ProcessService;

impl ProcessService {
    pub async fn start_server(dir: &Path, config: AppConfig) -> Result<()> {
        let mut child = Command::new("java")
            .kill_on_drop(true)
            .current_dir(dir)
            .args(config.jvm_args)
            .arg("-jar")
            .arg("paper.jar")
            .arg("--nogui")
            .stdout(Stdio::piped())
            .spawn()?;
        let stdout = child.stdout.take().context("Can't get stdout.")?;
        let mut reader = FramedRead::new(stdout, LinesCodec::new());

        while let Some(line) = reader.next().await {
            println!("{}", line?);
        }

        Ok(())
    }
}
