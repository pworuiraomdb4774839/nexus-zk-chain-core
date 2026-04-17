use std::collections::HashMap;

pub struct DhtNode {
    pub id: [u8; 32],
    pub addr: String,
}

pub struct DhtRoutingTable {
    nodes: HashMap<[u8; 32], DhtNode>,
    buckets: Vec<Vec<[u8; 32]>>,
}

impl DhtRoutingTable {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            buckets: vec![Vec::new(); 256],
        }
    }

    pub fn add_node(&mut self, node: DhtNode) {
        let bucket = (node.id[0] % 256) as usize;
        self.buckets[bucket].push(node.id);
        self.nodes.insert(node.id, node);
    }

    pub fn find_closest(&self, target: [u8; 32], count: usize) -> Vec<&DhtNode> {
        let b = (target[0] % 256) as usize;
        self.buckets[b].iter()
            .filter_map(|id| self.nodes.get(id))
            .take(count)
            .collect()
    }
}
