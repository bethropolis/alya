//! Address types and validation.

use std::fmt;

/// A validated memory address
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Address(usize);

impl Address {
    /// Create a new address (no validation)
    pub const fn new(value: usize) -> Self {
        Self(value)
    }

    /// Create a validated address
    pub fn checked(value: usize, max: usize) -> Result<Self, AddressError> {
        if value >= max {
            Err(AddressError::OutOfBounds { value, max })
        } else {
            Ok(Self(value))
        }
    }

    /// Get the raw value
    pub const fn value(self) -> usize {
        self.0
    }

    /// Add offset to address
    pub const fn offset(self, offset: usize) -> Self {
        Self(self.0 + offset)
    }

    /// Check if address is aligned to a boundary
    pub const fn is_aligned(self, alignment: usize) -> bool {
        self.0 % alignment == 0
    }
}

impl From<usize> for Address {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl From<Address> for usize {
    fn from(addr: Address) -> Self {
        addr.0
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#x}", self.0)
    }
}

/// Address-related errors
#[derive(Debug, Clone, PartialEq)]
pub enum AddressError {
    OutOfBounds { value: usize, max: usize },
    Unaligned { value: usize, alignment: usize },
}

impl fmt::Display for AddressError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AddressError::OutOfBounds { value, max } => {
                write!(f, "Address out of bounds: {:#x} >= {:#x}", value, max)
            }
            AddressError::Unaligned { value, alignment } => {
                write!(f, "Unaligned address: {:#x} (alignment: {})", value, alignment)
            }
        }
    }
}

impl std::error::Error for AddressError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_address_creation() {
        let addr = Address::new(0x1000);
        assert_eq!(addr.value(), 0x1000);
    }

    #[test]
    fn test_address_validation() {
        assert!(Address::checked(100, 1000).is_ok());
        assert!(Address::checked(1000, 1000).is_err());
        assert!(Address::checked(1001, 1000).is_err());
    }

    #[test]
    fn test_address_alignment() {
        let addr = Address::new(8);
        assert!(addr.is_aligned(4));
        assert!(addr.is_aligned(8));
        assert!(!addr.is_aligned(16));

        let addr2 = Address::new(10);
        assert!(!addr2.is_aligned(4));
    }
}
