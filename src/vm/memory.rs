use super::append_array::AppendArray;
use std::{
    cell::RefCell,
    ops::{Index, IndexMut},
};

pub struct Memory<'a> {
    arena: AppendArray<RefCell<MemoryBlock<'a>>, UNIT_CAPACITY>,
}

pub struct MemoryBlock<'a> {
    next: Option<&'a RefCell<MemoryBlock<'a>>>,
    prev: Option<&'a RefCell<MemoryBlock<'a>>>,
    block: [u8; MEMORY_BLOCK_LENGTH],
}

pub const MEMORY_BLOCK_LENGTH: usize = 128;

const UNIT_CAPACITY: usize = 8;

impl<'a> Memory<'a> {
    pub fn new() -> Self {
        let arena = AppendArray::new();
        arena.append(MemoryBlock::new());
        Memory { arena }
    }

    pub fn get_base_block(&'a self) -> &'a RefCell<MemoryBlock<'a>> {
        &self.arena[0]
    }

    pub fn get_next(&'a self, block: &'a RefCell<MemoryBlock<'a>>) -> &'a RefCell<MemoryBlock<'a>> {
        match block.borrow().next {
            Some(next) => return next,
            None => {}
        }

        let new_block = self.arena.append(MemoryBlock::new());
        block.borrow_mut().next = Some(new_block);
        new_block
    }

    pub fn get_prev(&'a self, block: &'a RefCell<MemoryBlock<'a>>) -> &'a RefCell<MemoryBlock<'a>> {
        match block.borrow().prev {
            Some(prev) => return prev,
            None => {}
        }

        let new_block = self.arena.append(MemoryBlock::new());
        block.borrow_mut().prev = Some(new_block);
        new_block
    }
}

impl<'a> MemoryBlock<'a> {
    pub fn new() -> RefCell<Self> {
        RefCell::new(MemoryBlock {
            next: None,
            prev: None,
            block: [0; MEMORY_BLOCK_LENGTH],
        })
    }
}

impl<'a> Index<usize> for MemoryBlock<'a> {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.block[index]
    }
}

impl<'a> IndexMut<usize> for MemoryBlock<'a> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.block[index]
    }
}
