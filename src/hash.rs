use sha2::{Digest, Sha256};

pub fn sha256(content: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(content);
    hasher.finalize().try_into().expect("Size is not 32 bytes.")
}

pub fn double_hash(content: &[u8]) -> [u8; 32] {
    sha256(sha256(content).as_slice())
}

pub fn hash_to_string(hash: &[u8; 32]) -> String {
    hash.iter()
        .map(|x| format!("{:02x}", x))
        .collect::<Vec<_>>()
        .join("")
}

pub fn bytes_to_string(hash: &[u8]) -> String {
    hash.iter()
        .map(|x| format!("{:02x}", x))
        .collect::<Vec<_>>()
        .join("")
}

pub fn concat(h1: &[u8; 32], h2: &[u8; 32]) -> [u8; 64] {
    let mut ans: [u8; 64] = [0; 64];
    let (left, right) = ans.split_at_mut(32);
    left.copy_from_slice(h1);
    right.copy_from_slice(h2);
    ans
}

pub fn reverse_bo(h: &[u8; 32]) -> [u8; 32] {
    h.into_iter()
        .copied()
        .rev()
        .collect::<Vec<u8>>()
        .try_into()
        .unwrap()
}
