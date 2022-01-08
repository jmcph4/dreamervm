use serde::{Deserialize, Serialize};

use crate::common::types::Word;

pub const MAX_STACK_DEPTH: usize = 65535;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum StackError {
    Full,
    Empty,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Stack(Vec<Word>);

impl Default for Stack {
    fn default() -> Self {
        Self::new()
    }
}

impl Stack {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn push(&mut self, elem: Word) -> Result<usize, StackError> {
        if self.0.len() == MAX_STACK_DEPTH {
            Err(StackError::Full)
        } else {
            self.0.push(elem);
            Ok(self.0.len())
        }
    }

    pub fn pop(&mut self) -> Result<Word, StackError> {
        self.0.pop().ok_or(StackError::Empty)
    }

    pub fn peek(&self) -> Option<Word> {
        self.0.get(0).copied()
    }

    pub fn depth(&self) -> usize {
        self.0.len()
    }

    pub fn full(&self) -> bool {
        self.0.len() == MAX_STACK_DEPTH
    }

    pub fn empty(&self) -> bool {
        self.0.is_empty()
    }
}
