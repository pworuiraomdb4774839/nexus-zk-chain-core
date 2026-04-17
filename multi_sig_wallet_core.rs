use std::collections::HashMap;

pub struct MultiSigWallet {
    owners: Vec<[u8; 20]>,
    threshold: usize,
    transactions: HashMap<u64, Vec<[u8; 20]>>,
}

impl MultiSigWallet {
    pub fn new(owners: Vec<[u8; 20]>, threshold: usize) -> Self {
        Self {
            owners,
            threshold,
            transactions: HashMap::new(),
        }
    }

    pub fn submit_tx(&mut self, tx_id: u64, sender: [u8; 20]) -> bool {
        self.owners.contains(&sender)
    }

    pub fn confirm_tx(&mut self, tx_id: u64, owner: [u8; 20]) {
        self.transactions.entry(tx_id).or_default().push(owner);
    }
}
