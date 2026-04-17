pub struct Relayer {
    pub supported_chains: Vec<u32>,
    pub fee_per_msg: u128,
    pub pending_messages: Vec<(u32, Vec<u8>)>,
}

impl Relayer {
    pub fn new(fee: u128) -> Self {
        Self {
            supported_chains: Vec::new(),
            fee_per_msg: fee,
            pending_messages: Vec::new(),
        }
    }

    pub fn add_chain(&mut self, chain_id: u32) {
        self.supported_chains.push(chain_id);
    }

    pub fn enqueue_msg(&mut self, chain_id: u32, msg: Vec<u8>) {
        self.pending_messages.push((chain_id, msg));
    }
}
