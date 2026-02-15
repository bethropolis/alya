//! Main VM facade — owns memory, stack, execution context, and runs programs.


use crate::error::{VmError, VmResult};
use crate::instruction::{Instruction, Program};
use crate::memory::Memory;
use crate::memory::stack::Stack;
use super::context::ExecutionContext;
use super::handlers::{arithmetic, logic, data_move, control, stack as stack_handler, memory as memory_handler};

/// Default memory size: 64KB
const DEFAULT_MEMORY_SIZE: usize = 65536;

/// Stack region starts at the top of memory


/// Maximum instructions to execute (prevents infinite loops)
const MAX_INSTRUCTIONS: u64 = 10_000_000;

/// The Alya Virtual Machine
pub struct VM {
    pub ctx: ExecutionContext,
    pub memory: Memory,
    pub stack: Stack,
    pub output: Vec<String>,
    pub print_immediately: bool,
}

impl VM {
    /// Create a new VM with default memory size
    pub fn new() -> Self {
        let memory = Memory::new(DEFAULT_MEMORY_SIZE);
        let stack = Stack::new(DEFAULT_MEMORY_SIZE); // Stack grows down from top
        Self {
            ctx: ExecutionContext::new(),
            memory,
            stack,
            output: Vec::new(),
            print_immediately: true,
        }
    }

    /// Create a new VM with specified memory size
    pub fn with_memory_size(size: usize) -> Self {
        let memory = Memory::new(size);
        let stack = Stack::new(size);
        Self {
            ctx: ExecutionContext::new(),
            memory,
            stack,
            output: Vec::new(),
            print_immediately: true,
        }
    }

    /// Run a program to completion
    pub fn run(&mut self, program: &Program) -> VmResult<()> {
        self.ctx.reset();
        
        // Load data section into memory (at address 0)
        self.memory.clear();
        if let Err(e) = self.memory.load_program(&program.data) {
             return Err(VmError::Execution(format!("Failed to load program data: {}", e)));
        }

        self.output.clear();
        let mut instruction_count: u64 = 0;

        while !self.ctx.halted && self.ctx.pc < program.len() {
            instruction_count += 1;
            if instruction_count > MAX_INSTRUCTIONS {
                return Err(VmError::Execution(format!(
                    "Exceeded maximum instruction count ({}). Possible infinite loop.",
                    MAX_INSTRUCTIONS
                )));
            }

            let instruction = program.get(self.ctx.pc)
                .ok_or_else(|| VmError::Execution(format!(
                    "Invalid program counter: {}",
                    self.ctx.pc
                )))?
                .clone();

            // Advance PC before execution (jumps may override)
            self.ctx.pc += 1;

            self.execute_instruction(&instruction)?;
        }

        Ok(())
    }

