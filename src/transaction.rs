// https://learnmeabitcoin.com/technical/transaction/

// use rand::prelude::*;
use rand::thread_rng;
use rsa::{RsaPrivateKey, RsaPublicKey};

use crate::hash::double_hash;

fn generate_key() {
    // let mut rng = rand::rng();
    let mut rng = thread_rng();
    let priv_key = RsaPrivateKey::new(&mut rng, 2048).expect("Failed to generate a key");
    let pub_key = RsaPublicKey::from(&priv_key);
}

pub struct Input<'s> {
    txid: [u8; 32],
    vout: u32,
    sign_size: usize,
    sign: &'s [u8],
}

pub struct Output<'p> {
    amount: usize,
    pub_key_size: usize,
    pub_key: &'p [u8],
}

pub struct Transaction<'t> {
    input_count: u32,
    inputs: &'t [Input<'t>],
    output_count: u32,
    outputs: &'t [Output<'t>],
    locktime: usize,
}

impl Transaction<'_> {
    pub fn new<'i>(tc: Self) -> &'i [u8] {
        todo!()
    }
}
