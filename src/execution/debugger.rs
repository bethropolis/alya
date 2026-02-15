use std::collections::HashSet;
use std::io::{self, Write};
use crate::instruction::Program;
use crate::execution::VM;
use crate::error::VmResult;
use crate::core::Register;

pub struct Debugger {
    vm: VM,
    breakpoints: HashSet<usize>,
}

impl Debugger {
    pub fn new(vm: VM) -> Self {
        Self {
            vm,
            breakpoints: HashSet::new(),
        }
    }

    pub fn run(&mut self, program: &Program) -> VmResult<()> {
        println!("Alya Debugger (v0.5)");
        println!("Type 'help' for commands.");
        
        self.vm.init(program)?;

        loop {
            if self.vm.ctx.halted {
                println!("Program halted.");
            } else if self.vm.ctx.pc >= program.len() {
                println!("Program reached end.");
            }

            print!("(debug) ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            if io::stdin().read_line(&mut input).is_err() {
                break;
            }

            let parts: Vec<&str> = input.trim().split_whitespace().collect();
            if parts.is_empty() {
                continue;
            }

            match parts[0] {
                "step" | "s" => {
                    if self.vm.ctx.halted {
                        println!("Error: Program is halted.");
                    } else {
                        let pc = self.vm.ctx.pc;
                        if let Some(instr) = program.get(pc) {
                            println!("Step {:04x}: {}", pc, instr.to_assembly());
                            self.vm.step(program)?;
                            println!();
                        }
                    }
                }
                "next" | "n" => {
                    if self.vm.ctx.halted {
                        println!("Error: Program is halted.");
                    } else {
                        let current_line = program.line_table.get(self.vm.ctx.pc).copied();
                        if let Some(line) = current_line {
                             println!("Stepping line {}...", line);
                             // Step until we reach a different line OR it's a call
                             while !self.vm.ctx.halted && self.vm.ctx.pc < program.len() && 
                                   program.line_table.get(self.vm.ctx.pc) == Some(&line) {
                                 self.vm.step(program)?;
                             }
                        } else {
                             self.vm.step(program)?;
                        }
                        println!();
                    }
                }
                "continue" | "c" => {
                    if self.vm.ctx.halted {
                        println!("Error: Program is halted.");
                    } else {
                        println!("Continuing...");
                        while !self.vm.ctx.halted && self.vm.ctx.pc < program.len() {
                            if self.breakpoints.contains(&self.vm.ctx.pc) {
                                println!("Breakpoint reached at {:04x}", self.vm.ctx.pc);
                                break;
                            }
                            self.vm.step(program)?;
                        }
                        println!();
                    }
                }
                "prof" => {
                    println!("--- Performance Profile ---");
                    println!("Total Instructions: {}", self.vm.instruction_count);
                    println!("Top Opcodes:");
                    let mut freq: Vec<_> = self.vm.instr_freq.iter().collect();
                    freq.sort_by(|a, b| b.1.cmp(a.1));
                    
                    use crate::core::Opcode;
                    for (&op_u8, count) in freq.iter().take(8) {
                        let name = Opcode::from_u8(op_u8).map(|o| o.name()).unwrap_or("unknown");
                        let percentage = (**count as f64 / self.vm.instruction_count as f64) * 100.0;
                        println!("  {:<15} : {:>8} ({:>5.1}%)", name, *count, percentage);
                    }
                    println!();
                }
                "break" | "b" => {
                    if parts.len() < 2 {
                        println!("Usage: break <pc>");
                    } else {
                        if let Ok(pc) = usize::from_str_radix(parts[1].trim_start_matches("0x"), 16) {
                            self.breakpoints.insert(pc);
                            println!("Breakpoint set at {:04x}", pc);
                        } else if let Ok(pc) = parts[1].parse::<usize>() {
                            self.breakpoints.insert(pc);
                            println!("Breakpoint set at {:04x}", pc);
                        } else {
                            println!("Error: Invalid PC");
                        }
                    }
                }
                "list" | "l" => {
                    let start = self.vm.ctx.pc.saturating_sub(5);
                    let end = (self.vm.ctx.pc + 5).min(program.len());
                    for i in start..end {
                        let prefix = if i == self.vm.ctx.pc { "=>" } else { "  " };
                        let bp = if self.breakpoints.contains(&i) { "B" } else { " " };
                        if let Some(instr) = program.get(i) {
                            println!("{} {} {:04x}: {}", prefix, bp, i, instr.to_assembly());
                        }
                    }
                    println!();
                }
                "print" | "p" => {
                    if parts.len() < 2 {
                        println!("Usage: print <reg>");
                    } else {
                        let reg_name = parts[1].trim_start_matches('@').to_lowercase();
                        if let Some(reg) = self.try_resolve_register(&reg_name) {
                            let val = self.vm.ctx.get_reg(reg);
                            println!("{} = {} (0x{:x})", parts[1], val, val);
                        } else {
                            println!("Error: Unknown register '{}'", parts[1]);
                        }
                    }
                }
                "info" => {
                    if parts.len() < 2 || parts[1] != "registers" {
                        println!("Usage: info registers");
                    } else {
                        for i in 0..16 {
                            let reg = Register::from_u8(i).unwrap();
                            let val = self.vm.ctx.get_reg(reg);
                            println!("{:<4} = {:<12} (0x{:x})", reg.name(), val, val);
                        }
                        println!("{:<4} = {:<12} (0x{:x})", "IP", self.vm.ctx.pc, self.vm.ctx.pc);
                    }
                }
                "help" | "?" => {
                    println!("Commands:");
                    println!("  step (s)        Execute one instruction");
                    println!("  next (n)        Execute until next source line");
                    println!("  continue (c)    Run until breakpoint or end");
                    println!("  prof            Show instruction profiling data");
                    println!("  break (b) <pc>  Set breakpoint at instruction index");
                    println!("  list (l)        Show surrounding assembly");
                    println!("  print (p) <reg> Display register value");
                    println!("  info registers  Show all GP registers");
                    println!("  quit (q)        Exit debugger");
                }
                "quit" | "q" => break,
                _ => println!("Unknown command: '{}'. Type 'help' for info.", parts[0]),
            }
        }

        Ok(())
    }

    fn try_resolve_register(&self, name: &str) -> Option<Register> {
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
            "f0" => Some(Register::F0),
            // ... add more if needed
            _ => None,
        }
    }
}
