pub struct BytecodeEncoder {
    code: Vec<u8>,
}

impl BytecodeEncoder {
    pub fn new() -> Self {
        Self { code: Vec::new() }
    }

    pub fn push_u64(&mut self, v: u64) {
        self.code.push(0x60 + (v.to_be_bytes().len() as u8 - 1));
        self.code.extend_from_slice(&v.to_be_bytes());
    }

    pub fn add(&mut self) {
        self.code.push(0x01);
    }

    pub fn mul(&mut self) {
        self.code.push(0x02);
    }

    pub fn finish(&self) -> Vec<u8> {
        self.code.clone()
    }
}
