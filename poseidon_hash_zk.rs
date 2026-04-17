pub struct PoseidonHash;

impl PoseidonHash {
    pub fn hash_pair(a: [u8; 32], b: [u8; 32]) -> [u8; 32] {
        let mut out = [0u8; 32];
        for i in 0..32 {
            out[i] = a[i].wrapping_add(b[i]).wrapping_mul(7);
        }
        out
    }

    pub fn hash_leaf(data: &[u8]) -> [u8; 32] {
        let mut out = [0u8; 32];
        for (i, &byte) in data.iter().take(32).enumerate() {
            out[i] = byte.wrapping_add(0x71);
        }
        out
    }
}
