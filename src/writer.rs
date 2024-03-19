/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use std::error::Error;

use mytiki_core_trail_storage::{
    content::{Empty, Schema},
    utils::S3Client,
    writer::BodyInitialize,
    Block, Metadata, Owner, Signer, Transaction,
};

pub struct Writer {
    client: S3Client,
}

impl Writer {
    pub async fn new() -> Self {
        let client = S3Client::from_env().await;
        Self { client }
    }

    pub async fn write_block(
        &self,
        owner: &Owner,
        transactions: &Vec<Transaction>,
    ) -> Result<(), Box<dyn Error>> {
        let metadata = Metadata::get(&self.client, owner).await;
        let mut metadata = if metadata.is_err() {
            let provider = Owner::new(owner.provider().clone(), None);
            let provider_meta = Metadata::get(&self.client, &provider).await?;
            Metadata::initialize(
                &self.client,
                Some(provider_meta.last_block().to_string()),
                &provider,
            )
            .await?
        } else {
            metadata?
        };
        let mut block = Block::new(&owner, metadata.last_block());
        for transaction in transactions {
            block.add(transaction)?;
        }
        let block = block.write(&self.client).await?;
        let id = block.id().clone().ok_or("No id for block")?;
        metadata.add_block(&self.client, owner, &id).await?;
        Ok(())
    }

    pub async fn initialize_provider(
        &self,
        owner: &Owner,
        initialize: &BodyInitialize,
    ) -> Result<(), Box<dyn Error>> {
        let signer = Signer::create(&self.client, owner, initialize.key()).await?;
        let no_owner = Owner::new(None, None);
        let provider = Owner::new(owner.provider().clone(), None);
        let no_owner_meta = Metadata::get(&self.client, &no_owner).await;
        let no_owner_meta = if no_owner_meta.is_err() {
            let meta = Metadata::initialize(&self.client, None, &no_owner).await?;
            let no_owner_signer = meta.signers().get(0).ok_or("Missing signer")?;
            let transaction = Transaction::new(
                &no_owner,
                None,
                &Schema::empty(),
                Empty::new(),
                "AA==",
                &no_owner_signer,
            )?;
            self.write_block(&no_owner, &vec![transaction]).await?;
            Metadata::get(&self.client, &no_owner).await?
        } else {
            no_owner_meta?
        };
        let provider_meta = Metadata::get(&self.client, &provider).await;
        if provider_meta.is_err() {
            Metadata::initialize(
                &self.client,
                Some(no_owner_meta.last_block().to_string()),
                &provider,
            )
            .await?;
            let transaction = Transaction::new(
                &no_owner,
                None,
                &Schema::empty(),
                Empty::new(),
                "AA==",
                &signer,
            )?;
            self.write_block(&provider, &vec![transaction]).await?;
            Metadata::get(&self.client, &provider).await?
        } else {
            provider_meta?
        };
        Ok(())
    }
}
