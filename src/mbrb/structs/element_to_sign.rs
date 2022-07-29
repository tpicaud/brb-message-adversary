use super::mbrb::Mbrb;
use super::bundle::Bundle;
use crate::crypto::{impl_ecdsa::*,iface_crypto::{PrivKey,PubKey}};

use csv::{WriterBuilder,ReaderBuilder,StringRecord};
use std::fs;
use hex::*;

pub struct ElementToSign {
    pub message: String,
    pub sn: u8,
    pub id: u8
}

impl ElementToSign {
    pub fn is_delivered(&self,mbrb: &Mbrb) -> bool {
        let file = fs::read_to_string(&mbrb.delivered_messages_file).unwrap();
        let messages: Vec<String> = file.split("\n").map(|x| x.to_string()).collect();

        for msg in messages {
            if msg.eq(&self.to_string().trim()) {
                return true;
            }
        }
        false
    }

    pub fn deliver(&self,mbrb: &Mbrb) {

        let file_read = fs::read_to_string(&mbrb.pending_messages_file).unwrap();
        let mut messages: Vec<String> = file_read.split("\n").map(|x| String::from(x)).collect();
        messages.pop();
        let line = messages.binary_search(&self.to_string()).unwrap();
        file_indexing::delete_line(&mbrb.pending_messages_file, line+1).unwrap();

        file_indexing::pust(&mbrb.delivered_messages_file, &(self.to_string()+"\n")).unwrap();
        log::info!("Message delivered --> {}",self.to_string());

        println!("---------- Message delivered ----------");
        println!("---> {} <---",self.to_string().trim());
        println!(">>> Signed by {} processus\n\n", self.get_sigs(mbrb).len());
    }

    fn is_pending(&self,mbrb: &Mbrb) -> bool {
        let file = fs::read_to_string(&mbrb.pending_messages_file).unwrap();
        let messages: Vec<String> = file.split("\n").map(|x| x.to_string()).collect();

        for msg in messages {
            if msg.eq(&self.to_string().trim()) {
                return true;
            }
        }
        false
    }

    fn pending(&self,mbrb: &Mbrb) {
        file_indexing::pust(&mbrb.pending_messages_file, &(self.to_string()+"\n")).unwrap();
        file_indexing::pust(&mbrb.saved_signatures_file,&self.to_string()).unwrap();
        println!("New bundle: {}\n--> Sent to pending messages...\n",self.to_string());
        log::info!("New pending message --> {}", self.to_string());
    }

    pub fn already_signed_by(&self, mbrb: &Mbrb, id: u8) -> bool {
        let pub_key = get_pub_key_from(id);
        let sigs = self.get_sigs(mbrb);
        for sig in sigs.iter() {
            if pub_key.verify(self.to_bytes(),hex_to_bytes(&sig)) {
                return true;
            }
        }
        false
    }

    pub fn get_sigs(&self, mbrb: &Mbrb) -> Vec<String> {
        let mut csv_reader = ReaderBuilder::new().has_headers(false).flexible(true).delimiter(b';').from_path(&mbrb.saved_signatures_file).unwrap();
        let records = csv_reader.records().map(|s| s.unwrap()).collect::<Vec<_>>();
        for i in 0..records.len() {
            if records[i].as_slice().starts_with(&self.to_string()) {
                return records[i].iter().map(|s| s.to_string()).collect::<Vec<_>>().split_first().unwrap().1.to_vec()
            }
        }
        vec![]
    }

    pub fn save_process_signature(&self, mbrb: &Mbrb) {
        let priv_key = ecdsa_priv_key_from_bytes(&hex_to_bytes(&mbrb.key_pair.0));
        if self.push_sig(mbrb, &to_hex(&priv_key.sign(self.to_bytes()))) {
            log::info!("Message signed --> {}",self.to_string())
        }
    }

    pub fn push_sig(&self, mbrb: &Mbrb, sig: &String) -> bool {
        let msg = &self.to_string();
        let mut csv_reader = ReaderBuilder::new().has_headers(false).flexible(true).delimiter(b';').from_path(&mbrb.saved_signatures_file).unwrap();

        let mut records = csv_reader.records().map(|s| s.unwrap()).collect::<Vec<_>>();
        for i in 0..records.len() {
            if records[i].as_slice().contains(msg) && !records[i].as_slice().contains(sig) {
                records[i].push_field(sig);
                file_indexing::replase_line(&mbrb.saved_signatures_file, to_csv_format(&records[i]).as_str(),i+1).unwrap();
                return true;
            }
        }
        if !self.is_pending(mbrb){
            self.pending(mbrb);
            return self.push_sig(mbrb, sig);
        }
        false
    }

    pub fn is_quorum_reached(&self, mbrb: &Mbrb) -> bool {
        let quorum:usize = ((mbrb.n+mbrb.t)/2).try_into().unwrap();
        self.get_sigs(mbrb).len()>quorum
    }

    pub fn to_bundle(&self, mbrb: &Mbrb) -> Bundle {

        let mut  sigs = Vec::<Vec<u8>>::new();
        for sig in self.get_sigs(mbrb) {
            sigs.push(hex_to_bytes(&sig));
        }

        Bundle {
            ets: ElementToSign {
                message: self.message.clone(),
                sn: self.sn.clone(),
                id: self.id.clone()
            },
            saved_signatures: sigs,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = self.message.as_bytes().to_vec();
        bytes.push(self.sn);
        bytes.push(self.id);
        bytes
    }

    pub fn to_string(&self) -> String {
        let mut message = self.message.clone();
        message.truncate(self.message.find("\0").unwrap());
        String::from("(message: '")+message.as_str().trim()+"', sn: "+self.sn.to_string().as_str()+", id: "+self.id.to_string().as_str()+")"
    }
}


/////////////////////////
// Auxiliary functions //
/////////////////////////

pub fn get_pub_key_from(id: u8) -> EcdsaPubKey {
    let pub_key = fs::read_to_string(&(String::from("ressources/key_pairs/public_keys/")+id.to_string().as_str()+".txt")).unwrap();
    ecdsa_pub_key_from_bytes(&hex_to_bytes(&pub_key))
}

fn to_csv_format(record: &StringRecord) -> String {
    let mut wtr = WriterBuilder::new().delimiter(b';').from_writer(vec![]);
    wtr.write_record(record).unwrap();
    let res = wtr.into_inner().unwrap();
    String::from_utf8(res).unwrap()
}

fn to_hex(bytes: &Vec<u8>) -> String {
    encode(bytes)
}

fn hex_to_bytes(s: &String) -> Vec<u8> {
    decode(s).unwrap()
}