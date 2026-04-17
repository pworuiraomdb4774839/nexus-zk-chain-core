use secp256k1::{SecretKey, PublicKey, Secp256k1};
use rand::rngs::OsRng;

pub struct WalletKeyPair {
    secret: SecretKey,
    public: PublicKey,
}

impl WalletKeyPair {
    pub fn generate() -> Self {
        let secp = Secp256k1::new();
        let (secret, public) = secp.generate_keypair(&mut OsRng);
        Self { secret, public }
    }

    pub fn public_key_hex(&self) -> String {
        hex::encode(&self.public.serialize())
    }

    pub fn sign_message(&self, msg: &[u8]) -> Vec<u8> {
        let secp = Secp256k1::new();
        let sig = secp.sign_ecdsa(msg, &self.secret);
        sig.serialize_der().to_vec()
    }
}
