use std::net::SocketAddrV4;

pub trait Network {

    /**
     * Waits for messages and returns it when one is received
     */
    fn receive(&self) -> Vec<u8>;

    /**
     * send message to a single consumer
     */
    fn send(&self,ip: SocketAddrV4, message: Vec<u8>);

    /**
     * Broadcast message
     */
    fn ur_broadcast(&self, message: Vec<u8>);

}