use crate::{block_header::BlockHeader, hash::hash_to_string};

pub fn get_nonce(
    block: &mut BlockHeader,
    target: &[u8; 32],
    max_attempts: usize,
    start: std::time::SystemTime,
) {
    loop {
        for i in 0..max_attempts {
            let hash = block.sha256();
            println!("{}: {}", i, hash_to_string(&hash));

            if hash < *target {
                println!("Block mined after {i} attempts.");
                return;
            }

            block.nonce = i;
        }
        println!("\n----------- Mining failed, resetting time. -----------\n");
        block.reset_time(start);
    }
}
