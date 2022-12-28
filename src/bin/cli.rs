use anyhow::{Ok, Result};

#[tokio::main]
async fn main() -> Result<()> {
    manmc::cli().await?;

    Ok(())
}
