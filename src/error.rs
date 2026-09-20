#[derive(Debug)]
pub enum CryptoError {
    InvalidKey,
    DecryptionFailed,
    EncryptionFailed,
    InvalidFileFormat,
    KeyDerivationFailed,
}

#[derive(Debug)]
pub enum McrError {
    Io(std::io::Error),
    Crypto(CryptoError),
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
