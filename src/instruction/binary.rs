use crate::instruction::Instruction;
use crate::core::{Opcode, Register};
use crate::error::VmError;


impl Instruction {
    /// Encode instruction to bytes
    pub fn encode(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        
        let opcode = self.opcode();
        bytes.push(opcode.to_u8());
        
        match self {
            Instruction::Halt | Instruction::Nop | Instruction::Return | Instruction::Syscall => {}
            
            Instruction::LoadImm { dest, value } => {
                bytes.push(dest.to_u8());
                bytes.extend_from_slice(&value.to_le_bytes());
            }
            
            Instruction::Move { dest, src } | 
            Instruction::Not { dest, src } => {
                bytes.push(dest.to_u8());
                bytes.push(src.to_u8());
            }

            Instruction::Push { src } => {
                bytes.push(src.to_u8());
            }

            Instruction::Pop { dest } |
            Instruction::Peek { dest } => {
                bytes.push(dest.to_u8());
            }
            
            Instruction::Swap { r1, r2 } => {
                bytes.push(r1.to_u8());
                bytes.push(r2.to_u8());
            }
            
            Instruction::Add { dest, left, right } |
            Instruction::Sub { dest, left, right } |
            Instruction::Mul { dest, left, right } |
            Instruction::Div { dest, left, right } |
            Instruction::Mod { dest, left, right } |
            Instruction::And { dest, left, right } |
            Instruction::Or { dest, left, right } |
            Instruction::Xor { dest, left, right } |
            Instruction::Shl { dest, left, right } |
            Instruction::Shr { dest, left, right } => {
                bytes.push(dest.to_u8());
                bytes.push(left.to_u8());
                bytes.push(right.to_u8());
            }
            
            Instruction::AddAssign { dest, src } |
            Instruction::SubAssign { dest, src } |
            Instruction::MulAssign { dest, src } |
            Instruction::DivAssign { dest, src } => {
                bytes.push(dest.to_u8());
                bytes.push(src.to_u8());
            }
            
            Instruction::Load { dest, addr_reg } => {
                 bytes.push(dest.to_u8());
                 bytes.push(addr_reg.to_u8());
            }
            Instruction::Store { src, addr_reg } => {
                 bytes.push(src.to_u8());
                 bytes.push(addr_reg.to_u8());
            }
            
            Instruction::LoadIndexed { dest, base_reg, index_reg } => {
                bytes.push(dest.to_u8());
                bytes.push(base_reg.to_u8());
                bytes.push(index_reg.to_u8());
            }
            Instruction::StoreIndexed { src, base_reg, index_reg } => {
                bytes.push(src.to_u8());
                bytes.push(base_reg.to_u8());
                bytes.push(index_reg.to_u8());
            }
            
            Instruction::Jump { target } |
            Instruction::JumpIfZero { target } |
            Instruction::JumpIfNotZero { target } |
            Instruction::JumpIfGt { target } |
            Instruction::JumpIfLt { target } |
            Instruction::JumpIfGe { target } |
            Instruction::JumpIfLe { target } |
            Instruction::JumpIfEq { target } |
            Instruction::JumpIfNe { target } |
            Instruction::JumpIfAbove { target } |
            Instruction::JumpIfBelow { target } |
            Instruction::JumpIfAe { target } |
            Instruction::JumpIfBe { target } |
            Instruction::Call { target } => {
                bytes.extend_from_slice(&(*target as u64).to_le_bytes());
            }
            
            Instruction::Compare { left, right } => {
                bytes.push(left.to_u8());
                bytes.push(right.to_u8());
            }

            Instruction::Alloc { dest, size } => {
                bytes.push(dest.to_u8());
                bytes.push(size.to_u8());
            }
            Instruction::Free { ptr } => {
                bytes.push(ptr.to_u8());
            }
            Instruction::MemCopy { dest, src, size } |
            Instruction::MemSet { dest, value: src, size } => {
                bytes.push(dest.to_u8());
                bytes.push(src.to_u8());
                bytes.push(size.to_u8());
            }
        }
        
        bytes
    }
    
