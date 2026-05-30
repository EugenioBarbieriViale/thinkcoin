use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::hash::{hash_to_string, sha256};

#[derive(Debug)]
pub struct BlockHeader {
    pub content_len: usize,
    pub previous_hash: [u8; 32],
    pub contents_hash: [u8; 32],
    pub timestamp: usize,
    pub nonce: usize,
}

impl BlockHeader {
    pub fn build(&self, contents: &str, len: usize) -> Self {
        // let previous_hash;
        // if self.previous_hash.len() == 0 {
        //     // This is the genesis block!
        //     previous_hash = sha256(&0_u8.to_be_bytes());
        // } else {
        //     previous_hash = self.sha256();
        // }

        let previous_hash = self.sha256();
        let contents_hash = sha256(contents.as_bytes());
        let timestamp = get_time();

        Self {
            content_len: len,
            previous_hash,
            contents_hash,
            timestamp,
            nonce: 0,
        }
    }

    pub fn reset_time(&mut self) {
        self.timestamp = get_time();
    }

    pub fn sha256(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(self.content_len.to_be_bytes());
        hasher.update(self.previous_hash);
        hasher.update(self.contents_hash);
        hasher.update(self.timestamp.to_be_bytes());
        hasher.update(self.nonce.to_be_bytes());
        hasher.finalize().try_into().expect("Size is not 32 bytes.")
    }

    pub fn show(&self) {
        println!("\n------- BLOCK HEADER -------");
        println!("{:<24} {}", "Content length:", self.content_len);
        println!(
            "{:<24} {}",
            "Hash of previous block:",
            hash_to_string(&self.previous_hash)
        );
        println!(
            "{:<24} {}",
            "Hash contents:",
            hash_to_string(&self.contents_hash)
        );
        println!("{:<24} {}", "Timestamp:", self.timestamp);
        println!("{:<24} {}", "Nonce:", self.nonce);
        println!("----------------------------\n");
    }
}

fn get_time() -> usize {
    let start = SystemTime::now();
    start
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        .try_into()
        .unwrap()
}
