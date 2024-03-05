/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::error::Error;
use super::{ ModelSigner, super::super::{api::Owner, utils::S3Client }};

fn current_version() -> i32 { 1 }

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Model {
    #[serde(default = "current_version")]
    version: i32,
    owner: Owner,
    last_block: String,
    blocks: Vec<String>,
    signers: Vec<ModelSigner>,
    #[serde(default = "Utc::now")]
    modified: DateTime<Utc>,
    #[serde(default = "Utc::now")]
    created: DateTime<Utc>
}

#[allow(unused)]
impl Model {
    pub async fn write(
        client: &S3Client,
        owner: &Owner,
        last_block: &str,
        blocks: Vec<String>,
        signers: Vec<ModelSigner>) -> Result<Self, Box<dyn Error>> {
        let now = Utc::now();
        let model = Self {
            version: current_version(),
            owner: owner.clone(),
            last_block: last_block.to_string(),
            blocks,
            signers,
            modified: now, created: now
        };
        let path = Self::path(owner);
        let body = serde_json::to_string(&model)?.as_bytes().to_vec();
        client.write(&path, &body).await?;
        Ok(model)
    }

    pub async fn read(client: &S3Client, owner: &Owner) -> Result<Self, Box<dyn Error>> {
        let path = Self::path(owner);
        let body = client.read(&path).await?;
        let res:Self = serde_json::from_str(&String::from_utf8(body)?)?;
        Ok(res)
    }

    pub fn version(&self) -> i32 { self.version }
    pub fn owner(&self) -> &Owner { &self.owner }
    pub fn last_block(&self) -> &str { &self.last_block }
    pub fn blocks(&self) -> &Vec<String> { &self.blocks }
    pub fn signers(&self) -> &Vec<ModelSigner> { &self.signers }
    pub fn modified(&self) -> DateTime<Utc> { self.modified }
    pub fn created(&self) -> DateTime<Utc> { self.created }

    fn path(owner: &Owner) -> String { 
        match owner.provider() { 
            Some(provider) => {
                match owner.address() {
                    Some(address) => format!("providers/{}/{}/metadata.json", provider, address),
                    None => format!("providers/{}/metadata.json", provider)
                }
            },
            None => "providers/metadata.json".to_string()
        } 
    }
}
