use crate::common::types::Word;
use crate::core::memory::Memory;
use crate::core::stack::Stack;

#[derive(Clone, Debug)]
pub struct State {
    pub pc: Word,
    pub reg: Word,
    pub stack: Stack,
    pub memory: Memory,
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
