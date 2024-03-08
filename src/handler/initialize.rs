/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Initialize {
    #[serde(default = "Utc::now")]
    timestamp: DateTime<Utc>,
    #[serde(default)]
    key: String
}

#[allow(unused)]
impl Initialize {
    pub fn timestamp(&self) -> DateTime<Utc> { self.timestamp }
    pub fn key(&self) -> &str { &self.key }
}
