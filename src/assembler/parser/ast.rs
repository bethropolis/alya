//! AST node types for the Alya assembler.

/// A single statement in an Alya program.
#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    /// Load an immediate value: @dest := value
    LoadImm { dest: String, value: u64 },
    
    /// Load address of a string literal: @dest := "string"
    LoadString { dest: String, value: String },

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

    /// System call (ID in R0, Args in R1...)
    Syscall,

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

    /// Heap allocation: @dest := alloc @size
    Alloc { dest: String, size_var: String },

    /// Heap free: free @ptr
    Free { ptr_var: String },

    /// Memory copy: memcpy @dest, @src, @size
    MemCopy { dest_var: String, src_var: String, size_var: String },

    /// Memory set: memset @dest, @value, @size
    MemSet { dest_var: String, value_var: String, size_var: String },

    /// Floating point binary op: @dest := @left fop @right
    FBinOp { dest: String, left: String, op: FBinOp, right: String },

    /// Floating point unary op: @dest := fop @src
    FUnaryOp { dest: String, op: FUnaryOp, src: String },

    /// Floating point comparison: fcmp @left, @right
    FCmp { left: String, right: String },

    /// Bitwise extension unary op: @dest := bop @src
    BitUnaryOp { dest: String, op: BitUnaryOp, src: String },

    /// Bitwise rotation: @dest := @left rot @right
    BitRotOp { dest: String, left: String, op: BitRotOp, right: String },
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
    UnsignedGreaterThan,
    UnsignedLessThan,
    UnsignedGreaterEqual,
    UnsignedLessEqual,
}

/// An operand that can be either a variable name or immediate value
#[derive(Debug, Clone, PartialEq)]
pub enum Operand {
    Variable(String),
    Immediate(u64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FBinOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FUnaryOp {
    Sqrt,
    Abs,
    Neg,
    ToFloat, // i2f
    ToInt,   // f2i
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BitUnaryOp {
    PopCnt,
    Clz,
    Ctz,
    BSwap,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BitRotOp {
    RotL,
    RotR,
}
