use std::collections::HashMap;

pub struct Delegation {
    pub delegator: [u8; 20],
    pub validator: [u8; 20],
    pub amount: u128,
    pub lock_until: u64,
}

pub struct StakeDelegationSystem {
    delegations: HashMap<[u8; 20], Vec<Delegation>>,
}

impl StakeDelegationSystem {
    pub fn new() -> Self {
        Self { delegations: HashMap::new() }
    }

    pub fn delegate(&mut self, del: Delegation) {
        self.delegations.entry(del.delegator).or_default().push(del);
    }

    pub fn total_staked(&self, validator: &[u8; 20]) -> u128 {
        let mut total = 0;
        for list in self.delegations.values() {
            for d in list {
                if &d.validator == validator {
                    total += d.amount;
                }
            }
        }
        total
    }
}
