use sha3::Sha3_256;

pub struct BlockHeader {
    pub version: u32,
    pub prev_hash: [u8; 32],
    pub merkle_root: [u8; 32],
    pub timestamp: u64,
    pub difficulty: u32,
    pub nonce: u64,
}

impl BlockHeader {
    pub fn compute_hash(&self) -> [u8; 32] {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.version.to_be_bytes());
        bytes.extend_from_slice(&self.prev_hash);
        bytes.extend_from_slice(&self.merkle_root);
        bytes.extend_from_slice(&self.timestamp.to_be_bytes());
        bytes.extend_from_slice(&self.difficulty.to_be_bytes());
        bytes.extend_from_slice(&self.nonce.to_be_bytes());
        
        let mut hasher = Sha3_256::new();
        hasher.update(bytes);
        let mut res = [0u8; 32];
        res.copy_from_slice(&hasher.finalize());
        res
    }
}
