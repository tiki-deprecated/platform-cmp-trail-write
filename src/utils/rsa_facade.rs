/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use std::error::Error;
use ring::{rsa::KeyPair, rand, signature};
use crate::utils::byte_helpers;

pub struct RsaFacade {
    key: KeyPair
}

impl RsaFacade {
    pub fn decode(key: &str) -> Result<Self, Box<dyn Error>> {
        let key = byte_helpers::base64_decode(key)?;
        match KeyPair::from_der(key.as_slice()) {
            Ok(key) => Ok(RsaFacade { key }),
            Err(e) => Err(e.to_string())?
        }
    }

    pub fn sign(&self, message: &Vec<u8>) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut signature = vec![0; self.key.public().modulus_len()];
        match self.key.sign(
            &signature::RSA_PKCS1_SHA256,
            &rand::SystemRandom::new(),
            message.as_slice(),
            &mut signature) {
            Ok(_) => Ok(signature),
            Err(e) => Err(e.to_string())?
        }
    }

    pub fn verify(&self, message: &Vec<u8>, signature: &Vec<u8>) -> bool {
        let pub_key = signature::UnparsedPublicKey::new(
            &signature::RSA_PKCS1_2048_8192_SHA256,
            self.key.public()
        );
        match pub_key.verify(message.as_slice(), signature.as_slice()) {
            Ok(_) => true,
            Err(_) => false
        }
    }
}


#[cfg(test)]
mod tests {
    use tokio_test::{assert_ok};
    use crate::utils::{rsa_facade::RsaFacade, byte_helpers};

    const B64_KEY: &str = "MIIEogIBAAKCAQEAv9enKJv+ZjVxi2cc4XiHhXTUPsvL4G5UKdwKW9TnIYFYW65uVWZEP5zWXAdzj/3e6EeSazlDSmJkqdsdhqYj3G8aw98ft26DXB3cTUbRtotD5Cmq2I2iMA7TdTPw97V6pOp8/U/UbYAhmtgctM9pXUnqnb9zm5UdWMcvlxQEjfUQux71SEtbxKEzYVwzEg5+MiCLRmrM77GJRdriN+sQUkHg8xvdudIiKQ6fwyfjhN5R2kprtRWnglUGT9hYo3t2FxI4jto1fqLfGa5y0mtUO/cbpEO9BrT1ORHecHz3dNYHK2HpwgLJpBY4Mz2DID7E+oIKxyGo2SuhVsbsrcB1GwIDAQABAoIBAAcdub6g4rPp4ZdZMNIQRX6m0cSujJZ7oTWYSu4THKu6f4uPEdqrG3b8m1r4j8nUkfMtzHmbuypEMhW24gZ/nS7tFCIV4bhNKiQ9m1FmghryWYdaIFM+FbkQo7liPtsBaVY6uH4w+uFA8n4q7A2s7+yc9E37rw8jXd5QLSy+eljFoNny3c3a/JQNF9klgyOotyAFmKi+XtaVZepfFb77M+xGMRvG+anJafSy3nV0ZE9RFf6likn7GSJO12qlGnFZu+wKQT+oPz7w6Wv1EDk+wmqS5pEa7Av4Y7NpGUoPl6bL4vvCTbS4SeEgtpwSf/wh2ZbEwt6D8uWAAMcsFI5AE0kCgYEAxo8oT5oManN1NMCopAd6tOGQyKNXJOQ77sDq8BhIrQEk1TBvtXPd4cmWNXy7vFMb+ddz9N56oIxm63SUuoXGS/R4rhqbkP+v2a6iwnD8OLXOuQPRsMkhiB+fWl+IMNlcgJpbEegWbMnNzU2AqIUH6CY2s0jbeK8IDhZvphE2BCUCgYEA91cQQz+XP8Qze2QBxXd6vwW3z3LX4up5PJw1wGl1pI5+uuNp+n1xRv7RRvadrAQ5n4rWXZ19G6nLzdOForOlPA+gQAL88QisbXmG4OT2rC4WkttCpNdHAiEM7UAlWjS+hkm0O+PSmtWWREXEAj2CSplJKyFOx6dA7Soi3KNssD8CgYBMqUMAENMQWol7F5NE2Vpn8drrjBz+MlxtXvCWSFnu6c0lvnCy1wxou2MSPZliKZhYivXLKgaga/TknXs61KFt+/KIDd/YSM/FNObEOck3wAITbsUMA2u92a+1vcKgUZukT3Qv4rKdyAB8bpro9YvK9s4RxGRwIOv0PHdY37ZCPQKBgA8Xqe9gjvseHsIVvSHug3fqgmfPKys2gYVYRtNh3ALZixQeUlYtl17sp5p76+0WKOn6T9BQjtTETKJXmNzvt1Jt5apiREr064iWlMteTUr+WPRHGs7yL+wKVj6X3m+drk6FatEIus4l4FB0LVyxoiSpK9TM6IC4TPbrzkrGUhiDAoGAcaSRlDCBDNSlFVQ5zxoALLCvMur9s133IDyHi7C4fg9k/MSQNPCR0v3PJF3Fg2QGQpKPAADbqc+MQD9lDy/BWI55okjh8PUV5Hnht6DNXUir1hNUifi9bsLQEQ5xMkwllCpUt/Q5whgE55kImO4Qv2wRClQEjSqn/WqxGeMCtAA=";

    #[test]
    fn decode_key_binary() {
        let key_pair = RsaFacade::decode(B64_KEY);
        assert_ok!(key_pair);
    }

    #[test]
    fn create_signature() {
        let message = byte_helpers::utf8_encode("Hello World");
        let signature = RsaFacade::decode(B64_KEY).unwrap().sign(&message);
        assert_ok!(signature);
    }

    #[test]
    fn verify_signature_valid() {
        let message = byte_helpers::utf8_encode("Hello World");
        let key_pair = RsaFacade::decode(B64_KEY).unwrap();
        let signature = key_pair.sign(&message).unwrap();
        let verify = key_pair.verify(&message, &signature);
        assert_eq!(true, verify);
    }

    #[test]
    fn verify_signature_invalid() {
        let message = byte_helpers::utf8_encode("Hello World");
        let key_pair = RsaFacade::decode(B64_KEY).unwrap();
        let signature = key_pair.sign(&byte_helpers::utf8_encode("oops")).unwrap();
        let verify = key_pair.verify(&message, &signature);
        assert_eq!(false, verify);
    }
}
