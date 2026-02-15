//! Code generation — converts AST statements into VM instructions.
//!
//! Key responsibility: maps named variables (e.g., `counter`, `x`, `r0`)
//! to physical registers (R0–R15). Uses a simple linear allocator.
//! Labels are resolved with a two-pass approach:
//!   Pass 1: Emit all instructions, recording label positions as they appear
//!   Pass 2: Resolve placeholders (jumps/calls to labels) using recorded positions

use std::collections::HashMap;
use crate::core::Register;
use crate::instruction::Instruction;
use crate::error::VmError;
use crate::assembler::parser::ast::*;

/// Generate a list of instructions and debug info from parsed statements.
pub fn generate(statements: Vec<SpannedStatement>) -> Result<(Vec<Instruction>, Vec<u8>, Vec<usize>), VmError> {
    let mut gen = CodeGenerator::new();
    gen.generate(statements)
}

struct CodeGenerator {
    /// Map from variable name to register
    var_map: HashMap<String, Register>,
    /// Next free general-purpose register index
    next_reg: u8,
    /// Map from label name to instruction index
    label_map: HashMap<String, usize>,
    /// Collected instructions (with possible unresolved label refs)
    instructions: Vec<InstructionSlot>,
    /// Accumulated data strings
    data_section: Vec<u8>,
    /// Line numbers corresponding to instructions
    line_table: Vec<usize>,
}

/// During codegen, some jumps have unknown targets. We use placeholders.
#[derive(Debug, Clone)]
enum InstructionSlot {
    Real(Instruction),
    Jump { label: String },
    JumpIf { comparison: Comparison, label: String },
    Call { label: String },
    /// Load address of a string in data section. Value is offset in data_section.
    LoadStringAddress { dest: Register, offset: usize },
}

impl CodeGenerator {
    fn new() -> Self {
        Self {
            var_map: HashMap::new(),
            next_reg: 0,
            label_map: HashMap::new(),
            instructions: Vec::new(),
            data_section: Vec::new(),
            line_table: Vec::new(),
        }
    }

    /// Get or allocate a register for a named variable.
    fn resolve_var(&mut self, name: &str) -> Result<Register, VmError> {
        if let Some(&reg) = self.var_map.get(name) {
            return Ok(reg);
        }

        // Check if it's a named register like "r0" ... "r15", "sp", "bp"
        if let Some(reg) = try_parse_register_name(name) {
            self.var_map.insert(name.to_string(), reg);
            return Ok(reg);
        }

        // Special case: if it's our scratch register and all GP are taken,
        // we "borrow" R15. This is slightly risky but usually fine in this VM.
        // A better fix would be push/pop, but let's try this first.
        if name == "__tmp" && self.next_reg >= Register::GP_COUNT as u8 {
             return Ok(Register::R15);
        }

        // Allocate the next free register, skipping any already claimed
        loop {
            if self.next_reg >= Register::GP_COUNT as u8 {
                return Err(VmError::Assembler(format!(
                    "Too many variables: cannot allocate register for '{}' (all {} GP registers in use)",
                    name, Register::GP_COUNT
                )));
            }

            let reg = Register::from_u8(self.next_reg)
                .map_err(|e| VmError::Assembler(format!("{}", e)))?;
            self.next_reg += 1;

            // Skip if already claimed by an explicit register name
            if self.var_map.values().any(|&r| r == reg) {
                continue;
            }

            self.var_map.insert(name.to_string(), reg);
            return Ok(reg);
        }
    }

    /// Resolve an Operand to a register, inserting a LoadImm if it's an immediate.
    fn resolve_operand(&mut self, operand: &Operand, line: usize) -> Result<Register, VmError> {
        match operand {
            Operand::Variable(name) => self.resolve_var(name),
            Operand::Immediate(value) => {
                // Reuse the same temporary register name everywhere to avoid exhaustion
                let temp_name = "__tmp";
                let reg = self.resolve_var(temp_name)?;
                self.push_instr(
                    Instruction::LoadImm { dest: reg, value: *value },
                    line
                );
                Ok(reg)
            }
        }
    }
    
    fn push_slot(&mut self, slot: InstructionSlot, line: usize) {
        self.instructions.push(slot);
        self.line_table.push(line);
    }

    fn push_instr(&mut self, instr: Instruction, line: usize) {
        self.push_slot(InstructionSlot::Real(instr), line);
    }

