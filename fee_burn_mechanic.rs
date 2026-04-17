pub struct FeeBurnSystem {
    total_burned: u128,
    burn_rate: u8,
}

impl FeeBurnSystem {
    pub fn new(rate: u8) -> Self {
        Self { total_burned: 0, burn_rate: rate }
    }

    pub fn process_fee(&mut self, fee: u128) -> u128 {
        let burn = fee * self.burn_rate as u128 / 100;
        self.total_burned += burn;
        fee - burn
    }

    pub fn get_burned_total(&self) -> u128 {
        self.total_burned
    }
}