    /// Execute a single instruction
    fn execute_instruction(&mut self, instruction: &Instruction) -> VmResult<()> {
        match instruction {
            // Control
            Instruction::Halt => {
                self.ctx.halted = true;
            }
            Instruction::Nop => {}

            // Data Movement
            Instruction::LoadImm { dest, value } => {
                data_move::handle_load_imm(&mut self.ctx, *dest, *value);
            }
            Instruction::Move { dest, src } => {
                data_move::handle_move(&mut self.ctx, *dest, *src);
            }
            Instruction::Swap { r1, r2 } => {
                data_move::handle_swap(&mut self.ctx, *r1, *r2);
            }

            // Arithmetic
            Instruction::Add { dest, left, right } => {
                arithmetic::handle_add(&mut self.ctx, *dest, *left, *right);
            }
            Instruction::Sub { dest, left, right } => {
                arithmetic::handle_sub(&mut self.ctx, *dest, *left, *right);
            }
            Instruction::Mul { dest, left, right } => {
                arithmetic::handle_mul(&mut self.ctx, *dest, *left, *right);
            }
            Instruction::Div { dest, left, right } => {
                arithmetic::handle_div(&mut self.ctx, *dest, *left, *right)?;
            }
            Instruction::Mod { dest, left, right } => {
                arithmetic::handle_mod(&mut self.ctx, *dest, *left, *right)?;
            }

            // Compound Assignment
            Instruction::AddAssign { dest, src } => {
                arithmetic::handle_add_assign(&mut self.ctx, *dest, *src);
            }
            Instruction::SubAssign { dest, src } => {
                arithmetic::handle_sub_assign(&mut self.ctx, *dest, *src);
            }
            Instruction::MulAssign { dest, src } => {
                arithmetic::handle_mul_assign(&mut self.ctx, *dest, *src);
            }
            Instruction::DivAssign { dest, src } => {
                arithmetic::handle_div_assign(&mut self.ctx, *dest, *src)?;
            }

            // Bitwise
            Instruction::And { dest, left, right } => {
                logic::handle_and(&mut self.ctx, *dest, *left, *right);
            }
            Instruction::Or { dest, left, right } => {
                logic::handle_or(&mut self.ctx, *dest, *left, *right);
            }
            Instruction::Xor { dest, left, right } => {
                logic::handle_xor(&mut self.ctx, *dest, *left, *right);
            }
            Instruction::Not { dest, src } => {
                logic::handle_not(&mut self.ctx, *dest, *src);
            }
            Instruction::Shl { dest, left, right } => {
                logic::handle_shl(&mut self.ctx, *dest, *left, *right);
            }
            Instruction::Shr { dest, left, right } => {
                logic::handle_shr(&mut self.ctx, *dest, *left, *right);
            }

            // Stack
            Instruction::Push { src } => {
                stack_handler::handle_push(&mut self.ctx, &mut self.stack, &mut self.memory, *src)?;
            }
            Instruction::Pop { dest } => {
                stack_handler::handle_pop(&mut self.ctx, &mut self.stack, &self.memory, *dest)?;
            }
            Instruction::Peek { dest } => {
                stack_handler::handle_peek(&mut self.ctx, &self.stack, &self.memory, *dest)?;
            }

            // Memory
            Instruction::Load { dest, addr_reg } => {
                memory_handler::handle_load(&mut self.ctx, &self.memory, *dest, *addr_reg)?;
            }
            Instruction::Store { src, addr_reg } => {
                memory_handler::handle_store(&mut self.ctx, &mut self.memory, *src, *addr_reg)?;
            }
            Instruction::LoadIndexed { dest, base_reg, index_reg } => {
                memory_handler::handle_load_indexed(&mut self.ctx, &self.memory, *dest, *base_reg, *index_reg)?;
            }
            Instruction::StoreIndexed { src, base_reg, index_reg } => {
                memory_handler::handle_store_indexed(&mut self.ctx, &mut self.memory, *src, *base_reg, *index_reg)?;
            }

            // Control Flow
            Instruction::Jump { target } => {
                control::handle_jump(&mut self.ctx, *target);
            }
            Instruction::Compare { left, right } => {
                control::handle_compare(&mut self.ctx, *left, *right);
            }
            Instruction::JumpIfZero { target } => {
                control::handle_jump_if_zero(&mut self.ctx, *target);
            }
            Instruction::JumpIfNotZero { target } => {
                control::handle_jump_if_not_zero(&mut self.ctx, *target);
            }
            Instruction::JumpIfGt { target } => {
                control::handle_jump_if_gt(&mut self.ctx, *target);
            }
            Instruction::JumpIfLt { target } => {
                control::handle_jump_if_lt(&mut self.ctx, *target);
            }
            Instruction::JumpIfGe { target } => {
                control::handle_jump_if_ge(&mut self.ctx, *target);
            }
            Instruction::JumpIfLe { target } => {
                control::handle_jump_if_le(&mut self.ctx, *target);
            }
            Instruction::JumpIfEq { target } => {
                control::handle_jump_if_eq(&mut self.ctx, *target);
            }
            Instruction::JumpIfNe { target } => {
                control::handle_jump_if_ne(&mut self.ctx, *target);
            }
            Instruction::JumpIfAbove { target } => {
                control::handle_jump_if_above(&mut self.ctx, *target);
            }
            Instruction::JumpIfBelow { target } => {
                control::handle_jump_if_below(&mut self.ctx, *target);
            }
            Instruction::JumpIfAe { target } => {
                control::handle_jump_if_ae(&mut self.ctx, *target);
            }
            Instruction::JumpIfBe { target } => {
                control::handle_jump_if_be(&mut self.ctx, *target);
            }

            // Functions
            Instruction::Call { target } => {
                control::handle_call(&mut self.ctx, *target)?;
            }
            Instruction::Return => {
                control::handle_return(&mut self.ctx)?;
            }

            // System
            Instruction::Syscall => {
                // We need to pass output buffer.
                // IO handler needs mutable access to output and print flags.
                // We can't pass &mut self because self.ctx is already borrowed mutably?
                // `execute_instruction` takes `&mut self`.
                // But `self.ctx` is borrowed for `handle_syscall`?
                // Wait, `handle_syscall(ctx, output, flag)` takes separate borrows.
                // `execute_instruction` has `&mut self`.
                // `self.ctx` is a field. `self.output` is a field.
                // Rust borrow checker allows splitting borrows if we access fields directly?
                // But `execute_instruction` signatures matches on `instruction`.
                // `instruction` is borrowed from `program`? No, `program` is passed to `run`, but `instruction` is cloned or ref?
                // In `run`: `let instruction = ... .clone();`
                // So `instruction` is owned or local ref.
                
                // Problem: `handle_xxx(&mut self.ctx, ...)`
                // If I call `io::handle_syscall(&mut self.ctx, &mut self.output, self.print_immediately)`, it should work
                // because I'm borrowing disjoint fields of `self`.
                super::handlers::io::handle_syscall(&mut self.ctx, &self.memory, &mut self.output, self.print_immediately);
            }
        }

        Ok(())
    }

