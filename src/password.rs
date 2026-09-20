use colored::Colorize;
use rpassword::ConfigBuilder;
use std::io;

pub fn get_password(prompt: &str, hide_password: bool) -> io::Result<Vec<u8>> {
    let config;
    if hide_password {
        config = ConfigBuilder::new().password_feedback_hide().build();
    } else {
        config = ConfigBuilder::new().password_feedback_mask('*').build();
    }
    let input = rpassword::prompt_password_with_config(prompt.bold(), config)?;
    Ok(input.trim().as_bytes().to_vec())
}
