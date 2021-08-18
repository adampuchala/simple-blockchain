use std::convert::TryInto;

use rand::rngs::OsRng;

use rsa::{Hash, PublicKey, RsaPrivateKey, RsaPublicKey};
use super::{RSA2048DigitalSign, Sha256Digest};

const RSA_BITS: usize = 2048;

pub trait RSA2048Provider {
    fn generate_rsa_keypair() -> (RsaPublicKey, RsaPrivateKey);
    fn digitally_sign_sha256(private_key: &RsaPrivateKey, digest: &Sha256Digest) -> RSA2048DigitalSign;
    fn get_public_key_from_private_key(private_key: &RsaPrivateKey) -> RsaPublicKey;
    fn validate_signed(signed_data: &[u8], sign: &RSA2048DigitalSign, public_key: &RsaPublicKey) -> bool;
}

pub struct RSA2048Util;

impl RSA2048Provider for RSA2048Util {
    fn generate_rsa_keypair() -> (RsaPublicKey, RsaPrivateKey) {
        let mut rng = OsRng;
        let private_key =  RsaPrivateKey::new(&mut rng, RSA_BITS).expect("private key generation fail!!!");
        (RsaPublicKey::from(&private_key), private_key)
    }

    fn digitally_sign_sha256(private_key: &RsaPrivateKey, digest: &Sha256Digest) -> RSA2048DigitalSign {
        let padding = rsa::PaddingScheme::new_pkcs1v15_sign(Some(Hash::SHA2_256));
        private_key.sign(padding, digest).unwrap().try_into().expect("Digitally sign generation failed!")
    }

    fn get_public_key_from_private_key(private_key: &RsaPrivateKey) -> RsaPublicKey {
        RsaPublicKey::from(private_key)
    }

    fn validate_signed(signed_data: &[u8], sign: &RSA2048DigitalSign, public_key: &RsaPublicKey) -> bool {
        let padding = rsa::PaddingScheme::new_pkcs1v15_sign(Some(Hash::SHA2_256));
        public_key.verify(padding, signed_data, sign).is_ok()
    }
}