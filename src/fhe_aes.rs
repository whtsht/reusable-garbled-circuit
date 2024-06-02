use crate::fhe_gf::{self, mult};

#[derive(Debug, PartialEq, Eq)]
pub struct State {
    // TODO: convert FHE value
    inner: [u8; 16],
}

impl State {
    pub fn new() -> Self {
        Self { inner: [0; 16] }
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

impl From<[u8; 16]> for State {
    fn from(inner: [u8; 16]) -> Self {
        Self { inner }
    }
}

impl From<State> for [u8; 16] {
    fn from(value: State) -> Self {
        value.inner
    }
}

fn s_box(b: u8) -> u8 {
    let inv_b = if b == 0 { 0 } else { fhe_gf::inv(b) };
    inv_b
        ^ inv_b.rotate_left(1)
        ^ inv_b.rotate_left(2)
        ^ inv_b.rotate_left(3)
        ^ inv_b.rotate_left(4)
        ^ 0x63
}

fn inv_s_box(s: u8) -> u8 {
    let inv_b = s.rotate_left(1) ^ s.rotate_left(3) ^ s.rotate_left(6) ^ 0x05;

    if inv_b == 0 {
        0
    } else {
        fhe_gf::inv(inv_b)
    }
}

impl State {
    pub fn sub_bytes(&mut self) {
        for b in self.inner.iter_mut() {
            *b = s_box(*b);
        }
    }

    pub fn inv_sub_bytes(&mut self) {
        for b in self.inner.iter_mut() {
            *b = inv_s_box(*b);
        }
    }

    pub fn shift_rows(&mut self) {
        let mut out = [0; 16];

        out[0] = self.inner[0];
        out[4] = self.inner[4];
        out[8] = self.inner[8];
        out[12] = self.inner[12];

        out[1] = self.inner[5];
        out[5] = self.inner[9];
        out[9] = self.inner[13];
        out[13] = self.inner[1];

        out[2] = self.inner[10];
        out[6] = self.inner[14];
        out[10] = self.inner[2];
        out[14] = self.inner[6];

        out[3] = self.inner[15];
        out[7] = self.inner[3];
        out[11] = self.inner[7];
        out[15] = self.inner[11];

        self.inner = out;
    }

    pub fn inv_shift_rows(&mut self) {
        let mut out = [0; 16];

        out[0] = self.inner[0];
        out[4] = self.inner[4];
        out[8] = self.inner[8];
        out[12] = self.inner[12];

        out[1] = self.inner[13];
        out[5] = self.inner[1];
        out[9] = self.inner[5];
        out[13] = self.inner[9];

        out[2] = self.inner[10];
        out[6] = self.inner[14];
        out[10] = self.inner[2];
        out[14] = self.inner[6];

        out[3] = self.inner[7];
        out[7] = self.inner[11];
        out[11] = self.inner[15];
        out[15] = self.inner[3];

        self.inner = out;
    }

    pub fn mix_columns(&mut self) {
        for i in 0..4 {
            let s0 = self.inner[i * 4];
            let s1 = self.inner[i * 4 + 1];
            let s2 = self.inner[i * 4 + 2];
            let s3 = self.inner[i * 4 + 3];

            self.inner[i * 4] = mult(0x02, s0) ^ mult(0x03, s1) ^ s2 ^ s3;
            self.inner[i * 4 + 1] = s0 ^ mult(0x02, s1) ^ mult(0x03, s2) ^ s3;
            self.inner[i * 4 + 2] = s0 ^ s1 ^ mult(0x02, s2) ^ mult(0x03, s3);
            self.inner[i * 4 + 3] = mult(0x03, s0) ^ s1 ^ s2 ^ mult(0x02, s3);
        }
    }

    pub fn inv_mix_columns(&mut self) {
        for i in 0..4 {
            let s0 = self.inner[i * 4];
            let s1 = self.inner[i * 4 + 1];
            let s2 = self.inner[i * 4 + 2];
            let s3 = self.inner[i * 4 + 3];

            self.inner[i * 4] = mult(0x0e, s0) ^ mult(0x0b, s1) ^ mult(0x0d, s2) ^ mult(0x09, s3);
            self.inner[i * 4 + 1] =
                mult(0x09, s0) ^ mult(0x0e, s1) ^ mult(0x0b, s2) ^ mult(0x0d, s3);
            self.inner[i * 4 + 2] =
                mult(0x0d, s0) ^ mult(0x09, s1) ^ mult(0x0e, s2) ^ mult(0x0b, s3);
            self.inner[i * 4 + 3] =
                mult(0x0b, s0) ^ mult(0x0d, s1) ^ mult(0x09, s2) ^ mult(0x0e, s3);
        }
    }

    pub fn add_round_key(&mut self, w: &[u8]) {
        for c in 0..4 {
            self.inner[c * 4] ^= w[c * 4];
            self.inner[c * 4 + 1] ^= w[c * 4 + 1];
            self.inner[c * 4 + 2] ^= w[c * 4 + 2];
            self.inner[c * 4 + 3] ^= w[c * 4 + 3];
        }
    }
}

pub fn sub_word(word: &mut [u8]) {
    word[0] = s_box(word[0]);
    word[1] = s_box(word[1]);
    word[2] = s_box(word[2]);
    word[3] = s_box(word[3]);
}

pub fn xor_word(a: &[u8], b: &[u8]) -> [u8; 4] {
    [a[0] ^ b[0], a[1] ^ b[1], a[2] ^ b[2], a[3] ^ b[3]]
}

const RCON: [[u8; 4]; 10] = [
    [0x01, 0x00, 0x00, 0x00],
    [0x02, 0x00, 0x00, 0x00],
    [0x04, 0x00, 0x00, 0x00],
    [0x08, 0x00, 0x00, 0x00],
    [0x10, 0x00, 0x00, 0x00],
    [0x20, 0x00, 0x00, 0x00],
    [0x40, 0x00, 0x00, 0x00],
    [0x80, 0x00, 0x00, 0x00],
    [0x1b, 0x00, 0x00, 0x00],
    [0x36, 0x00, 0x00, 0x00],
];

#[allow(clippy::manual_memcpy)]
pub fn key_expansion(key: &[u8], nk: usize, nr: usize) -> Vec<u8> {
    let mut w = vec![0; ((nr + 1) * 4) * 4];
    for i in 0..nk * 4 {
        w[i] = key[i];
    }

    let mut i = nk * 4;
    while i < ((nr + 1) * 4) * 4 {
        let mut temp = w[(i - 4)..=(i - 4) + 3].to_vec();
        if (i / 4) % nk == 0 {
            temp.rotate_left(1);
            sub_word(&mut temp);
            let xor_temp_rcon = xor_word(&temp, &RCON[((i / 4) - 1) / nk]);
            temp.copy_from_slice(&xor_temp_rcon);
        } else if nk > 6 && (i / 4) % nk == 4 {
            sub_word(&mut temp);
        }
        let xor_w_temp = xor_word(&w[i - nk * 4..=i - nk * 4 + 3], &temp);
        w[i..=i + 3].copy_from_slice(&xor_w_temp);
        i += 4;
    }
    w
}

pub fn cipher(input: [u8; 16], nk: usize, nr: usize, key: &Vec<u8>) -> [u8; 16] {
    let w = key_expansion(key, nk, nr);
    let mut state = State::from(input);

    state.add_round_key(&w[0..16]);

    for round in 1..nr {
        state.sub_bytes();
        state.shift_rows();
        state.mix_columns();
        state.add_round_key(&w[16 * round..16 * (round + 1)]);
    }

    state.sub_bytes();
    state.shift_rows();
    state.add_round_key(&w[16 * nr..16 * (nr + 1)]);

    state.into()
}

pub fn inv_cipher(input: [u8; 16], nk: usize, nr: usize, key: &Vec<u8>) -> [u8; 16] {
    let w = key_expansion(key, nk, nr);
    let mut state = State::from(input);

    state.add_round_key(&w[16 * nr..16 * (nr + 1)]);

    for round in (1..nr).rev() {
        state.inv_shift_rows();
        state.inv_sub_bytes();
        state.add_round_key(&w[16 * round..16 * (round + 1)]);
        state.inv_mix_columns();
    }

    state.inv_shift_rows();
    state.inv_sub_bytes();
    state.add_round_key(&w[0..16]);

    state.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_s_box() {
        assert_eq!(s_box(0x53), 0xed);
        assert_eq!(s_box(0x00), 0x63);
        assert_eq!(s_box(0xf0), 0x8c);
        assert_eq!(s_box(0xec), 0xce);
    }

    #[test]
    fn test_cipher_128() {
        let nr = 10;
        let nk = 4;
        let input: [u8; 16] = [
            0x32, 0x43, 0xf6, 0xa8, 0x88, 0x5a, 0x30, 0x8d, 0x31, 0x31, 0x98, 0xa2, 0xe0, 0x37,
            0x07, 0x34,
        ];
        let key: Vec<u8> = vec![
            0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6, 0xab, 0xf7, 0x15, 0x88, 0x09, 0xcf,
            0x4f, 0x3c,
        ];

        let result = cipher(input, nk, nr, &key);
        assert_eq!(
            result,
            [
                0x39, 0x25, 0x84, 0x1d, 0x02, 0xdc, 0x09, 0xfb, 0xdc, 0x11, 0x85, 0x97, 0x19, 0x6a,
                0x0b, 0x32,
            ]
        );
    }

    #[test]
    fn test_inv_shift_rows() {
        let input: [u8; 16] = [
            0x32, 0x43, 0xf6, 0xa8, 0x88, 0x5a, 0x30, 0x8d, 0x31, 0x31, 0x98, 0xa2, 0xe0, 0x37,
            0x07, 0x34,
        ];

        let mut state = State::from(input);
        state.shift_rows();
        state.inv_shift_rows();
        assert_eq!(state, State::from(input));
    }

    #[test]
    fn test_inv_mix_columns() {
        let input: [u8; 16] = [
            0x32, 0x43, 0xf6, 0xa8, 0x88, 0x5a, 0x30, 0x8d, 0x31, 0x31, 0x98, 0xa2, 0xe0, 0x37,
            0x07, 0x34,
        ];

        let mut state = State::from(input);
        state.mix_columns();
        state.inv_mix_columns();
        assert_eq!(state, State::from(input));
    }

    #[test]
    fn test_inv_s_box() {
        assert_eq!(inv_s_box(s_box(0x53)), 0x53);
        assert_eq!(inv_s_box(s_box(0xff)), 0xff);
        assert_eq!(inv_s_box(s_box(0x00)), 0x00);
        assert_eq!(inv_s_box(s_box(0xa4)), 0xa4);
    }

    #[test]
    fn test_inv_cipher() {
        let nr = 10;
        let nk = 4;
        let input: [u8; 16] = [
            0x32, 0x43, 0xf6, 0xa8, 0x88, 0x5a, 0x30, 0x8d, 0x31, 0x31, 0x98, 0xa2, 0xe0, 0x37,
            0x07, 0x34,
        ];
        let key: Vec<u8> = vec![
            0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6, 0xab, 0xf7, 0x15, 0x88, 0x09, 0xcf,
            0x4f, 0x3c,
        ];

        let ciphered = cipher(input, nk, nr, &key);
        let decrypted = inv_cipher(ciphered, nk, nr, &key);

        assert_eq!(input, decrypted);
    }
}