    /// Helper to get opcode from instruction
    pub fn opcode(&self) -> Opcode {
        match self {
            Instruction::Halt => Opcode::Halt,
            Instruction::Nop => Opcode::Nop,
            Instruction::LoadImm { .. } => Opcode::LoadImm,
            Instruction::Move { .. } => Opcode::Move,
            Instruction::Swap { .. } => Opcode::Swap,
            Instruction::Add { .. } => Opcode::Add,
            Instruction::Sub { .. } => Opcode::Sub,
            Instruction::Mul { .. } => Opcode::Mul,
            Instruction::Div { .. } => Opcode::Div,
            Instruction::Mod { .. } => Opcode::Mod,
            Instruction::AddAssign { .. } => Opcode::AddAssign,
            Instruction::SubAssign { .. } => Opcode::SubAssign,
            Instruction::MulAssign { .. } => Opcode::MulAssign,
            Instruction::DivAssign { .. } => Opcode::DivAssign,
            Instruction::And { .. } => Opcode::And,
            Instruction::Or { .. } => Opcode::Or,
            Instruction::Xor { .. } => Opcode::Xor,
            Instruction::Not { .. } => Opcode::Not,
            Instruction::Shl { .. } => Opcode::Shl,
            Instruction::Shr { .. } => Opcode::Shr,
            Instruction::Push { .. } => Opcode::Push,
            Instruction::Pop { .. } => Opcode::Pop,
            Instruction::Peek { .. } => Opcode::Peek,
            Instruction::Load { .. } => Opcode::Load,
            Instruction::Store { .. } => Opcode::Store,
            Instruction::LoadIndexed { .. } => Opcode::LoadIndexed,
            Instruction::StoreIndexed { .. } => Opcode::StoreIndexed,
            Instruction::Jump { .. } => Opcode::Jump,
            Instruction::JumpIfZero { .. } => Opcode::JumpIfZero,
            Instruction::JumpIfNotZero { .. } => Opcode::JumpIfNotZero,
            Instruction::JumpIfGt { .. } => Opcode::JumpIfGt,
            Instruction::JumpIfLt { .. } => Opcode::JumpIfLt,
            Instruction::JumpIfGe { .. } => Opcode::JumpIfGe,
            Instruction::JumpIfLe { .. } => Opcode::JumpIfLe,
            Instruction::JumpIfEq { .. } => Opcode::JumpIfEq,
            Instruction::JumpIfNe { .. } => Opcode::JumpIfNe,
            Instruction::JumpIfAbove { .. } => Opcode::JumpIfAbove,
            Instruction::JumpIfBelow { .. } => Opcode::JumpIfBelow,
            Instruction::JumpIfAe { .. } => Opcode::JumpIfAe,
            Instruction::JumpIfBe { .. } => Opcode::JumpIfBe,
            Instruction::Compare { .. } => Opcode::Compare,
            Instruction::Call { .. } => Opcode::Call,
            Instruction::Return => Opcode::Return,
            Instruction::Syscall => Opcode::Syscall,
            Instruction::Alloc { .. } => Opcode::Alloc,
            Instruction::Free { .. } => Opcode::Free,
            Instruction::MemCopy { .. } => Opcode::MemCopy,
            Instruction::MemSet { .. } => Opcode::MemSet,
        }
    }

