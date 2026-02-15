//! Main memory manager implementation.

use super::MemoryAccess;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryPermission {
    Read = 0x01,
    Write = 0x02,
    Execute = 0x04,
}

#[derive(Debug, Clone)]
pub struct Segment {
    pub name: String,
    pub start: usize,
    pub end: usize,
    pub permissions: u8, // Bitmask of MemoryPermission
}

/// Main memory storage
pub struct Memory {
    bytes: Vec<u8>,
    segments: Vec<Segment>,
}

impl Memory {
    /// Create new memory with default segments:
    /// 0x0000 - 0x7FFF: Code (32KB, RX)
    /// 0x8000 - 0xBFFF: Heap (16KB, RW)
    /// 0xC000 - 0xFFFF: Stack (16KB, RW)
    pub fn new(size: usize) -> Self {
        let mut segments = Vec::new();
        
        if size >= 0x10000 {
            // Code Segment (RX)
            segments.push(Segment {
                name: "Code".to_string(),
                start: 0,
                end: 0x7FFF,
                permissions: MemoryPermission::Read as u8 | MemoryPermission::Execute as u8,
            });

            // Heap Segment (RW)
            segments.push(Segment {
                name: "Heap".to_string(),
                start: 0x8000,
                end: 0xBFFF,
                permissions: MemoryPermission::Read as u8 | MemoryPermission::Write as u8,
            });

            // Stack Segment (RW)
            segments.push(Segment {
                name: "Stack".to_string(),
                start: 0xC000,
                end: size.saturating_sub(1),
                permissions: MemoryPermission::Read as u8 | MemoryPermission::Write as u8,
            });
        } else {
            // For small memory (mostly tests), create one single RWX segment
            segments.push(Segment {
                name: "General".to_string(),
                start: 0,
                end: size.saturating_sub(1),
                permissions: MemoryPermission::Read as u8 | MemoryPermission::Write as u8 | MemoryPermission::Execute as u8,
            });
        }

        Self {
            bytes: vec![0; size],
            segments,
        }
    }

    /// Clear all memory (set to zero)
    pub fn clear(&mut self) {
        self.bytes.fill(0);
    }

    /// Load program data into memory at address 0
    pub fn load_program(&mut self, data: &[u8]) -> Result<(), MemoryError> {
        if data.len() > self.bytes.len() {
            return Err(MemoryError::ProgramTooLarge {
                program_size: data.len(),
                memory_size: self.bytes.len(),
            });
        }

        self.bytes[..data.len()].copy_from_slice(data);
        Ok(())
    }

    /// Check if a memory range has the required permissions
    pub fn check_access(&self, addr: usize, len: usize, perm: MemoryPermission) -> Result<(), MemoryError> {
        if addr + len > self.bytes.len() {
            return Err(MemoryError::OutOfBounds {
                address: addr,
                size: self.bytes.len(), // This is actually memory size, but matches error definition
            });
        }

        // Find which segment the address falls into
        for segment in &self.segments {
            if addr >= segment.start && addr <= segment.end {
                // Check if the entire range fits in this segment
                if addr + len - 1 > segment.end {
                    return Err(MemoryError::SegmentationFault {
                        address: addr,
                        message: format!("Access spans multiple segments (end of {} is {:#x})", segment.name, segment.end),
                    });
                }

                // Check permissions
                if (segment.permissions & (perm as u8)) == 0 {
                    return Err(MemoryError::SegmentationFault {
                        address: addr,
                        message: format!("Segment {} does not have {:?} permission", segment.name, perm),
                    });
                }

                return Ok(());
            }
        }

        Err(MemoryError::SegmentationFault {
            address: addr,
            message: "Address does not belong to any segment".to_string(),
        })
    }

    /// Get a slice of memory for reading (checked)
    pub fn slice(&self, start: usize, len: usize) -> Result<&[u8], MemoryError> {
        self.check_access(start, len, MemoryPermission::Read)?;
        Ok(&self.bytes[start..start + len])
    }
}

impl MemoryAccess for Memory {
    fn read_byte(&self, addr: usize) -> Result<u8, MemoryError> {
        self.check_access(addr, 1, MemoryPermission::Read)?;
        Ok(self.bytes[addr])
    }

    fn write_byte(&mut self, addr: usize, value: u8) -> Result<(), MemoryError> {
        self.check_access(addr, 1, MemoryPermission::Write)?;
        self.bytes[addr] = value;
        Ok(())
    }

    fn read_qword(&self, addr: usize) -> Result<u64, MemoryError> {
        self.check_access(addr, 8, MemoryPermission::Read)?;

        // Fast path: direct pointer access
        unsafe {
            let ptr = self.bytes.as_ptr().add(addr) as *const u64;
            Ok(u64::from_le(std::ptr::read_unaligned(ptr)))
        }
    }

    fn write_qword(&mut self, addr: usize, value: u64) -> Result<(), MemoryError> {
        self.check_access(addr, 8, MemoryPermission::Write)?;

        // Fast path: direct pointer access
        unsafe {
            let ptr = self.bytes.as_mut_ptr().add(addr) as *mut u64;
            std::ptr::write_unaligned(ptr, value.to_le());
        }
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
    SegmentationFault { address: usize, message: String },
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
            MemoryError::SegmentationFault { address, message } => {
                write!(f, "Segmentation fault at {:#x}: {}", address, message)
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
