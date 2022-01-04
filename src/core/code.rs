use crate::common::types::word_bytes;
use crate::core::instruction::{Instruction, InstructionParseError};

#[derive(Clone, Debug)]
pub struct VecCode(pub Vec<Instruction>);

#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
pub struct CodeParseError {
    err: InstructionParseError,
    pos: usize,
}

impl TryFrom<&[u8]> for VecCode {
    type Error = CodeParseError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        /*
         * The sketch for this parsing code is basically:
         *
         *  - Parse each byte in `data` left-to-right
         *  - Make a decision about what slice to pass to `Instruction::try_from`
         *  - Pass that slice to `Instruction::try_from`
         *      - On success, append the resulting `Instruction` to `res`
         *      - On failure, return the appropriate error
         *
         */
        let mut i: usize = 0;
        let mut res: Vec<Instruction> = vec![];

        while i < data.len() {
            let curr_byte: u8 = data[i];

            /*
             * For any given byte (when parsing left-to-right!) there are two
             * possibilities. We take cases:
             *
             * Case 1: The byte is 0x06 (which is the SET opcode).
             *         This means that a well-formed instruction *must* look
             *         like this:
             *
             *         +------+------+------+------+------+------+------+------+------+
             *         | 0x06 | aaaa | bbbb | cccc | dddd | eeee | ffff | gggg | hhhh |
             *         +------+------+------+------+------+------+------+------+------+
             *
             *         Thus, we must skip over this entire subsequence.
             *
             * Case 2: The byte is not 0x06.
             *         This means that a well-formed instruction *must* look
             *         like this:
             *
             *         +------+
             *         | aaaa |
             *         +------+
             *
             *         Thus, we skip over just this byte (a special case of the
             *         above logic!).
             */
            let (curr_slice, next_pos): (&[u8], usize) = match curr_byte {
                0x06 => (&data[i..=(i + word_bytes())], i + word_bytes() + 1),
                _ => (&data[i..=i], i + 1),
            };

            /* interpret the chosen slice and handle failure accordingly */
            match Instruction::try_from(curr_slice) {
                Ok(t) => res.push(t),
                Err(e) => return Err(Self::Error { err: e, pos: i }),
            }

            /* jump to wherever we need to go now */
            i = next_pos;
        }

        Ok(Self(res))
    }
}

impl TryFrom<Vec<u8>> for Code {
    type Error = CodeParseError;

    fn try_from(data: Vec<u8>) -> Result<Self, Self::Error> {
        Self::try_from(data.as_ref())
    }
}

pub type Code = VecCode;
