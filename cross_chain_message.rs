use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CrossChainMsg {
    pub source_chain_id: u32,
    pub dest_chain_id: u32,
    pub sender: [u8; 20],
    pub receiver: [u8; 20],
    pub payload: Vec<u8>,
    pub timestamp: u64,
}

impl CrossChainMsg {
    pub fn new(source: u32, dest: u32) -> Self {
        Self {
            source_chain_id: source,
            dest_chain_id: dest,
            sender: [0u8; 20],
            receiver: [0u8; 20],
            payload: Vec::new(),
            timestamp: 0,
        }
    }

    pub fn set_payload(&mut self, data: Vec<u8>) {
        self.payload = data;
    }
}
