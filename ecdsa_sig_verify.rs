use secp256k1::{Secp256k1, Message, Signature, PublicKey};

pub fn verify_signature(
    pubkey: &PublicKey,
    msg_hash: &[u8; 32],
    sig: &Signature
) -> bool {
    let secp = Secp256k1::new();
    let msg = Message::from_digest_slice(msg_hash).unwrap();
    secp.verify_ecdsa(&msg, sig, pubkey).is_ok()
}

pub fn hash_message(msg: &[u8]) -> [u8; 32] {
    use sha2::Sha256;
    let mut hasher = Sha256::new();
    hasher.update(msg);
    let mut res = [0u8; 32];
    res.copy_from_slice(&hasher.finalize());
    res
}
