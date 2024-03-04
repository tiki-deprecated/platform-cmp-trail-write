/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use std::error::Error;
use chrono::{DateTime, Utc};
use super::{ModelTxn, Model, super::{Signer, super::{api::Owner, utils::S3Client}}};

pub struct Service {
    id: Option<String>,
    owner: Owner,
    previous_id: String,
    timestamp: Option<DateTime<Utc>>,
    transactions: Vec<ModelTxn>
}

#[allow(unused)]
impl Service {
    pub fn new(owner: &Owner, previous_id: &str) -> Self {
        Service {
            id: None,
            owner: owner.clone(),
            previous_id: previous_id.to_string(),
            timestamp: None,
            transactions: Vec::new()
        }
    }

    pub fn add(
        &mut self,
        timestamp: DateTime<Utc>,
        asset_ref: &str,
        contents: &str,
        user_signature: &str,
        signer: &Signer
    ) -> Result<&Self, Box<dyn Error>> {
        let txn = ModelTxn::new(
            self.owner.address(),
            timestamp,
            asset_ref,
            contents,
            user_signature,
            signer)?;
        self.transactions.push(txn);
        Ok(self)
    }

    pub async fn write(&mut self, client: &S3Client) -> Result<&Self, Box<dyn Error>> {
        let block = Model::write(
            client,
            &self.owner,
            &self.previous_id,
            &self.transactions,
        ).await?;
        self.id = Some(block.id().to_string());
        self.timestamp = Some(block.timestamp());
        Ok(self)
    }

    pub fn id(&self) -> &Option<String> { &self.id }
    pub fn owner(&self) -> &Owner { &self.owner }
    pub fn previous_id(&self) -> &str { &self.previous_id }
    pub fn timestamp(&self) -> Option<DateTime<Utc>> { self.timestamp }
    pub fn transactions(&self) -> &Vec<ModelTxn> { &self.transactions }
}