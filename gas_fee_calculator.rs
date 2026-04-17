pub struct GasCalculator {
    base_fee: u64,
    gas_per_byte: u64,
    gas_per_op: u64,
}

impl GasCalculator {
    pub fn new() -> Self {
        Self {
            base_fee: 21000,
            gas_per_byte: 10,
            gas_per_op: 5,
        }
    }

    pub fn calculate_tx_gas(&self, data_len: usize, op_count: u64) -> u64 {
        self.base_fee + (data_len as u64 * self.gas_per_byte) + (op_count * self.gas_per_op)
    }

    pub fn calculate_fee(&self, gas: u64, gas_price: u128) -> u128 {
        gas as u128 * gas_price
    }
}
