use std::time::SystemTime;

use crate::block_header::BlockHeader;
use crate::hash::{hash_to_string, sha256};
use crate::mine::get_nonce;

mod block_header;
mod hash;
mod mine;

fn main() {
    let start = SystemTime::now();

    let b = BlockHeader {
        content_len: 5,
        previous_hash: [4; 32],
        contents_hash: [2; 32],
        timestamp: 0,
        nonce: 0,
    };

    let mut next = b.build(start, "hello", 5);
    b.show();
    next.show();

    let mut target: [u8; 32] = [0; 32];
    target[1] = 0x0F;
    println!("{}", hash_to_string(&target));
    get_nonce(&mut next, &target, 100000, start);
    next.show();
}
