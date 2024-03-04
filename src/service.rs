/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

mod signer;

use std::env;
use signer::Signer;

mod metadata;
use metadata::Metadata;

mod block;
use block::Block;

use std::error::Error;
use super::{api::{Owner, Transaction}, utils::S3Client};

pub struct Service {
    client: S3Client,
}

impl Service {
    pub async fn new(region: &str, bucket: &str) -> Self {
        let client = S3Client::new(region, bucket).await;
        Self { client }
    }
    
    pub async fn new_from_env() -> Self {
        let region = match env::var("TIKI_REGION") {
            Ok(region) => region,
            Err(_) => panic!("Please set TIKI_REGION"),
        };
        let bucket = match env::var("TIKI_BUCKET") {
            Ok(bucket) => bucket,
            Err(_) => panic!("Please set TIKI_BUCKET"),
        };
        Self::new(&region, &bucket).await
    }
    
    pub async fn write(&self, owner: &Owner, transactions: &Vec<Transaction>) -> Result<(), Box<dyn Error>> {
        let mut metadata = Metadata::get(&self.client, owner).await?;
        let signer = metadata.signers()
            .get(metadata.signers().len() - 1)
            .ok_or(format!("No signer found for provider: {}", owner.provider()))?;
        let mut block = Block::new(&owner, metadata.last_block());
        for transaction in transactions {
            block.add(
                transaction.timestamp(), 
                transaction.asset_ref(), 
                transaction.contents(), 
                transaction.user_signature(),
                &signer
            )?;
        }
        let block = block.write(&self.client).await?;
        let id = block.id().clone().ok_or("No id for block")?;
        metadata.add_block(&self.client, &owner, &id).await?;
        Ok(())
    }
}


