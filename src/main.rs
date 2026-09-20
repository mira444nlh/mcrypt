mod arguments;
mod crypto;
mod error;
mod file;
mod password;

use clap::Parser;
use error::McrError;

fn main() -> Result<(), McrError> {
    let args = arguments::Cli::parse();

    match args.command {
        arguments::Commands::Decrypt { input, output } => {
            let data = file::read(&input)?;
            let password = password::get_password("Password: ", args.silent)?;
            let result = crypto::decrypt(&data, &password)?;
            file::write(&output.unwrap_or(input), &result)?;
        }
        arguments::Commands::Encrypt { input, output } => {
            let data = file::read(&input)?;
            let password = password::get_password("Password: ", args.silent)?;
            let result = crypto::encrypt(&data, &password)?;
            file::write(&output.unwrap_or(input), &result)?;
        }
    }

    Ok(())
}
