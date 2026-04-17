use std::collections::HashMap;

pub enum VmOp {
    Push(u64),
    Add,
    Sub,
    Mul,
    Store([u8; 32]),
    Load([u8; 32]),
}

pub struct MiniVM {
    stack: Vec<u64>,
    storage: HashMap<[u8; 32], u64>,
}

impl MiniVM {
    pub fn new() -> Self {
        Self { stack: Vec::new(), storage: HashMap::new() }
    }

    pub fn execute(&mut self, ops: &[VmOp]) {
        for op in ops {
            match op {
                VmOp::Push(v) => self.stack.push(*v),
                VmOp::Add => {
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    self.stack.push(a + b);
                }
                VmOp::Store(k) => {
                    let v = self.stack.pop().unwrap();
                    self.storage.insert(*k, v);
                }
                _ => {}
            }
        }
    }
}
