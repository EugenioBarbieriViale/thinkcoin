// use hash::calculate_hash;
// use std::hash::Hash;

use hash::{Hash, hash_to_string};

mod block_header;
mod hash;

// #[derive(Hash)]
struct BlockHeader {
    content_len: usize,
    previous_hash: [u8; 32],
    contents_hash: [u8; 32],
}

fn main() {
    let b = BlockHeader {
        content_len: 10,
        previous_hash: [1; 32],
        contents_hash: [1; 32],
    };
}
