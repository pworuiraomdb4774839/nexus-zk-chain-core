use std::collections::HashMap;

pub struct FungibleToken {
    total_supply: u128,
    balances: HashMap<[u8; 20], u128>,
    allowances: HashMap<([u8; 20], [u8; 20]), u128>,
}

impl FungibleToken {
    pub fn new(initial_supply: u128, owner: [u8; 20]) -> Self {
        let mut balances = HashMap::new();
        balances.insert(owner, initial_supply);
        Self {
            total_supply: initial_supply,
            balances,
            allowances: HashMap::new(),
        }
    }

    pub fn transfer(&mut self, from: [u8; 20], to: [u8; 20], amount: u128) -> bool {
        let f_bal = self.balances.get(&from).copied().unwrap_or(0);
        if f_bal < amount { return false; }
        *self.balances.get_mut(&from).unwrap() -= amount;
        *self.balances.entry(to).or_insert(0) += amount;
        true
    }
}
