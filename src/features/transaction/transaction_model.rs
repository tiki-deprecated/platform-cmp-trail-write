/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use std::error::Error;
use chrono::{DateTime, Utc};
use num_bigint::BigInt;
use serde::{Deserialize, Serialize};
use crate::utils::{byte_helpers, compact_size, rsa_facade::RsaFacade};

fn current_version() -> i32 { 2 }

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionModel {
    id: Option<String>,
    #[serde(default = "current_version")]
    version: i32,
    address: String,
    #[serde(default = "Utc::now")]
    timestamp: DateTime<Utc>,
    #[serde(default)]
    asset_ref: String,
    contents: String,
    user_signature: String,
    app_signature: Option<String>
}

impl TransactionModel {

    pub fn serialize(&self, signer: &RsaFacade) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut res = Vec::<u8>::new();

        let version = &BigInt::from(self.version);
        res.append(&mut compact_size::encode(byte_helpers::encode_bigint(version)));
        let address = self.address.as_str();
        res.append(&mut compact_size::encode(byte_helpers::base64_decode(address)?));
        let timestamp = &BigInt::from(self.timestamp.timestamp());
        res.append(&mut compact_size::encode(byte_helpers::encode_bigint(timestamp)));
        let asset_ref = self.asset_ref.as_str();
        res.append(&mut compact_size::encode(byte_helpers::utf8_encode(asset_ref)));
        let contents = self.contents.as_str();
        res.append(&mut compact_size::encode(byte_helpers::base64_decode(contents)?));
        let user_signature = self.user_signature.as_str();
        res.append(&mut compact_size::encode(byte_helpers::base64_decode(user_signature)?));

        let app_signature = signer.sign(&res)?;
        res.append(&mut compact_size::encode(app_signature));

