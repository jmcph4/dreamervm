use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::common::types::Word;

pub trait LinearlyAddressable {
    fn read(&self, address: Word) -> Word;
    fn write(&mut self, address: Word, data: Word);
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HashMemory(HashMap<Word, Word>);

impl Default for HashMemory {
    fn default() -> Self {
        HashMemory::new()
    }
}

impl HashMemory {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
}

impl LinearlyAddressable for HashMemory {
    fn read(&self, address: Word) -> Word {
        if self.0.contains_key(&address) {
            self.0.get(&address).copied().unwrap()
        } else {
            Default::default()
        }
    }

    fn write(&mut self, address: Word, data: Word) {
        self.0.insert(address, data);
    }
}

pub type Memory = HashMemory;
