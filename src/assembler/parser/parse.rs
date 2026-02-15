//! Parser — converts source lines into AST statements.

use crate::assembler::lexer::token::{Token, Keyword, tokenize_line};
use crate::error::VmError;
use super::ast::*;

/// Parse source code into a list of statements.
pub fn parse(source: &str) -> Result<Vec<Statement>, VmError> {
    let mut statements = Vec::new();

    for (line_num, line) in source.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with(';') {
            continue;
        }

        let tokens = tokenize_line(trimmed);
        if tokens.is_empty() {
            continue;
        }

        let stmt = parse_line(&tokens, line_num + 1)
            .map_err(|e| VmError::Assembler(format!("Line {}: {}", line_num + 1, e)))?;

        if let Some(s) = stmt {
            statements.push(s);
        }
    }

    Ok(statements)
}

/// Parse a single line of tokens into a statement.
fn parse_line(tokens: &[Token], _line_num: usize) -> Result<Option<Statement>, String> {
    if tokens.is_empty() {
        return Ok(None);
    }



    // Check for label definition: identifier followed by ':'
    if let Token::Identifier(name) = &tokens[0] {
        if tokens.len() > 1 && tokens[1] == Token::Colon {
            return Ok(Some(Statement::Label(name.clone())));
        }
    }

    // halt
    if matches!(&tokens[0], Token::Keyword(Keyword::Halt)) {
        return Ok(Some(Statement::Halt));
    }

    // nop
    if matches!(&tokens[0], Token::Keyword(Keyword::Nop)) {
        return Ok(Some(Statement::Nop));
    }

    // return
    if matches!(&tokens[0], Token::Keyword(Keyword::Return)) {
        return Ok(Some(Statement::Return));
    }

    // syscall
    if matches!(&tokens[0], Token::Keyword(Keyword::Syscall)) {
        return Ok(Some(Statement::Syscall));
    }

    // print @reg
    if matches!(&tokens[0], Token::Keyword(Keyword::Print)) {
        if tokens.len() >= 2 {
            if let Token::Register(name) = &tokens[1] {
                return Ok(Some(Statement::Print(name.clone())));
            }
        }
        return Err("Expected register after 'print'".to_string());
    }

    // debug @reg
    if matches!(&tokens[0], Token::Keyword(Keyword::Debug)) {
        if tokens.len() >= 2 {
            if let Token::Register(name) = &tokens[1] {
                return Ok(Some(Statement::Debug(name.clone())));
            }
        }
        return Err("Expected register after 'debug'".to_string());
    }

    // push @reg
    if matches!(&tokens[0], Token::Keyword(Keyword::Push)) {
        if tokens.len() >= 2 {
            if let Token::Register(name) = &tokens[1] {
                return Ok(Some(Statement::Push(name.clone())));
            }
        }
        return Err("Expected register after 'push'".to_string());
    }

    // goto label
    if matches!(&tokens[0], Token::Keyword(Keyword::Goto)) {
        if tokens.len() >= 2 {
            if let Token::Identifier(name) = &tokens[1] {
                return Ok(Some(Statement::Goto(name.clone())));
            }
        }
        return Err("Expected label after 'goto'".to_string());
    }

    // call label
    if matches!(&tokens[0], Token::Keyword(Keyword::Call)) {
        if tokens.len() >= 2 {
            if let Token::Identifier(name) = &tokens[1] {
                return Ok(Some(Statement::Call(name.clone())));
            }
        }
        return Err("Expected label after 'call'".to_string());
    }

    // free @ptr
    if matches!(&tokens[0], Token::Keyword(Keyword::Free)) {
        if tokens.len() >= 2 {
            if let Token::Register(ptr) = &tokens[1] {
                return Ok(Some(Statement::Free { ptr_var: ptr.clone() }));
            }
        }
        return Err("Expected register after 'free'".to_string());
    }

    // memcpy @dest @src @size
    if matches!(&tokens[0], Token::Keyword(Keyword::MemCopy)) {
        if tokens.len() >= 4 {
            if let (Token::Register(dest), Token::Register(src), Token::Register(size)) =
                (&tokens[1], &tokens[2], &tokens[3])
            {
                return Ok(Some(Statement::MemCopy {
                    dest_var: dest.clone(),
                    src_var: src.clone(),
                    size_var: size.clone(),
                }));
            }
        }
        return Err("Expected 'memcpy @dest @src @size'".to_string());
    }

    // FP Binary: fadd @dest @left @right
    if matches!(&tokens[0], Token::Keyword(Keyword::FAdd) | Token::Keyword(Keyword::FSub) | 
                           Token::Keyword(Keyword::FMul) | Token::Keyword(Keyword::FDiv)) {
        if tokens.len() >= 4 {
            if let (Token::Register(dest), Token::Register(left), Token::Register(right)) =
                (&tokens[1], &tokens[2], &tokens[3])
            {
                let op = match &tokens[0] {
                    Token::Keyword(Keyword::FAdd) => FBinOp::Add,
                    Token::Keyword(Keyword::FSub) => FBinOp::Sub,
                    Token::Keyword(Keyword::FMul) => FBinOp::Mul,
                    Token::Keyword(Keyword::FDiv) => FBinOp::Div,
                    _ => unreachable!(),
                };
                return Ok(Some(Statement::FBinOp {
                    dest: dest.clone(),
                    left: left.clone(),
                    op,
                    right: right.clone(),
                }));
            }
        }
        return Err(format!("Expected '{:?} @dest @left @right'", tokens[0]));
    }

    // FP Unary: fsqrt @dest @src
    if matches!(&tokens[0], Token::Keyword(Keyword::FSqrt) | Token::Keyword(Keyword::FAbs) | 
                           Token::Keyword(Keyword::FNeg) | Token::Keyword(Keyword::F2I) | 
                           Token::Keyword(Keyword::I2F)) {
        if tokens.len() >= 3 {
            if let (Token::Register(dest), Token::Register(src)) = (&tokens[1], &tokens[2]) {
                let op = match &tokens[0] {
                    Token::Keyword(Keyword::FSqrt) => FUnaryOp::Sqrt,
                    Token::Keyword(Keyword::FAbs) => FUnaryOp::Abs,
                    Token::Keyword(Keyword::FNeg) => FUnaryOp::Neg,
                    Token::Keyword(Keyword::F2I) => FUnaryOp::ToInt,
                    Token::Keyword(Keyword::I2F) => FUnaryOp::ToFloat,
                    _ => unreachable!(),
                };
                return Ok(Some(Statement::FUnaryOp {
                    dest: dest.clone(),
                    op,
                    src: src.clone(),
                }));
            }
        }
        return Err(format!("Expected '{:?} @dest @src'", tokens[0]));
    }

    // FCmp: fcmp @left @right
    if matches!(&tokens[0], Token::Keyword(Keyword::FCmp)) {
        if tokens.len() >= 3 {
            if let (Token::Register(left), Token::Register(right)) = (&tokens[1], &tokens[2]) {
                return Ok(Some(Statement::FCmp {
                    left: left.clone(),
                    right: right.clone(),
                }));
            }
        }
        return Err("Expected 'fcmp @left @right'".to_string());
    }

    // Bit Unary: popcnt @dest @src
    if matches!(&tokens[0], Token::Keyword(Keyword::PopCnt) | Token::Keyword(Keyword::Clz) | 
                           Token::Keyword(Keyword::Ctz) | Token::Keyword(Keyword::BSwap)) {
        if tokens.len() >= 3 {
            if let (Token::Register(dest), Token::Register(src)) = (&tokens[1], &tokens[2]) {
                let op = match &tokens[0] {
                    Token::Keyword(Keyword::PopCnt) => BitUnaryOp::PopCnt,
                    Token::Keyword(Keyword::Clz) => BitUnaryOp::Clz,
                    Token::Keyword(Keyword::Ctz) => BitUnaryOp::Ctz,
                    Token::Keyword(Keyword::BSwap) => BitUnaryOp::BSwap,
                    _ => unreachable!(),
                };
                return Ok(Some(Statement::BitUnaryOp {
                    dest: dest.clone(),
                    op,
                    src: src.clone(),
                }));
            }
        }
        return Err(format!("Expected '{:?} @dest @src'", tokens[0]));
    }

    // Bit Rot: rotl @dest @left @right
    if matches!(&tokens[0], Token::Keyword(Keyword::RotL) | Token::Keyword(Keyword::RotR)) {
        if tokens.len() >= 4 {
            if let (Token::Register(dest), Token::Register(left), Token::Register(right)) =
                (&tokens[1], &tokens[2], &tokens[3])
            {
                let op = match &tokens[0] {
                    Token::Keyword(Keyword::RotL) => BitRotOp::RotL,
                    Token::Keyword(Keyword::RotR) => BitRotOp::RotR,
                    _ => unreachable!(),
                };
                return Ok(Some(Statement::BitRotOp {
                    dest: dest.clone(),
                    left: left.clone(),
                    op,
                    right: right.clone(),
                }));
            }
        }
        return Err(format!("Expected '{:?} @dest @left @right'", tokens[0]));
    }

    // memset @dest @value @size
    if matches!(&tokens[0], Token::Keyword(Keyword::MemSet)) {
        if tokens.len() >= 4 {
            if let (Token::Register(dest), Token::Register(value), Token::Register(size)) =
                (&tokens[1], &tokens[2], &tokens[3])
            {
                return Ok(Some(Statement::MemSet {
                    dest_var: dest.clone(),
                    value_var: value.clone(),
                    size_var: size.clone(),
                }));
            }
        }
        return Err("Expected 'memset @dest @value @size'".to_string());
    }

    // store @value at @addr
    if matches!(&tokens[0], Token::Keyword(Keyword::Store)) {
        if tokens.len() >= 4 {
            if let (Token::Register(value), Token::Keyword(Keyword::At), Token::Register(addr)) =
                (&tokens[1], &tokens[2], &tokens[3])
            {
                return Ok(Some(Statement::Store {
                    value_var: value.clone(),
                    addr_var: addr.clone(),
                }));
            }
        }
        return Err("Expected 'store @value at @addr'".to_string());
    }

    // if @a <cmp> @b goto label
    if matches!(&tokens[0], Token::Keyword(Keyword::If)) {
        return parse_if(tokens);
    }

    // @reg ... (assignment or compound)
    if let Token::Register(name) = &tokens[0] {
        return parse_register_statement(tokens, name);
    }

    Err(format!("Unexpected token: {:?}", tokens[0]))
}

