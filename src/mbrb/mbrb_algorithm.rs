use super::structs::{mbrb::Mbrb,element_to_sign::ElementToSign,bundle::Bundle};

use std::sync::Arc;


/////////////////////////////////////////////////////////////////////////////////////////////////////
/**
 * Return a tuple (bool, Vec<u8>) which contains true if the Vec must be broadcast, false otherwise
 */
pub fn mbrb_on_receive(message: &Vec<u8>, mbrb: &Mbrb) -> (bool, Vec<u8>) {

    if is_bundle(message) {
        let bundle = bundle_from_message(message.clone());
        if bundle.are_sigs_valid(mbrb) && !bundle.ets.is_delivered(mbrb) && bundle.contains_sig_for(bundle.ets.id) {
            bundle.save_signatures(mbrb);
            if !bundle.ets.already_signed_by(mbrb, mbrb.id) && !bundle.ets.is_quorum_reached(mbrb) {
                bundle.ets.save_process_signature(&mbrb);
                log::info!("Bundle sent to broadcast function: {}", bundle.ets.to_string());
                return (true,bundle.ets.to_bundle(mbrb).to_bytes());
            }
            if bundle.ets.is_quorum_reached(mbrb) {
                bundle.ets.deliver(mbrb);
                return (true,bundle.ets.to_bundle(mbrb).to_bytes());
            }
        } else {
            if !bundle.ets.is_delivered(mbrb) {println!("Invalid bundle received");}
        }
    } else { println!("not bundle"); }
    (false,message.to_owned())
}
/////////////////////////////////////////////////////////////////////////////////////////////////////


fn create_mbrb_instance(_id: u8, _n: u8, _t: u8, _delivered_messages_file: String, _pending_messages_file: String, _saved_signatures_file: String) -> Mbrb {
    let mut _d =(_n-3*_t)/2;
    if !_n>(3*_t+2*_d) {
        _d -=1;
    }
    let _d:usize = _d.try_into().unwrap();

    println!("Process id: {}\n------------------------------", _id);
    println!("n: {}\nt: {}\nd: {}\n",_n,_t,_d);
    Mbrb {
        n: _n,
        t: _t,
        id: _id,
        d: _d,
        delivered_messages_file: _delivered_messages_file,
        pending_messages_file: _pending_messages_file,
        saved_signatures_file: _saved_signatures_file,
        key_pair: get_key_pair(_id)
    }
}

pub fn get_mbrb_copy(id: u8, n: u8, t: u8) -> (Arc<Mbrb>,Arc<Mbrb>) {

    let delivered_messages_file = String::from("ressources/tmp/messages/")+id.to_string().as_str()+"_delivered_messages.txt";
    let pending_messages_file = String::from("ressources/tmp/messages/")+id.to_string().as_str()+"_pending_messages.txt";
    let saved_signatures_file = String::from("ressources/tmp/saved_signatures/")+id.to_string().as_str()+"_saved_signatures.csv";

    let mbrb = create_mbrb_instance(id,n,t,delivered_messages_file,pending_messages_file,saved_signatures_file);

    std::fs::File::create(&mbrb.delivered_messages_file).unwrap();
    std::fs::File::create(&mbrb.pending_messages_file).unwrap();
    std::fs::File::create(&mbrb.saved_signatures_file).unwrap();
    
    let shared_mbrb = Arc::new(mbrb);
    (shared_mbrb.clone(),shared_mbrb.clone())
}

pub fn create_new_bundle(msg: String, mbrb: &Mbrb,_sn: u8)-> Option<Bundle> {
    if msg.len()<32 {
        log::info!("Creating new bundle with message: {}", &msg);
        let mut msg_boxed = [0;32];
        for i in 0..msg.len() {
            msg_boxed[i] = msg.as_bytes()[i];
        }

        let ets = ElementToSign {
            message: String::from_utf8(msg_boxed.to_vec()).unwrap(),
            sn: _sn,
            id: mbrb.id
        };
        ets.save_process_signature(mbrb);
        log::info!("Bundle sent to broadcast function: {}", ets.to_string());
        return Some(ets.to_bundle(mbrb));
    } else {
        println!("Message too long");
        log::error!("Message too long, must be strictly under 32 characters");
    }
    None
}



// Auxiliary functions

fn is_bundle(message: &Vec<u8>) -> bool {
    if message.len()>=105 {
        if let Ok(_) = String::from_utf8(message.split_at(32).0.to_vec()) {
            if message.split_at(32).0 != [0;32] {
                return true
            }
        }
    }
    false
}

fn bundle_from_message(message: Vec<u8>) -> Bundle {
    let (_message,rest) = message.split_at(32);
    let (_sn,rest) = rest.split_at(1);
    let (_id,mut rest) = rest.split_at(1);

    let mut sigs = Vec::<Vec<u8>>::new();
    while !rest.is_empty() {
        let (first,tmp) = rest.split_first().unwrap();
        let (sig,tmp) = tmp.split_at(first.clone().try_into().unwrap());
        rest = tmp;
        sigs.push(sig.to_vec());
    }

    let bundle = Bundle {
        ets: ElementToSign {
            message: String::from_utf8(_message.to_vec()).unwrap(),
            sn: _sn[0],
            id: _id[0],
        },
        saved_signatures: sigs
    };
    log::info!("Bundle received: {} with {} signatures",bundle.ets.to_string(),bundle.saved_signatures.len());
    bundle
}

fn get_key_pair(id: u8) -> (String,String) {
    let priv_key = std::fs::read_to_string(String::from("ressources/key_pairs/private_keys/")+id.to_string().as_str()+".txt").unwrap();
    let pub_key = std::fs::read_to_string(String::from("ressources/key_pairs/public_keys/")+id.to_string().as_str()+".txt").unwrap();
    (priv_key,pub_key)
}