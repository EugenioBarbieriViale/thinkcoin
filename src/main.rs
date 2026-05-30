use crate::block_header::BlockHeader;
use crate::hash::{hash_to_string, reverse_bo, sha256};
use crate::merkle::get_root_hash;
use crate::mine::mine_block;

mod block_header;
mod hash;
mod merkle;
mod mine;
mod transaction;

fn main() {
    let mut target: [u8; 32] = [0; 32];
    target[2] = 0x0F;
    println!("\n------- TARGET HASH -------");
    println!("{}", hash_to_string(&target));
    println!("---------------------------\n");

    let mut blockchain: Vec<BlockHeader> = vec![];

    let mut genesis = BlockHeader {
        content_len: 5,
        previous_hash: sha256(&0_u8.to_be_bytes()),
        contents_hash: sha256("hello".as_bytes()),
        timestamp: 0,
        nonce: 0,
    };

    for i in 0..10 {
        let previous = blockchain.last().unwrap_or_else(|| &genesis);
        let mut next = previous.build("new hello", 5);
        mine_block(&mut next, &target, 100000);
        next.show();
        blockchain.push(next);
    }
}
