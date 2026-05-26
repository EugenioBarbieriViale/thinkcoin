use crate::hash::sha256;

use super::BlockHeader;

impl BlockHeader {
    // pub fn into_bytes(&self) -> &[u8] {
    //
    // }

    pub fn build(previous: Self, contents: &str, len: usize) -> Self {
        let next_hash;
        if previous.previous_hash.len() == 0 {
            // This is the genesis block!
            next_hash = sha256(&[0]);
        } else {
            next_hash = sha256(&previous.previous_hash)
        }

        let contents_hash = sha256(contents.as_bytes());

        Self {
            content_len: len,
            previous_hash: next_hash,
            contents_hash,
        }
    }
}
