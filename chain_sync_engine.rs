pub struct SyncSession {
    pub peer_addr: String,
    pub start_height: u64,
    pub end_height: u64,
    pub downloaded: u64,
}

pub struct ChainSyncEngine {
    sessions: Vec<SyncSession>,
    local_height: u64,
}

impl ChainSyncEngine {
    pub fn new(local_height: u64) -> Self {
        Self { sessions: Vec::new(), local_height }
    }

    pub fn start_sync(&mut self, peer: String, target: u64) {
        self.sessions.push(SyncSession {
            peer_addr: peer,
            start_height: self.local_height,
            end_height: target,
            downloaded: 0,
        });
    }

    pub fn progress(&self) -> f64 {
        let total: u64 = self.sessions.iter().map(|s| s.end_height - s.start_height).sum();
        let done: u64 = self.sessions.iter().map(|s| s.downloaded).sum();
        if total == 0 { 0.0 } else { done as f64 / total as f64 }
    }
}