    /// Get collected output
    pub fn output(&self) -> &[String] {
        &self.output
    }
}

impl Default for VM {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Register;

    fn make_program(instructions: Vec<Instruction>) -> Program {
        Program::from_instructions("test", instructions)
    }

    // Helper to emit a print calculation
    fn emit_print(src: Register) -> Vec<Instruction> {
        vec![
            Instruction::Move { dest: Register::R1, src },
            Instruction::LoadImm { dest: Register::R0, value: 1 },
            Instruction::Syscall,
        ]
    }

    #[test]
    fn test_hello_world() {
        let mut instrs = vec![
            Instruction::LoadImm { dest: Register::R0, value: 42 },
        ];
        instrs.extend(emit_print(Register::R0));
        instrs.push(Instruction::Halt);

        let program = make_program(instrs);

        let mut vm = VM::new();
        vm.print_immediately = false;
        vm.run(&program).unwrap();

        assert_eq!(vm.output(), &["42"]);
    }

    #[test]
    fn test_arithmetic() {
        let mut instrs = vec![
            Instruction::LoadImm { dest: Register::R0, value: 10 },
            Instruction::LoadImm { dest: Register::R1, value: 20 },
            Instruction::Add { dest: Register::R2, left: Register::R0, right: Register::R1 },
        ];
        instrs.extend(emit_print(Register::R2));
        instrs.push(Instruction::Halt);

        let program = make_program(instrs);

        let mut vm = VM::new();
        vm.print_immediately = false;
        vm.run(&program).unwrap();

        assert_eq!(vm.output(), &["30"]);
    }

    #[test]
    fn test_stack_operations() {
        let mut instrs = vec![
            Instruction::LoadImm { dest: Register::R0, value: 42 },
            Instruction::Push { src: Register::R0 },
            Instruction::Pop { dest: Register::R1 },
        ];
        instrs.extend(emit_print(Register::R1));
        instrs.push(Instruction::Halt);

        let program = make_program(instrs);

        let mut vm = VM::new();
        vm.print_immediately = false;
        vm.run(&program).unwrap();

        assert_eq!(vm.output(), &["42"]);
    }

