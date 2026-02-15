use crate::memory::{MemoryAccess};
use crate::memory::heap::Heap;
use crate::core::Register;
use crate::execution::context::ExecutionContext;

/// Execute Syscall
/// R0 = Syscall ID
/// R1... = Arguments
pub fn handle_syscall(ctx: &mut ExecutionContext, heap: &Heap, memory: &mut dyn MemoryAccess, output: &mut Vec<String>, print_immediately: bool) {
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
        4 => {
            // Malloc (Arg: R1 = Size, Ret: R0 = Ptr)
            let size = ctx.get_reg(Register::R1) as usize;
            match heap.alloc(memory, size) {
                Ok(ptr) => ctx.set_reg(Register::R0, ptr as u64),
                Err(e) => {
                    let msg = format!("Syscall Malloc error: {}", e);
                    if print_immediately { eprintln!("{}", msg); }
                    output.push(msg);
                    ctx.set_reg(Register::R0, 0);
                }
            }
        }
        5 => {
            // Free (Arg: R1 = Ptr)
            let ptr = ctx.get_reg(Register::R1) as usize;
            if let Err(e) = heap.free(memory, ptr) {
                let msg = format!("Syscall Free error: {}", e);
                if print_immediately { eprintln!("{}", msg); }
                output.push(msg);
            }
        }
        6 => {
            // Print Float (Arg: R1)
            let bits = ctx.get_reg(Register::R1);
            let value = f64::from_bits(bits);
            if print_immediately {
                println!("{}", value);
            }
            output.push(format!("{}", value));
        }
        _ => {
            let msg = format!("Unknown syscall ID: {}", id);
            if print_immediately {
                eprintln!("{}", msg);
            }
        }
    }
}
