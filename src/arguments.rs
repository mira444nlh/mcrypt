use clap::{Parser, Subcommand};
use std::path::PathBuf;

// TODO implement getters
#[derive(Parser)]
#[command(name = "mcrypt")]
#[command(version = "0.2")]
#[command(about = "A simple file encryptor", long_about = None)]
pub struct Cli {
    #[arg(short, long)]
    pub silent: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Encrypt {
        input: PathBuf,

        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    Decrypt {
        input: PathBuf,

        #[arg(short, long)]
        output: Option<PathBuf>,
    },
}
