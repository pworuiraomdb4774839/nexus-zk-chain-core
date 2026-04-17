use std::collections::BTreeMap;

pub struct AccountState {
    pub nonce: u64,
    pub balance: u128,
    pub code_hash: [u8; 32],
    pub storage_root: [u8; 32],
}

pub struct StateTrie {
    store: BTreeMap<[u8; 20], AccountState>,
}

impl StateTrie {
    pub fn new() -> Self {
        Self { store: BTreeMap::new() }
    }

    pub fn set_account(&mut self, address: [u8; 20], state: AccountState) {
        self.store.insert(address, state);
    }

    pub fn get_account(&self, address: &[u8; 20]) -> Option<&AccountState> {
        self.store.get(address)
    }

    pub fn increment_nonce(&mut self, address: &[u8; 20]) {
        if let Some(acc) = self.store.get_mut(address) {
            acc.nonce += 1;
        }
    }
}
