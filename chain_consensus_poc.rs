use std::collections::HashMap;

pub struct ConsensusNode {
    pub node_id: u64,
    pub stake: u128,
    pub is_active: bool,
}

pub struct PoSConsensus {
    nodes: HashMap<u64, ConsensusNode>,
    total_stake: u128,
}

impl PoSConsensus {
    pub fn new() -> Self {
        Self { nodes: HashMap::new(), total_stake: 0 }
    }

    pub fn register_node(&mut self, node_id: u64, stake: u128) {
        self.nodes.insert(node_id, ConsensusNode {
            node_id,
            stake,
            is_active: true,
        });
        self.total_stake += stake;
    }

    pub fn select_validator(&self) -> Option<u64> {
        if self.total_stake == 0 { return None; }
        let mut selected = None;
        let mut threshold = 0;
        let rand_val = rand::random::<u128>() % self.total_stake;
        
        for (id, node) in &self.nodes {
            if node.is_active {
                threshold += node.stake;
                if threshold > rand_val {
                    selected = Some(*id);
                    break;
                }
            }
        }
        selected
    }
}
