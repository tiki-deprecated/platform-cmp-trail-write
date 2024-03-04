/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use std::error::Error;
use chrono::{DateTime, Utc};
use ring::rsa::KeyPair;
use ring::signature;
use super::{ Model, {super::super::{api::Owner, utils::{S3Client, byte_helpers}}}};

pub struct Service {
    key_pair: KeyPair,
    created: DateTime<Utc>,
    uri: String
}

impl Service {
    pub async fn create(client: &S3Client, owner: &Owner, key: &str) -> Result<Self, Box<dyn Error>> {
        let path = Self::path(owner.provider());
        let model = Model::write(client, &path, key).await?;
        Ok(Self::from_model(&path, &model)?)
    }

    pub async fn get(client: &S3Client, owner: &Owner) -> Result<Self, Box<dyn Error>> {
        Self::get_from_path(client, &Self::path(owner.provider())).await
    }

    pub async fn get_from_path(client: &S3Client, path: &str) -> Result<Self, Box<dyn Error>> {
        let model = Model::read(client, path).await?;
        Ok(Self::from_model(path, &model)?)
    }

    pub fn sign(&self, message: &Vec<u8>) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut signature = vec![0; self.key_pair.public().modulus_len()];
        match self.key_pair.sign(
            &signature::RSA_PKCS1_SHA256,
            &ring::rand::SystemRandom::new(),
            message.as_slice(),
            &mut signature) {
            Ok(_) => Ok(signature),
            Err(e) => Err(e.to_string())?
        }
    }

    pub fn verify(&self, message: &Vec<u8>, signature: &Vec<u8>) -> bool {
        let pub_key = signature::UnparsedPublicKey::new(
            &signature::RSA_PKCS1_2048_8192_SHA256,
            self.key_pair.public()
        );
        match pub_key.verify(message.as_slice(), signature.as_slice()) {
            Ok(_) => true,
            Err(_) => false
        }
    }

    pub fn created(&self) -> DateTime<Utc> { self.created }
    pub fn uri(&self) -> &str { &self.uri }
    pub fn key_pair(&self) -> &KeyPair { &self.key_pair }

    fn from_model(path: &str, model: &Model) -> Result<Self, Box<dyn Error>> {
        let key = byte_helpers::base64_decode(model.key())?;
        match KeyPair::from_der(key.as_slice()) {
            Ok(key_pair) => Ok(Self {
                key_pair, created:
                model.created(),
                uri: path.to_string()
            }),
            Err(e) => Err(e.to_string())?
        }
    }

    fn path(provider: &str) -> String { format!("{}.key", provider) }
}
