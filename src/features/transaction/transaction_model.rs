/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use chrono::{DateTime, Utc};

pub struct TransactionModel {
    id: Option<String>,
    version: i32,
    address: String,
    timestamp: DateTime<Utc>,
    asset_ref: String,
    contents: Vec<u8>,
    merkel_proof: Option<Vec<u8>>,
    user_signature: Option<Vec<u8>>,
    app_signature: Option<Vec<u8>>
}

impl TransactionModel {
    pub fn new(address: String, contents: Vec<u8>, asset_ref: Option<String>) -> Self {
        TransactionModel {
            id: None,
            version: 2,
            address,
            timestamp: Utc::now(),
            asset_ref: asset_ref.unwrap_or("".to_string()),
            contents,
            merkel_proof: None,
            user_signature: None,
            app_signature: None,
        }
    }

    // pub fn serialize(self, include_signature: bool) -> Vec<u8> {
    //
    // }
}
