//! Register definitions and operations.

use std::fmt;

/// All registers available in the Alya VM.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Register {
    // General-purpose registers (0-15)
    R0 = 0,
    R1 = 1,
    R2 = 2,
    R3 = 3,
    R4 = 4,
    R5 = 5,
    R6 = 6,
    R7 = 7,
    R8 = 8,
    R9 = 9,
    R10 = 10,
    R11 = 11,
    R12 = 12,
    R13 = 13,
    R14 = 14,
    R15 = 15,

    // Special registers (16-19)
    SP = 16, // Stack Pointer
    BP = 17, // Base Pointer
    IP = 18, // Instruction Pointer (read-only from user perspective)
    FL = 19, // Flags
}

impl Register {
    /// Total number of registers
    pub const COUNT: usize = 20;

    /// Number of general-purpose registers
    pub const GP_COUNT: usize = 16;

    /// Convert from byte representation
    pub fn from_u8(value: u8) -> Result<Self, RegisterError> {
        match value {
            0 => Ok(Register::R0),
            1 => Ok(Register::R1),
            2 => Ok(Register::R2),
            3 => Ok(Register::R3),
            4 => Ok(Register::R4),
            5 => Ok(Register::R5),
            6 => Ok(Register::R6),
            7 => Ok(Register::R7),
            8 => Ok(Register::R8),
            9 => Ok(Register::R9),
            10 => Ok(Register::R10),
            11 => Ok(Register::R11),
            12 => Ok(Register::R12),
            13 => Ok(Register::R13),
            14 => Ok(Register::R14),
            15 => Ok(Register::R15),
            16 => Ok(Register::SP),
            17 => Ok(Register::BP),
            18 => Ok(Register::IP),
            19 => Ok(Register::FL),
            _ => Err(RegisterError::InvalidCode(value)),
        }
    }

    /// Convert to byte representation
    pub const fn to_u8(self) -> u8 {
        self as u8
    }

    /// Check if this is a general-purpose register
    pub const fn is_general_purpose(self) -> bool {
        (self as u8) < 16
    }

    /// Check if this is a special register
    pub const fn is_special(self) -> bool {
        !self.is_general_purpose()
    }

    /// Get register name as string
    pub const fn name(self) -> &'static str {
        match self {
            Register::R0 => "r0",
            Register::R1 => "r1",
            Register::R2 => "r2",
            Register::R3 => "r3",
            Register::R4 => "r4",
            Register::R5 => "r5",
            Register::R6 => "r6",
            Register::R7 => "r7",
            Register::R8 => "r8",
            Register::R9 => "r9",
            Register::R10 => "r10",
            Register::R11 => "r11",
            Register::R12 => "r12",
            Register::R13 => "r13",
            Register::R14 => "r14",
            Register::R15 => "r15",
            Register::SP => "sp",
            Register::BP => "bp",
            Register::IP => "ip",
            Register::FL => "fl",
        }
    }
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "@{}", self.name())
    }
}

/// Errors related to register operations
#[derive(Debug, Clone, PartialEq)]
pub enum RegisterError {
    InvalidCode(u8),
    InvalidName(String),
}

impl fmt::Display for RegisterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RegisterError::InvalidCode(code) => {
                write!(f, "Invalid register code: {}", code)
            }
            RegisterError::InvalidName(name) => {
                write!(f, "Invalid register name: {}", name)
            }
        }
    }
}

impl std::error::Error for RegisterError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_roundtrip() {
        for i in 0..Register::COUNT as u8 {
            let reg = Register::from_u8(i).unwrap();
            assert_eq!(reg.to_u8(), i);
        }
    }

    #[test]
    fn test_invalid_register() {
        assert!(Register::from_u8(20).is_err());
        assert!(Register::from_u8(255).is_err());
    }

    #[test]
    fn test_register_categories() {
        assert!(Register::R0.is_general_purpose());
        assert!(Register::R15.is_general_purpose());
        assert!(!Register::SP.is_general_purpose());

        assert!(Register::SP.is_special());
        assert!(!Register::R5.is_special());
    }

    #[test]
    fn test_register_display() {
        assert_eq!(format!("{}", Register::R0), "@r0");
        assert_eq!(format!("{}", Register::SP), "@sp");
    }
}
