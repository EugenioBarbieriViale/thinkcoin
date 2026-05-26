// use std::hash::{DefaultHasher, Hash, Hasher};
//
// pub fn calculate_hash<T: Hash>(t: &T) -> String {
//     let mut s = DefaultHasher::new();
//     t.hash(&mut s);
//     format!("{:x}", s.finish())
// }

// use std::hash::{DefaultHasher, Hash, Hasher};

use super::BlockHeader;

use sha2::{Digest, Sha256};

pub trait Hash {
    // fn hash(&self) -> String;
    fn hash(&self) -> Vec<u8>;
}

impl Hash for BlockHeader {
    // fn hash(&self) -> String {
    fn hash(&self) -> Vec<u8> {
        // let mut hasher = Sha256::new();
        // let mut block_str = self.previous_hash.to_owned();
        // block_str.push_str(&self.content_hash);
        // block_str.into_bytes()

        let mut hasher = Sha256::new();
        hasher.update(self.previous_hash.clone().into_bytes());
        hasher.update(self.content_hash.clone().into_bytes());
        hasher.finalize().to_vec()

        // let bytes_hash: Vec<String> = hasher.finalize().map(|e| format!("{:x}", e)).to_vec();
        // println!("{:?}", bytes_hash);
        // match str::from_utf8(&bytes_hash) {
        //     Ok(v) => v.to_string(),
        //     Err(e) => panic!("Error: {}", e),
        // }
        // hasher
        //     .finalize()
        //     .map(|e| {
        //         let b = e.to_be_bytes();
        //         std::str::from_utf8(&b).unwrap().to_string()
        //     })
        //     .to_vec()
        //     .join("")
    }
}
