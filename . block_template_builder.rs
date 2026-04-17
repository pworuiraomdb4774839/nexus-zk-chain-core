pub struct TxPriority {
    pub tx_id: [u8; 32],
    pub gas_price: u128,
}

pub struct BlockTemplate {
    pub height: u64,
    pub txs: Vec<[u8; 32]>,
    pub total_gas: u64,
}

pub struct TemplateBuilder;

impl TemplateBuilder {
    pub fn build(height: u64, mut priorities: Vec<TxPriority>, max_gas: u64) -> BlockTemplate {
        priorities.sort_by(|a, b| b.gas_price.cmp(&a.gas_price));
        let mut txs = Vec::new();
        let mut gas = 0;
        for p in priorities {
            if gas + 21000 <= max_gas {
                txs.push(p.tx_id);
                gas += 21000;
            }
        }
        BlockTemplate { height, txs, total_gas: gas }
    }
}
