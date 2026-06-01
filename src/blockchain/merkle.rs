use super::hash::{Hash256, concat, reverse_bo};

type Hash = [u8; 32];

fn merkle_tree(ts: Vec<Hash>) -> Vec<Hash> {
    let new: Vec<Hash> = ts
        .chunks(2)
        .map(|h| {
            let (h1, h2) = match h.len() % 2 {
                0 => (h[0], h[1]),
                1 => (h[0], h[0]),
                _ => unreachable!(),
            };
            concat(&h1, &h2).double_hash256()
        })
        .collect();
    match new.len() {
        ..=1 => new,
        2.. => merkle_tree(new),
    }
}

pub fn get_root_hash(ts: Vec<Hash>) -> Hash {
    let ts = ts.into_iter().map(|h| reverse_bo(&h)).collect();
    let rh = merkle_tree(ts);
    if rh.len() != 1 {
        panic!("Something went wrong with computing the root hash.");
    }
    rh[0]
}