        Ok(res)
    }

    pub fn deserialize(bytes: &Vec<u8>, id: &str) -> Result<Self, Box<dyn Error>>  {
        let decoded = compact_size::decode(bytes);

        let version = byte_helpers::decode_bigint(&decoded[0]);
        let version = version.to_string().parse::<i32>()?;
        let address = byte_helpers::base64_encode(&decoded[1]);

        let timestamp = byte_helpers::decode_bigint(&decoded[2]);
        let timestamp =
            DateTime::from_timestamp(timestamp.to_string().parse::<i64>()?, 0)
            .ok_or("Failed to parse timestamp")?;

        let asset_ref = byte_helpers::utf8_decode(&decoded[3])?;
        let contents = byte_helpers::base64_encode(&decoded[4]);
        let user_signature = byte_helpers::base64_encode(&decoded[5]);
        let app_signature = byte_helpers::base64_encode(&decoded[6]);

        Ok(TransactionModel {
            id: Some(id.to_string()),
            version,
            address,
            timestamp,
            asset_ref,
            contents,
            user_signature,
            app_signature: Some(app_signature)
        })
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use tokio_test::assert_ok;
    use crate::features::transaction::transaction_model::TransactionModel;
    use crate::utils::{rsa_facade::RsaFacade, byte_helpers};

    const B64_KEY: &str = "MIIEogIBAAKCAQEAv9enKJv+ZjVxi2cc4XiHhXTUPsvL4G5UKdwKW9TnIYFYW65uVWZEP5zWXAdzj/3e6EeSazlDSmJkqdsdhqYj3G8aw98ft26DXB3cTUbRtotD5Cmq2I2iMA7TdTPw97V6pOp8/U/UbYAhmtgctM9pXUnqnb9zm5UdWMcvlxQEjfUQux71SEtbxKEzYVwzEg5+MiCLRmrM77GJRdriN+sQUkHg8xvdudIiKQ6fwyfjhN5R2kprtRWnglUGT9hYo3t2FxI4jto1fqLfGa5y0mtUO/cbpEO9BrT1ORHecHz3dNYHK2HpwgLJpBY4Mz2DID7E+oIKxyGo2SuhVsbsrcB1GwIDAQABAoIBAAcdub6g4rPp4ZdZMNIQRX6m0cSujJZ7oTWYSu4THKu6f4uPEdqrG3b8m1r4j8nUkfMtzHmbuypEMhW24gZ/nS7tFCIV4bhNKiQ9m1FmghryWYdaIFM+FbkQo7liPtsBaVY6uH4w+uFA8n4q7A2s7+yc9E37rw8jXd5QLSy+eljFoNny3c3a/JQNF9klgyOotyAFmKi+XtaVZepfFb77M+xGMRvG+anJafSy3nV0ZE9RFf6likn7GSJO12qlGnFZu+wKQT+oPz7w6Wv1EDk+wmqS5pEa7Av4Y7NpGUoPl6bL4vvCTbS4SeEgtpwSf/wh2ZbEwt6D8uWAAMcsFI5AE0kCgYEAxo8oT5oManN1NMCopAd6tOGQyKNXJOQ77sDq8BhIrQEk1TBvtXPd4cmWNXy7vFMb+ddz9N56oIxm63SUuoXGS/R4rhqbkP+v2a6iwnD8OLXOuQPRsMkhiB+fWl+IMNlcgJpbEegWbMnNzU2AqIUH6CY2s0jbeK8IDhZvphE2BCUCgYEA91cQQz+XP8Qze2QBxXd6vwW3z3LX4up5PJw1wGl1pI5+uuNp+n1xRv7RRvadrAQ5n4rWXZ19G6nLzdOForOlPA+gQAL88QisbXmG4OT2rC4WkttCpNdHAiEM7UAlWjS+hkm0O+PSmtWWREXEAj2CSplJKyFOx6dA7Soi3KNssD8CgYBMqUMAENMQWol7F5NE2Vpn8drrjBz+MlxtXvCWSFnu6c0lvnCy1wxou2MSPZliKZhYivXLKgaga/TknXs61KFt+/KIDd/YSM/FNObEOck3wAITbsUMA2u92a+1vcKgUZukT3Qv4rKdyAB8bpro9YvK9s4RxGRwIOv0PHdY37ZCPQKBgA8Xqe9gjvseHsIVvSHug3fqgmfPKys2gYVYRtNh3ALZixQeUlYtl17sp5p76+0WKOn6T9BQjtTETKJXmNzvt1Jt5apiREr064iWlMteTUr+WPRHGs7yL+wKVj6X3m+drk6FatEIus4l4FB0LVyxoiSpK9TM6IC4TPbrzkrGUhiDAoGAcaSRlDCBDNSlFVQ5zxoALLCvMur9s133IDyHi7C4fg9k/MSQNPCR0v3PJF3Fg2QGQpKPAADbqc+MQD9lDy/BWI55okjh8PUV5Hnht6DNXUir1hNUifi9bsLQEQ5xMkwllCpUt/Q5whgE55kImO4Qv2wRClQEjSqn/WqxGeMCtAA=";
    fn mock_txn() -> TransactionModel {
        TransactionModel {
            id: None,
            version: 2,
            address: byte_helpers::base64_encode(&byte_helpers::utf8_encode("DUMMY ADDR")),
            timestamp: Utc::now(),
            asset_ref: String::new(),
            contents: byte_helpers::base64_encode(&byte_helpers::utf8_encode("DUMMY CONTENTS")),
            user_signature: byte_helpers::base64_encode(&byte_helpers::utf8_encode("DUMMY USER SIGNATURE")),
            app_signature: None
        }
    }

    #[test]
    fn serialize() {
        let txn = mock_txn();
        let key = RsaFacade::decode(B64_KEY).unwrap();
        let res = txn.serialize(&key);
        assert_ok!(res);
    }

    #[test]
    fn deserialize() {
        let txn = mock_txn();
        let key = RsaFacade::decode(B64_KEY).unwrap();
        let serialized = txn.serialize(&key).unwrap();
        let id = "DUMMY ID";
        let res = TransactionModel::deserialize(&serialized, id);

        assert_ok!(&res);
        let res = res.unwrap();
        assert_eq!(id, res.id.unwrap());
        assert_eq!(txn.version, res.version);
        assert_eq!(txn.address, res.address);
        assert_eq!(txn.timestamp.timestamp(), res.timestamp.timestamp());
        assert_eq!(txn.asset_ref, res.asset_ref);
        assert_eq!(txn.contents, res.contents);
        assert_eq!(txn.user_signature, res.user_signature);
        assert_eq!(true, res.app_signature.is_some());
    }
}
