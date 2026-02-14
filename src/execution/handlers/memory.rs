//! Memory instruction handlers.

use crate::core::Register;
use crate::execution::context::ExecutionContext;
use crate::memory::{Memory, MemoryAccess};
use crate::error::VmError;

/// Execute Load: dest = memory[addr_reg]
pub fn handle_load(ctx: &mut ExecutionContext, memory: &Memory, dest: Register, addr_reg: Register) -> Result<(), VmError> {
    let addr = ctx.get_reg(addr_reg) as usize;
    let value = memory.read_qword(addr)
        .map_err(|e| VmError::Memory(format!("{}", e)))?;
    ctx.set_reg(dest, value);
    Ok(())
}

/// Execute Store: memory[addr_reg] = src
pub fn handle_store(ctx: &mut ExecutionContext, memory: &mut Memory, src: Register, addr_reg: Register) -> Result<(), VmError> {
    let addr = ctx.get_reg(addr_reg) as usize;
    let value = ctx.get_reg(src);
    memory.write_qword(addr, value)
        .map_err(|e| VmError::Memory(format!("{}", e)))
}

/// Execute LoadIndexed: dest = memory[base_reg + index_reg * 8]
pub fn handle_load_indexed(ctx: &mut ExecutionContext, memory: &Memory, dest: Register, base_reg: Register, index_reg: Register) -> Result<(), VmError> {
    let base = ctx.get_reg(base_reg) as usize;
    let index = ctx.get_reg(index_reg) as usize;
    let addr = base + index * 8;
    let value = memory.read_qword(addr)
        .map_err(|e| VmError::Memory(format!("{}", e)))?;
    ctx.set_reg(dest, value);
    Ok(())
}

/// Execute StoreIndexed: memory[base_reg + index_reg * 8] = src
pub fn handle_store_indexed(ctx: &mut ExecutionContext, memory: &mut Memory, src: Register, base_reg: Register, index_reg: Register) -> Result<(), VmError> {
    let base = ctx.get_reg(base_reg) as usize;
    let index = ctx.get_reg(index_reg) as usize;
    let addr = base + index * 8;
    let value = ctx.get_reg(src);
    memory.write_qword(addr, value)
        .map_err(|e| VmError::Memory(format!("{}", e)))
}