/// Parse an if-conditional: if @a <cmp> @b goto label
fn parse_if(tokens: &[Token]) -> Result<Option<Statement>, String> {
    // if @a <cmp> @b goto label
    // tokens[0] = if
    // tokens[1] = @a
    // tokens[2] = comparison
    // tokens[3] = @b or number
    // tokens[4] = goto
    // tokens[5] = label

    if tokens.len() < 6 {
        return Err("Incomplete if statement".to_string());
    }

    let left = match &tokens[1] {
        Token::Register(name) => name.clone(),
        _ => return Err("Expected register after 'if'".to_string()),
    };

    let comparison = match &tokens[2] {
        Token::Equal => Comparison::Equal,
        Token::NotEqual => Comparison::NotEqual,
        Token::GreaterThan => Comparison::GreaterThan,
        Token::LessThan => Comparison::LessThan,
        Token::GreaterEqual => Comparison::GreaterEqual,
        Token::LessEqual => Comparison::LessEqual,
        _ => return Err(format!("Expected comparison operator, got {:?}", tokens[2])),
    };

    let right = match &tokens[3] {
        Token::Register(name) => Operand::Variable(name.clone()),
        Token::Number(n) => Operand::Immediate(*n),
        _ => return Err("Expected register or number after comparison".to_string()),
    };

    let mut is_unsigned = false;
    let mut goto_idx = 4;

    // Check for "unsigned" keyword
    if tokens.len() > 4 && matches!(&tokens[4], Token::Keyword(Keyword::Unsigned)) {
        is_unsigned = true;
        goto_idx = 5;
    }

    if tokens.len() < goto_idx + 2 {
        return Err("Incomplete if statement".to_string());
    }

    if !matches!(&tokens[goto_idx], Token::Keyword(Keyword::Goto)) {
        return Err("Expected 'goto' in if statement".to_string());
    }

    let label = match &tokens[goto_idx + 1] {
        Token::Identifier(name) => name.clone(),
        _ => return Err("Expected label after 'goto'".to_string()),
    };

    let final_comparison = if is_unsigned {
        match comparison {
            Comparison::GreaterThan => Comparison::UnsignedGreaterThan,
            Comparison::LessThan => Comparison::UnsignedLessThan,
            Comparison::GreaterEqual => Comparison::UnsignedGreaterEqual,
            Comparison::LessEqual => Comparison::UnsignedLessEqual,
            Comparison::Equal => Comparison::Equal, // Equal is same for signed/unsigned
            Comparison::NotEqual => Comparison::NotEqual, // NotEqual is same for signed/unsigned
            _ => return Err("Invalid comparison for unsigned".to_string()),
        }
    } else {
        comparison
    };

    Ok(Some(Statement::If {
        left,
        comparison: final_comparison,
        right,
        label,
    }))
}

