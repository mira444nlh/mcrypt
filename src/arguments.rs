use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

// TODO implement getters
#[derive(Parser)]
#[command(name = "mcrypt")]
#[command(version = "0.2")]
#[command(about = "A simple file encryptor", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Encrypt {
        input: String,

        #[arg(short, long)]
        output: Option<String>,
    },
    Decrypt {
        input: String,

        #[arg(short, long)]
        output: Option<String>,
    },
}
