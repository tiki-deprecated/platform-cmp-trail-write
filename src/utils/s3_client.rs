/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use std::error::Error;
use aws_config::BehaviorVersion;
use aws_sdk_s3::Client;
use md5::{Md5, Digest};
use super::byte_helpers::base64_encode;

pub struct S3Client {
    s3: Client,
    bucket: String
}

impl S3Client {
    pub async fn new(region: &str, bucket: &str) -> Self {
        let config = aws_config::defaults(BehaviorVersion::latest())
            .region(aws_sdk_s3::config::Region::new(String::from(region)))
            .load()
            .await;
        Self {
            s3: Client::new(&config),
            bucket: bucket.to_string()
        }
    }

    pub async fn read(
        &self,
        key: &str,
    ) -> Result<Vec<u8>, Box<dyn Error>> {
        let res = self.s3.get_object().bucket(&self.bucket).key(key)
            .send().await?;
        let bytes = res.body.collect().await?;
        Ok(bytes.to_vec())
    }

    pub async fn write(
        &self,
        key: &str,
        body: &Vec<u8>,
    ) -> Result<(), Box<dyn Error>> {
        self.s3.put_object().bucket(&self.bucket).key(key)
            .content_md5(Self::md5(body))
            .body(aws_sdk_s3::primitives::ByteStream::from(body.clone()))
            .send().await?;
        Ok(())
    }

    fn md5(bytes: &Vec<u8>) -> String {
        let mut hasher = Md5::new();
        hasher.update(bytes);
        let res = hasher.finalize();
        base64_encode(&res[..].to_vec())
    }
}
