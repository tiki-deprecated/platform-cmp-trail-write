/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

mod signer;
use signer::Signer;

mod metadata;
use metadata::Metadata;

mod block;
use block::Block;

use std::{error::Error, env};
use super::{api::{Owner, Transaction, Initialize}, utils::S3Client};

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
    
    pub async fn write_block(&self, owner: &Owner, transactions: &Vec<Transaction>) -> Result<(), Box<dyn Error>> {
        let metadata = Metadata::get(&self.client, owner).await;
        let mut metadata = if metadata.is_err() {
            let provider = Owner::new(&owner.provider().clone().ok_or("No provider")?)?;
            let provider_meta = Metadata::get(&self.client, &provider).await?;
            Metadata::initialize(&self.client, Some(provider_meta.last_block().to_string()), &provider).await?
        } else { metadata? };
        let signer = metadata.signers()
            .get(metadata.signers().len() - 1)
            .ok_or("No block signer found.")?;
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
        metadata.add_block(&self.client, owner, &id).await?;
        Ok(())
    }
    
    pub async fn initialize_provider(&self, owner: &Owner, initialize: &Initialize) -> Result<(), Box<dyn Error>> {
        Signer::create(&self.client, owner, initialize.key()).await?;
        
        let no_owner = Owner::default();
        let provider = Owner::new(&owner.provider().clone().ok_or("No provider")?)?;
        
        let no_owner_meta =  Metadata::get(&self.client, &no_owner).await;
        let no_owner_meta = if no_owner_meta.is_err() { 
            Metadata::initialize(&self.client, None, &no_owner).await?;
            self.write_block(&no_owner, &vec![Transaction::default()]).await?;
            Metadata::get(&self.client, &no_owner).await?
        } else { no_owner_meta? };
        
        let provider_meta = Metadata::get(&self.client, &provider).await;
        if provider_meta.is_err() {
            Metadata::initialize(&self.client, Some(no_owner_meta.last_block().to_string()), &provider).await?;
            self.write_block(&provider, &vec![Transaction::default()]).await?;
            Metadata::get(&self.client, &provider).await?
        } else { provider_meta? };
        
        Ok(())
    }
}


