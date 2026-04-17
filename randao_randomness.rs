use sha3::Sha3_512;

pub struct RandaoCommit {
    pub commitment: [u8; 64],
    pub reveal: Option<[u8; 64]>,
}

pub struct RandaoSystem {
    commits: Vec<RandaoCommit>,
}

impl RandaoSystem {
    pub fn new() -> Self {
        Self { commits: Vec::new() }
    }

    pub fn commit(&mut self, value: [u8; 64]) {
        let mut hasher = Sha3_512::new();
        hasher.update(value);
        let mut commit = [0u8; 64];
        commit.copy_from_slice(&hasher.finalize());
        self.commits.push(RandaoCommit {
            commitment: commit,
            reveal: None,
        });
    }

    pub fn final_randomness(&self) -> [u8; 64] {
        let mut res = [0u8; 64];
        for c in &self.commits {
            if let Some(r) = c.reveal {
                for i in 0..64 {
                    res[i] ^= r[i];
                }
            }
        }
        res
    }
}
