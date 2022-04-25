// use std::fmt::Write;
// use std::num::ParseIntError;
use crate::simulate_std::prelude::*;
use core::fmt::Write;
use core::num::ParseIntError;

pub fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

pub fn encode_hex(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        write!(s, "{:02x}", b).unwrap();
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode() {
        let h = decode_hex("04EBCA94D733E399B2DB96EACDD3F69A8BB0F74224E2B44E3357812211D2E62EFBC91BB553098E25E33A799ADC7F76FEB208DA7C6522CDB0719A305180CC54A82E").unwrap();
        assert_eq!(
            h,
            vec![
                4, 235, 202, 148, 215, 51, 227, 153, 178, 219, 150, 234, 205, 211, 246, 154, 139,
                176, 247, 66, 36, 226, 180, 78, 51, 87, 129, 34, 17, 210, 230, 46, 251, 201, 27,
                181, 83, 9, 142, 37, 227, 58, 121, 154, 220, 127, 118, 254, 178, 8, 218, 124, 101,
                34, 205, 176, 113, 154, 48, 81, 128, 204, 84, 168, 46
            ]
        )
    }

    #[test]
    fn test_encode() {
        let h = encode_hex(&[1, 2, 3]);
        assert_eq!(h, String::from("010203"))
    }
}
