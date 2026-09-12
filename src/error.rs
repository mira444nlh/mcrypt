#[derive(Debug)]
pub enum CryptoError {
    InvalidKey,
    DecryptionFailed,
    EncryptionFailed,
    InvalidFileFormat,
    KeyDerivationFailed,
}

#[derive(Debug)]
pub enum ConfigError {
    MissingArgument,
    InvalidAction,
}

#[derive(Debug)]
pub enum McrError {
    Io(std::io::Error),
    Crypto(CryptoError),
    Config(ConfigError),
}

impl From<std::io::Error> for McrError {
    fn from(error: std::io::Error) -> Self {
        McrError::Io(error)
    }
}

impl From<CryptoError> for McrError {
    fn from(error: CryptoError) -> Self {
        McrError::Crypto(error)
    }
}

impl From<ConfigError> for McrError {
    fn from(error: ConfigError) -> Self {
        McrError::Config(error)
    }
}