/// Parse a statement starting with @register
fn parse_register_statement(tokens: &[Token], name: &str) -> Result<Option<Statement>, String> {
    if tokens.len() < 2 {
        return Err(format!("Incomplete statement for @{}", name));
    }

    // Check for indexed store: @base[@index] := @value
    // tokens: @base [ @index ] := @value
    if tokens[1] == Token::LeftBracket {
        return parse_indexed_store(tokens);
    }

    // Check for swap: @a <=> @b
    if tokens.len() >= 3 && tokens[1] == Token::SwapOp {
        if let Token::Register(other) = &tokens[2] {
            return Ok(Some(Statement::Swap {
                left: name.to_string(),
                right: other.clone(),
            }));
        }
        return Err("Expected register after '<=>'".to_string());
    }

    // Compound assignment: @reg += value/@reg
    match &tokens[1] {
        Token::AddAssign => return parse_compound_assign(tokens, name, CompoundOp::Add),
        Token::SubAssign => return parse_compound_assign(tokens, name, CompoundOp::Sub),
        Token::MulAssign => return parse_compound_assign(tokens, name, CompoundOp::Mul),
        Token::DivAssign => return parse_compound_assign(tokens, name, CompoundOp::Div),
        _ => {}
    }

    // Assignment: @reg := ...
    if tokens[1] != Token::Assign {
        return Err(format!("Expected ':=' or compound assignment after @{}", name));
    }

    if tokens.len() < 3 {
        return Err(format!("Expected value after ':=' for @{}", name));
    }

    // @reg := pop
    if matches!(&tokens[2], Token::Keyword(Keyword::Pop)) {
        return Ok(Some(Statement::Pop(name.to_string())));
    }

    // @reg := peek
    if matches!(&tokens[2], Token::Keyword(Keyword::Peek)) {
        return Ok(Some(Statement::Peek(name.to_string())));
    }

    // @reg := alloc @size
    if matches!(&tokens[2], Token::Keyword(Keyword::Alloc)) {
        if tokens.len() >= 4 {
            if let Token::Register(size) = &tokens[3] {
                return Ok(Some(Statement::Alloc {
                    dest: name.to_string(),
                    size_var: size.clone(),
                }));
            }
        }
        return Err("Expected register after 'alloc'".to_string());
    }

    // @reg := load @addr
    if matches!(&tokens[2], Token::Keyword(Keyword::Load)) {
        if tokens.len() >= 4 {
            if let Token::Register(addr) = &tokens[3] {
                return Ok(Some(Statement::Load {
                    dest_var: name.to_string(),
                    addr_var: addr.clone(),
                }));
            }
        }
        return Err("Expected register after 'load'".to_string());
    }

    // @reg := ~@src (bitwise NOT)
    if tokens[2] == Token::Tilde {
        if tokens.len() >= 4 {
            if let Token::Register(src) = &tokens[3] {
                return Ok(Some(Statement::UnaryOp {
                    dest: name.to_string(),
                    op: UnaryOp::Not,
                    operand: src.clone(),
                }));
            }
        }
        return Err("Expected register after '~'".to_string());
    }

    // @dest := @src  (simple move or binary op)
    // @dest := number (load immediate)
    // @dest := @left op @right (binary op)
    // @dest := @base[@index] (indexed load)

    match &tokens[2] {
        Token::StringLiteral(s) => {
            return Ok(Some(Statement::LoadString {
                dest: name.to_string(),
                value: s.clone(),
            }));
        }
        Token::Number(value) => {
            return Ok(Some(Statement::LoadImm {
                dest: name.to_string(),
                value: *value,
            }));
        }
        Token::Minus => {
            // Handle negative immediate: @dest := -number
            if tokens.len() >= 4 {
                if let Token::Number(val) = &tokens[3] {
                    // Convert -val to u64 (two's complement)
                    let neg_val = (-(*val as i64)) as u64;
                    return Ok(Some(Statement::LoadImm {
                        dest: name.to_string(),
                        value: neg_val,
                    }));
                }
            }
        }
        Token::Register(src_name) => {
            // Check if it's a binary operation: @dest := @src op @right
            if tokens.len() >= 5 {
                // Check for indexed load: @dest := @base[@index]
                if tokens[3] == Token::LeftBracket {
                    if let Token::Register(index) = &tokens[4] {
                        if tokens.len() >= 6 && tokens[5] == Token::RightBracket {
                            return Ok(Some(Statement::LoadIndexed {
                                dest: name.to_string(),
                                base_var: src_name.clone(),
                                index_var: index.clone(),
                            }));
                        }
                    }
                    return Err("Expected @index] in indexed load".to_string());
                }

                let op = match &tokens[3] {
                    Token::Plus => BinOp::Add,
                    Token::Minus => BinOp::Sub,
                    Token::Star => BinOp::Mul,
                    Token::Slash => BinOp::Div,
                    Token::Percent => BinOp::Mod,
                    Token::Ampersand => BinOp::And,
                    Token::Pipe => BinOp::Or,
                    Token::Caret => BinOp::Xor,
                    Token::ShiftLeft => BinOp::Shl,
                    Token::ShiftRight => BinOp::Shr,
                    _ => {
                        // Just a move: @dest := @src
                        return Ok(Some(Statement::MoveVar {
                            dest: name.to_string(),
                            src: src_name.clone(),
                        }));
                    }
                };

                // Handle right operand which could be negative number
                let right = if tokens[4] == Token::Minus && tokens.len() >= 6 {
                   if let Token::Number(n) = &tokens[5] {
                        let neg_val = (-(*n as i64)) as u64;
                        Operand::Immediate(neg_val)
                   } else {
                       return Err("Expected number after '-' in right operand".to_string());
                   }
                } else {
                    match &tokens[4] {
                        Token::Register(r) => Operand::Variable(r.clone()),
                        Token::Number(n) => Operand::Immediate(*n),
                        _ => return Err("Expected register or number as right operand".to_string()),
                    }
                };

                return Ok(Some(Statement::BinOp {
                    dest: name.to_string(),
                    left: src_name.clone(),
                    op,
                    right,
                }));
            }

            // Simple move: @dest := @src
            return Ok(Some(Statement::MoveVar {
                dest: name.to_string(),
                src: src_name.clone(),
            }));
        }
        _ => {}
    }

    Err(format!("Unexpected token after ':=' : {:?}", tokens[2]))
}

