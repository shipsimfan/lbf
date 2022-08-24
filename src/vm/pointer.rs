use super::memory::{Memory, MemoryBlock, MEMORY_BLOCK_LENGTH};
use std::cell::RefCell;

pub struct Pointer<'a> {
    memory: &'a Memory<'a>,
    current_block: &'a RefCell<MemoryBlock<'a>>,
    index: usize,
}

impl<'a> Pointer<'a> {
    pub fn new(memory: &'a Memory<'a>) -> Self {
        Pointer {
            memory,
            current_block: memory.get_base_block(),
            index: 0,
        }
    }

    pub fn get(&self) -> u8 {
        self.current_block.borrow()[self.index]
    }

    pub fn inc(&self) {
        self.set(self.get().wrapping_add(1));
    }

    pub fn dec(&self) {
        self.set(self.get().wrapping_sub(1));
    }

    pub fn set(&self, value: u8) {
        self.current_block.borrow_mut()[self.index] = value;
    }

    pub fn right(&mut self) {
        if self.index == MEMORY_BLOCK_LENGTH - 1 {
            self.index = 0;
            self.current_block = self.memory.get_next(self.current_block);
        } else {
            self.index += 1;
        }
    }

    pub fn left(&mut self) {
        if self.index == 0 {
            self.index = MEMORY_BLOCK_LENGTH - 1;
            self.current_block = self.memory.get_prev(self.current_block);
        } else {
            self.index -= 1;
        }
    }
}
