use thiserror::Error;

#[derive(Error, Debug)]
pub enum CryptoError {
    #[error("Invalid key")]
    InvalidKey,

    #[error("Incorrect password")]
    DecryptionFailed,

    #[error("Encryption failed")]
    EncryptionFailed,

    #[error("Invalid file format")]
    InvalidFileFormat,

    #[error("Key derivation failed")]
    KeyDerivationFailed,
}

#[derive(Error, Debug)]
pub enum McrError {
    #[error("Input-output error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Cryptographic error: {0}")]
    Crypto(#[from] CryptoError),
}
