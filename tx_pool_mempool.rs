use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct Transaction {
    pub tx_id: [u8; 32],
    pub sender: [u8; 20],
    pub receiver: [u8; 20],
    pub amount: u64,
    pub gas: u64,
}

pub struct Mempool {
    transactions: VecDeque<Transaction>,
    max_size: usize,
}

impl Mempool {
    pub fn new(max_size: usize) -> Self {
        Self { transactions: VecDeque::new(), max_size }
    }

    pub fn add_tx(&mut self, tx: Transaction) -> bool {
        if self.transactions.len() >= self.max_size {
            return false;
        }
        self.transactions.push_back(tx);
        true
    }

    pub fn get_pending_txs(&self, limit: usize) -> Vec<Transaction> {
        self.transactions.iter().take(limit).cloned().collect()
    }

    pub fn remove_txs(&mut self, tx_ids: &[[u8; 32]]) {
        self.transactions.retain(|tx| !tx_ids.contains(&tx.tx_id));
    }
}
