use sha2::{Digest, Sha256};

pub fn sha256(content: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(content);
    hasher.finalize().try_into().expect("Size is not 32 bytes.")
}

pub fn hash_to_string(hash: &[u8; 32]) -> String {
    hash.iter()
        .map(|x| format!("{:x}", x))
        .collect::<Vec<_>>()
        .join("")
}
