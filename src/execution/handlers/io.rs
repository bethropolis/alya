use crate::memory::{Memory, MemoryAccess};
use crate::core::Register;
use crate::execution::context::ExecutionContext;

/// Execute Syscall
/// R0 = Syscall ID
/// R1... = Arguments
pub fn handle_syscall(ctx: &mut ExecutionContext, memory: &Memory, output: &mut Vec<String>, print_immediately: bool) {
    let id = ctx.get_reg(Register::R0);
    
    match id {
        1 => {
            // Print Integer (Arg: R1)
            let value = ctx.get_reg(Register::R1);
            if print_immediately {
                println!("{}", value);
            }
            output.push(format!("{}", value));
        }
        2 => {
            // Print String (Arg: R1 = Address)
            let addr = ctx.get_reg(Register::R1) as usize;
            let mut bytes = Vec::new();
            let mut curr = addr;
            
            // Read null-terminated string
            loop {
                match memory.read_byte(curr) {
                    Ok(0) => break,
                    Ok(b) => {
                        bytes.push(b);
                        curr += 1;
                    }
                    Err(_) => break, // Stop on error
                }
                // Safety limit
                if bytes.len() > 1024 { break; }
            }
            
            let s = String::from_utf8_lossy(&bytes);
            if print_immediately {
                println!("{}", s);
            }
            output.push(s.to_string());
        }
        3 => {
            // Debug (Arg: R1)
            let value = ctx.get_reg(Register::R1);
            let msg = format!("DEBUG R1 = {} (0x{:x})", value, value);
             if print_immediately {
                eprintln!("{}", msg);
            }
            output.push(msg);
        }
        _ => {
            let msg = format!("Unknown syscall ID: {}", id);
            if print_immediately {
                eprintln!("{}", msg);
            }
        }
    }
}
