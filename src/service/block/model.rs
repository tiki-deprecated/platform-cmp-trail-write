/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use std::error::Error;
use chrono::{DateTime, Utc};
use num_bigint::BigInt;
use super::{ModelTxn, super::{Owner, super::utils::{S3Client, MerkleTree, byte_helpers, compact_size}}};

fn current_version() -> i32 { 1 }

#[derive(Debug, Clone)]
pub struct Model {
    id: String,
    version: i32,
    timestamp: DateTime<Utc>,
    previous_id: String,
    transaction_root: String,
    transactions: Vec<ModelTxn>,
    bytes: Vec<u8>
}

impl Model {
    //write and read are the pub fns

    pub async fn write(
        client: &S3Client,
        owner: &Owner,
        previous_id: &str,
        transactions: &Vec<ModelTxn>
    ) -> Result<Self, Box<dyn Error>> {
        let mut transaction_bytes= vec![];
        for txn in transactions { transaction_bytes.push(txn.bytes().clone()) }

        let mut root_tree = MerkleTree::new(&transaction_bytes);
        root_tree.build();
        let transaction_root = root_tree.root()
            .clone()
            .ok_or("failed to build transaction root")?;

        let mut bytes = Vec::<u8>::new();
        let version = current_version();
        let version_bigint = &BigInt::from(current_version());
        bytes.append(&mut compact_size::encode(byte_helpers::encode_bigint(version_bigint)));
        let timestamp = Utc::now();
        let timestamp_bigint = &BigInt::from(Utc::now().timestamp());
        bytes.append(&mut compact_size::encode(byte_helpers::encode_bigint(timestamp_bigint)));
        bytes.append(&mut compact_size::encode(byte_helpers::base64_decode(previous_id)?));
        bytes.append(&mut compact_size::encode(transaction_root.clone()));
        let num_transactions = BigInt::from(transactions.len());
        bytes.append(&mut compact_size::encode(byte_helpers::encode_bigint(&num_transactions)));
        transaction_bytes.iter().for_each(|txn| bytes.append(&mut txn.clone()));

        let id = Self::calculate_id(&bytes);
        client.write(&Self::path(owner, &id), &bytes).await?;
        Ok(Self {
            id,
            timestamp,
            version,
            previous_id: previous_id.to_string(),
            transaction_root: byte_helpers::base64_encode(&transaction_root),
            transactions: transactions.clone(),
            bytes
        })
    }

    pub async fn read(client: &S3Client, owner: &Owner, id: &str) -> Result<Self, Box<dyn Error>> {
        let bytes = client.read(&Self::path(owner, id)).await?;
    
        let decoded = compact_size::decode(&bytes);
        let version = byte_helpers::decode_bigint(&decoded[0]);
        let version = version.to_string().parse::<i32>()?;
        let timestamp = byte_helpers::decode_bigint(&decoded[1]);
        let timestamp = DateTime::from_timestamp(timestamp.to_string().parse::<i64>()?, 0)
                .ok_or("Failed to parse timestamp")?;
        let previous_id = byte_helpers::base64_encode(&decoded[2]);
        let transaction_root = byte_helpers::base64_encode(&decoded[3]);
        
        let num_transactions = byte_helpers::decode_bigint(&decoded[4]);
        let num_transactions = num_transactions.to_string().parse::<usize>()?;
        let mut transactions:Vec<ModelTxn> = vec![];
        for i in 0..num_transactions {
            let transaction = ModelTxn::read(&decoded[5+i])?;
            transactions.push(transaction);
        }
        Ok(Self{
            id: id.to_string(),
            timestamp,
            version,
            previous_id,
            transaction_root,
            transactions,
            bytes
        })
    }

    fn calculate_id(bytes: &Vec<u8>) -> String {
        let id = byte_helpers::sha3(&bytes);
        byte_helpers::base64_encode(&id)
    }

    fn path(owner: &Owner, id: &str) -> String {
        format!("{}/{}/{}.block", owner.provider(), owner.address(), id)
    }

    pub fn id(&self) -> &str { &self.id }
    pub fn version(&self) -> i32 { self.version }
    pub fn timestamp(&self) -> DateTime<Utc> { self.timestamp }
    pub fn previous_id(&self) -> &str { &self.previous_id }
    pub fn transaction_root(&self) -> &str { &self.transaction_root }
    pub fn transactions(&self) -> &Vec<ModelTxn> { &self.transactions }
}
