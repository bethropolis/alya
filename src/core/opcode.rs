//! Opcode definitions for bytecode instructions.

use std::fmt;

/// Bytecode operation codes.
///
/// Organized by function for easy reference and future expansion.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Opcode {
    // Control (0x00-0x0F)
    Halt = 0x00,
    Nop = 0x01,

    // Data Movement (0x10-0x1F)
    LoadImm = 0x10,
    Move = 0x11,
    Swap = 0x12,

    // Arithmetic (0x20-0x2F)
    Add = 0x20,
    Sub = 0x21,
    Mul = 0x22,
    Div = 0x23,
    Mod = 0x24,

    // Compound Assignment (0x30-0x3F)
    AddAssign = 0x30,
    SubAssign = 0x31,
    MulAssign = 0x32,
    DivAssign = 0x33,

    // Bitwise (0x40-0x4F)
    And = 0x40,
    Or = 0x41,
    Xor = 0x42,
    Not = 0x43,
    Shl = 0x44,
    Shr = 0x45,

    // Stack (0x50-0x5F)
    Push = 0x50,
    Pop = 0x51,
    Peek = 0x52,

    // Memory (0x60-0x6F)
    Load = 0x60,
    Store = 0x61,
    LoadIndexed = 0x62,
    StoreIndexed = 0x63,
    Alloc = 0x64,
    Free = 0x65,
    MemCopy = 0x66,
    MemSet = 0x67,

    // Control Flow (0x70-0x7F)
    Jump = 0x70,
    JumpIfZero = 0x71,
    JumpIfNotZero = 0x72,
    JumpIfGt = 0x73,
    JumpIfLt = 0x74,
    JumpIfGe = 0x75,
    JumpIfLe = 0x76,
    JumpIfEq = 0x77,
    JumpIfNe = 0x78,
    JumpIfAbove = 0x49,
    JumpIfBelow = 0x4A,
    JumpIfAe = 0x4B,
    JumpIfBe = 0x4C,

    // Compare (used before conditional jumps)
    Compare = 0x79,

    // Functions (0x80-0x8F)
    Call = 0x80,
    Return = 0x81,

    // System (0x90-0x9F)
    Syscall = 0x99,

    // Floating Point (0xA0-0xAF)
    FAdd = 0xA0,
    FSub = 0xA1,
    FMul = 0xA2,
    FDiv = 0xA3,
    FSqrt = 0xA4,
    FAbs = 0xA5,
    FNeg = 0xA6,
    F2I = 0xA7,
    I2F = 0xA8,
    FCmp = 0xA9,

    // Bit Manipulation (0xB0-0xBF)
    PopCnt = 0xB0,
    Clz = 0xB1,
    Ctz = 0xB2,
    BSwap = 0xB3,
    RotL = 0xB4,
    RotR = 0xB5,

    // Debug (0xF0-0xFF)
    Breakpoint = 0xF1,
    TraceOn = 0xF2,
    TraceOff = 0xF3,
}

impl Opcode {
    /// Convert from byte representation
    pub fn from_u8(value: u8) -> Result<Self, OpcodeError> {
        match value {
            0x00 => Ok(Opcode::Halt),
            0x01 => Ok(Opcode::Nop),
            0x10 => Ok(Opcode::LoadImm),
            0x11 => Ok(Opcode::Move),
            0x12 => Ok(Opcode::Swap),
            0x20 => Ok(Opcode::Add),
            0x21 => Ok(Opcode::Sub),
            0x22 => Ok(Opcode::Mul),
            0x23 => Ok(Opcode::Div),
            0x24 => Ok(Opcode::Mod),
            0x30 => Ok(Opcode::AddAssign),
            0x31 => Ok(Opcode::SubAssign),
            0x32 => Ok(Opcode::MulAssign),
            0x33 => Ok(Opcode::DivAssign),
            0x40 => Ok(Opcode::And),
            0x41 => Ok(Opcode::Or),
            0x42 => Ok(Opcode::Xor),
            0x43 => Ok(Opcode::Not),
            0x44 => Ok(Opcode::Shl),
            0x45 => Ok(Opcode::Shr),
            0x50 => Ok(Opcode::Push),
            0x51 => Ok(Opcode::Pop),
            0x52 => Ok(Opcode::Peek),
            0x60 => Ok(Opcode::Load),
            0x61 => Ok(Opcode::Store),
            0x62 => Ok(Opcode::LoadIndexed),
            0x63 => Ok(Opcode::StoreIndexed),
            0x64 => Ok(Opcode::Alloc),
            0x65 => Ok(Opcode::Free),
            0x66 => Ok(Opcode::MemCopy),
            0x67 => Ok(Opcode::MemSet),
            0x70 => Ok(Opcode::Jump),
            0x71 => Ok(Opcode::JumpIfZero),
            0x72 => Ok(Opcode::JumpIfNotZero),
            0x73 => Ok(Opcode::JumpIfGt),
            0x74 => Ok(Opcode::JumpIfLt),
            0x75 => Ok(Opcode::JumpIfGe),
            0x76 => Ok(Opcode::JumpIfLe),
            0x77 => Ok(Opcode::JumpIfEq),
            0x78 => Ok(Opcode::JumpIfNe),
            0x49 => Ok(Opcode::JumpIfAbove),
            0x4A => Ok(Opcode::JumpIfBelow),
            0x4B => Ok(Opcode::JumpIfAe),
            0x4C => Ok(Opcode::JumpIfBe),
            0x79 => Ok(Opcode::Compare),
            0x80 => Ok(Opcode::Call),
            0x81 => Ok(Opcode::Return),
            0x99 => Ok(Opcode::Syscall),
            0xA0 => Ok(Opcode::FAdd),
            0xA1 => Ok(Opcode::FSub),
            0xA2 => Ok(Opcode::FMul),
            0xA3 => Ok(Opcode::FDiv),
            0xA4 => Ok(Opcode::FSqrt),
            0xA5 => Ok(Opcode::FAbs),
            0xA6 => Ok(Opcode::FNeg),
            0xA7 => Ok(Opcode::F2I),
            0xA8 => Ok(Opcode::I2F),
            0xA9 => Ok(Opcode::FCmp),
            0xB0 => Ok(Opcode::PopCnt),
            0xB1 => Ok(Opcode::Clz),
            0xB2 => Ok(Opcode::Ctz),
            0xB3 => Ok(Opcode::BSwap),
            0xB4 => Ok(Opcode::RotL),
            0xB5 => Ok(Opcode::RotR),
            0xF1 => Ok(Opcode::Breakpoint),
            0xF2 => Ok(Opcode::TraceOn),
            0xF3 => Ok(Opcode::TraceOff),
            _ => Err(OpcodeError::Unknown(value)),
        }
    }

