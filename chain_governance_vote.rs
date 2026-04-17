use std::collections::HashMap;

pub enum VoteChoice {
    Yes,
    No,
    Abstain,
}

pub struct Proposal {
    pub id: u64,
    pub title: String,
    pub votes: HashMap<[u8; 20], VoteChoice>,
}

pub struct Governance {
    proposals: HashMap<u64, Proposal>,
}

impl Governance {
    pub fn new() -> Self {
        Self { proposals: HashMap::new() }
    }

    pub fn create_proposal(&mut self, id: u64, title: String) {
        self.proposals.insert(id, Proposal {
            id,
            title,
            votes: HashMap::new(),
        });
    }

    pub fn cast_vote(&mut self, pid: u64, voter: [u8; 20], choice: VoteChoice) {
        if let Some(p) = self.proposals.get_mut(&pid) {
            p.votes.insert(voter, choice);
        }
    }
}
