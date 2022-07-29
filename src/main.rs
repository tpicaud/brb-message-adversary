mod net;
mod user;
mod mbrb;
mod crypto;
mod main_backend;


fn main() {

    main_backend::start();

    let (connections,mbrb) = main_backend::initiate_process();

    main_backend::receive(connections.0, mbrb.0);             // New thread

    main_backend::listen_command(connections.1, mbrb.1);      // Blocking instruction
}