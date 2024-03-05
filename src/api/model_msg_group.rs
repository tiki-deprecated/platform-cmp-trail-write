/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use std::error::Error;

#[derive(Debug, PartialEq, Eq)]
pub enum ModelMsgGroupType { Initialize, Transaction }

#[derive(Debug)]
pub struct ModelMsgGroup {
    typ: ModelMsgGroupType,
    id: String
}

#[allow(unused)]
impl ModelMsgGroup {
    pub fn new(group: &str) -> Result<Self, Box<dyn Error>> {
        let split = group.split_once(':').unwrap_or((group, ""));
        let typ = match split.0 { 
            "init" => ModelMsgGroupType::Initialize,
            "txn" => ModelMsgGroupType::Transaction,
            _ => return Err("invalid group type")?
        };
        Ok(Self { typ, id: split.1.to_string() })
    }
    
    pub fn typ(&self) -> &ModelMsgGroupType { &self.typ }
    pub fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(test)]
mod tests {
    use tokio_test::assert_err;
    use super::{ModelMsgGroup, ModelMsgGroupType};

    #[test]
    fn from_txn() {
        let typ = "txn";
        let provider = "abc1234";
        let address = "4321cba";
        let group = format!("{}:{}:{}", typ, provider, address);
        let model = ModelMsgGroup::new(&group).unwrap();
        assert_eq!(model.typ, ModelMsgGroupType::Transaction);
        assert_eq!(model.id, "abc1234:4321cba");
    }

    #[test]
    fn from_init() {
        let typ = "init";
        let group = format!("{}", typ);
        let model = ModelMsgGroup::new(&group).unwrap();
        assert_eq!(model.typ, ModelMsgGroupType::Initialize);
        assert_eq!(model.id, "");
    }

    #[test]
    fn from_invalid() {
        let typ = "dummy";
        let provider = "abc1234";
        let address = "4321cba";
        let group = format!("{}:{}:{}", typ, provider, address);
        let model = ModelMsgGroup::new(&group);
        assert_err!(model);
    }
}

