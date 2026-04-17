use sha2::Sha512;

pub struct VrfOutput {
    pub randomness: [u8; 64],
    pub proof: [u8; 64],
}

pub struct VrfProvider;

impl VrfProvider {
    pub fn generate(secret: &[u8; 32], msg: &[u8]) -> VrfOutput {
        let mut hasher = Sha512::new();
        hasher.update(secret);
        hasher.update(msg);
        let mut rand = [0u8; 64];
        rand.copy_from_slice(&hasher.finalize());
        
        let mut proof = [0u8; 64];
        for i in 0..64 {
            proof[i] = rand[i].wrapping_add(0x33);
        }
        
        VrfOutput { randomness: rand, proof }
    }
}
