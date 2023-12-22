/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use std::error::Error;
use num_bigint::BigInt;
use crate::utils::{byte_helpers, compact_size, merkle_tree::MerkleTree};
use crate::features::{
    block::block_model::BlockModel,
    transaction::transaction_model::TransactionModel
};
use crate::utils::rsa_facade::RsaFacade;

fn create(txns: Vec<TransactionModel>) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut ids = Vec::<Vec<u8>>::new();
    let mut serialized_txns = Vec::<u8>::new();
    for i in 0..txns.len() {
        let txn = txns.get(i).ok_or("Transaction requires an ID")?;
        let id = txn.id().clone().ok_or("Malformed transaction ID")?;
        ids.push(byte_helpers::base64_decode(&id)?);
        serialized_txns.append(&mut txn.serialize(&dummy_signer())?);
    }

    let mut tree = MerkleTree::new(ids);
    tree.build();
    let root = byte_helpers::base64_encode(&tree.root()
        .clone()
        .ok_or("Failed to build Merkle root")?);

    let mut res = Vec::<u8>::new();
    res.append(&mut build_header(&root)?.serialize()?);
    let txn_count = byte_helpers::encode_bigint(&BigInt::from(txns.len()));
    let mut txn_count = compact_size::encode(txn_count);
    res.append(&mut txn_count);
    res.append(&mut serialized_txns);
    Ok(res)
}

fn build_header(transaction_root: &str) -> Result<BlockModel, Box<dyn Error>> {
    // TODO get last block
    let previous_hash = byte_helpers::base64_encode(&Vec::<u8>::new());

    let mut block = BlockModel::new(&previous_hash, transaction_root);
    block.set_id()?;
    Ok(block)
}

fn dummy_signer() -> RsaFacade {
    let dummy_key: &str = "MIIEogIBAAKCAQEAv9enKJv+ZjVxi2cc4XiHhXTUPsvL4G5UKdwKW9TnIYFYW65uVWZEP5zWXAdzj/3e6EeSazlDSmJkqdsdhqYj3G8aw98ft26DXB3cTUbRtotD5Cmq2I2iMA7TdTPw97V6pOp8/U/UbYAhmtgctM9pXUnqnb9zm5UdWMcvlxQEjfUQux71SEtbxKEzYVwzEg5+MiCLRmrM77GJRdriN+sQUkHg8xvdudIiKQ6fwyfjhN5R2kprtRWnglUGT9hYo3t2FxI4jto1fqLfGa5y0mtUO/cbpEO9BrT1ORHecHz3dNYHK2HpwgLJpBY4Mz2DID7E+oIKxyGo2SuhVsbsrcB1GwIDAQABAoIBAAcdub6g4rPp4ZdZMNIQRX6m0cSujJZ7oTWYSu4THKu6f4uPEdqrG3b8m1r4j8nUkfMtzHmbuypEMhW24gZ/nS7tFCIV4bhNKiQ9m1FmghryWYdaIFM+FbkQo7liPtsBaVY6uH4w+uFA8n4q7A2s7+yc9E37rw8jXd5QLSy+eljFoNny3c3a/JQNF9klgyOotyAFmKi+XtaVZepfFb77M+xGMRvG+anJafSy3nV0ZE9RFf6likn7GSJO12qlGnFZu+wKQT+oPz7w6Wv1EDk+wmqS5pEa7Av4Y7NpGUoPl6bL4vvCTbS4SeEgtpwSf/wh2ZbEwt6D8uWAAMcsFI5AE0kCgYEAxo8oT5oManN1NMCopAd6tOGQyKNXJOQ77sDq8BhIrQEk1TBvtXPd4cmWNXy7vFMb+ddz9N56oIxm63SUuoXGS/R4rhqbkP+v2a6iwnD8OLXOuQPRsMkhiB+fWl+IMNlcgJpbEegWbMnNzU2AqIUH6CY2s0jbeK8IDhZvphE2BCUCgYEA91cQQz+XP8Qze2QBxXd6vwW3z3LX4up5PJw1wGl1pI5+uuNp+n1xRv7RRvadrAQ5n4rWXZ19G6nLzdOForOlPA+gQAL88QisbXmG4OT2rC4WkttCpNdHAiEM7UAlWjS+hkm0O+PSmtWWREXEAj2CSplJKyFOx6dA7Soi3KNssD8CgYBMqUMAENMQWol7F5NE2Vpn8drrjBz+MlxtXvCWSFnu6c0lvnCy1wxou2MSPZliKZhYivXLKgaga/TknXs61KFt+/KIDd/YSM/FNObEOck3wAITbsUMA2u92a+1vcKgUZukT3Qv4rKdyAB8bpro9YvK9s4RxGRwIOv0PHdY37ZCPQKBgA8Xqe9gjvseHsIVvSHug3fqgmfPKys2gYVYRtNh3ALZixQeUlYtl17sp5p76+0WKOn6T9BQjtTETKJXmNzvt1Jt5apiREr064iWlMteTUr+WPRHGs7yL+wKVj6X3m+drk6FatEIus4l4FB0LVyxoiSpK9TM6IC4TPbrzkrGUhiDAoGAcaSRlDCBDNSlFVQ5zxoALLCvMur9s133IDyHi7C4fg9k/MSQNPCR0v3PJF3Fg2QGQpKPAADbqc+MQD9lDy/BWI55okjh8PUV5Hnht6DNXUir1hNUifi9bsLQEQ5xMkwllCpUt/Q5whgE55kImO4Qv2wRClQEjSqn/WqxGeMCtAA=";
    RsaFacade::decode(dummy_key).unwrap()
}
