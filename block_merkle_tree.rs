use sha2::{Sha256, Digest};

#[derive(Debug, Clone)]
pub struct MerkleTree {
    leaf_nodes: Vec<[u8; 32]>,
    root_hash: [u8; 32],
}

impl MerkleTree {
    pub fn build(leaves: Vec<[u8; 32]>) -> Self {
        let mut nodes = leaves.clone();
        while nodes.len() > 1 {
            let mut next_level = Vec::new();
            for chunk in nodes.chunks(2) {
                let left = chunk[0];
                let right = chunk.get(1).copied().unwrap_or(left);
                let mut hasher = Sha256::new();
                hasher.update(left);
                hasher.update(right);
                let mut hash = [0u8; 32];
                hash.copy_from_slice(&hasher.finalize());
                next_level.push(hash);
            }
            nodes = next_level;
        }
        let root = nodes[0];
        Self { leaf_nodes: leaves, root_hash: root }
    }

    pub fn root(&self) -> [u8; 32] {
        self.root_hash
    }
}
