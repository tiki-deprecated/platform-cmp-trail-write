/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use std::error::Error;
use chrono::{DateTime, Utc};
use num_bigint::BigInt;
use serde::{Deserialize, Serialize};
use super::{Signer, super::utils::{byte_helpers, compact_size}};

fn current_version() -> i32 { 2 }

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    id: Option<String>,
    #[serde(default = "current_version")]
    version: i32,
    address: String,
    #[serde(default = "Utc::now")]
    timestamp: DateTime<Utc>,
    #[serde(default)]
    asset_ref: String,
    contents: String,
    user_signature: String,
    app_signature: Option<String>
}

#[allow(unused)]
impl Transaction {
    pub fn serialize(&self, signer: &Signer) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut res = Vec::<u8>::new();
        let version = &BigInt::from(self.version);
        res.append(&mut compact_size::encode(byte_helpers::encode_bigint(version)));
        let address = self.address.as_str();
        res.append(&mut compact_size::encode(byte_helpers::base64_decode(address)?));
        let timestamp = &BigInt::from(self.timestamp.timestamp());
        res.append(&mut compact_size::encode(byte_helpers::encode_bigint(timestamp)));
        let asset_ref = self.asset_ref.as_str();
        res.append(&mut compact_size::encode(byte_helpers::utf8_encode(asset_ref)));
        let contents = self.contents.as_str();
        res.append(&mut compact_size::encode(byte_helpers::base64_decode(contents)?));
        let user_signature = self.user_signature.as_str();
        res.append(&mut compact_size::encode(byte_helpers::base64_decode(user_signature)?));
        let app_signature = signer.sign(&res)?;
        res.append(&mut compact_size::encode(app_signature));
        Ok(res)
    }

    pub fn deserialize(bytes: &Vec<u8>) -> Result<Self, Box<dyn Error>>  {
        let id = byte_helpers::sha3(bytes);
        let id = Some(byte_helpers::base64_encode(&id));
        let decoded = compact_size::decode(bytes);
        let version = byte_helpers::decode_bigint(&decoded[0]);
        let version = version.to_string().parse::<i32>()?;
        let address = byte_helpers::base64_encode(&decoded[1]);
        let timestamp = byte_helpers::decode_bigint(&decoded[2]);
        let timestamp =
            DateTime::from_timestamp(timestamp.to_string().parse::<i64>()?, 0)
                .ok_or("Failed to parse timestamp")?;
        let asset_ref = byte_helpers::utf8_decode(&decoded[3])?;
        let contents = byte_helpers::base64_encode(&decoded[4]);
        let user_signature = byte_helpers::base64_encode(&decoded[5]);
        let app_signature = Some(byte_helpers::base64_encode(&decoded[6]));
        Ok(Transaction { id, version, address, timestamp, asset_ref, contents, user_signature, app_signature })
    }

    pub fn set_id_from_bytes(&mut self, bytes: &Vec<u8>) -> () {
        let id = byte_helpers::sha3(bytes);
        self.id = Some(byte_helpers::base64_encode(&id));
    }

    pub fn set_id(&mut self, signer: &Signer) -> Result<(), Box<dyn Error>> {
        let bytes = Transaction::serialize(self, signer)?;
        self.set_id_from_bytes(&bytes);
        Ok(())
    }

    pub fn id(&self) -> &Option<String> { &self.id }
    pub fn version(&self) -> i32 { self.version }
    pub fn address(&self) -> &str { &self.address }
    pub fn timestamp(&self) -> DateTime<Utc> { self.timestamp }
    pub fn asset_ref(&self) -> &str { &self.asset_ref }
    pub fn contents(&self) -> &str { &self.contents }
    pub fn user_signature(&self) -> &str { &self.user_signature }
    pub fn app_signature(&self) -> &Option<String> { &self.app_signature }
}
