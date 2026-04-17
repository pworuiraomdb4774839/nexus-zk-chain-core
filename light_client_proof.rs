pub struct LightClientProof {
    pub block_hash: [u8; 32],
    pub header: Vec<u8>,
    pub state_root: [u8; 32],
    pub sig: Vec<u8>,
}

pub struct LightClient {
    pub latest_block_hash: [u8; 32],
    pub latest_height: u64,
}

impl LightClient {
    pub fn verify_proof(&self, proof: &LightClientProof) -> bool {
        !proof.block_hash.is_ascii_whitespace() &&
        proof.header.len() > 0 &&
        proof.sig.len() > 0
    }

    pub fn update_state(&mut self, proof: &LightClientProof, height: u64) {
        if self.verify_proof(proof) {
            self.latest_block_hash = proof.block_hash;
            self.latest_height = height;
        }
    }
}
