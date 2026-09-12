use crate::error::CryptoError;
use argon2::Argon2;
use chacha20poly1305::{
    KeyInit, XChaCha20Poly1305, XNonce,
    aead::{Aead, Generate},
};
use rand::random;

fn derive_cipher(password: &[u8], salt: &[u8]) -> Result<XChaCha20Poly1305, CryptoError> {
    let mut key = [0u8; 32];
    Argon2::default()
        .hash_password_into(&password, &salt, &mut key)
        .map_err(|_| CryptoError::KeyDerivationFailed)?;

    XChaCha20Poly1305::new_from_slice(&key).map_err(|_| CryptoError::InvalidKey)
}

fn build_mcrypt_packet(salt: &[u8], nonce: &[u8], ciphertext: &[u8]) -> Vec<u8> {
    let mut complete_text: Vec<u8> = String::from("MCRYPT").into_bytes(); // 6 bytes
    complete_text.extend(salt); // 16 bytes
    complete_text.extend(nonce); // 24 bytes
    complete_text.extend(ciphertext);
    complete_text
}

pub fn encrypt(data: &[u8], password: &[u8]) -> Result<Vec<u8>, CryptoError> {
    let salt: [u8; 16] = random();
    let nonce = XNonce::generate();

    let cipher = derive_cipher(&password, &salt)?;
    let ciphertext = cipher
        .encrypt(&nonce, data.as_ref())
        .map_err(|_| CryptoError::EncryptionFailed)?;
    let cipherblob = build_mcrypt_packet(&salt, &nonce, &ciphertext);

    Ok(cipherblob)
}

pub fn decrypt(data: &[u8], password: &[u8]) -> Result<Vec<u8>, CryptoError> {
    if data.len() < 48 {
        return Err(CryptoError::InvalidFileFormat);
    }
    let salt = &data[6..22]; // 16 bytes
    let nonce = &data[22..46] // 24 bytes
        .try_into()
        .map_err(|_| CryptoError::InvalidFileFormat)?;

    let ciphertext = &data[46..];

    let cipher = derive_cipher(password, salt)?;
    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|_| CryptoError::DecryptionFailed)?;

    Ok(plaintext)
}
