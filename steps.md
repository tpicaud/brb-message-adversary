# Steps

## **1.** Learn Rust

### clone [github/rustlings](https://github.com/rust-lang/rustlings).
- Some exercises that you need to solve by fixing errors in small codes. It's a good way to learn Rust.


## **2.** Explore crates (=librairies)

### *Objective : Search for crates to do the network stuff.*
- tokio (too complex)
- std::net


## **3.** Create simple client/server program

### *Objective : Create an echo server that can receive a message and send it back.*
- use of std::net
- recover main() aguments
- learn about thread
- learn about Option / Result (very common in Rust)


## **4.** v1 : First version of the mbrb_algorithm

### *Objective : Create a client that allow two processes to communicate*
- use of TCP
- learn about ownership in Rust
- learn about modules and Rust project structure


## **5.** v2

### *Objective : Allow more than 3 processes to communicate*
- introduce broadcast
- use of UDP
- learn about std::sync::Arc to share data
- create text file with ipv4:port addresses to simulate the network
- code a test script to launch multiple processes


## **6.** Introduce cryptography

### *Objective : create a generic structure for keypairs generation, signature and messages verification*
- use of ECDSA (p256 crate)
- create a trait with implementation
- create files for keypairs storage (1 file for every pub_key and 1 file for each priv_key)

## **7.** v3 : complete version of the algorithm (still unstable)

### *Objective : run the reliable broadcast on multiple processes without byzantines*

- identify each processes and give them keypairs
- create Bundle (m,sn,i,sigs)
- enable Bundle transfer (encoding, decoding)
- sign/verify messages
- store delivered messages in a file

## **8.** Fix algorithm by storing locally the signatures
## *Objective: introduce csv and save signatures locally in hexadecimal for better readability*
- use of *csv* crate 
- use of *hex* crate for conversion to/from bytes
- use of *file_indexing* crate for a better file modification flexibility
- give a "id_saved_signatures.csv" file for each processus

## **9.** Add log files and message adversary
## *Objective: improve the algorithm*
- use of log4rs crate
- use of rand crate to simulate message adversary