mod config;
mod crypto;
mod error;
mod file;
mod password;

use error::McrError;

fn main() -> Result<(), McrError> {
    let cfg = config::Config::parse()?;
    let data = file::read(&cfg.input())?;
    let password = password::get_password()?;

    let result = match cfg.action() {
        config::Action::Encrypt => crypto::encrypt(&data, &password)?,
        config::Action::Decrypt => crypto::decrypt(&data, &password)?,
    };

    file::write(&cfg.output(), &result)?;
    Ok(())
}
