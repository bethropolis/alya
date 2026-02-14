//! Main memory manager implementation.

use super::MemoryAccess;
use std::fmt;

/// Main memory storage
pub struct Memory {
    bytes: Vec<u8>,
}

impl Memory {
    /// Create new memory with specified size
    pub fn new(size: usize) -> Self {
        Self {
            bytes: vec![0; size],
        }
    }

    /// Load program bytecode into memory at address 0
    pub fn load_program(&mut self, bytecode: &[u8]) -> Result<(), MemoryError> {
        if bytecode.len() > self.bytes.len() {
            return Err(MemoryError::ProgramTooLarge {
                program_size: bytecode.len(),
                memory_size: self.bytes.len(),
            });
        }

        self.bytes[..bytecode.len()].copy_from_slice(bytecode);
        Ok(())
    }

    /// Clear all memory
    pub fn clear(&mut self) {
        self.bytes.fill(0);
    }

    /// Get a slice of memory for reading
    pub fn slice(&self, start: usize, len: usize) -> Result<&[u8], MemoryError> {
        if start + len > self.bytes.len() {
            return Err(MemoryError::OutOfBounds {
                address: start,
                size: self.bytes.len(),
            });
        }
        Ok(&self.bytes[start..start + len])
    }
}

impl MemoryAccess for Memory {
    fn read_byte(&self, addr: usize) -> Result<u8, MemoryError> {
        self.bytes
            .get(addr)
            .copied()
            .ok_or(MemoryError::OutOfBounds {
                address: addr,
                size: self.bytes.len(),
            })
    }

    fn write_byte(&mut self, addr: usize, value: u8) -> Result<(), MemoryError> {
        if addr >= self.bytes.len() {
            return Err(MemoryError::OutOfBounds {
                address: addr,
                size: self.bytes.len(),
            });
        }
        self.bytes[addr] = value;
        Ok(())
    }

    fn read_qword(&self, addr: usize) -> Result<u64, MemoryError> {
        if addr + 8 > self.bytes.len() {
            return Err(MemoryError::OutOfBounds {
                address: addr,
                size: self.bytes.len(),
            });
        }

        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&self.bytes[addr..addr + 8]);
        Ok(u64::from_le_bytes(bytes))
    }

    fn write_qword(&mut self, addr: usize, value: u64) -> Result<(), MemoryError> {
        if addr + 8 > self.bytes.len() {
            return Err(MemoryError::OutOfBounds {
                address: addr,
                size: self.bytes.len(),
            });
        }

        let bytes = value.to_le_bytes();
        self.bytes[addr..addr + 8].copy_from_slice(&bytes);
        Ok(())
    }

    fn size(&self) -> usize {
        self.bytes.len()
    }
}

impl fmt::Debug for Memory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Memory {{ size: {} bytes }}", self.bytes.len())
    }
}

/// Memory-related errors
#[derive(Debug, Clone, PartialEq)]
pub enum MemoryError {
    OutOfBounds { address: usize, size: usize },
    ProgramTooLarge { program_size: usize, memory_size: usize },
    Unaligned { address: usize, alignment: usize },
}

impl fmt::Display for MemoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MemoryError::OutOfBounds { address, size } => {
                write!(f, "Memory access out of bounds: address {:#x}, size {:#x}", address, size)
            }
            MemoryError::ProgramTooLarge { program_size, memory_size } => {
                write!(f, "Program too large: {} bytes, memory: {} bytes", program_size, memory_size)
            }
            MemoryError::Unaligned { address, alignment } => {
                write!(f, "Unaligned memory access: address {:#x}, alignment {}", address, alignment)
            }
        }
    }
}

impl std::error::Error for MemoryError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_creation() {
        let mem = Memory::new(1024);
        assert_eq!(mem.size(), 1024);
    }

    #[test]
    fn test_byte_operations() {
        let mut mem = Memory::new(256);

        mem.write_byte(0, 42).unwrap();
        assert_eq!(mem.read_byte(0).unwrap(), 42);

        assert!(mem.write_byte(256, 0).is_err());
        assert!(mem.read_byte(256).is_err());
    }

    #[test]
    fn test_qword_operations() {
        let mut mem = Memory::new(256);

        let value = 0x0123456789ABCDEF;
        mem.write_qword(0, value).unwrap();
        assert_eq!(mem.read_qword(0).unwrap(), value);

        // Verify little-endian
        assert_eq!(mem.read_byte(0).unwrap(), 0xEF);
        assert_eq!(mem.read_byte(7).unwrap(), 0x01);
    }

    #[test]
    fn test_program_loading() {
        let mut mem = Memory::new(256);
        let program = vec![0x10, 0x20, 0x30, 0x40];

        mem.load_program(&program).unwrap();
        assert_eq!(mem.read_byte(0).unwrap(), 0x10);
        assert_eq!(mem.read_byte(3).unwrap(), 0x40);
    }
}
