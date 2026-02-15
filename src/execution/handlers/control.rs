//! Control flow instruction handlers.

use crate::core::Register;
use crate::execution::context::ExecutionContext;
use crate::error::VmError;

/// Execute Compare: set flags based on left - right (SUB behavior)
pub fn handle_compare(ctx: &mut ExecutionContext, left: Register, right: Register) {
    let u_a = ctx.get_reg(left);
    let u_b = ctx.get_reg(right);
    let s_a = u_a as i64;
    let s_b = u_b as i64;

    // Zero: equality check (unsigned and signed are identical bitwise)
    ctx.flags.set_zero(u_a == u_b);

    // Carry: Unsigned borrow (a < b)
    ctx.flags.set_carry(u_a < u_b);

    // Negative & Overflow: require signed subtraction
    let (diff, overflow) = s_a.overflowing_sub(s_b);
    ctx.flags.set_negative(diff < 0);
    ctx.flags.set_overflow(overflow);
}

/// Execute Jump: unconditional jump
pub fn handle_jump(ctx: &mut ExecutionContext, target: usize) {
    ctx.pc = target;
}

/// Execute JumpIfZero
pub fn handle_jump_if_zero(ctx: &mut ExecutionContext, target: usize) {
    if ctx.flags.zero() {
        ctx.pc = target;
    }
}

/// Execute JumpIfNotZero
pub fn handle_jump_if_not_zero(ctx: &mut ExecutionContext, target: usize) {
    if !ctx.flags.zero() {
        ctx.pc = target;
    }
}

/// Execute JumpIfGt (Signed Greater: !Z && (N == V))
pub fn handle_jump_if_gt(ctx: &mut ExecutionContext, target: usize) {
    let z = ctx.flags.zero();
    let n = ctx.flags.negative();
    let v = ctx.flags.overflow();
    
    if !z && (n == v) {
        ctx.pc = target;
    }
}

/// Execute JumpIfLt (Signed Less: N != V)
pub fn handle_jump_if_lt(ctx: &mut ExecutionContext, target: usize) {
    let n = ctx.flags.negative();
    let v = ctx.flags.overflow();

    if n != v {
        ctx.pc = target;
    }
}

/// Execute JumpIfGe (Signed Greater Equal: N == V)
pub fn handle_jump_if_ge(ctx: &mut ExecutionContext, target: usize) {
    let n = ctx.flags.negative();
    let v = ctx.flags.overflow();

    if n == v {
        ctx.pc = target;
    }
}

/// Execute JumpIfLe (Signed Less Equal: Z || (N != V))
pub fn handle_jump_if_le(ctx: &mut ExecutionContext, target: usize) {
    let z = ctx.flags.zero();
    let n = ctx.flags.negative();
    let v = ctx.flags.overflow();

    if z || (n != v) {
        ctx.pc = target;
    }
}

/// Execute JumpIfEq (equal: zero flag set)
pub fn handle_jump_if_eq(ctx: &mut ExecutionContext, target: usize) {
    if ctx.flags.zero() {
        ctx.pc = target;
    }
}

/// Execute JumpIfNe (not equal: zero flag not set)
pub fn handle_jump_if_ne(ctx: &mut ExecutionContext, target: usize) {
    if !ctx.flags.zero() {
        ctx.pc = target;
    }
}

/// Execute JumpIfAbove (Unsigned >: !C && !Z)
pub fn handle_jump_if_above(ctx: &mut ExecutionContext, target: usize) {
    let c = ctx.flags.carry();
    let z = ctx.flags.zero();

    if !c && !z {
        ctx.pc = target;
    }
}

/// Execute JumpIfBelow (Unsigned < : C)
pub fn handle_jump_if_below(ctx: &mut ExecutionContext, target: usize) {
    if ctx.flags.carry() {
        ctx.pc = target;
    }
}

/// Execute JumpIfAe (Unsigned >= : !C)
pub fn handle_jump_if_ae(ctx: &mut ExecutionContext, target: usize) {
    if !ctx.flags.carry() {
        ctx.pc = target;
    }
}

/// Execute JumpIfBe (Unsigned <= : C || Z)
pub fn handle_jump_if_be(ctx: &mut ExecutionContext, target: usize) {
    let c = ctx.flags.carry();
    let z = ctx.flags.zero();

    if c || z {
        ctx.pc = target;
    }
}

const MAX_STACK_DEPTH: usize = 1024;

/// Execute Call: push return address, jump to target
pub fn handle_call(ctx: &mut ExecutionContext, target: usize) -> Result<(), VmError> {
    if ctx.call_stack.len() >= MAX_STACK_DEPTH {
        return Err(VmError::Execution("Stack overflow: maximum recursion depth exceeded".to_string()));
    }
    ctx.call_stack.push(ctx.pc);
    ctx.pc = target;
    Ok(())
}

/// Execute Return: pop return address, jump back
pub fn handle_return(ctx: &mut ExecutionContext) -> Result<(), VmError> {
    let return_addr = ctx.call_stack.pop()
        .ok_or_else(|| VmError::Execution("Return without matching call".to_string()))?;
    ctx.pc = return_addr;
    Ok(())
}
