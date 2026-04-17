use std::net::SocketAddr;
use std::collections::HashSet;

pub struct PeerNode {
    pub addr: SocketAddr,
    pub version: String,
    pub height: u64,
}

pub struct P2PNetwork {
    peers: HashSet<PeerNode>,
    max_peers: usize,
}

impl P2PNetwork {
    pub fn new(max_peers: usize) -> Self {
        Self { peers: HashSet::new(), max_peers }
    }

    pub fn add_peer(&mut self, peer: PeerNode) -> bool {
        if self.peers.len() >= self.max_peers {
            return false;
        }
        self.peers.insert(peer);
        true
    }

    pub fn broadcast_height(&self, new_height: u64) {
        for peer in &self.peers {
            println!("Send height {} to peer {}", new_height, peer.addr);
        }
    }

    pub fn active_peers_count(&self) -> usize {
        self.peers.len()
    }
}
