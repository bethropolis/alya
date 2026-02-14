//! CPU flags for conditional operations.

use std::fmt;

/// Individual flag bits
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Flag {
    Zero = 0,     // Result was zero
    Negative = 1, // Result was negative
    Carry = 2,    // Arithmetic overflow/carry
}

/// Flags register state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Flags {
    bits: u64,
}

impl Flags {
    /// Create new flags with all cleared
    pub const fn new() -> Self {
        Self { bits: 0 }
    }

    /// Create from raw bits
    pub const fn from_bits(bits: u64) -> Self {
        Self { bits }
    }

    /// Get raw bits
    pub const fn bits(self) -> u64 {
        self.bits
    }

    /// Set a specific flag
    pub fn set(&mut self, flag: Flag, value: bool) {
        let mask = 1u64 << (flag as u64);
        if value {
            self.bits |= mask;
        } else {
            self.bits &= !mask;
        }
    }

    /// Get a specific flag
    pub fn get(self, flag: Flag) -> bool {
        let mask = 1u64 << (flag as u64);
        (self.bits & mask) != 0
    }

    /// Set zero flag
    pub fn set_zero(&mut self, value: bool) {
        self.set(Flag::Zero, value);
    }

    /// Get zero flag
    pub fn zero(self) -> bool {
        self.get(Flag::Zero)
    }

    /// Set negative flag
    pub fn set_negative(&mut self, value: bool) {
        self.set(Flag::Negative, value);
    }

    /// Get negative flag
    pub fn negative(self) -> bool {
        self.get(Flag::Negative)
    }

    /// Set carry flag
    pub fn set_carry(&mut self, value: bool) {
        self.set(Flag::Carry, value);
    }

    /// Get carry flag
    pub fn carry(self) -> bool {
        self.get(Flag::Carry)
    }

    /// Update flags based on a result value
    pub fn update_from_result(&mut self, result: u64, overflow: bool) {
        self.set_zero(result == 0);
        self.set_negative((result as i64) < 0);
        self.set_carry(overflow);
    }

    /// Clear all flags
    pub fn clear(&mut self) {
        self.bits = 0;
    }
}

impl Default for Flags {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Flags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[Z:{} N:{} C:{}]",
            if self.zero() { '1' } else { '0' },
            if self.negative() { '1' } else { '0' },
            if self.carry() { '1' } else { '0' },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flags_set_get() {
        let mut flags = Flags::new();
        assert!(!flags.zero());

        flags.set_zero(true);
        assert!(flags.zero());

        flags.set_negative(true);
        assert!(flags.negative());
        assert!(flags.zero()); // Zero should still be set
    }

    #[test]
    fn test_update_from_result() {
        let mut flags = Flags::new();

        // Zero result
        flags.update_from_result(0, false);
        assert!(flags.zero());
        assert!(!flags.negative());
        assert!(!flags.carry());

        // Negative result
        flags.update_from_result(u64::MAX, false);
        assert!(!flags.zero());
        assert!(flags.negative());
    }
}
