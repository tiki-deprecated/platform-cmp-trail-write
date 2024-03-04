/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

mod block_model;
mod model;
mod service;

use std::error::Error;
use chrono::{DateTime, Utc};
use num_bigint::BigInt;
use crate::utils::{byte_helpers, compact_size};

fn current_version() -> i32 { 1 }

#[derive(Debug)]
pub struct Block {
    id: Option<String>,
    version: i32,
    timestamp: DateTime<Utc>,
    previous_hash: String,
    transaction_root: String
}

#[allow(unused)]
impl Block {
    pub fn new(previous_hash: &str, transaction_root: &str) -> Self {
        Block {
            id: None,
            version: current_version(),
            timestamp: Utc::now(),
            previous_hash: previous_hash.to_string(),
            transaction_root: transaction_root.to_string()
        }
    }

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

    pub fn deserialize(bytes: &Vec<u8>) -> Result<Self, Box<dyn Error>> {
        let id = byte_helpers::sha3(bytes);
        let id = Some(byte_helpers::base64_encode(&id));
        let decoded = compact_size::decode(bytes);
        let version = byte_helpers::decode_bigint(&decoded[0]);
        let version = version.to_string().parse::<i32>()?;
        let timestamp = byte_helpers::decode_bigint(&decoded[1]);
        let timestamp =
            DateTime::from_timestamp(timestamp.to_string().parse::<i64>()?, 0)
                .ok_or("Failed to parse timestamp")?;
        let previous_hash = byte_helpers::base64_encode(&decoded[2]);
        let transaction_root = byte_helpers::base64_encode(&decoded[3]);
        Ok(Block { id, version, timestamp, previous_hash, transaction_root })
    }

    pub fn set_id_from_bytes(&mut self, bytes: &Vec<u8>) -> () {
        let id = byte_helpers::sha3(bytes);
        self.id = Some(byte_helpers::base64_encode(&id));
    }

    pub fn set_id(&mut self) -> Result<(), Box<dyn Error>> {
        let bytes = self.serialize()?;
        self.set_id_from_bytes(&bytes);
        Ok(())
    }

    pub fn id(&self) -> &Option<String> { &self.id }
    pub fn version(&self) -> i32 { self.version }
    pub fn timestamp(&self) -> DateTime<Utc> { self.timestamp }
    pub fn previous_hash(&self) -> &str { &self.previous_hash }
    pub fn transaction_root(&self) -> &str { &self.transaction_root }
}

#[cfg(test)]
mod tests {
    use tokio_test::assert_ok;
    use crate::service::block::Block;
    use crate::utils::byte_helpers;
    fn mock_block() -> Block {
        Block::new(
            &byte_helpers::base64_encode(&byte_helpers::utf8_encode("DUMMY PREVIOUS HASH")),
            &byte_helpers::base64_encode(&byte_helpers::utf8_encode("DUMMY TRANSACTION ROOT"))
        )
    }

    #[test]
    fn serialize() {
        let block = mock_block();
        let res = block.serialize();
        assert_ok!(&res);
    }

    #[test]
    fn compare_id() {
        let mut block = mock_block();
        let res = block.serialize();
        block.set_id_from_bytes(&res.unwrap());
        assert_eq!(true, block.id.is_some());

        let id = block.id.clone().unwrap();
        let res = block.set_id();
        assert_ok!(res);
        assert_eq!(id, block.id.unwrap());
    }

    #[test]
    fn deserialize() {
        let block = mock_block();
        let serialized = block.serialize().unwrap();
        let res = Block::deserialize(&serialized);

        assert_ok!(&res);
        let res = res.unwrap();
        assert_eq!(true, res.id.is_some());
        assert_eq!(block.version, res.version);
        assert_eq!(block.timestamp.timestamp(), res.timestamp.timestamp());
        assert_eq!(block.previous_hash, res.previous_hash);
        assert_eq!(block.transaction_root, res.transaction_root);
    }
}
