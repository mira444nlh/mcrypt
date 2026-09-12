use crate::error::ConfigError;
use std::env;
use std::path::PathBuf;
pub enum Action {
    Encrypt,
    Decrypt,
}

pub struct Config {
    action: Action,
    input: PathBuf,
    output: PathBuf,
}

fn get_action_from_str(action: &str) -> Result<Action, ConfigError> {
    if action == "encrypt" {
        Ok(Action::Encrypt)
    } else if action == "decrypt" {
        Ok(Action::Decrypt)
    } else {
        Err(ConfigError::InvalidAction)
    }
}

impl Config {
    pub fn action(&self) -> &Action {
        &self.action
    }

    pub fn input(&self) -> &PathBuf {
        &self.input
    }

    pub fn output(&self) -> &PathBuf {
        &self.output
    }

    pub fn parse() -> Result<Config, ConfigError> {
        let args: Vec<String> = env::args().collect();
        if args.len() < 4 {
            return Err(ConfigError::MissingArgument);
        }

        let action = get_action_from_str(&args[1])?;
        let input: PathBuf = args[2].clone().into();
        let output: PathBuf = args[3].clone().into();

        Ok(Config { action, input, output })
    }
}
