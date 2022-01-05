use std::fmt;

use serde::{Deserialize, Serialize};
use serde_json;

use crate::common::types::Word;
use crate::core::memory::Memory;
use crate::core::stack::Stack;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct State {
    pub pc: Word,
    pub reg: Word,
    pub stack: Stack,
    pub memory: Memory,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let serialised: String = serde_json::to_string(&self).unwrap();
        write!(f, "{}", serialised)
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            pc: Word::default(),
            reg: Word::default(),
            stack: Stack::default(),
            memory: Memory::new(),
        }
    }
}

impl State {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn program_counter(&self) -> Word {
        self.pc
    }
}
