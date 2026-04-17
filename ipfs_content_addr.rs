use sha2::Sha256;
use multihash::{Code, MultihashDigest};

pub struct IpfsContent {
    pub data: Vec<u8>,
}

impl IpfsContent {
    pub fn new(data: Vec<u8>) -> Self {
        Self { data }
    }

    pub fn cid_v0(&self) -> String {
        let hash = Code::Sha2_256.digest(&self.data);
        let cid_bytes = [&[0x12, 0x20], hash.digest()].concat();
        base58::encode(&cid_bytes)
    }
}
