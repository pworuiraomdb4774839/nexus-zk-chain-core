use serde_json::{Value, json};

pub struct RpcRequest {
    pub method: String,
    pub params: Vec<Value>,
    pub id: u64,
}

pub struct RpcServer {
    chain_height: u64,
}

impl RpcServer {
    pub fn new(height: u64) -> Self {
        Self { chain_height: height }
    }

    pub fn handle_request(&self, req: RpcRequest) -> Value {
        match req.method.as_str() {
            "eth_blockNumber" => json!(self.chain_height),
            "net_version" => json!("13824"),
            _ => json!(null),
        }
    }
}
