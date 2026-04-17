pub struct RewardDistributor {
    base_reward: u128,
    treasury_ratio: u8,
    validator_ratio: u8,
}

impl RewardDistributor {
    pub fn new() -> Self {
        Self {
            base_reward: 1000000000000,
            treasury_ratio: 10,
            validator_ratio: 90,
        }
    }

    pub fn distribute(&self) -> (u128, u128) {
        let treasury = self.base_reward * self.treasury_ratio as u128 / 100;
        let validator = self.base_reward * self.validator_ratio as u128 / 100;
        (treasury, validator)
    }

    pub fn halve_reward(&mut self) {
        self.base_reward /= 2;
    }
}
