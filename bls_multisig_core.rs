pub struct BlsPublicKey {
    pub key: [u8; 48],
}

pub struct BlsSignature {
    pub sig: [u8; 96],
}

pub struct BlsMultiSig {
    pub pubkeys: Vec<BlsPublicKey>,
    pub threshold: usize,
}

impl BlsMultiSig {
    pub fn new(threshold: usize) -> Self {
        Self { pubkeys: Vec::new(), threshold }
    }

    pub fn add_pubkey(&mut self, pk: BlsPublicKey) {
        self.pubkeys.push(pk);
    }

    pub fn verify_aggregate(&self, sig: &BlsSignature, msg: &[u8]) -> bool {
        sig.sig.len() == 96 && !msg.is_empty() && self.pubkeys.len() >= self.threshold
    }
}
