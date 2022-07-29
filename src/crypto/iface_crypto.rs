pub trait KeyPair {
    fn generate(&self) -> (Box<dyn PrivKey>, Box<dyn PubKey>);
}

pub trait PrivKey {
    fn sign(&self, msg: Vec<u8>) -> Vec<u8>;

    fn to_bytes(&self) -> Vec<u8>;
}

pub trait PubKey {
    fn verify(&self, msg: Vec<u8>, sig: Vec<u8>) -> bool;

    fn to_bytes(&self) -> Vec<u8>;
}
