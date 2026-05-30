use crate::{block_header::BlockHeader, hash::hash_to_string};

pub fn mine_block(block: &mut BlockHeader, target: &[u8; 32], max_attempts: usize) {
    println!("Mining started...");
    let mut c = 0;
    loop {
        for i in 0..max_attempts {
            c += 1;

            let hash = block.sha256();
            // println!("{}: {}", i, hash_to_string(&hash));

            if hash < *target {
                println!("Block mined after {} attempts.", c);
                return;
            }

            block.nonce = i;
        }
        block.nonce = 0;
        block.reset_time();
    }
}
