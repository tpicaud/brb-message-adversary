#![allow(unused)]
use super::impl_udp::UdpConnection;
use super::interface_network::Network;
use super::super::mbrb::structs::mbrb::Mbrb;

use std::net::SocketAddrV4;
use std::sync::Arc;

use rand::Rng;
use rand::prelude::*;

pub fn pre_broadcast(message: Vec<u8>, connection: &Arc<UdpConnection>, d: &usize) {
    if msg_adversary_is_present() {
        let mut counter = 0;

        let mut rng = rand::thread_rng();
        let mut shuffled_members = connection.members.clone();

        // Remove the current socket address
        if let Ok(process_addr) = shuffled_members.binary_search(&connection.socket_addr.to_string()) {
            shuffled_members.remove(process_addr);
        }

        shuffled_members.shuffle(&mut rng);


        // uncomplete broadcast
        for i in 0..shuffled_members.len()-d {
            let member_addr = shuffled_members[i].parse::<SocketAddrV4>().expect("Unexpected socket address, check ressources/processus.txt. Make sure to remove empty lines.");
            if connection.socket_addr!=member_addr {
                connection.send(member_addr, message.clone());
                counter+=1;
            }
        }
        println!("message sent to {} over {} processes",counter,connection.n_members-1);
    } else {
        connection.ur_broadcast(message);
    }
}

fn msg_adversary_is_present() -> bool {
    let mut msg_adversary_line = file_indexing::read_line_index("ressources/config_file.txt", 2).unwrap();
    msg_adversary_line.pop().unwrap();
    let is_present = msg_adversary_line.split(':').nth(1).unwrap();
    is_present.parse::<bool>().unwrap()
}

fn get_msg_adversary_blocking_probability() -> f64 {
    let mut msg_adversary_proba_line = file_indexing::read_line_index("ressources/config_file.txt", 3).unwrap();
    msg_adversary_proba_line.pop().unwrap();
    let proba = msg_adversary_proba_line.split(':').nth(1).unwrap();
    proba.parse::<f64>().unwrap()
}