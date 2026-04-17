use std::collections::HashMap;

pub struct ShardRouter {
    shard_count: u32,
    account_shard: HashMap<[u8; 20], u32>,
}

impl ShardRouter {
    pub fn new(shard_count: u32) -> Self {
        Self {
            shard_count,
            account_shard: HashMap::new(),
        }
    }

    pub fn assign_shard(&mut self, addr: [u8; 20]) -> u32 {
        let shard = (addr[0] as u32) % self.shard_count;
        self.account_shard.insert(addr, shard);
        shard
    }

    pub fn get_shard(&self, addr: &[u8; 20]) -> Option<u32> {
        self.account_shard.get(addr).copied()
    }
}