    /// Main generation entry point.
    fn generate(&mut self, statements: Vec<SpannedStatement>) -> Result<(Vec<Instruction>, Vec<u8>, Vec<usize>), VmError> {
        // Emit instructions for each statement; labels record positions as they appear.
        for stmt in statements {
            self.emit_statement(stmt)?;
        }

        // Resolve all label references
        let instrs = self.resolve_labels()?;
        Ok((instrs, self.data_section.clone(), self.line_table.clone()))
    }

    fn emit_statement(&mut self, spanned: SpannedStatement) -> Result<(), VmError> {
        let line = spanned.line;
        match spanned.node {
            Statement::Label(name) => {
                // Record the current instruction index for this label
                self.label_map.insert(name, self.instructions.len());
            }
            Statement::Halt => {
                self.push_instr(Instruction::Halt, line);
            }
            Statement::Nop => {
                self.push_instr(Instruction::Nop, line);
            }
            Statement::Return => {
                self.push_instr(Instruction::Return, line);
            }
            Statement::LoadImm { dest, value } => {
                let reg = self.resolve_var(&dest)?;
                self.push_instr(
                    Instruction::LoadImm { dest: reg, value },
                    line
                );
            }
            Statement::MoveVar { dest, src } => {
                let dest_reg = self.resolve_var(&dest)?;
                let src_reg = self.resolve_var(&src)?;
                self.push_instr(
                    Instruction::Move { dest: dest_reg, src: src_reg },
                    line
                );
            }
            Statement::Swap { left, right } => {
                let r1 = self.resolve_var(&left)?;
                let r2 = self.resolve_var(&right)?;
                self.push_instr(
                    Instruction::Swap { r1, r2 },
                    line
                );
            }
            Statement::BinOp { dest, left, op, right } => {
                let left_reg = self.resolve_var(&left)?;
                let right_reg = self.resolve_operand(&right, line)?;
                let dest_reg = self.resolve_var(&dest)?;

                let instr = match op {
                    BinOp::Add => Instruction::Add { dest: dest_reg, left: left_reg, right: right_reg },
                    BinOp::Sub => Instruction::Sub { dest: dest_reg, left: left_reg, right: right_reg },
                    BinOp::Mul => Instruction::Mul { dest: dest_reg, left: left_reg, right: right_reg },
                    BinOp::Div => Instruction::Div { dest: dest_reg, left: left_reg, right: right_reg },
                    BinOp::Mod => Instruction::Mod { dest: dest_reg, left: left_reg, right: right_reg },
                    BinOp::And => Instruction::And { dest: dest_reg, left: left_reg, right: right_reg },
                    BinOp::Or  => Instruction::Or  { dest: dest_reg, left: left_reg, right: right_reg },
                    BinOp::Xor => Instruction::Xor { dest: dest_reg, left: left_reg, right: right_reg },
                    BinOp::Shl => Instruction::Shl { dest: dest_reg, left: left_reg, right: right_reg },
                    BinOp::Shr => Instruction::Shr { dest: dest_reg, left: left_reg, right: right_reg },
                };
                self.push_instr(instr, line);
            }
            Statement::UnaryOp { dest, op: UnaryOp::Not, operand } => {
                let dest_reg = self.resolve_var(&dest)?;
                let src_reg = self.resolve_var(&operand)?;
                self.push_instr(
                    Instruction::Not { dest: dest_reg, src: src_reg },
                    line
                );
            }
            Statement::CompoundAssign { dest, op, operand } => {
                let dest_reg = self.resolve_var(&dest)?;
                let src_reg = self.resolve_operand(&operand, line)?;

                let instr = match op {
                    CompoundOp::Add => Instruction::AddAssign { dest: dest_reg, src: src_reg },
                    CompoundOp::Sub => Instruction::SubAssign { dest: dest_reg, src: src_reg },
                    CompoundOp::Mul => Instruction::MulAssign { dest: dest_reg, src: src_reg },
                    CompoundOp::Div => Instruction::DivAssign { dest: dest_reg, src: src_reg },
                };
                self.push_instr(instr, line);
            }
            Statement::Push(name) => {
                let reg = self.resolve_var(&name)?;
                self.push_instr(
                    Instruction::Push { src: reg },
                    line
                );
            }
            Statement::Pop(name) => {
                let reg = self.resolve_var(&name)?;
                self.push_instr(
                    Instruction::Pop { dest: reg },
                    line
                );
            }
            Statement::Peek(name) => {
                let reg = self.resolve_var(&name)?;
                self.push_instr(
                    Instruction::Peek { dest: reg },
                    line
                );
            }
            Statement::Syscall => {
                self.push_instr(Instruction::Syscall, line);
            }
            Statement::Print(name) => {
                let reg = self.resolve_var(&name)?;
                
                self.push_instr(Instruction::Push { src: Register::R0 }, line);
                self.push_instr(Instruction::Push { src: Register::R1 }, line);
                
                self.push_instr(Instruction::Move { dest: Register::R1, src: reg }, line);
                self.push_instr(Instruction::LoadImm { dest: Register::R0, value: 1 }, line);
                self.push_instr(Instruction::Syscall, line);
                
                self.push_instr(Instruction::Pop { dest: Register::R1 }, line);
                self.push_instr(Instruction::Pop { dest: Register::R0 }, line);
            }
            Statement::Debug(name) => {
                 // Lower debug @reg to Syscall ID 3
                let reg = self.resolve_var(&name)?;
                
                self.push_instr(Instruction::Push { src: Register::R0 }, line);
                self.push_instr(Instruction::Push { src: Register::R1 }, line);
                
                self.push_instr(Instruction::Move { dest: Register::R1, src: reg }, line);
                self.push_instr(Instruction::LoadImm { dest: Register::R0, value: 3 }, line);
                self.push_instr(Instruction::Syscall, line);
                
                self.push_instr(Instruction::Pop { dest: Register::R1 }, line);
                self.push_instr(Instruction::Pop { dest: Register::R0 }, line);
            }
            Statement::Goto(label) => {
                self.push_slot(InstructionSlot::Jump { label }, line);
            }
            Statement::Call(label) => {
                self.push_slot(InstructionSlot::Call { label }, line);
            }
            Statement::If { left, comparison, right, label } => {
                let left_reg = self.resolve_var(&left)?;
                let right_reg = self.resolve_operand(&right, line)?;
                // Emit Compare instruction
                self.push_instr(
                    Instruction::Compare { left: left_reg, right: right_reg },
                    line
                );
                // Emit conditional jump placeholder
                self.push_slot(InstructionSlot::JumpIf {
                    comparison,
                    label,
                }, line);
            }
            Statement::Store { value_var, addr_var } => {
                let src_reg = self.resolve_var(&value_var)?;
                let addr_reg = self.resolve_var(&addr_var)?;
                self.push_instr(
                    Instruction::Store { src: src_reg, addr_reg },
                    line
                );
            }
            Statement::Load { dest_var, addr_var } => {
                let dest_reg = self.resolve_var(&dest_var)?;
                let addr_reg = self.resolve_var(&addr_var)?;
                self.push_instr(
                    Instruction::Load { dest: dest_reg, addr_reg },
                    line
                );
            }
            Statement::StoreIndexed { base_var, index_var, value } => {
                let base_reg = self.resolve_var(&base_var)?;
                let index_reg = self.resolve_var(&index_var)?;
                let value_reg = self.resolve_operand(&value, line)?;
                self.push_instr(
                    Instruction::StoreIndexed { src: value_reg, base_reg, index_reg },
                    line
                );
            }
            Statement::LoadIndexed { dest, base_var, index_var } => {
                let dest_reg = self.resolve_var(&dest)?;
                let base_reg = self.resolve_var(&base_var)?;
                let index_reg = self.resolve_var(&index_var)?;
                self.push_instr(
                    Instruction::LoadIndexed { dest: dest_reg, base_reg, index_reg },
                    line
                );
            }
            Statement::LoadString { dest, value } => {
                let reg = self.resolve_var(&dest)?;
                
                let offset = self.data_section.len();
                self.data_section.extend_from_slice(value.as_bytes());
                self.data_section.push(0);

                self.push_slot(InstructionSlot::LoadStringAddress { dest: reg, offset }, line);
            }
            Statement::Alloc { dest, size_var } => {
                let dest_reg = self.resolve_var(&dest)?;
                let size_reg = self.resolve_var(&size_var)?;
                self.push_instr(
                    Instruction::Alloc { dest: dest_reg, size: size_reg },
                    line
                );
            }
            Statement::Free { ptr_var } => {
                let ptr_reg = self.resolve_var(&ptr_var)?;
                self.push_instr(
                    Instruction::Free { ptr: ptr_reg },
                    line
                );
            }
            Statement::MemCopy { dest_var, src_var, size_var } => {
                let dest_reg = self.resolve_var(&dest_var)?;
                let src_reg = self.resolve_var(&src_var)?;
                let size_reg = self.resolve_var(&size_var)?;
                self.push_instr(
                    Instruction::MemCopy { dest: dest_reg, src: src_reg, size: size_reg },
                    line
                );
            }
            Statement::MemSet { dest_var, value_var, size_var } => {
                let dest_reg = self.resolve_var(&dest_var)?;
                let value_reg = self.resolve_var(&value_var)?;
                let size_reg = self.resolve_var(&size_var)?;
                self.push_instr(
                    Instruction::MemSet { dest: dest_reg, value: value_reg, size: size_reg },
                    line
                );
            }
            Statement::FBinOp { dest, left, op, right } => {
                let dest_reg = self.resolve_var(&dest)?;
                let left_reg = self.resolve_var(&left)?;
                let right_reg = self.resolve_var(&right)?;
                let instr = match op {
                    FBinOp::Add => Instruction::FAdd { dest: dest_reg, left: left_reg, right: right_reg },
                    FBinOp::Sub => Instruction::FSub { dest: dest_reg, left: left_reg, right: right_reg },
                    FBinOp::Mul => Instruction::FMul { dest: dest_reg, left: left_reg, right: right_reg },
                    FBinOp::Div => Instruction::FDiv { dest: dest_reg, left: left_reg, right: right_reg },
                };
                self.push_instr(instr, line);
            }
            Statement::FUnaryOp { dest, op, src } => {
                let dest_reg = self.resolve_var(&dest)?;
                let src_reg = self.resolve_var(&src)?;
                let instr = match op {
                    FUnaryOp::Sqrt => Instruction::FSqrt { dest: dest_reg, src: src_reg },
                    FUnaryOp::Abs => Instruction::FAbs { dest: dest_reg, src: src_reg },
                    FUnaryOp::Neg => Instruction::FNeg { dest: dest_reg, src: src_reg },
                    FUnaryOp::ToInt => Instruction::F2I { dest: dest_reg, src: src_reg },
                    FUnaryOp::ToFloat => Instruction::I2F { dest: dest_reg, src: src_reg },
                };
                self.push_instr(instr, line);
            }
            Statement::FCmp { left, right } => {
                let left_reg = self.resolve_var(&left)?;
                let right_reg = self.resolve_var(&right)?;
                self.push_instr(
                    Instruction::FCmp { left: left_reg, right: right_reg },
                    line
                );
            }
            Statement::BitUnaryOp { dest, op, src } => {
                let dest_reg = self.resolve_var(&dest)?;
                let src_reg = self.resolve_var(&src)?;
                let instr = match op {
                    BitUnaryOp::PopCnt => Instruction::PopCnt { dest: dest_reg, src: src_reg },
                    BitUnaryOp::Clz => Instruction::Clz { dest: dest_reg, src: src_reg },
                    BitUnaryOp::Ctz => Instruction::Ctz { dest: dest_reg, src: src_reg },
                    BitUnaryOp::BSwap => Instruction::BSwap { dest: dest_reg, src: src_reg },
                };
                self.push_instr(instr, line);
            }
            Statement::BitRotOp { dest, left, op, right } => {
                let dest_reg = self.resolve_var(&dest)?;
                let left_reg = self.resolve_var(&left)?;
                let right_reg = self.resolve_var(&right)?;
                let instr = match op {
                    BitRotOp::RotL => Instruction::RotL { dest: dest_reg, left: left_reg, right: right_reg },
                    BitRotOp::RotR => Instruction::RotR { dest: dest_reg, left: left_reg, right: right_reg },
                };
                self.push_instr(instr, line);
            }
        }
        Ok(())
    }

