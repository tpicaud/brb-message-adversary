use rand_core::OsRng;

use p256::{ecdsa::{
    Signature,
    SigningKey, signature::Signer,
    VerifyingKey, signature::Verifier
}, EncodedPoint};

use super::iface_crypto::{KeyPair, PrivKey, PubKey };



/**
 * KeyPair implementation
 */
pub struct EcdsaKeyPair;

impl KeyPair for EcdsaKeyPair {
    fn generate(&self) -> (Box<dyn PrivKey>, Box<dyn PubKey>) {
        let sig_key = SigningKey::random(&mut OsRng);
        let pub_key = EcdsaPubKey { pub_key: VerifyingKey::from(&sig_key) };
        let priv_key = EcdsaPrivKey { priv_key: sig_key };
        (Box::new(priv_key), Box::new(pub_key))
    }
}


/**
 * Private key implementation
 */
pub struct EcdsaPrivKey {
    priv_key: SigningKey
}

impl PrivKey for EcdsaPrivKey {
    fn sign(&self, msg: Vec<u8>) -> Vec<u8> {
        self.priv_key.sign(&msg[..]).to_der().as_bytes().to_vec()
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.priv_key.to_bytes().to_vec()
    }
}

pub fn ecdsa_priv_key_from_bytes(b: &[u8]) -> EcdsaPrivKey {
    match SigningKey::from_bytes(b) {
        Ok(key) => EcdsaPrivKey { priv_key: key },
        Err(err) => panic!("{}", err)
    }
}


/**
 * Public key implementation
 */
pub struct EcdsaPubKey {
    pub_key: VerifyingKey
}

impl PubKey for EcdsaPubKey {

    fn verify(&self, msg: Vec<u8>, sig: Vec<u8>) -> bool {
        let res = Signature::from_der(&sig[..]);
        match res {
            Ok(signature) => self.pub_key.verify(&msg[..], &signature).is_ok(),
            Err(_) => false
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        VerifyingKey::to_encoded_point(&self.pub_key, true).as_bytes().to_vec()
    }

}

pub fn ecdsa_pub_key_from_bytes(b: &[u8]) -> EcdsaPubKey {
    match EncodedPoint::from_bytes(b) {
        Ok(point) => {
            match VerifyingKey::from_encoded_point(&point) {
                Ok(key) => EcdsaPubKey { pub_key: key },
                Err(err) => panic!("{}", err)
            }
        },
        Err(err) => panic!("{}", err)
    }
}
