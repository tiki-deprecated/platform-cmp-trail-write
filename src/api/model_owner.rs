/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use std::error::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ModelOwner {
    provider: Option<String>,
    address: Option<String>,
}

#[allow(unused)]
impl ModelOwner {
    pub fn default() -> Self { Self { provider: None, address: None } }
    
    pub fn new(sub: &str) -> Result<Self, Box<dyn Error>> {
        let split = sub.split_once(':').unwrap_or((sub, ""));
        let address = if split.1.eq("") { None } else { Some(split.1.to_string()) }; 
        Ok(Self { provider: Some(split.0.to_string()), address })
    }

    pub fn provider(&self) -> &Option<String> { &self.provider }
    pub fn address(&self) -> &Option<String> { &self.address }
}

#[cfg(test)]
mod tests {
    use super::ModelOwner;

    #[test]
    fn from_none() {
        let model = ModelOwner::default();
        assert_eq!(model.provider().is_none(), true);
        assert_eq!(model.address().is_none(), true);
    }

    #[test]
    fn from_provider() {
        let provider = "abc1234";
        let sub = format!("{}", provider);
        let model = ModelOwner::new(&sub).unwrap();
        assert_eq!(model.provider().clone().unwrap(), "abc1234");
        assert_eq!(model.address().is_none(), true);
    }

    #[test]
    fn from_address() {
        let provider = "abc1234";
        let address = "4321cba";
        let sub = format!("{}:{}", provider, address);
        let model = ModelOwner::new(&sub).unwrap();
        assert_eq!(model.provider().clone().unwrap(), "abc1234");
        assert_eq!(model.address().clone().unwrap(), "4321cba");
    }
}
