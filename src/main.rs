use rand::{rngs::OsRng, RngCore};

mod symmetric_key_encryption;
mod util;

use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes256Gcm, Nonce};

pub fn encrypt(key: &[u8], plaintext: &[u8]) -> (Vec<u8>, Vec<u8>) {
    // Create cipher instance
    let cipher = Aes256Gcm::new_from_slice(key).expect("Invalid key length");

    // Generate a random nonce
    let mut nonce = [0u8; 12];
    OsRng.fill_bytes(&mut nonce);

    // Perform encryption
    let ciphertext = cipher
        .encrypt(Nonce::from_slice(&nonce), plaintext.as_ref())
        .expect("Encryption failed");

    (nonce.to_vec(), ciphertext)
}

pub fn decrypt(key: &[u8], (nonce, ciphertext): &(Vec<u8>, Vec<u8>)) -> Vec<u8> {
    // Create cipher instance
    let cipher = Aes256Gcm::new_from_slice(key).expect("Invalid key length");

    // Perform decryption
    let plaintext = cipher
        .decrypt(Nonce::from_slice(nonce), ciphertext.as_ref())
        .expect("Decryption failed");

    plaintext
}

#[derive(Debug, PartialEq, Eq)]
pub struct GarbledCircuit2 {
    base_table: [u8; 2],
    garbled_input_label: [Vec<u8>; 2],
    garbled_output_label: [Vec<u8>; 2],
    garbled_table: [(Vec<u8>, Vec<u8>); 2],
}

impl GarbledCircuit2 {
    pub fn new(base_table: [u8; 2]) -> Self {
        let garbled_input_label = [generate_os_rand(32), generate_os_rand(32)];
        let garbled_output_label = [generate_os_rand(32), generate_os_rand(32)];
        let garbled_table = [];
    }
}

pub const NOT_CIRCUIT: [u8; 2] = [1, 0];
// pub const AND_CIRCUIT: [u8; 4] = [0, 0, 0, 1];

fn main() {
    let a = "hello world".as_bytes();
    let key = generate_os_rand(32);
    let b = encrypt(&key, a);
    println!("{:?}", b);

    let c = decrypt(&key, &b);

    println!("{:?}", std::str::from_utf8(&c));
}