    /// Decode instruction from bytes. Returns (Instruction, bytes_read).
    pub fn decode(bytes: &[u8]) -> Result<(Instruction, usize), VmError> {
        if bytes.is_empty() {
            return Err(VmError::Execution("Unexpected end of bytecode".to_string()));
        }
        
        let opcode_byte = bytes[0];
        let opcode = Opcode::from_u8(opcode_byte)
            .map_err(|e| VmError::Execution(format!("Invalid opcode: {}", e)))?;
            
        let mut pos = 1;
        
        let instr = match opcode {
            Opcode::Halt => Instruction::Halt,
            Opcode::Nop => Instruction::Nop,
            Opcode::Return => Instruction::Return,
            Opcode::Syscall => Instruction::Syscall,
            
            Opcode::LoadImm => {
                if bytes.len() < pos + 9 { return Err(VmError::Execution("Unexpected end of bytecode".to_string())); }
                let dest = Register::from_u8(bytes[pos]).map_err(|e| VmError::Execution(e.to_string()))?;
                pos += 1;
                let mut buf = [0u8; 8];
                buf.copy_from_slice(&bytes[pos..pos+8]);
                let value = u64::from_le_bytes(buf);
                pos += 8;
                Instruction::LoadImm { dest, value }
            }
            
            Opcode::Move => {
                if bytes.len() < pos + 2 { return Err(VmError::Execution("Unexpected end of bytecode".to_string())); }
                let dest = Register::from_u8(bytes[pos]).map_err(|e| VmError::Execution(e.to_string()))?;
                let src = Register::from_u8(bytes[pos+1]).map_err(|e| VmError::Execution(e.to_string()))?;
                pos += 2;
                Instruction::Move { dest, src }
            }
            
            Opcode::Swap => {
                if bytes.len() < pos + 2 { return Err(VmError::Execution("Unexpected end of bytecode".to_string())); }
                let r1 = Register::from_u8(bytes[pos]).map_err(|e| VmError::Execution(e.to_string()))?;
                let r2 = Register::from_u8(bytes[pos+1]).map_err(|e| VmError::Execution(e.to_string()))?;
                pos += 2;
                Instruction::Swap { r1, r2 }
            }
            
            Opcode::Add | Opcode::Sub | Opcode::Mul | Opcode::Div | Opcode::Mod |
            Opcode::And | Opcode::Or | Opcode::Xor | Opcode::Shl | Opcode::Shr => {
                if bytes.len() < pos + 3 { return Err(VmError::Execution("Unexpected end of bytecode".to_string())); }
                let dest = Register::from_u8(bytes[pos]).map_err(|e| VmError::Execution(e.to_string()))?;
                let left = Register::from_u8(bytes[pos+1]).map_err(|e| VmError::Execution(e.to_string()))?;
                let right = Register::from_u8(bytes[pos+2]).map_err(|e| VmError::Execution(e.to_string()))?;
                pos += 3;
                
                match opcode {
                    Opcode::Add => Instruction::Add { dest, left, right },
                    Opcode::Sub => Instruction::Sub { dest, left, right },
                    Opcode::Mul => Instruction::Mul { dest, left, right },
                    Opcode::Div => Instruction::Div { dest, left, right },
                    Opcode::Mod => Instruction::Mod { dest, left, right },
                    Opcode::And => Instruction::And { dest, left, right },
                    Opcode::Or  => Instruction::Or  { dest, left, right },
                    Opcode::Xor => Instruction::Xor { dest, left, right },
                    Opcode::Shl => Instruction::Shl { dest, left, right },
                    Opcode::Shr => Instruction::Shr { dest, left, right },
                    _ => unreachable!(),
                }
            }
            
            Opcode::AddAssign | Opcode::SubAssign | Opcode::MulAssign | Opcode::DivAssign => {
                if bytes.len() < pos + 2 { return Err(VmError::Execution("Unexpected end of bytecode".to_string())); }
                let dest = Register::from_u8(bytes[pos]).map_err(|e| VmError::Execution(e.to_string()))?;
                let src = Register::from_u8(bytes[pos+1]).map_err(|e| VmError::Execution(e.to_string()))?;
                pos += 2;
                match opcode {
                    Opcode::AddAssign => Instruction::AddAssign { dest, src },
                    Opcode::SubAssign => Instruction::SubAssign { dest, src },
                    Opcode::MulAssign => Instruction::MulAssign { dest, src },
                    Opcode::DivAssign => Instruction::DivAssign { dest, src },
                    _ => unreachable!(),
                }
            }
            
            Opcode::Not => {
                if bytes.len() < pos + 2 { return Err(VmError::Execution("Unexpected end of bytecode".to_string())); }
                let dest = Register::from_u8(bytes[pos]).map_err(|e| VmError::Execution(e.to_string()))?;
                let src = Register::from_u8(bytes[pos+1]).map_err(|e| VmError::Execution(e.to_string()))?;
                pos += 2;
                Instruction::Not { dest, src }
            }
            
            Opcode::Push => {
                if bytes.len() < pos + 1 { return Err(VmError::Execution("Unexpected end of bytecode".to_string())); }
                let src = Register::from_u8(bytes[pos]).map_err(|e| VmError::Execution(e.to_string()))?;
                pos += 1;
                Instruction::Push { src }
            }
            
            Opcode::Pop => {
                if bytes.len() < pos + 1 { return Err(VmError::Execution("Unexpected end of bytecode".to_string())); }
                let dest = Register::from_u8(bytes[pos]).map_err(|e| VmError::Execution(e.to_string()))?;
                pos += 1;
                Instruction::Pop { dest }
            }
            Opcode::Peek => {
                if bytes.len() < pos + 1 { return Err(VmError::Execution("Unexpected end of bytecode".to_string())); }
                let dest = Register::from_u8(bytes[pos]).map_err(|e| VmError::Execution(e.to_string()))?;
                pos += 1;
                Instruction::Peek { dest }
            }
            
            Opcode::Load => {
                if bytes.len() < pos + 2 { return Err(VmError::Execution("Unexpected end of bytecode".to_string())); }
                let dest = Register::from_u8(bytes[pos]).map_err(|e| VmError::Execution(e.to_string()))?;
                let addr_reg = Register::from_u8(bytes[pos+1]).map_err(|e| VmError::Execution(e.to_string()))?;
                pos += 2;
                Instruction::Load { dest, addr_reg }
            }
            Opcode::Store => {
                if bytes.len() < pos + 2 { return Err(VmError::Execution("Unexpected end of bytecode".to_string())); }
                let src = Register::from_u8(bytes[pos]).map_err(|e| VmError::Execution(e.to_string()))?;
                let addr_reg = Register::from_u8(bytes[pos+1]).map_err(|e| VmError::Execution(e.to_string()))?;
                pos += 2;
                Instruction::Store { src, addr_reg }
            }
            
            Opcode::LoadIndexed => {
                if bytes.len() < pos + 3 { return Err(VmError::Execution("Unexpected end of bytecode".to_string())); }
                let dest = Register::from_u8(bytes[pos]).map_err(|e| VmError::Execution(e.to_string()))?;
                let base_reg = Register::from_u8(bytes[pos+1]).map_err(|e| VmError::Execution(e.to_string()))?;
                let index_reg = Register::from_u8(bytes[pos+2]).map_err(|e| VmError::Execution(e.to_string()))?;
                pos += 3;
                Instruction::LoadIndexed { dest, base_reg, index_reg }
            }
            Opcode::StoreIndexed => {
                if bytes.len() < pos + 3 { return Err(VmError::Execution("Unexpected end of bytecode".to_string())); }
                let src = Register::from_u8(bytes[pos]).map_err(|e| VmError::Execution(e.to_string()))?;
                let base_reg = Register::from_u8(bytes[pos+1]).map_err(|e| VmError::Execution(e.to_string()))?;
                let index_reg = Register::from_u8(bytes[pos+2]).map_err(|e| VmError::Execution(e.to_string()))?;
                pos += 3;
                Instruction::StoreIndexed { src, base_reg, index_reg }
            }
            
            Opcode::Jump | Opcode::JumpIfZero | Opcode::JumpIfNotZero | 
            Opcode::JumpIfGt | Opcode::JumpIfLt | Opcode::JumpIfGe | 
            Opcode::JumpIfLe | Opcode::JumpIfEq | Opcode::JumpIfNe | 
            Opcode::JumpIfAbove | Opcode::JumpIfBelow | 
            Opcode::JumpIfAe | Opcode::JumpIfBe |
            Opcode::Call => {
                if bytes.len() < pos + 8 { return Err(VmError::Execution("Unexpected end of bytecode".to_string())); }
                let mut buf = [0u8; 8];
                buf.copy_from_slice(&bytes[pos..pos+8]);
                let target_u64 = u64::from_le_bytes(buf);
                let target = target_u64 as usize;
                pos += 8;
                match opcode {
                    Opcode::Jump => Instruction::Jump { target },
                    Opcode::JumpIfZero => Instruction::JumpIfZero { target },
                    Opcode::JumpIfNotZero => Instruction::JumpIfNotZero { target },
                    Opcode::JumpIfGt => Instruction::JumpIfGt { target },
                    Opcode::JumpIfLt => Instruction::JumpIfLt { target },
                    Opcode::JumpIfGe => Instruction::JumpIfGe { target },
                    Opcode::JumpIfLe => Instruction::JumpIfLe { target },
                    Opcode::JumpIfEq => Instruction::JumpIfEq { target },
                    Opcode::JumpIfNe => Instruction::JumpIfNe { target },
                    Opcode::JumpIfAbove => Instruction::JumpIfAbove { target },
                    Opcode::JumpIfBelow => Instruction::JumpIfBelow { target },
                    Opcode::JumpIfAe => Instruction::JumpIfAe { target },
                    Opcode::JumpIfBe => Instruction::JumpIfBe { target },
                    Opcode::Call => Instruction::Call { target },
                    _ => unreachable!(),
                }
            }
            
            Opcode::Compare => {
                if bytes.len() < pos + 2 { return Err(VmError::Execution("Unexpected end of bytecode".to_string())); }
                let left = Register::from_u8(bytes[pos]).map_err(|e| VmError::Execution(e.to_string()))?;
                let right = Register::from_u8(bytes[pos+1]).map_err(|e| VmError::Execution(e.to_string()))?;
                pos += 2;
                Instruction::Compare { left, right }
            }

            Opcode::Alloc => {
                if bytes.len() < pos + 2 { return Err(VmError::Execution("Unexpected end of bytecode".to_string())); }
                let dest = Register::from_u8(bytes[pos]).map_err(|e| VmError::Execution(e.to_string()))?;
                let size = Register::from_u8(bytes[pos+1]).map_err(|e| VmError::Execution(e.to_string()))?;
                pos += 2;
                Instruction::Alloc { dest, size }
            }
            Opcode::Free => {
                if bytes.len() < pos + 1 { return Err(VmError::Execution("Unexpected end of bytecode".to_string())); }
                let ptr = Register::from_u8(bytes[pos]).map_err(|e| VmError::Execution(e.to_string()))?;
                pos += 1;
                Instruction::Free { ptr }
            }
            Opcode::MemCopy => {
                if bytes.len() < pos + 3 { return Err(VmError::Execution("Unexpected end of bytecode".to_string())); }
                let dest = Register::from_u8(bytes[pos]).map_err(|e| VmError::Execution(e.to_string()))?;
                let src = Register::from_u8(bytes[pos+1]).map_err(|e| VmError::Execution(e.to_string()))?;
                let size = Register::from_u8(bytes[pos+2]).map_err(|e| VmError::Execution(e.to_string()))?;
                pos += 3;
                Instruction::MemCopy { dest, src, size }
            }
            Opcode::MemSet => {
                if bytes.len() < pos + 3 { return Err(VmError::Execution("Unexpected end of bytecode".to_string())); }
                let dest = Register::from_u8(bytes[pos]).map_err(|e| VmError::Execution(e.to_string()))?;
                let value = Register::from_u8(bytes[pos+1]).map_err(|e| VmError::Execution(e.to_string()))?;
                let size = Register::from_u8(bytes[pos+2]).map_err(|e| VmError::Execution(e.to_string()))?;
                pos += 3;
                Instruction::MemSet { dest, value, size }
            }
            
            _ => return Err(VmError::Execution(format!("Unsupported opcode for decoding: {:?}", opcode))),
        };
        
        Ok((instr, pos))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Register;

    #[test]
    fn test_encode_decode_simple() {
        let instructions = vec![
            Instruction::Halt,
            Instruction::Nop,
            Instruction::Return,
            Instruction::Syscall,
        ];

        for instr in instructions {
            let bytes = instr.encode();
            let (decoded, len) = Instruction::decode(&bytes).unwrap();
            assert_eq!(instr, decoded);
            assert_eq!(bytes.len(), len);
        }
    }

    #[test]
    fn test_encode_decode_imm() {
        let instr = Instruction::LoadImm { dest: Register::R0, value: 0x1234567890ABCDEF };
        let bytes = instr.encode();
        assert_eq!(bytes.len(), 1 + 1 + 8); // Op + Reg + u64
        let (decoded, len) = Instruction::decode(&bytes).unwrap();
        assert_eq!(instr, decoded);
        assert_eq!(bytes.len(), len);
    }

    #[test]
    fn test_encode_decode_regs() {
        let instr = Instruction::Add { dest: Register::R0, left: Register::R1, right: Register::R2 };
        let bytes = instr.encode();
        assert_eq!(bytes.len(), 1 + 3); // Op + 3 regs
        let (decoded, len) = Instruction::decode(&bytes).unwrap();
        assert_eq!(instr, decoded);
        assert_eq!(bytes.len(), len);
    }

    #[test]
    fn test_encode_decode_jump() {
        let instr = Instruction::Jump { target: 0xDEADBEEF };
        let bytes = instr.encode();
        assert_eq!(bytes.len(), 1 + 8); // Op + u64
        let (decoded, len) = Instruction::decode(&bytes).unwrap();
        assert_eq!(instr, decoded);
        assert_eq!(bytes.len(), len);
    }
}
