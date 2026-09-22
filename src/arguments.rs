use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "mcrypt",
    version = "0.2",
    about = "A simple file encryption and decryption utility",
    long_about = "\
mcrypt is a simple command-line utility for encrypting and decrypting files.

Files are encrypted using a password and can be restored with the same password.

Use 'mcrypt encrypt --help' or 'mcrypt decrypt --help'
for more information about a specific command."
)]
pub struct Cli {
    /// Hide password input feedback
    #[arg(short, long)]
    pub silent: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Encrypt a file
    Encrypt {
        /// Path to the input file
        input: PathBuf,

        /// Path to the output file
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// Decrypt a file
    Decrypt {
        /// Path to the encrypted file
        input: PathBuf,

        /// Path to the output file
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
}
