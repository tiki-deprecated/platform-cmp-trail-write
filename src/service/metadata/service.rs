/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use std::error::Error;
use chrono::{DateTime, Utc};
use super::{Model, ModelSigner, super::{Signer, super::{api::Owner, utils::S3Client}}};

pub struct Service {
    version: i32,
    last_block: String,
    owner: Owner,
    modified: DateTime<Utc>,
    created: DateTime<Utc>,
    blocks: Vec<String>,
    signers: Vec<Signer>
}

impl Service {
    pub async fn initialize(
        client: &S3Client,
        parent: Option<String>,
        owner: &Owner
    ) -> Result<Self, Box<dyn Error>> {
        let last_block = parent.unwrap_or(String::from("0x00"));
        let signer: Signer = Signer::get(client, owner).await?;
        let signers = vec![ModelSigner::new(signer.uri(), signer.created())];
        let model = Model::write(client, owner, &last_block, vec![], signers).await?;
        Ok(Self {
            version: model.version(),
            last_block: model.last_block().to_string(),
            owner: model.owner().clone(),
            modified: model.modified(),
            created: model.created(),
            blocks: model.blocks().clone(),
            signers: vec![signer]
        })
    }
    
    pub async fn get(client: &S3Client, owner: &Owner) -> Result<Self, Box<dyn Error>> {
        let model = Model::read(client, owner).await?;
        let signers = Self::get_signers(client, &model).await?;
        Ok(Self {
            version: model.version(),
            last_block: model.last_block().to_string(),
            owner: model.owner().clone(),
            modified: model.modified(),
            created: model.created(),
            blocks: model.blocks().clone(),
            signers
        })
    }

    pub async fn add_block(
        &mut self, 
        client: &S3Client, 
        owner: &Owner, 
        block: &str
    ) -> Result<&Self, Box<dyn Error>> { 
        let mut blocks = self.blocks.clone();
        blocks.push(block.to_string());
        self.blocks = blocks;
        self.last_block = block.to_string();
        self.modified = Utc::now();
        let signers = self.signers.iter()
            .map(|s| ModelSigner::new(s.uri(), s.created()))
            .collect();
        Model::write(client, owner, block, self.blocks.clone(), signers).await?;
        Ok(self)
    }

    pub fn version(&self) -> i32 { self.version }
    pub fn last_block(&self) -> &str { &self.last_block }
    pub fn owner(&self) -> &Owner { &self.owner }
    pub fn modified(&self) -> DateTime<Utc> { self.modified }
    pub fn created(&self) -> DateTime<Utc> { self.created }
    pub fn blocks(&self) -> &Vec<String> { &self.blocks }
    pub fn signers(&self) -> &Vec<Signer> { &self.signers }

    async fn get_signers(client: &S3Client, model: &Model) -> Result<Vec<Signer>, Box<dyn Error>> {
        let mut signers: Vec<Signer> = Vec::new();
        for s in model.signers() {
            let signer = Signer::get_from_path(client, s.uri()).await?;
            signers.push(signer);
        }
        Ok(signers)
    }
}
