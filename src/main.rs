// use hash::calculate_hash;
// use std::hash::Hash;

use hash::Hash;

mod hash;

// #[derive(Hash)]
struct BlockHeader {
    content_len: usize,
    previous_hash: String,
    content_hash: String,
}

fn main() {
    let b = BlockHeader {
        content_len: 10,
        previous_hash: "there".to_string(),
        content_hash: "hello".to_string(),
    };

    // let hash = calculate_hash(&b);
    let hash = b.hash();
    println!("{hash}");
}
