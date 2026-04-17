pub struct ChainMetrics {
    pub block_count: u64,
    pub tx_count: u64,
    pub avg_block_time_ms: u64,
    pub total_gas_used: u128,
}

impl ChainMetrics {
    pub fn new() -> Self {
        Self {
            block_count: 0,
            tx_count: 0,
            avg_block_time_ms: 0,
            total_gas_used: 0,
        }
    }

    pub fn record_block(&mut self, txs: u64, gas: u128, duration: u64) {
        self.block_count += 1;
        self.tx_count += txs;
        self.total_gas_used += gas;
        self.avg_block_time_ms = (self.avg_block_time_ms * (self.block_count - 1) + duration) / self.block_count;
    }
}
