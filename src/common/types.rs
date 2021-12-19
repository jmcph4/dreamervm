pub type Word = u64;

const BITS_PER_BYTE: usize = 8;

pub fn word_bytes() -> usize {
    (Word::BITS as usize) / BITS_PER_BYTE
}
