/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use std::error::Error;
use chrono::{DateTime, Utc};
use num_bigint::BigInt;
use super::super::{Signer, super::utils::{byte_helpers, compact_size}};

fn current_version() -> i32 { 2 }

#[derive(Debug, Clone)]
pub struct ModelTxn {
    id: String,
    version: i32,
    address: String,
    timestamp: DateTime<Utc>,
    asset_ref: String,
    contents: String,
    user_signature: String,
    app_signature: String,
    bytes: Vec<u8>
}

impl ModelTxn {
    pub fn new(
        address: &str, 
        timestamp: DateTime<Utc>,
        asset_ref: &str, 
        contents: &str,
        user_signature: &str,
        signer: &Signer
    ) -> Result<Self, Box<dyn Error>> {
        let mut bytes = Vec::<u8>::new();
        let version = current_version();
        let version_bigint = &BigInt::from(current_version());
        bytes.append(&mut compact_size::encode(byte_helpers::encode_bigint(version_bigint)));
        bytes.append(&mut compact_size::encode(byte_helpers::base64_decode(address)?));
        let timestamp_bigint = &BigInt::from(timestamp.timestamp());
        bytes.append(&mut compact_size::encode(byte_helpers::encode_bigint(timestamp_bigint)));
        bytes.append(&mut compact_size::encode(byte_helpers::utf8_encode(asset_ref)));
        bytes.append(&mut compact_size::encode(byte_helpers::base64_decode(contents)?));
        bytes.append(&mut compact_size::encode(byte_helpers::base64_decode(user_signature)?));
        let app_signature = signer.sign(&bytes)?;
        bytes.append(&mut compact_size::encode(app_signature.clone()));
        Ok(Self {
            id: Self::calculate_id(&bytes), 
            version,
            address: address.to_string(), 
            timestamp, 
            asset_ref: asset_ref.to_string(), 
            contents: contents.to_string(),
            user_signature: user_signature.to_string(),
            app_signature: byte_helpers::base64_encode(&app_signature),
            bytes
        })
    }

    pub fn read(bytes: &Vec<u8>) -> Result<Self, Box<dyn Error>>  {
        let decoded = compact_size::decode(bytes);
        let version = byte_helpers::decode_bigint(&decoded[0]);
        let version = version.to_string().parse::<i32>()?;
        let address = byte_helpers::base64_encode(&decoded[1]);
        let timestamp = byte_helpers::decode_bigint(&decoded[2]);
        let timestamp = DateTime::from_timestamp(timestamp.to_string().parse::<i64>()?, 0)
                .ok_or("Failed to parse timestamp")?;
        let asset_ref = byte_helpers::utf8_decode(&decoded[3])?;
        let contents = byte_helpers::base64_encode(&decoded[4]);
        let user_signature = byte_helpers::base64_encode(&decoded[5]);
        let app_signature = byte_helpers::base64_encode(&decoded[6]);
        let id = Self::calculate_id(bytes);
        Ok(Self { 
            id, 
            version, 
            address, 
            timestamp, 
            asset_ref, 
            contents, 
            user_signature, 
            app_signature, 
            bytes: bytes.clone()
        })
    }

    fn calculate_id(bytes: &Vec<u8>) -> String {
        let id = byte_helpers::sha3(&bytes);
        byte_helpers::base64_encode(&id)
    }
    
    pub fn id(&self) -> &str { &self.id }
    pub fn version(&self) -> i32 { self.version }
    pub fn address(&self) -> &str { &self.address }
    pub fn timestamp(&self) -> DateTime<Utc> { self.timestamp }
    pub fn asset_ref(&self) -> &str { &self.asset_ref }
    pub fn contents(&self) -> &str { &self.contents }
    pub fn user_signature(&self) -> &str { &self.user_signature }
    pub fn app_signature(&self) -> &str { &self.app_signature }
    pub fn bytes(&self) -> &Vec<u8> { &self.bytes }
}
