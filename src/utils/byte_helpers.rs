/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use num_bigint::BigInt;
use std::error::Error;
use base64::{engine::general_purpose, Engine as _};

/// Encode a BigInt into a byte array using big-endian two's-complement
pub fn encode_bigint(num: &BigInt) -> Vec<u8> { num.to_signed_bytes_be() }

/// Decode a big-endian two's-complement byte array into a BigInt
pub fn decode_bigint(bytes: &Vec<u8>) -> BigInt { BigInt::from_signed_bytes_be(bytes.as_slice()) }

/// Encodes a byte array as a hex string
pub fn hex_encode(bytes: Vec<u8>) -> String { format!("{:X?}", bytes) }

/// Encodes a byte array as a url encoded base64 string without padding
pub fn base64_url_encode(bytes: Vec<u8>) -> String {
    general_purpose::URL_SAFE_NO_PAD.encode(&bytes)
}

/// Decodes a url encoded base64 string without padding
pub fn base64_url_decode(string: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    Ok(general_purpose::URL_SAFE_NO_PAD.decode(string)?)
}

/// Encodes a UTF8 string as a byte array
pub fn utf8_encode(string: &str) -> Vec<u8> { string.as_bytes().to_vec() }

/// Decodes a UTF8 byte array
pub fn utf8_decode(bytes: Vec<u8>) -> Result<String, Box<dyn Error>> {
    Ok(String::from_utf8(bytes)?)
}
