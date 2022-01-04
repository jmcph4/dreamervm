use crate::common::types::{word_bytes, Word};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Instruction {
    Nop,
    Halt,
    Load,
    Store,
    Push,
    Pop,
    Set(Word),
    Read,
    Write,
    Jump,
    JumpIf,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Cmp,
    And,
    Or,
    Not,
    Xor,
}

#[derive(Clone, Copy, Debug)]
pub enum InstructionParseError {
    NoData,
    InvalidOpcode,
    MissingLiteral,
    InappropriateLiteral,
    IncompleteLiteral,
}

impl TryFrom<Vec<u8>> for Instruction {
    type Error = InstructionParseError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        Instruction::try_from(value.as_ref())
    }
}

impl TryFrom<&[u8]> for Instruction {
    type Error = InstructionParseError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err(Self::Error::NoData)
        } else if value.len() > 1 {
            if value[0] == 0x06 {
                if value.len() == 1 + word_bytes() {
                    Ok(Self::Set(Word::from_be_bytes(
                        value[1..].try_into().unwrap(),
                    )))
                } else {
                    Err(Self::Error::IncompleteLiteral)
                }
            } else {
                Err(Self::Error::InappropriateLiteral)
            }
        } else {
            match value[0] {
                0x00 => Ok(Self::Nop),
                0x01 => Ok(Self::Halt),
                0x02 => Ok(Self::Load),
                0x03 => Ok(Self::Store),
                0x04 => Ok(Self::Push),
                0x05 => Ok(Self::Pop),
                0x07 => Ok(Self::Read),
                0x08 => Ok(Self::Write),
                0x09 => Ok(Self::Jump),
                0x0A => Ok(Self::JumpIf),
                0x0B => Ok(Self::Add),
                0x0C => Ok(Self::Sub),
                0x0D => Ok(Self::Mul),
                0x0E => Ok(Self::Div),
                0x0F => Ok(Self::Mod),
                0x10 => Ok(Self::Cmp),
                0x11 => Ok(Self::And),
                0x12 => Ok(Self::Or),
                0x13 => Ok(Self::Not),
                0x14 => Ok(Self::Xor),
                0x06 => Err(Self::Error::MissingLiteral),
                _ => Err(Self::Error::InvalidOpcode),
            }
        }
    }
}

impl Instruction {
    pub fn from_bytes(bytes: Vec<u8>) -> Option<Self> {
        Instruction::try_from(bytes).ok()
    }

    pub fn to_byte(&self) -> u8 {
        match self {
            Self::Nop => 0x00,
            Self::Halt => 0x01,
            Self::Load => 0x02,
            Self::Store => 0x03,
            Self::Push => 0x04,
            Self::Pop => 0x05,
            Self::Set(_) => 0x06,
            Self::Read => 0x07,
            Self::Write => 0x08,
            Self::Jump => 0x09,
            Self::JumpIf => 0x0A,
            Self::Add => 0x0B,
            Self::Sub => 0x0C,
            Self::Mul => 0x0D,
            Self::Div => 0x0E,
            Self::Mod => 0x0F,
            Self::Cmp => 0x10,
            Self::And => 0x11,
            Self::Or => 0x12,
            Self::Not => 0x13,
            Self::Xor => 0x14,
        }
    }
}
