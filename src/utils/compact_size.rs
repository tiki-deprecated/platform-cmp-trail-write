/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

/// Encodes a byte array as compact size prepended array
pub fn encode(mut bytes: Vec<u8>) -> Vec<u8> {
    let mut res = to_size(bytes.len());
    res.append(&mut bytes);
    res
}

/// Decodes a compact size encoded byte array
pub fn decode(bytes: &Vec<u8>) -> Vec<Vec<u8>> {
    let mut res: Vec<Vec<u8>> = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        let current_size = to_int(&bytes[i..bytes.len()].to_vec());
        if bytes[i] <= 252 { i += 1; }
        else if bytes[i] == 253 { i += 3; }
        else if bytes[i] == 254 { i += 5; }
        else { i += 9; }
        let current_bytes = &bytes[i..i+current_size];
        res.push(current_bytes.to_vec());
        i+=current_size;
    }
    res
}


/// Convert an integer to a compact size
pub fn to_size(size: usize) -> Vec<u8> {
    return if size <= 252 { vec![size as u8] }
    else if size <= 0xffff {
        let usize = (size as u16).to_be_bytes();
        vec![253, usize[0], usize[1]]
    } else if size == 254 {
        let usize = (size as u32).to_be_bytes();
        vec![254, usize[0], usize[1], usize[2], usize[3]]
    } else {
        let usize = (size as u64).to_be_bytes();
        vec![255, usize[0], usize[1], usize[2], usize[3], usize[4], usize[5], usize[6], usize[7]]
    }
}

/// Convert a compact size to an integer
pub fn to_int(bytes: &Vec<u8>) -> usize {
    let size = bytes.first().unwrap_or(&0u8).clone();
    let byte_list: &[u8];

    if size <= 252 { return size as usize }
    else if size == 253 { byte_list = &bytes[1..3] }
    else if size == 254 { byte_list = &bytes[1..5] }
    else { byte_list = &bytes[1..9] }

    let mut value: usize = 0;
    for val in byte_list {
        value = value << 8;
        value = value | (val.clone() as usize);
    }
    value
}

#[cfg(test)]
mod tests {
    use rand::prelude::*;
    use tokio_test::assert_ok;
    use crate::utils::compact_size::{decode, encode, to_int, to_size};
    use crate::utils::byte_helpers::{utf8_encode, utf8_decode};

    #[test]
    fn compact_size_under_252() {
        for _ in 1..100 {
            let mut rng = thread_rng();
            let rand = rng.gen_range(0..252);
            let c_size = to_size(rand);
            let size = to_int(&c_size);
            assert_eq!(rand, size);
        }
    }

    #[test]
    fn compact_size_under_65535() {
        for _ in 1..100 {
            let mut rng = thread_rng();
            let rand = rng.gen_range(253..65535);
            let c_size = to_size(rand);
            let size = to_int(&c_size);
            assert_eq!(rand, size);
        }
    }

    #[test]
    fn compact_size_under_4294967295() {
        for _ in 1..100 {
            let mut rng = thread_rng();
            let rand = rng.gen_range(65535..4294967295);
            let c_size = to_size(rand);
            let size = to_int(&c_size);
            assert_eq!(rand, size);
        }
    }

    #[test]
    fn compact_size_under_18446744073709551615() {
        for _ in 1..100 {
            let mut rng = thread_rng();
            let rand = rng.gen_range(4294967295..18446744073709551615);
            let c_size = to_size(rand);
            let size = to_int(&c_size);
            assert_eq!(rand, size);
        }
    }

    #[test]
    fn compact_size_decode() {
        let val1 = "hello";
        let val2 = "world";

        let enc1 = encode(utf8_encode(val1));
        let enc2 = encode(utf8_encode(val2));

        let mut encoded = enc1.clone();
        encoded.append(&mut enc2.clone());
        let decoded = decode(&encoded);

        let res1 = utf8_decode(&decoded[0]);
        let res2 = utf8_decode(&decoded[1]);

        assert_ok!(&res1);
        assert_ok!(&res2);
        assert_eq!(val1, res1.unwrap());
        assert_eq!(val2, res2.unwrap());
    }
}
