pub struct BlockImportStep {
    pub verify_header: bool,
    pub verify_txs: bool,
    pub update_state: bool,
    pub broadcast: bool,
}

pub struct BlockImporter {
    pub last_imported_height: u64,
}

impl BlockImporter {
    pub fn new() -> Self {
        Self { last_imported_height: 0 }
    }

    pub fn execute_pipeline(&mut self, height: u64, steps: BlockImportStep) -> bool {
        if !steps.verify_header || !steps.verify_txs {
            return false;
        }
        if steps.update_state {
            self.last_imported_height = height;
        }
        true
    }
}
