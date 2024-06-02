fn xtimes(x: u8) -> u8 {
    if x & 0x80 == 0 {
        x << 1
    } else {
        (x << 1) ^ 0x1b
    }
}

pub fn mult(a: u8, b: u8) -> u8 {
    let mut result = 0;
    let mut temp = a;
    for i in 0..8 {
        if b & (1 << i) != 0 {
            result ^= temp;
        }
        temp = xtimes(temp);
    }
    result
}

pub fn pow(base: u8, exponent: u8) -> u8 {
    let mut result = 1;
    let mut temp = base;
    for _ in 0..exponent {
        result = mult(result, temp);
    }
    result
}

pub fn inv(a: u8) -> u8 {
    let t1 = a;
    let t2 = mult(t1, t1);
    let t3 = mult(t2, t2);
    let t4 = mult(t3, t3);
    let t5 = mult(t4, t4);
    let t6 = mult(t5, t5);
    let t7 = mult(t6, t6);
    let t8 = mult(t7, t7);

    mult(mult(mult(mult(mult(mult(t8, t7), t6), t5), t4), t3), t2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gf_mult() {
        assert_eq!(mult(0x57, 0x13), 0xfe);
        assert_eq!(mult(0x57, 0x01), 0x57);
        assert_eq!(mult(0x57, 0x02), 0xae);
        assert_eq!(mult(0x57, 0x04), 0x47);
        assert_eq!(mult(0x57, 0x08), 0x8e);
        assert_eq!(mult(0x57, 0x10), 0x07);
        assert_eq!(mult(0x57, 0x20), 0x0e);
        assert_eq!(mult(0x57, 0x40), 0x1c);
        assert_eq!(mult(0x57, 0x80), 0x38);

        let b = 0x13;
        let b_inv = inv(b);
        assert_eq!(mult(b, b_inv), 0x01);

        let b = 0x53;
        let b_inv = inv(b);
        assert_eq!(mult(b, b_inv), 0x01);

        let b = 0xaa;
        let b_inv = inv(b);
        assert_eq!(mult(b, b_inv), 0x01);
    }
}
