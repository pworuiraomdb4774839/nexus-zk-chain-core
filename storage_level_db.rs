use std::collections::BTreeMap;

pub struct LevelDbLikeStore {
    data: BTreeMap<Vec<u8>, Vec<u8>>,
}

impl LevelDbLikeStore {
    pub fn new() -> Self {
        Self { data: BTreeMap::new() }
    }

    pub fn put(&mut self, key: Vec<u8>, value: Vec<u8>) {
        self.data.insert(key, value);
    }

    pub fn get(&self, key: &[u8]) -> Option<&Vec<u8>> {
        self.data.get(key)
    }

    pub fn delete(&mut self, key: &[u8]) {
        self.data.remove(key);
    }

    pub fn iter_range(&self, start: &[u8], end: &[u8]) -> impl Iterator<Item = (&Vec<u8>, &Vec<u8>)> {
        self.data.range(start.to_vec()..=end.to_vec())
    }
}
