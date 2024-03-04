/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ModelTransaction {
    #[serde(default = "Utc::now")]
    timestamp: DateTime<Utc>,
    #[serde(default)]
    asset_ref: String,
    contents: String,
    user_signature: String
}

impl ModelTransaction {
    pub fn timestamp(&self) -> DateTime<Utc> { self.timestamp }
    pub fn asset_ref(&self) -> &str { &self.asset_ref }
    pub fn contents(&self) -> &str { &self.contents }
    pub fn user_signature(&self) -> &str { &self.user_signature }
}
