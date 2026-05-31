use postcard::to_allocvec;
use serde::Serialize;
use sha2::{Digest, Sha256};

pub trait Hash256 {
    fn hash256(&self) -> [u8; 32];
    fn double_hash256(&self) -> [u8; 32];
}

impl<T: ?Sized + Serialize> Hash256 for T {
    fn hash256(&self) -> [u8; 32] {
        let bytes: Vec<u8> = to_allocvec(self).expect("Serialization failed.");
        Sha256::digest(&bytes).into()
    }

    fn double_hash256(&self) -> [u8; 32] {
        Sha256::digest(self.hash256()).into()
    }
}

// pub fn sha256(content: &[u8]) -> [u8; 32] {
//     Sha256::digest(&content).into()
// }
//
// pub fn double_hash(content: &[u8]) -> [u8; 32] {
//     sha256(sha256(content).as_slice())
// }

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
