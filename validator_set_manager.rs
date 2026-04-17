use std::collections::BTreeSet;

pub struct ValidatorSet {
    active: BTreeSet<[u8; 20]>,
    pending: BTreeSet<[u8; 20]>,
    min_stake: u128,
}

impl ValidatorSet {
    pub fn new(min_stake: u128) -> Self {
        Self {
            active: BTreeSet::new(),
            pending: BTreeSet::new(),
            min_stake,
        }
    }

    pub fn join_pending(&mut self, addr: [u8; 20], stake: u128) -> bool {
        if stake >= self.min_stake {
            self.pending.insert(addr);
            true
        } else {
            false
        }
    }

    pub fn activate_pending(&mut self) {
        self.active.extend(self.pending.iter().cloned());
        self.pending.clear();
    }
}
