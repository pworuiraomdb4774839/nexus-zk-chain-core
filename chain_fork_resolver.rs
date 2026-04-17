use std::collections::HashMap;

pub struct Fork {
    pub fork_hash: [u8; 32],
    pub height: u64,
    pub weight: u128,
}

pub struct ForkResolver {
    forks: HashMap<[u8; 32], Fork>,
    canonical_hash: [u8; 32],
}

impl ForkResolver {
    pub fn new(canon: [u8; 32]) -> Self {
        Self { forks: HashMap::new(), canonical_hash: canon }
    }

    pub fn register_fork(&mut self, fork: Fork) {
        self.forks.insert(fork.fork_hash, fork);
    }

    pub fn choose_canonical(&mut self) -> [u8; 32] {
        let mut best = self.canonical_hash;
        let mut best_weight = 0;
        for f in self.forks.values() {
            if f.weight > best_weight {
                best_weight = f.weight;
                best = f.fork_hash;
            }
        }
        self.canonical_hash = best;
        best
    }
}
