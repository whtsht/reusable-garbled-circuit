use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes128Gcm, Nonce};
use anyhow::{anyhow, Result};

use crate::util::generate_os_rand;

pub fn encrypt(key: &[u8], plaintext: &[u8]) -> Result<(Vec<u8>, Vec<u8>)> {
    // Create cipher instance
    let cipher = Aes128Gcm::new_from_slice(key).map_err(|_| anyhow!("Invalid key length"))?;

    // Generate a random nonce
    let nonce = generate_os_rand(96);

    // Perform encryption
    let ciphertext = cipher
        .encrypt(Nonce::from_slice(&nonce), plaintext.as_ref())
        .map_err(|e| anyhow!("Encryption failed: {:?}", e))?;

    Ok((nonce.to_vec(), ciphertext))
}

pub fn double_encrypt(
    key1: &[u8],
    key2: &[u8],
    plaintext: &[u8],
) -> Result<((Vec<u8>, Vec<u8>), Vec<u8>)> {
    let (nonce1, ciphertext1) = encrypt(key1, plaintext)?;
    let (nonce2, ciphertext2) = encrypt(key2, &ciphertext1)?;

    Ok(((nonce1.to_vec(), nonce2.to_vec()), ciphertext2))
}

pub fn decrypt(key: &[u8], (nonce, ciphertext): (&Vec<u8>, &Vec<u8>)) -> Result<Vec<u8>> {
    // Create cipher instance
    let cipher = Aes128Gcm::new_from_slice(key).map_err(|_| anyhow!("Invalid key length"))?;

    // Perform decryption
    let plaintext = cipher
        .decrypt(Nonce::from_slice(nonce), ciphertext.as_ref())
        .map_err(|e| anyhow!("Encryption failed: {:?}", e))?;

    Ok(plaintext)
}

pub fn double_decrypt(
    key1: &[u8],
    key2: &[u8],
    ((nonce1, nonce2), ciphertext2): &((Vec<u8>, Vec<u8>), Vec<u8>),
) -> Result<Vec<u8>> {
    let ciphertext1 = decrypt(key2, (nonce2, ciphertext2))?;
    decrypt(key1, (nonce1, &ciphertext1))
}
