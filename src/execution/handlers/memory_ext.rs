use crate::execution::context::ExecutionContext;
use crate::memory::MemoryAccess;
use crate::memory::heap::Heap;
use crate::core::Register;
use crate::error::VmError;

pub fn handle_alloc(ctx: &mut ExecutionContext, heap: &Heap, memory: &mut dyn MemoryAccess, dest: Register, size_reg: Register) -> Result<(), VmError> {
    let size = ctx.get_reg(size_reg) as usize;
    let ptr = heap.alloc(memory, size).map_err(VmError::from)?;
    ctx.set_reg(dest, ptr as u64);
    Ok(())
}

pub fn handle_free(ctx: &mut ExecutionContext, heap: &Heap, memory: &mut dyn MemoryAccess, ptr_reg: Register) -> Result<(), VmError> {
    let ptr = ctx.get_reg(ptr_reg) as usize;
    heap.free(memory, ptr).map_err(VmError::from)?;
    Ok(())
}

pub fn handle_memcpy(ctx: &mut ExecutionContext, memory: &mut dyn MemoryAccess, dest_reg: Register, src_reg: Register, size_reg: Register) -> Result<(), VmError> {
    let dest = ctx.get_reg(dest_reg) as usize;
    let src = ctx.get_reg(src_reg) as usize;
    let size = ctx.get_reg(size_reg) as usize;
    
    // Naive implementation: byte by byte to handle potential overlap or segment boundaries
    for i in 0..size {
        let byte = memory.read_byte(src + i).map_err(VmError::from)?;
        memory.write_byte(dest + i, byte).map_err(VmError::from)?;
    }
    
    Ok(())
}

pub fn handle_memset(ctx: &mut ExecutionContext, memory: &mut dyn MemoryAccess, dest_reg: Register, value_reg: Register, size_reg: Register) -> Result<(), VmError> {
    let dest = ctx.get_reg(dest_reg) as usize;
    let value = ctx.get_reg(value_reg) as u8;
    let size = ctx.get_reg(size_reg) as usize;
    
    for i in 0..size {
        memory.write_byte(dest + i, value).map_err(VmError::from)?;
    }
    
    Ok(())
}
