// https://learnmeabitcoin.com/technical/transaction/

use rand::thread_rng;
use rsa::{RsaPrivateKey, RsaPublicKey};

use crate::hash::Hash256;

fn generate_key() {
    let mut rng = thread_rng();
    let priv_key = RsaPrivateKey::new(&mut rng, 2048).expect("Failed to generate a key");
    let pub_key = RsaPublicKey::from(&priv_key);
}

pub struct Input {
    txid: [u8; 32],
    vout: u32,
    sign: RsaPrivateKey,
}

pub struct Output {
    amount: usize,
    pub_key: RsaPublicKey,
}

pub struct Transaction<'t> {
    input_count: u32,
    inputs: &'t [Input],
    output_count: u32,
    outputs: &'t [Output],
    locktime: usize,
}

impl Transaction<'_> {
    pub fn new<'i>(tc: Self) -> &'i [u8] {
        todo!()
    }

    pub fn get_txid<'i>(&self) -> [u8; 32] {
        todo!()
    }
}