    /// Replace all label placeholders with resolved instruction indices.
    fn resolve_labels(&self) -> Result<Vec<Instruction>, VmError> {
        let mut result = Vec::with_capacity(self.instructions.len());

        for slot in &self.instructions {
            match slot {
                InstructionSlot::Real(i) => {
                    result.push(i.clone());
                }
                InstructionSlot::Jump { label } => {
                    let target = self.label_map.get(label)
                        .ok_or_else(|| VmError::Assembler(format!("Undefined label: '{}'", label)))?;
                    result.push(Instruction::Jump { target: *target });
                }
                InstructionSlot::Call { label } => {
                    let target = self.label_map.get(label)
                        .ok_or_else(|| VmError::Assembler(format!("Undefined label: '{}'", label)))?;
                    result.push(Instruction::Call { target: *target });
                }
                InstructionSlot::JumpIf { comparison, label } => {
                    let target = self.label_map.get(label)
                        .ok_or_else(|| VmError::Assembler(format!("Undefined label: '{}'", label)))?;
                    let jump = match comparison {
                        Comparison::Equal => Instruction::JumpIfEq { target: *target },
                        Comparison::NotEqual => Instruction::JumpIfNe { target: *target },
                        Comparison::GreaterThan => Instruction::JumpIfGt { target: *target },
                        Comparison::LessThan => Instruction::JumpIfLt { target: *target },
                        Comparison::GreaterEqual => Instruction::JumpIfGe { target: *target },
                        Comparison::LessEqual => Instruction::JumpIfLe { target: *target },
                        Comparison::UnsignedGreaterThan => Instruction::JumpIfAbove { target: *target },
                        Comparison::UnsignedLessThan => Instruction::JumpIfBelow { target: *target },
                        Comparison::UnsignedGreaterEqual => Instruction::JumpIfAe { target: *target },
                        Comparison::UnsignedLessEqual => Instruction::JumpIfBe { target: *target },
                    };
                    result.push(jump);
                }
                InstructionSlot::LoadStringAddress { dest, offset } => {
                    // Load the address (offset in memory)
                    // We assume data is loaded at memory address 0
                    result.push(Instruction::LoadImm { 
                         dest: *dest, 
                         value: *offset as u64 
                    });
                }
            }
        }

        Ok(result)
    }
}

