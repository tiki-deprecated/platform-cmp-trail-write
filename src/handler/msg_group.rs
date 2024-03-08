/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use std::error::Error;

#[derive(Debug, PartialEq, Eq)]
pub enum MsgGroupType { Initialize, Transaction }

#[derive(Debug)]
pub struct MsgGroup {
    typ: MsgGroupType,
    id: String
}

#[allow(unused)]
impl MsgGroup {
    pub fn new(group: &str) -> Result<Self, Box<dyn Error>> {
        let split = group.split_once(':').unwrap_or((group, ""));
        let typ = match split.0 { 
            "init" => MsgGroupType::Initialize,
            "txn" => MsgGroupType::Transaction,
            _ => return Err("invalid group type")?
        };
        Ok(Self { typ, id: split.1.to_string() })
    }
    
    pub fn typ(&self) -> &MsgGroupType { &self.typ }
    pub fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(test)]
mod tests {
    use tokio_test::assert_err;
    use super::{MsgGroup, MsgGroupType};

    #[test]
    fn from_txn() {
        let typ = "txn";
        let provider = "abc1234";
        let address = "4321cba";
        let group = format!("{}:{}:{}", typ, provider, address);
        let model = MsgGroup::new(&group).unwrap();
        assert_eq!(model.typ, MsgGroupType::Transaction);
        assert_eq!(model.id, "abc1234:4321cba");
    }

    #[test]
    fn from_init() {
        let typ = "init";
        let group = format!("{}", typ);
        let model = MsgGroup::new(&group).unwrap();
        assert_eq!(model.typ, MsgGroupType::Initialize);
        assert_eq!(model.id, "");
    }

    #[test]
    fn from_invalid() {
        let typ = "dummy";
        let provider = "abc1234";
        let address = "4321cba";
        let group = format!("{}:{}:{}", typ, provider, address);
        let model = MsgGroup::new(&group);
        assert_err!(model);
    }
}

