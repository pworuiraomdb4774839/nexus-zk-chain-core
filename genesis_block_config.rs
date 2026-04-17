pub struct GenesisAccount {
    pub address: [u8; 20],
    pub balance: u128,
}

pub struct GenesisConfig {
    pub chain_id: u32,
    pub initial_height: u64,
    pub genesis_accounts: Vec<GenesisAccount>,
    pub genesis_hash: [u8; 32],
}

impl GenesisConfig {
    pub fn default() -> Self {
        Self {
            chain_id: 13824,
            initial_height: 0,
            genesis_accounts: Vec::new(),
            genesis_hash: [0u8; 32],
        }
    }

    pub fn add_genesis_account(&mut self, address: [u8; 20], balance: u128) {
        self.genesis_accounts.push(GenesisAccount { address, balance });
    }
}