/// Try to parse a register name like "r0", "r1", ..., "r15", "sp", "bp"
fn try_parse_register_name(name: &str) -> Option<Register> {
    match name {
        "r0" => Some(Register::R0),
        "r1" => Some(Register::R1),
        "r2" => Some(Register::R2),
        "r3" => Some(Register::R3),
        "r4" => Some(Register::R4),
        "r5" => Some(Register::R5),
        "r6" => Some(Register::R6),
        "r7" => Some(Register::R7),
        "r8" => Some(Register::R8),
        "r9" => Some(Register::R9),
        "r10" => Some(Register::R10),
        "r11" => Some(Register::R11),
        "r12" => Some(Register::R12),
        "r13" => Some(Register::R13),
        "r14" => Some(Register::R14),
        "r15" => Some(Register::R15),
        "sp" => Some(Register::SP),
        "bp" => Some(Register::BP),
        "hp" => Some(Register::HP),
        "ip" => Some(Register::IP),
        "fl" => Some(Register::FL),
        "f0" => Some(Register::F0),
        "f1" => Some(Register::F1),
        "f2" => Some(Register::F2),
        "f3" => Some(Register::F3),
        "f4" => Some(Register::F4),
        "f5" => Some(Register::F5),
        "f6" => Some(Register::F6),
        "f7" => Some(Register::F7),
        "f8" => Some(Register::F8),
        "f9" => Some(Register::F9),
        "f10" => Some(Register::F10),
        "f11" => Some(Register::F11),
        "f12" => Some(Register::F12),
        "f13" => Some(Register::F13),
        "f14" => Some(Register::F14),
        "f15" => Some(Register::F15),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assembler::parser;

    #[test]
    fn test_codegen_hello() {
        let stmts = parser::parse("@r0 := 42\nprint @r0\nhalt\n").unwrap();
        let (instructions, _, _) = generate(stmts).unwrap();
        // 0: LoadImm
        // Print expands to: Push, Push, Move, LoadImm, Syscall, Pop, Pop (7 instrs)
        // Total 1 + 7 + 1 (Halt) = 9
        assert_eq!(instructions.len(), 9);
        assert!(matches!(&instructions[0], Instruction::LoadImm { value: 42, .. }));
        // Check for syscall
        assert!(matches!(&instructions[5], Instruction::Syscall));
        assert!(matches!(&instructions[8], Instruction::Halt));
    }

    #[test]
    fn test_codegen_jump() {
        let stmts = parser::parse("goto end\n@r0 := 99\nend:\nhalt\n").unwrap();
        let (instructions, _, _) = generate(stmts).unwrap();
        // goto end -> Jump { target: 2 } (skipping the loadimm)
        // @r0 := 99 -> LoadImm
        // end: -> (no instruction, label points to index 2)
        // halt -> Halt at index 2
        assert_eq!(instructions.len(), 3);
        assert!(matches!(&instructions[0], Instruction::Jump { target: 2 }));
        assert!(matches!(&instructions[2], Instruction::Halt));
    }
}
