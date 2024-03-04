/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use num_bigint::BigInt;
use std::error::Error;
use std::num::ParseIntError;
use base64::{engine::general_purpose, Engine as _};
use sha3::{Digest, Sha3_256};

/// Encode a BigInt into a byte array using big-endian two's-complement
pub fn encode_bigint(num: &BigInt) -> Vec<u8> { num.to_signed_bytes_be() }

/// Decode a big-endian two's-complement byte array into a BigInt
pub fn decode_bigint(bytes: &Vec<u8>) -> BigInt { BigInt::from_signed_bytes_be(bytes.as_slice()) }

/// Encodes a byte array as a hex string
pub fn hex_encode(bytes: &Vec<u8>) -> String { format!("{:X?}", bytes) }

/// Decodes a hex string as byte array
pub fn decode_hex(s: &str) -> Result<Vec<u8>, Box<dyn Error>> {
   let res: Result<Vec<u8>, ParseIntError> = (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect();
    Ok(res?)
}

/// Encodes a byte array as a base64 string
pub fn base64_encode(bytes: &Vec<u8>) -> String {
    general_purpose::STANDARD.encode(bytes)
}

/// Encodes a byte array as a URL safe base64 string
pub fn base64url_encode(bytes: &Vec<u8>) -> String {
    general_purpose::URL_SAFE_NO_PAD.encode(bytes)
}

/// Decodes a base64 string
pub fn base64_decode(string: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    Ok(general_purpose::STANDARD.decode(string)?)
}

/// Decodes a url encoded base64 string
pub fn base64url_decode(string: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    Ok(general_purpose::URL_SAFE_NO_PAD.decode(string)?)
}

/// Encodes a UTF8 string as a byte array
pub fn utf8_encode(string: &str) -> Vec<u8> { string.as_bytes().to_vec() }

/// Decodes a UTF8 byte array
pub fn utf8_decode(bytes: &Vec<u8>) -> Result<String, Box<dyn Error>> {
    Ok(String::from_utf8(bytes.clone())?)
}

/// Calculate the SHA3-256 hash of a byte array
pub fn sha3(message: &Vec<u8>) -> Vec<u8> {
    let mut hasher = Sha3_256::new();
    hasher.update(message.as_slice());
    hasher.finalize()[..].to_vec()
}
