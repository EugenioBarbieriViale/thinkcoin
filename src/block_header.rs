use crate::hash::Hash256;
use serde::Serialize;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::hash::hash_to_string;

#[derive(Debug, Serialize)]
pub struct BlockHeader {
    pub content_len: usize,
    pub previous_hash: [u8; 32],
    pub contents_hash: [u8; 32],
    pub timestamp: usize,
    pub nonce: usize,
}

impl BlockHeader {
    pub fn build(&self, contents: &str, len: usize) -> Self {
        let previous_hash = self.hash256();
        let contents_hash = contents.as_bytes().hash256();
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
