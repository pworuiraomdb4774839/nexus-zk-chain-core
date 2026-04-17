use rand::Rng;
use sha3::{Sha3_256, Digest};

pub struct ZkProof {
    pub commitment: [u8; 32],
    pub proof_data: Vec<u8>,
}

impl ZkProof {
    pub fn generate_random_proof() -> Self {
        let mut rng = rand::thread_rng();
        let mut commit = [0u8; 32];
        rng.fill(&mut commit);
        
        let mut proof = Vec::with_capacity(64);
        for _ in 0..64 {
            proof.push(rng.gen::<u8>());
        }
        
        Self { commitment: commit, proof_data: proof }
    }

    pub fn verify_commitment(&self) -> bool {
        let mut hasher = Sha3_256::new();
        hasher.update(&self.proof_data[0..32]);
        let result = hasher.finalize();
        result.as_slice() == &self.commitment
    }
}
