//! Token types for the lexer.

/// Represents a single token from the source.
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    /// @name — a register or named variable
    Register(String),
    /// A numeric literal (decimal, hex, binary)
    Number(u64),
    /// A label reference (identifier without @)
    Identifier(String),
    /// :=
    Assign,
    /// +=
    AddAssign,
    /// -=
    SubAssign,
    /// *=
    MulAssign,
    /// /=
    DivAssign,
    /// <=>
    SwapOp,
    /// +
    Plus,
    /// -
    Minus,
    /// *
    Star,
    /// /
    Slash,
    /// %
    Percent,
    /// &
    Ampersand,
    /// |
    Pipe,
    /// ^
    Caret,
    /// ~
    Tilde,
    /// <<
    ShiftLeft,
    /// >>
    ShiftRight,
    /// ==
    Equal,
    /// !=
    NotEqual,
    /// >
    GreaterThan,
    /// <
    LessThan,
    /// >=
    GreaterEqual,
    /// <=
    LessEqual,
    /// [
    LeftBracket,
    /// ]
    RightBracket,
    /// :
    Colon,
    /// Keywords
    Keyword(Keyword),
    /// End of line
    Eol,
}

/// Language keywords
#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    Print,
    Halt,
    Push,
    Pop,
    Peek,
    Goto,
    If,
    Call,
    Return,
    Load,
    Store,
    At,
    Debug,
    Nop,
}

