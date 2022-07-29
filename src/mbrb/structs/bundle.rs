use super::mbrb::Mbrb;
use super::element_to_sign::{self,ElementToSign};
use crate::crypto::{iface_crypto::{PubKey}};

use hex::*;

pub struct Bundle {
    pub ets: ElementToSign,
    pub saved_signatures: Vec<Vec<u8>>
}

impl Bundle {

    pub fn are_sigs_valid(&self,mbrb: &Mbrb) -> bool {

        let ids = get_all_ids(mbrb);

        for sig in self.saved_signatures.clone() {
            let mut sig_validity = false;
            for id in ids.clone() {
                if element_to_sign::get_pub_key_from(id).verify(self.ets.to_bytes(),sig.clone()) {
                    sig_validity = true;
                }
            }
            if sig_validity==false {
                log::warn!("Reject bundle {} due to invalid signature(s)",self.ets.to_string());
                return false
            }
        }
        true
    }

    pub fn contains_sig_for(&self, id: u8) -> bool {
        for sig in self.saved_signatures.iter() {
            if element_to_sign::get_pub_key_from(id).verify(self.ets.to_bytes(), sig.clone()) {
                return true;
            }
        }
        false
    }

    pub fn save_signatures(&self, mbrb: &Mbrb) {
        let mut saved_sigs = 0;
        for sig in self.saved_signatures.clone() {
            if self.ets.push_sig(mbrb,&to_hex(&sig)) {
                saved_sigs+=1;
            }
        }
        log::info!("Bundle signatures saved: {}",saved_sigs);
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::<u8>::new();
        bytes.extend(self.ets.message.as_bytes());
        bytes.push(self.ets.sn);
        bytes.push(self.ets.id);

        for sig in &self.saved_signatures {
            bytes.push(sig.len().try_into().unwrap());
            bytes.extend(sig);
        }

        bytes
    }
    
}

/////////////////////////
// Auxiliary functions //
/////////////////////////

fn get_all_ids(mbrb: &Mbrb) -> Vec<u8> {

    let mut ids = Vec::<u8>::new();
    for id in 0..mbrb.n {
        ids.push(id.try_into().unwrap());
    }
    ids
}

fn to_hex(bytes: &Vec<u8>) -> String {
    encode(bytes)
}