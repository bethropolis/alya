//! AST node types for the Alya assembler.

/// A single statement in an Alya program.
#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    /// Load an immediate value: @dest := value
    LoadImm { dest: String, value: u64 },

    /// Move register to register: @dest := @src
    MoveVar { dest: String, src: String },

    /// Swap two variables: @left <=> @right
    Swap { left: String, right: String },

    /// Binary operation: @dest := @left op @right
    BinOp { dest: String, left: String, op: BinOp, right: Operand },

    /// Unary operation: @dest := ~@operand
    UnaryOp { dest: String, op: UnaryOp, operand: String },

    /// Compound assignment: @dest op= operand
    CompoundAssign { dest: String, op: CompoundOp, operand: Operand },

    /// Push: push @src
    Push(String),

    /// Pop: @dest := pop
    Pop(String),

    /// Peek: @dest := peek
    Peek(String),

    /// Print: print @src
    Print(String),

    /// Debug: debug @src
    Debug(String),

    /// Halt
    Halt,

    /// Nop
    Nop,

    /// Label definition: name:
    Label(String),

    /// Unconditional jump: goto label
    Goto(String),

    /// Conditional jump: if @left cmp @right goto label
    If { left: String, comparison: Comparison, right: Operand, label: String },

    /// Function call: call label
    Call(String),

    /// Return
    Return,

    /// Store value at address: store @value at @addr
    Store { value_var: String, addr_var: String },

    /// Load from address: @dest := load @addr
    Load { dest_var: String, addr_var: String },

    /// Indexed store: @base[@index] := @value
    StoreIndexed { base_var: String, index_var: String, value: Operand },

    /// Indexed load: @dest := @base[@index]
    LoadIndexed { dest: String, base_var: String, index_var: String },
}

/// Binary operators
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    And,
    Or,
    Xor,
    Shl,
    Shr,
}

/// Unary operators
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    Not,
}

/// Compound assignment operators
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompoundOp {
    Add,
    Sub,
    Mul,
    Div,
}

/// Comparison operators for conditional jumps
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterEqual,
    LessEqual,
}

/// An operand that can be either a variable name or immediate value
#[derive(Debug, Clone, PartialEq)]
pub enum Operand {
    Variable(String),
    Immediate(u64),
}
