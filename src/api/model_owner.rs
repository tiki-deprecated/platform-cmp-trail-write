/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ModelOwner {
    provider: String,
    address: String,
}

#[allow(unused)]
impl ModelOwner {
    pub fn new(sub: &str) -> Self {
        let split: Vec<&str> = sub.split(':').collect();
        Self { provider: split[0].to_string(), address: split[1].to_string() }
    }
    pub fn provider(&self) -> &str {
        &self.provider
    }
    pub fn address(&self) -> &str {
        &self.address
    }
}

#[cfg(test)]
mod tests {
    use super::ModelOwner;

    #[test]
    fn from_sub() {
        let provider = "abc1234";
        let address = "4321cba";
        let sub = format!("{}:{}", provider, address);
        let model = ModelOwner::new(&sub);
        assert_eq!(model.provider(), "abc1234");
        assert_eq!(model.address(), "4321cba");
    }
}
