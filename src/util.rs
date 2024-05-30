use rand::{self, rngs::OsRng, RngCore};

pub fn generate_os_rand(length: u32) -> Vec<u8> {
    let mut label = vec![0; length as usize / 8];
    OsRng.fill_bytes(&mut label);
    label
}