/// Parse compound assignment: @reg += operand
fn parse_compound_assign(tokens: &[Token], name: &str, op: CompoundOp) -> Result<Option<Statement>, String> {
    if tokens.len() < 3 {
        return Err(format!("Expected value after compound assignment for @{}", name));
    }

    let operand = match &tokens[2] {
        Token::Register(r) => Operand::Variable(r.clone()),
        Token::Number(n) => Operand::Immediate(*n),
        _ => return Err("Expected register or number for compound assignment".to_string()),
    };

    Ok(Some(Statement::CompoundAssign {
        dest: name.to_string(),
        op,
        operand,
    }))
}

/// Parse indexed store: @base[@index] := @value
/// This is called from the main parse_line when we detect @base [ ...
fn parse_indexed_store(tokens: &[Token]) -> Result<Option<Statement>, String> {
    // @base [ @index ] := @value
    // tokens[0] = @base
    // tokens[1] = [
    // tokens[2] = @index
    // tokens[3] = ]
    // tokens[4] = :=
    // tokens[5] = @value

    if tokens.len() < 6 {
        return Err("Incomplete indexed store statement".to_string());
    }

    let base = match &tokens[0] {
        Token::Register(name) => name.clone(),
        _ => return Err("Expected register for indexed store base".to_string()),
    };

    let index = match &tokens[2] {
        Token::Register(name) => name.clone(),
        _ => return Err("Expected register for index".to_string()),
    };

    if tokens[3] != Token::RightBracket {
        return Err("Expected ']'".to_string());
    }

    if tokens[4] != Token::Assign {
        return Err("Expected ':=' in indexed store".to_string());
    }

    let value = match &tokens[5] {
        Token::Register(name) => Operand::Variable(name.clone()),
        Token::Number(n) => Operand::Immediate(*n),
        _ => return Err("Expected register or number for indexed store value".to_string()),
    };

    Ok(Some(Statement::StoreIndexed {
        base_var: base,
        index_var: index,
        value,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hello() {
        let stmts = parse("@r0 := 42\nprint @r0\nhalt\n").unwrap();
        assert_eq!(stmts.len(), 3);
        assert!(matches!(&stmts[0], Statement::LoadImm { dest, value: 42 } if dest == "r0"));
        assert!(matches!(&stmts[1], Statement::Print(name) if name == "r0"));
        assert!(matches!(&stmts[2], Statement::Halt));
    }

    #[test]
    fn test_parse_arithmetic() {
        let stmts = parse("@r2 := @r0 + @r1\n").unwrap();
        assert_eq!(stmts.len(), 1);
        if let Statement::BinOp { dest, left, op, right } = &stmts[0] {
            assert_eq!(dest, "r2");
            assert_eq!(left, "r0");
            assert_eq!(*op, BinOp::Add);
            assert_eq!(*right, Operand::Variable("r1".to_string()));
        } else {
            panic!("Expected BinOp");
        }
    }

    #[test]
    fn test_parse_label() {
        let stmts = parse("loop_start:\n").unwrap();
        assert_eq!(stmts.len(), 1);
        assert!(matches!(&stmts[0], Statement::Label(name) if name == "loop_start"));
    }

    #[test]
    fn test_parse_if() {
        let stmts = parse("if @counter < @limit goto loop_start\n").unwrap();
        assert_eq!(stmts.len(), 1);
        if let Statement::If { left, comparison, right, label } = &stmts[0] {
            assert_eq!(left, "counter");
            assert_eq!(*comparison, Comparison::LessThan);
            assert_eq!(*right, Operand::Variable("limit".to_string()));
            assert_eq!(label, "loop_start");
        } else {
            panic!("Expected If");
        }
    }
}
