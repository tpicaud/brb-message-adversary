use super::net::interface_network::Network;
use super::net::{impl_udp::{self,UdpConnection},message_adversary};
use super::mbrb::{structs::mbrb::Mbrb,mbrb_algorithm};
use super::user::user_interface;
//use crypto::key_generator;

use std::sync::Arc;
use std::thread;
use std::error::Error;

use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Root};
use file_indexing;


pub fn start() {
    user_interface::start();
}

pub fn initiate_process() -> ((Arc<UdpConnection>,Arc<UdpConnection>),(Arc<Mbrb>,Arc<Mbrb>)) {
        // open udp socket and get a copy for each thread
        let connections = impl_udp::get_connection_copy(user_interface::get_user_start_input());

        // Initiate log file
        init_log(connections.0.id.clone()).unwrap();

        // Get values in config file
        let mut t_line = file_indexing::read_line_index("ressources/config_file.txt", 1).unwrap();
        t_line.pop().unwrap();
        let _t = t_line.split(':').nth(1).unwrap().parse::<u8>().unwrap();

        // Set values for n and t
        let n:u8 = connections.0.n_members;      // Number of processes in the network      
        let t:u8 = _t;                            // Number of byzantine processes among n (tolerance)

        // create Mbrb instance and get a copy for each thread
        let mbrb = mbrb_algorithm::get_mbrb_copy((connections.0.id).try_into().unwrap(),n,t);


        log::info!("process initiated");

        (connections,mbrb)
}

/**
 * Create a new thread or messages reception
 */
pub fn receive(connection: Arc<UdpConnection>, mbrb: Arc<Mbrb>) {

    thread::spawn(move || {
        loop {
            let stream = connection.receive();     // blocking instruction
            let (is_sendable, bundle_as_bytes) = mbrb_algorithm::mbrb_on_receive(&stream, &mbrb);
            if is_sendable {
                message_adversary::pre_broadcast(bundle_as_bytes,&connection, &mbrb.d);
                //connection.ur_broadcast(bundle_as_bytes);
            }
        }  
    });
}

/**
 * Check input command and invoke mbrb_broadcast operation it matches
 */
pub fn listen_command(connection: Arc<UdpConnection>, mbrb: Arc<Mbrb>) {
    let mut sn:u8 = 0;
    loop {
        let message = user_interface::listen_broadcast_command();       // blocking instruction
        if let Some(bundle) = mbrb_algorithm::create_new_bundle(message,&mbrb,sn) {
            message_adversary::pre_broadcast(bundle.to_bytes(),&connection, &mbrb.d);
            //connection.ur_broadcast(bundle.to_bytes());
            sn+=1;
        }
    }
}

fn init_log(id: u8) -> Result<(), Box<dyn Error>> {
    
    let log_file_name = String::from("ressources/logs/")+id.to_string().as_str()+".log";

    std::fs::File::create(&log_file_name).unwrap().set_len(0).unwrap();

    let logfile = FileAppender::builder()
    .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
    .build(&log_file_name)?;

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder()
                    .appender("logfile")
                    .build(LevelFilter::Info))?;

    log4rs::init_config(config)?;

    Ok(())
}