/// Tokenize a single line of source code.
pub fn tokenize_line(line: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = line.chars().collect();
    let len = chars.len();
    let mut i = 0;

    while i < len {
        // Skip whitespace
        if chars[i].is_whitespace() {
            i += 1;
            continue;
        }

        // Skip comments (;)
        if chars[i] == ';' {
            break;
        }

        // Register or named variable: @name
        if chars[i] == '@' {
            i += 1;
            let start = i;
            while i < len && (chars[i].is_alphanumeric() || chars[i] == '_') {
                i += 1;
            }
            let name = chars[start..i].iter().collect::<String>();
            tokens.push(Token::Register(name));
            continue;
        }

        // Number: decimal, 0x hex, 0b binary
        if chars[i].is_ascii_digit() {
            let start = i;
            if chars[i] == '0' && i + 1 < len {
                if chars[i + 1] == 'x' || chars[i + 1] == 'X' {
                    i += 2;
                    let hex_start = i;
                    while i < len && chars[i].is_ascii_hexdigit() {
                        i += 1;
                    }
                    let hex_str: String = chars[hex_start..i].iter().collect();
                    let value = u64::from_str_radix(&hex_str, 16).unwrap_or(0);
                    tokens.push(Token::Number(value));
                    continue;
                } else if chars[i + 1] == 'b' || chars[i + 1] == 'B' {
                    i += 2;
                    let bin_start = i;
                    while i < len && (chars[i] == '0' || chars[i] == '1') {
                        i += 1;
                    }
                    let bin_str: String = chars[bin_start..i].iter().collect();
                    let value = u64::from_str_radix(&bin_str, 2).unwrap_or(0);
                    tokens.push(Token::Number(value));
                    continue;
                }
            }
            while i < len && chars[i].is_ascii_digit() {
                i += 1;
            }
            let num_str: String = chars[start..i].iter().collect();
            let value = num_str.parse::<u64>().unwrap_or(0);
            tokens.push(Token::Number(value));
            continue;
        }

        // Multi-char operators
        if i + 2 < len {
            let three: String = chars[i..i + 3].iter().collect();
            if three == "<=>" {
                tokens.push(Token::SwapOp);
                i += 3;
                continue;
            }
        }

        if i + 1 < len {
            let two: String = chars[i..i + 2].iter().collect();
            match two.as_str() {
                ":=" => { tokens.push(Token::Assign); i += 2; continue; }
                "+=" => { tokens.push(Token::AddAssign); i += 2; continue; }
                "-=" => { tokens.push(Token::SubAssign); i += 2; continue; }
                "*=" => { tokens.push(Token::MulAssign); i += 2; continue; }
                "/=" => { tokens.push(Token::DivAssign); i += 2; continue; }
                "<<" => { tokens.push(Token::ShiftLeft); i += 2; continue; }
                ">>" => { tokens.push(Token::ShiftRight); i += 2; continue; }
                "==" => { tokens.push(Token::Equal); i += 2; continue; }
                "!=" => { tokens.push(Token::NotEqual); i += 2; continue; }
                ">=" => { tokens.push(Token::GreaterEqual); i += 2; continue; }
                "<=" => { tokens.push(Token::LessEqual); i += 2; continue; }
                _ => {}
            }
        }

        // Single-char operators
        match chars[i] {
            '+' => { tokens.push(Token::Plus); i += 1; continue; }
            '-' => { tokens.push(Token::Minus); i += 1; continue; }
            '*' => { tokens.push(Token::Star); i += 1; continue; }
            '/' => { tokens.push(Token::Slash); i += 1; continue; }
            '%' => { tokens.push(Token::Percent); i += 1; continue; }
            '&' => { tokens.push(Token::Ampersand); i += 1; continue; }
            '|' => { tokens.push(Token::Pipe); i += 1; continue; }
            '^' => { tokens.push(Token::Caret); i += 1; continue; }
            '~' => { tokens.push(Token::Tilde); i += 1; continue; }
            '>' => { tokens.push(Token::GreaterThan); i += 1; continue; }
            '<' => { tokens.push(Token::LessThan); i += 1; continue; }
            '[' => { tokens.push(Token::LeftBracket); i += 1; continue; }
            ']' => { tokens.push(Token::RightBracket); i += 1; continue; }
            ':' => { tokens.push(Token::Colon); i += 1; continue; }
            _ => {}
        }

        // Identifiers and keywords
        if chars[i].is_alphabetic() || chars[i] == '_' {
            let start = i;
            while i < len && (chars[i].is_alphanumeric() || chars[i] == '_') {
                i += 1;
            }
            let word: String = chars[start..i].iter().collect();
            let token = match word.as_str() {
                "print" => Token::Keyword(Keyword::Print),
                "halt" => Token::Keyword(Keyword::Halt),
                "push" => Token::Keyword(Keyword::Push),
                "pop" => Token::Keyword(Keyword::Pop),
                "peek" => Token::Keyword(Keyword::Peek),
                "goto" => Token::Keyword(Keyword::Goto),
                "if" => Token::Keyword(Keyword::If),
                "call" => Token::Keyword(Keyword::Call),
                "return" => Token::Keyword(Keyword::Return),
                "load" => Token::Keyword(Keyword::Load),
                "store" => Token::Keyword(Keyword::Store),
                "at" => Token::Keyword(Keyword::At),
                "debug" => Token::Keyword(Keyword::Debug),
                "nop" => Token::Keyword(Keyword::Nop),
                _ => Token::Identifier(word),
            };
            tokens.push(token);
            continue;
        }

        // Skip unrecognized characters
        i += 1;
    }

    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_load_imm() {
        let tokens = tokenize_line("@r0 := 42");
        assert_eq!(tokens, vec![
            Token::Register("r0".to_string()),
            Token::Assign,
            Token::Number(42),
        ]);
    }

    #[test]
    fn test_tokenize_arithmetic() {
        let tokens = tokenize_line("@r2 := @r0 + @r1");
        assert_eq!(tokens, vec![
            Token::Register("r2".to_string()),
            Token::Assign,
            Token::Register("r0".to_string()),
            Token::Plus,
            Token::Register("r1".to_string()),
        ]);
    }

    #[test]
    fn test_tokenize_comment() {
        let tokens = tokenize_line("@r0 := 42  ; this is a comment");
        assert_eq!(tokens, vec![
            Token::Register("r0".to_string()),
            Token::Assign,
            Token::Number(42),
        ]);
    }

    #[test]
    fn test_tokenize_binary_literal() {
        let tokens = tokenize_line("@r0 := 0b1100");
        assert_eq!(tokens, vec![
            Token::Register("r0".to_string()),
            Token::Assign,
            Token::Number(12),
        ]);
    }

    #[test]
    fn test_tokenize_swap() {
        let tokens = tokenize_line("@r2 <=> @r3");
        assert_eq!(tokens, vec![
            Token::Register("r2".to_string()),
            Token::SwapOp,
            Token::Register("r3".to_string()),
        ]);
    }
}
