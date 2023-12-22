/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use std::error::Error;
use chrono::{DateTime, Utc};
use num_bigint::BigInt;
use crate::utils::{byte_helpers, compact_size};

fn current_version() -> i32 { 1 }

#[derive(Debug)]
pub struct BlockModel {
    id: Option<String>,
    version: i32,
    timestamp: DateTime<Utc>,
    previous_hash: String,
    transaction_root: String
}

impl BlockModel {
    pub fn serialize(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut res = Vec::<u8>::new();
        let version = &BigInt::from(self.version);
        res.append(&mut compact_size::encode(byte_helpers::encode_bigint(version)));
        let timestamp = &BigInt::from(self.timestamp.timestamp());
        res.append(&mut compact_size::encode(byte_helpers::encode_bigint(timestamp)));
        let previous_hash = self.previous_hash.as_str();
        res.append(&mut compact_size::encode(byte_helpers::base64_decode(previous_hash)?));
        let transaction_root = self.transaction_root.as_str();
        res.append(&mut compact_size::encode(byte_helpers::base64_decode(transaction_root)?));
        Ok(res)
    }

    pub fn deserialize(bytes: &Vec<u8>, id: String) -> Result<Self, Box<dyn Error>> {
        let decoded = compact_size::decode(bytes);
        let version = byte_helpers::decode_bigint(&decoded[0]);
        let version = version.to_string().parse::<i32>()?;
        let timestamp = byte_helpers::decode_bigint(&decoded[1]);
        let timestamp =
            DateTime::from_timestamp(timestamp.to_string().parse::<i64>()?, 0)
                .ok_or("Failed to parse timestamp")?;
        let previous_hash = byte_helpers::base64_encode(&decoded[2]);
        let transaction_root = byte_helpers::base64_encode(&decoded[3]);
        Ok(BlockModel {
            id: Some(id),
            version,
            timestamp,
            previous_hash,
            transaction_root
        })
    }
}
