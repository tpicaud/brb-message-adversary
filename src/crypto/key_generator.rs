#![allow(unused)]

use hex::encode;

use std::fs::{File,OpenOptions,self};
use super::{impl_ecdsa::{EcdsaKeyPair,EcdsaPrivKey,EcdsaPubKey},iface_crypto::*};
use std::io::prelude::*;
use std::io::LineWriter;

pub fn create_key_pairs(n: u8) {
    fs::remove_dir_all("ressources/key_pairs/private_keys").unwrap();
    fs::create_dir("ressources/key_pairs/private_keys").unwrap();
    let file = File::create("ressources/key_pairs/public_keys.txt").unwrap() ;
    file.set_len(0).unwrap();
    for id in 0..n {
        // generating keypair
        let (priv_key,pub_key) = EcdsaKeyPair{}.generate();
        write_pub(id,to_hex(&pub_key.to_bytes()));
        write_priv(id,to_hex(&priv_key.to_bytes()));
        println!("{:?}",pub_key.to_bytes());
    }
}

fn write_pub(id: u8, pub_key: String) {
    let pub_key_file = String::from("ressources/key_pairs/public_keys/")+id.to_string().as_str()+".txt";
    fs::File::create(&pub_key_file);
    file_indexing::pust(pub_key_file.as_str(), pub_key.as_str());
}

fn write_priv(id: u8, priv_key: String) {

    let priv_key_file = String::from("ressources/key_pairs/private_keys/")+id.to_string().as_str()+".txt";
    fs::File::create(&priv_key_file);
    file_indexing::pust(priv_key_file.as_str(), priv_key.as_str());    
}

fn to_hex(bytes: &Vec<u8>) -> String {
    encode(bytes)
}