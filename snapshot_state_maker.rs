use std::collections::HashMap;

pub struct StateSnapshot {
    pub snapshot_id: [u8; 16],
    pub height: u64,
    pub accounts: HashMap<[u8; 20], [u8; 32]>,
}

pub struct SnapshotManager {
    snapshots: Vec<StateSnapshot>,
}

impl SnapshotManager {
    pub fn new() -> Self {
        Self { snapshots: Vec::new() }
    }

    pub fn create_snapshot(&mut self, height: u64, accounts: HashMap<[u8; 20], [u8; 32]>) {
        self.snapshots.push(StateSnapshot {
            snapshot_id: rand::random(),
            height,
            accounts,
        });
    }

    pub fn get_latest(&self) -> Option<&StateSnapshot> {
        self.snapshots.last()
    }
}