    #[test]
    fn test_jump() {
        let mut instrs = vec![
            Instruction::Jump { target: 2 },       // 0: skip to 2
            Instruction::LoadImm { dest: Register::R0, value: 999 }, // 1: skipped
            Instruction::LoadImm { dest: Register::R0, value: 42 },  // 2: loads 42
        ];
        instrs.extend(emit_print(Register::R0));
        instrs.push(Instruction::Halt);

        let program = make_program(instrs);

        let mut vm = VM::new();
        vm.print_immediately = false;
        vm.run(&program).unwrap();

        assert_eq!(vm.output(), &["42"]);
    }

    // Need to adjust targets for manual syscall expansion (3 instrs vs 1)
    #[test]
    fn test_conditional_jump() {
        // Old target 5 -> New target will be shifted
        // 0: LoadImm 5
        // 1: LoadImm 10
        // 2: Compare
        // 3: JumpIfLt -> Target ?
        // 4: LoadImm 0
        // 5: LoadImm 1
        // 6: Print (3 instrs)
        // 9: Halt
        
        // Target should be 5.
        
        let instructions = vec![
            Instruction::LoadImm { dest: Register::R0, value: 5 },
            Instruction::LoadImm { dest: Register::R1, value: 10 },
            Instruction::Compare { left: Register::R0, right: Register::R1 },
            Instruction::JumpIfLt { target: 5 },    // 3: r0 < r1, should jump
            Instruction::LoadImm { dest: Register::R2, value: 0 }, // 4: skipped
            Instruction::LoadImm { dest: Register::R2, value: 1 }, // 5: r0 < r1 is true
            // Print R2
            Instruction::Move { dest: Register::R1, src: Register::R2 }, // 6
            Instruction::LoadImm { dest: Register::R0, value: 1 }, // 7
            Instruction::Syscall, // 8
            Instruction::Halt, // 9
        ];

        let program = make_program(instructions);
        let mut vm = VM::new();
        vm.print_immediately = false;
        vm.run(&program).unwrap();

        assert_eq!(vm.output(), &["1"]);
    }

    #[test]
    fn test_call_return() {
         // 0: Jump to main (Target ?)
         // 1: function: add 10 to R0
         // 2: Add
         // 3: Return
         // Main:
         // 4?
         
         // 0: Jump { target: 4 }
         // 1: LoadImm R1, 10
         // 2: Add R0, R0, R1
         // 3: Return
         // 4: LoadImm R0, 5
         // 5: Call { target: 1 }
         // 6: Print R0 -> Move(6), Load(7), Syscall(8)
         // 9: Halt
         
        let instructions = vec![
            Instruction::Jump { target: 4 },
            Instruction::LoadImm { dest: Register::R1, value: 10 },
            Instruction::Add { dest: Register::R0, left: Register::R0, right: Register::R1 },
            Instruction::Return,
            Instruction::LoadImm { dest: Register::R0, value: 5 },
            Instruction::Call { target: 1 },
            Instruction::Move { dest: Register::R1, src: Register::R0 },
            Instruction::LoadImm { dest: Register::R0, value: 1 },
            Instruction::Syscall,
            Instruction::Halt,
        ];

        let program = make_program(instructions);
        let mut vm = VM::new();
        vm.print_immediately = false;
        vm.run(&program).unwrap();

        assert_eq!(vm.output(), &["15"]);
    }

    #[test]
    fn test_memory_operations() {
        let instructions = vec![
            Instruction::LoadImm { dest: Register::R0, value: 1000 },  // address
            Instruction::LoadImm { dest: Register::R1, value: 42 },    // value
            Instruction::Store { src: Register::R1, addr_reg: Register::R0 },
            Instruction::LoadImm { dest: Register::R2, value: 0 },     // clear R2
            Instruction::Load { dest: Register::R2, addr_reg: Register::R0 },
            Instruction::Move { dest: Register::R1, src: Register::R2 },
            Instruction::LoadImm { dest: Register::R0, value: 1 },
            Instruction::Syscall,
            Instruction::Halt,
        ];

        let program = make_program(instructions);
        let mut vm = VM::new();
        vm.print_immediately = false;
        vm.run(&program).unwrap();

        assert_eq!(vm.output(), &["42"]);
    }
}
