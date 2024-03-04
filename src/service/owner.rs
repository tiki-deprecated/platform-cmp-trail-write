/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Owner {
    provider: String,
    address: String,
}

#[allow(unused)]
impl Owner {
    pub fn new(sub: &str) -> Owner {
        let split: Vec<&str> = sub.split(':').collect();
        Owner {
            provider: split[0].to_string(),
            address: split[1].to_string()
        }
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
    use crate::service::owner::Owner;

    #[test]
    fn from_sub() {
        let provider = "abc1234";
        let address = "4321cba";
        let sub = format!("{}:{}", provider, address);
        let model = Owner::new(&sub);
        assert_eq!(model.provider(), "abc1234");
        assert_eq!(model.address(), "4321cba");
    }
}