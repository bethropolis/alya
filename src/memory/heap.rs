use crate::memory::MemoryAccess;
use crate::memory::manager::MemoryError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Block {
    pub size: usize,
    pub free: bool,
    pub next: Option<usize>, // Offset to next block in memory
}

impl Block {
    pub const SIZE: usize = 24; // Header size (8 size + 8 next + 8 free/padded)

    pub fn from_bytes(bytes: &[u8]) -> Self {
        let size = u64::from_le_bytes(bytes[0..8].try_into().unwrap()) as usize;
        let next_offset = u64::from_le_bytes(bytes[8..16].try_into().unwrap());
        let free = bytes[16] != 0;
        let next = if next_offset == 0 { None } else { Some(next_offset as usize) };
        
        Self { size, free, next }
    }

    pub fn to_bytes(&self) -> [u8; 24] {
        let mut bytes = [0u8; 24];
        bytes[0..8].copy_from_slice(&(self.size as u64).to_le_bytes());
        bytes[8..16].copy_from_slice(&(self.next.unwrap_or(0) as u64).to_le_bytes());
        bytes[16] = if self.free { 1 } else { 0 };
        bytes
    }
}

pub struct Heap {
    start: usize,
    size: usize,
}

impl Heap {
    pub fn new(start: usize, size: usize) -> Self {
        Self { start, size }
    }

    /// Initialize heap with one large free block
    pub fn init<M: MemoryAccess + ?Sized>(&self, memory: &mut M) -> Result<(), MemoryError> {
        let initial_block = Block {
            size: self.size - Block::SIZE,
            free: true,
            next: None,
        };
        self.write_block(memory, self.start, initial_block)
    }

    fn read_block<M: MemoryAccess + ?Sized>(&self, memory: &M, addr: usize) -> Result<Block, MemoryError> {
        let mut bytes = [0u8; 24];
        for i in 0..24 {
            bytes[i] = memory.read_byte(addr + i)?;
        }
        Ok(Block::from_bytes(&bytes))
    }

    fn write_block<M: MemoryAccess + ?Sized>(&self, memory: &mut M, addr: usize, block: Block) -> Result<(), MemoryError> {
        let bytes = block.to_bytes();
        for i in 0..24 {
            memory.write_byte(addr + i, bytes[i])?;
        }
        Ok(())
    }

    pub fn alloc<M: MemoryAccess + ?Sized>(&self, memory: &mut M, size: usize) -> Result<usize, MemoryError> {
        let mut current_addr = self.start;
        
        while current_addr < self.start + self.size {
            let mut block = self.read_block(memory, current_addr)?;
            
            if block.free && block.size >= size {
                // If block is much larger, split it
                if block.size > size + Block::SIZE + 8 {
                    let next_addr = current_addr + Block::SIZE + size;
                    let next_block = Block {
                        size: block.size - size - Block::SIZE,
                        free: true,
                        next: block.next,
                    };
                    self.write_block(memory, next_addr, next_block)?;
                    
                    block.size = size;
                    block.next = Some(next_addr);
                }
                
                block.free = false;
                self.write_block(memory, current_addr, block)?;
                return Ok(current_addr + Block::SIZE);
            }
            
            if let Some(next) = block.next {
                current_addr = next;
            } else {
                break;
            }
        }
        
        Err(MemoryError::SegmentationFault { 
            address: current_addr, 
            message: "Heap out of memory".to_string() 
        })
    }

    pub fn free<M: MemoryAccess + ?Sized>(&self, memory: &mut M, ptr: usize) -> Result<(), MemoryError> {
        if ptr < self.start + Block::SIZE || ptr >= self.start + self.size {
            return Err(MemoryError::OutOfBounds { address: ptr, size: self.size });
        }
        
        let block_addr = ptr - Block::SIZE;
        let mut block = self.read_block(memory, block_addr)?;
        block.free = true;
        self.write_block(memory, block_addr, block)?;
        
        // Optional: Coalesce adjacent free blocks could be implemented here
        Ok(())
    }
}
