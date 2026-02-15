//! Program container — a sequence of instructions.

use super::Instruction;

/// A program is a named sequence of instructions.
#[derive(Debug, Clone)]
pub struct Program {
    pub name: String,
    pub instructions: Vec<Instruction>,
    pub data: Vec<u8>,
}

impl Program {
    /// Create a new empty program
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            instructions: Vec::new(),
            data: Vec::new(),
        }
    }

    /// Create a program with instructions and data
    pub fn with_data(name: impl Into<String>, instructions: Vec<Instruction>, data: Vec<u8>) -> Self {
        Self {
            name: name.into(),
            instructions,
            data,
        }
    }

    /// Create a program from a vector of instructions
    pub fn from_instructions(name: impl Into<String>, instructions: Vec<Instruction>) -> Self {
        Self {
            name: name.into(),
            instructions,
            data: Vec::new(),
        }
    }

    /// Add an instruction
    pub fn push(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);
    }

    /// Get instruction at index
    pub fn get(&self, index: usize) -> Option<&Instruction> {
        self.instructions.get(index)
    }

    /// Number of instructions
    pub fn len(&self) -> usize {
        self.instructions.len()
    }

    /// Check if program is empty
    pub fn is_empty(&self) -> bool {
        self.instructions.is_empty()
    }
}
