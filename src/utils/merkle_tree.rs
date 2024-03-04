/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use std::collections::HashMap;
use super::byte_helpers;

pub struct MerkleTree {
    proofs: HashMap<String, Vec<u8>>,
    hashes: Vec<Vec<u8>>,
    root: Option<Vec<u8>>,
    depth: i32
}

impl MerkleTree {
    pub fn new(hashes: &Vec<Vec<u8>>) -> Self {
        Self {
            proofs: HashMap::<String, Vec<u8>>::new(),
            hashes: hashes.clone(),
            root: None,
            depth: 1
        }
    }

    pub fn build(&mut self) -> () {
        if self.hashes.len() == 1 {
            let mut proof_res = Vec::<u8>::new();
            proof_res.push(1);
            proof_res.append(&mut self.hashes[0].clone());
            self.proofs.insert(byte_helpers::base64_encode(&self.hashes[0].clone()), proof_res);
            let mut root_res = Vec::<u8>::new();
            root_res.append(&mut self.hashes[0].clone());
            root_res.append(&mut self.hashes[0].clone());
            self.root = Some(byte_helpers::sha3(&root_res));
        } else { self.root = Some(self.calculate(self.hashes.clone())); }
    }

    fn calculate(&mut self, mut input_hashes: Vec<Vec<u8>>) -> Vec<u8> {
        let mut output_hashes = Vec::<Vec<u8>>::new();
        if input_hashes.len() % 2 == 1 {
            let last = input_hashes.last().expect("Empty list of hashes");
            input_hashes.push(last.clone())
        }
        for i in (0..input_hashes.len()).step_by(2) {
            let mut left = input_hashes[i].clone();
            let mut right = input_hashes[i+1].clone();
            let mut hash = Vec::<u8>::new();
            hash.append(&mut left);
            hash.append(&mut right);
            output_hashes.push(byte_helpers::sha3(&hash));
        }
        self.calculate_proofs(&output_hashes, &input_hashes);
        if output_hashes.len() > 1 {
            self.depth += 1;
            self.calculate(output_hashes)
        } else { output_hashes[0].clone() }
    }

    fn calculate_proofs(&mut self, output_hashes: &Vec<Vec<u8>>, input_hashes: &Vec<Vec<u8>>) -> () {
        let hpo = 2usize.pow(self.depth as u32);
        for i in 0..output_hashes.len() {
            for j in 0..hpo {
                let index = (i*hpo)+j;
                if index == self.hashes.len() { break; }
                let p_index = byte_helpers::base64_encode(&self.hashes[index]);
                let mut proof_res = Vec::<u8>::new();
                proof_res.append(&mut match self.proofs.get(&p_index) {
                    Some(h) => h.clone(),
                    None => Vec::new()
                });
                if j < hpo/2 {
                    proof_res.push(1);
                    proof_res.append(&mut input_hashes[(i*2)+1].clone());
                }
                else {
                    proof_res.push(0);
                    proof_res.append(&mut input_hashes[i*2].clone());
                }
                self.proofs.insert(p_index, proof_res);
            }
        }
    }

    pub fn validate(hash: &Vec<u8>, proof: &Vec<u8>, root: &Vec<u8>) -> bool {
        let pos = proof[0];
        let hash_pair = &proof[1..33];
        let mut hash_check = Vec::<u8>::new();
        if pos == 0 {
            hash_check.append(&mut hash_pair.to_vec());
            hash_check.append(&mut hash.clone());
        } else {
            hash_check.append(&mut hash.clone());
            hash_check.append(&mut hash_pair.to_vec());
        }
        let hash = byte_helpers::sha3(&hash_check);
        if proof.len() > 33 { Self::validate(&hash, &proof[33..].to_vec(), root) }
        else { byte_helpers::base64_encode(&hash) == byte_helpers::base64_encode(root) }
    }

    pub fn is_valid(&self, hash: &Vec<u8>) -> bool {
        let hash_b64 = byte_helpers::base64_encode(hash);
        let proof = self.proofs[&hash_b64].clone();
        let root = self.root.clone().expect("Missing root. Must first call .build()");
        Self::validate(hash, &proof, &root)
    }

    pub fn hashes(&self) -> &Vec<Vec<u8>> { &self.hashes }
    pub fn root(&self) -> &Option<Vec<u8>> { &self.root }
    pub fn proofs(&self) -> &HashMap<String, Vec<u8>> { &self.proofs }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;
    use super::{MerkleTree, byte_helpers};

    #[test]
    fn build_one() {
        let id = byte_helpers::sha3(&byte_helpers::utf8_encode(&Uuid::new_v4().to_string()));
        let mut tree = MerkleTree::new(&vec![id.clone()]);
        tree.build();

        assert_eq!(true, tree.root.is_some());
        assert_eq!(1, tree.proofs.len());
        assert_eq!(1, tree.depth);

        let res = tree.is_valid(&id);
        assert_eq!(true, res);
    }

    #[test]
    fn build_ten() {
        let mut hashes = Vec::<Vec<u8>>::new();
        for _ in 0..10 {
            let id = byte_helpers::sha3(&byte_helpers::utf8_encode(&Uuid::new_v4().to_string()));
            hashes.push(id);
        }
        let mut tree = MerkleTree::new(&hashes);
        tree.build();

        assert_eq!(true, tree.root.is_some());
        assert_eq!(10, tree.proofs.len());
        assert_eq!(10, tree.hashes.len());
        assert_eq!(4, tree.depth);

        for i in 0..10 {
            let hash = hashes[i].clone();
            let res = tree.is_valid(&hash);
            assert_eq!(true, res);
        }
    }

    #[test]
    fn build_two_fifty() {
        let mut hashes = Vec::<Vec<u8>>::new();
        for _ in 0..250 {
            let id = byte_helpers::sha3(&byte_helpers::utf8_encode(&Uuid::new_v4().to_string()));
            hashes.push(id);
        }
        let mut tree = MerkleTree::new(&hashes);
        tree.build();

        assert_eq!(true, tree.root.is_some());
        assert_eq!(250, tree.proofs.len());
        assert_eq!(250, tree.hashes.len());
        assert_eq!(8, tree.depth);

        for i in 0..250 {
            let hash = hashes[i].clone();
            let res = tree.is_valid(&hash);
            assert_eq!(true, res);
        }
    }
}
