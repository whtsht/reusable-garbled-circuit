use tfhe::{
    core_crypto::commons::traits::CastFrom,
    prelude::{FheEncrypt, FheEq},
    ClientKey, FheBool, FheUint8,
};

pub struct EncryptedMap {
    inner: Vec<FheUint8>,
}

impl EncryptedMap {
    fn new(key: &ClientKey) -> Self {
        let mut encrypted_values = Vec::with_capacity(256);
        for val in 0..=255u8 {
            let encrypted_val = FheUint8::encrypt(val, key);
            encrypted_values.push(encrypted_val);
        }

        Self {
            inner: encrypted_values,
        }
    }

    pub fn get(&self, value: u8) -> &FheUint8 {
        &self.inner[value as usize]
    }
}

fn fhe_if(cond: FheBool, then: FheUint8, else_: FheUint8, map: &EncryptedMap) -> FheUint8 {
    let cond = FheUint8::cast_from(cond);

    &cond * then + (map.get(1) - &cond) * else_
}

fn xtimes(x: FheUint8, map: &EncryptedMap) -> FheUint8 {
    // if x & 0x80 == 0 {
    //     x << 1
    // } else {
    //     (x << 1) ^ 0x1b
    // }
    let cond = (&x & map.get(0x80)).eq(map.get(0x00));
    fhe_if(
        cond,
        &x << map.get(0x01),
        (&x << map.get(0x01)) ^ map.get(0x1b),
        map,
    )
}

pub fn mult(a: FheUint8, b: FheUint8, map: &EncryptedMap) -> FheUint8 {
    // let mut result = 0;
    // let mut temp = a;
    // for i in 0..8 {
    //     if b & (1 << i) != 0 {
    //         result ^= temp;
    //     }
    //     temp = xtimes(temp);
    // }
    // result
    let mut result = map.get(0x00).clone();
    let mut temp = a;
    for i in 0..8 {
        result = fhe_if(
            (&b & (map.get(1) << map.get(i))).ne(map.get(0)),
            &result ^ &temp,
            result,
            map,
        );
        temp = xtimes(temp, map);
    }
    result
}
//
// pub fn pow(base: u8, exponent: u8) -> u8 {
//     let mut result = 1;
//     let mut temp = base;
//     for _ in 0..exponent {
//         result = mult(result, temp);
//     }
//     result
// }
//
// pub fn inv(a: u8) -> u8 {
//     let t1 = a;
//     let t2 = mult(t1, t1);
//     let t3 = mult(t2, t2);
//     let t4 = mult(t3, t3);
//     let t5 = mult(t4, t4);
//     let t6 = mult(t5, t5);
//     let t7 = mult(t6, t6);
//     let t8 = mult(t7, t7);
//
//     mult(mult(mult(mult(mult(mult(t8, t7), t6), t5), t4), t3), t2)
// }
//
#[cfg(test)]
mod tests {
    use super::*;

    use tfhe::prelude::*;
    use tfhe::{generate_keys, set_server_key, ConfigBuilder, FheUint32, FheUint8};

    #[test]
    fn test_gf_mult() {
        // Basic configuration to use homomorphic integers
        let config = ConfigBuilder::default().build();

        // Key generation
        let (client_key, server_key) = generate_keys(config);

        let map = EncryptedMap::new(&client_key);

        set_server_key(server_key);
        let result: u8 = mult(
            FheUint8::encrypt(0x57u8, &client_key),
            FheUint8::encrypt(0x01u8, &client_key),
            &map,
        )
        .decrypt(&client_key);
        assert_eq!(result, 0x57);
        // assert_eq!(mult(0x57, 0x01), 0x57);
        // assert_eq!(mult(0x57, 0x02), 0xae);
        // assert_eq!(mult(0x57, 0x04), 0x47);
        // assert_eq!(mult(0x57, 0x08), 0x8e);
        // assert_eq!(mult(0x57, 0x10), 0x07);
        // assert_eq!(mult(0x57, 0x20), 0x0e);
        // assert_eq!(mult(0x57, 0x40), 0x1c);
        // assert_eq!(mult(0x57, 0x80), 0x38);

        // let b = 0x13;
        // let b_inv = inv(b);
        // assert_eq!(mult(b, b_inv), 0x01);
        //
        // let b = 0x53;
        // let b_inv = inv(b);
        // assert_eq!(mult(b, b_inv), 0x01);
        //
        // let b = 0xaa;
        // let b_inv = inv(b);
        // assert_eq!(mult(b, b_inv), 0x01);
    }
}
