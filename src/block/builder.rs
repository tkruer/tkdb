use std::usize;

use crate::block::block::Block;
use crate::key::KeyVec;

pub struct BlockBuilder {
    offsets: Vec<u16>,
    data: Vec<u8>,
    block_size: usize,
    first_key: KeyVec
}

impl BlockBuilder {
    pub fn new(block_size: usize) -> Self {
        Self {
            offsets: Vec::new(),
            data: Vec::new(),
            block_size,
            first_key: KeyVec::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.offsets.is_empty()
    }

    pub fn build(self) -> Block {
        if self.is_empty() {
            panic!("block should not be empty");
        }
        Block {
            data: self.data,
            offsets: self.offsets,
        }
    }
}
