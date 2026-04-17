#[derive(Clone)]
pub struct BatchTx {
    pub batch_id: [u8; 16],
    pub txs: Vec<[u8; 32]>,
    pub submitter: [u8; 20],
    pub gas_saved: u64,
}

pub struct BatchExecutor {
    batches: Vec<BatchTx>,
}

impl BatchExecutor {
    pub fn new() -> Self {
        Self { batches: Vec::new() }
    }

    pub fn create_batch(&mut self, tx_ids: Vec<[u8; 32]>, submitter: [u8; 20]) -> BatchTx {
        let batch = BatchTx {
            batch_id: rand::random(),
            txs: tx_ids,
            submitter,
            gas_saved: rand::random::<u64>() % 100000,
        };
        self.batches.push(batch.clone());
        batch
    }
}