    /// Convert to byte representation
    pub const fn to_u8(self) -> u8 {
        self as u8
    }

    /// Get opcode name
    pub const fn name(self) -> &'static str {
        match self {
            Opcode::Halt => "halt",
            Opcode::Nop => "nop",
            Opcode::LoadImm => "loadimm",
            Opcode::Move => "move",
            Opcode::Swap => "swap",
            Opcode::Add => "add",
            Opcode::Sub => "sub",
            Opcode::Mul => "mul",
            Opcode::Div => "div",
            Opcode::Mod => "mod",
            Opcode::AddAssign => "add_assign",
            Opcode::SubAssign => "sub_assign",
            Opcode::MulAssign => "mul_assign",
            Opcode::DivAssign => "div_assign",
            Opcode::And => "and",
            Opcode::Or => "or",
            Opcode::Xor => "xor",
            Opcode::Not => "not",
            Opcode::Shl => "shl",
            Opcode::Shr => "shr",
            Opcode::Push => "push",
            Opcode::Pop => "pop",
            Opcode::Peek => "peek",
            Opcode::Load => "load",
            Opcode::Store => "store",
            Opcode::LoadIndexed => "load_indexed",
            Opcode::StoreIndexed => "store_indexed",
            Opcode::Alloc => "alloc",
            Opcode::Free => "free",
            Opcode::MemCopy => "memcpy",
            Opcode::MemSet => "memset",
            Opcode::Jump => "jump",
            Opcode::JumpIfZero => "jump_if_zero",
            Opcode::JumpIfNotZero => "jump_if_not_zero",
            Opcode::JumpIfGt => "jump_if_gt",
            Opcode::JumpIfLt => "jump_if_lt",
            Opcode::JumpIfGe => "jump_if_ge",
            Opcode::JumpIfLe => "jump_if_le",
            Opcode::JumpIfEq => "jump_if_eq",
            Opcode::JumpIfNe => "jump_if_ne",
            Opcode::JumpIfAbove => "jump_if_above",
            Opcode::JumpIfBelow => "jump_if_below",
            Opcode::JumpIfAe => "jump_if_ae",
            Opcode::JumpIfBe => "jump_if_be",
            Opcode::Compare => "compare",
            Opcode::Call => "call",
            Opcode::Return => "return",
            Opcode::Syscall => "syscall",
            Opcode::FAdd => "fadd",
            Opcode::FSub => "fsub",
            Opcode::FMul => "fmul",
            Opcode::FDiv => "fdiv",
            Opcode::FSqrt => "fsqrt",
            Opcode::FAbs => "fabs",
            Opcode::FNeg => "fneg",
            Opcode::F2I => "f2i",
            Opcode::I2F => "i2f",
            Opcode::FCmp => "fcmp",
            Opcode::PopCnt => "popcnt",
            Opcode::Clz => "clz",
            Opcode::Ctz => "ctz",
            Opcode::BSwap => "bswap",
            Opcode::RotL => "rotl",
            Opcode::RotR => "rotr",
            Opcode::Breakpoint => "breakpoint",
            Opcode::TraceOn => "trace_on",
            Opcode::TraceOff => "trace_off",
        }
    }
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// Errors related to opcode operations
#[derive(Debug, Clone, PartialEq)]
pub enum OpcodeError {
    Unknown(u8),
}

impl fmt::Display for OpcodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OpcodeError::Unknown(code) => {
                write!(f, "Unknown opcode: {:#x}", code)
            }
        }
    }
}

impl std::error::Error for OpcodeError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opcode_roundtrip() {
        let opcodes = vec![
            Opcode::Halt, Opcode::Add, Opcode::Jump, Opcode::Syscall, Opcode::Compare,
        ];

        for opcode in opcodes {
            let byte = opcode.to_u8();
            let decoded = Opcode::from_u8(byte).unwrap();
            assert_eq!(decoded, opcode);
        }
    }

    #[test]
    fn test_unknown_opcode() {
        assert!(Opcode::from_u8(0xFF).is_err());
    }
}
