use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    arg_required_else_help = true,
)]
pub struct Cli {
    #[clap(subcommand)]
    pub sub_command: SubCommands,
    #[clap()]
    pub dir: Option<PathBuf>,
}

#[derive(Debug, Subcommand)]
pub enum SubCommands {
    /// Setup your minecraft server environment
    #[clap()]
    Setup {
        #[clap(short = 'v')]
        version: Option<String>,
        #[clap(short = 'b')]
        build: Option<u32>,
    },
    #[clap()]
    Start {},
}
