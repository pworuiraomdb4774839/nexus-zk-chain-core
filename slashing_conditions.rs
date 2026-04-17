use std::collections::HashSet;

pub struct SlashEvent {
    pub validator: [u8; 20],
    pub reason: String,
    pub penalty_pct: u8,
}

pub struct SlashingEngine {
    punished: HashSet<[u8; 20]>,
}

impl SlashingEngine {
    pub fn new() -> Self {
        Self { punished: HashSet::new() }
    }

    pub fn check_double_sign(&self, h1: u64, h2: u64) -> bool {
        h1 == h2
    }

    pub fn apply_slash(&mut self, event: SlashEvent) -> u128 {
        self.punished.insert(event.validator);
        let base = 1000000000000u128;
        base * event.penalty_pct as u128 / 100
    }
}
