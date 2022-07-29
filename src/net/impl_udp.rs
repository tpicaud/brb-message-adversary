use super::interface_network::{Network};
use std::net::{UdpSocket, SocketAddrV4};
use std::sync::Arc;
use std::fs;


/**
 * Connection struct for Tcp standard library
 */
pub struct UdpConnection {
    pub socket: UdpSocket,
    pub port: u16,
    pub id: u8,
    pub members: Vec<String>,
    pub n_members: u8,
    pub socket_addr: SocketAddrV4,
}

/**
 * Implementation of Network with std::net
 */
impl Network for UdpConnection {

    fn receive(&self) -> Vec<u8> {
        let mut buf = [0;1024];
        let (amount,_) = self.socket.recv_from(&mut buf).unwrap();
        buf[..amount].to_vec()
    }

    fn send(&self,to_socket_addr: SocketAddrV4, message: Vec<u8>) {
        if message.len()<=1024 {
            self.socket.send_to(&message, to_socket_addr).expect("couldn't send data");
        } else {
            println!("Message too long (too much signatures)");
        }
    }

    fn ur_broadcast(&self, message: Vec<u8>) {
        for member in self.members.clone().into_iter() {
            let member_addr = member.parse::<SocketAddrV4>().expect("Unexpected socket address, check ressources/processus.txt. Make sure to remove empty lines.");
            if self.socket_addr!=member_addr {
                self.send(member_addr, message.clone());
            }
        }
    }
}

/**
 * Create a new struct UdpConnection
 */
pub fn create_udp_connection((_id,_sock_addr): (String,String)) -> UdpConnection {
    println!("Listenning : {:?}",_sock_addr);
    UdpConnection {
        //name: "Udp connection".to_string(),
        socket: UdpSocket::bind(_sock_addr.parse::<SocketAddrV4>().unwrap()).unwrap(),
        port: _sock_addr.parse::<SocketAddrV4>().unwrap().port(),
        id: _id.parse::<u8>().unwrap(), //get_members().binary_search(&_sock_addr).unwrap().try_into().unwrap(),
        members: get_addresses(),
        n_members: get_addresses().len().try_into().unwrap(),
        socket_addr: _sock_addr.parse::<SocketAddrV4>().unwrap(),
    }
}


/**
 * Create copy of the UdpSocket
 */
pub fn get_connection_copy((_id,_sock_addr): (String,String)) -> (Arc<UdpConnection>,Arc<UdpConnection>) {
    if get_file().contains(&(_id.clone()+&","+&_sock_addr.clone())) {
        let socket = create_udp_connection((_id.clone(),_sock_addr.clone()));
        let shared_sock = Arc::new(socket);
        (shared_sock.clone(),shared_sock.clone())
    } else {
        panic!("Please chose a socket address that is contained in ressources/processus.txt");
    }
}

fn get_addresses() -> Vec<String> {
    let lines = get_file();
    let addresses:Vec<String> = lines.iter().map(|s| s.split_at(2).1.to_string()).collect();
    addresses
}

/**
 * Retrun the socket address of all the members of the network
 */
pub fn get_file() -> Vec<String> {
    let filename = "ressources/processus.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let res:Vec<String> = contents.split("\n").map(|s| s.to_string()).collect();
    res
}