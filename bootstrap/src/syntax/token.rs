//! Rage Bootstrap
//! Syntax Token

#[derive(Debug, Clone)]
pub struct Token {
    kind: TokenKind,
    index: u32,
    length: u32,
}

impl Token {
    pub fn new(kind: TokenKind, index: u32, length: u32) -> Self {
        Self { kind, index, length }
    }
}

#[derive(Debug, Clone)]
pub enum TokenKind {
    Identifier,
    Operator(Operator),
    Delimiter(Delimiter),
    Literal(Literal),
    Macro(Macro),
    Comment(Comment),
    Tuple(Tuple),
    Path(Path),
    /// *a
    Pointer,
    /// &a
    Reference,
    /// a.b 
    Member,

    UNKNOWN,
}

#[derive(Debug, Clone)]
pub enum Operator{
    /* Arithmetic */ 

    /// a + b
    Addition,
    /// a - b
    Subtraction,
    /// a * b
    Multiplication,
    /// a / b
    Division,
    /// a % b
    Modulo,

    /* Relational */

    /// a == b
    Equal,
    /// a != b
    NotEqual,
    /// a > b
    Greater,
    /// a < b
    Lesser,
    /// a >= b
    GreaterOrEqual,
    /// a <= b
    LesserOrEqual,

    /* Logical */ 
    
    /// !a
    LogicNOT,
    /// a && b
    LogicAND,
    /// a || b
    LogicOR,

    /* Bitwise */ 
    
    /// ~a
    BitwiseNOT,
    /// a & b
    BitwiseAND,
    /// a | b
    BitwiseOR,
    /// a ^ b
    BitwiseXOR,
    /// a << b
    BitwiseLeftShift,
    /// a >> b
    BitwiseRightShift,
    /// a <<< b
    BitwiseLeftRotate,
    /// a >>> b
    BitwiseRightRotate,

    /* Assignment */

    /// a = b
    Assign,
    /// a += b
    AddAssign,
    /// a -= b
    SubtractAssign,
    /// a *= b
    MultiplyAssign,
    /// a /= b
    DivideAssign,
    /// a %= b
    ModuloAssign,
    /// a &= b
    BitwiseANDAssign,
    /// a |= b
    BitwiseORAssign,
    /// a ^= b
    BitwiseXORAssign,
    /// a <<= b
    BitwiseLeftShiftAssign,
    /// a >>= b
    BitwiseRightShiftAssign,
    //BitwiseLeftRotateAssign,
    //BitwiseRightRotateAssign,
}

#[derive(Debug, Clone)]
pub enum Delimiter {
    OpenParen,
    CloseParen,
    OpenCurly,
    CloseCurly,
    OpenSquare,
    CloseSquare,
    OpenImplied,
    CloseImplied,
    Terminate,
    Seperator,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    Bool,
    Char,
    Hex,
    Binary,
    Integer,
    Float,
    String,
}

#[derive(Debug, Clone)]
pub enum Macro {
    Attribute,
    Directive,
    Substitute,
    Capture,
    Repeat,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Comment {
    Line,
    Block,
    Documentation,
}

#[derive(Debug, Clone)]
pub enum Tuple {
    Open,
    Close,
}

#[derive(Debug, Clone)]
pub enum Path {
    Namespace,
    Relative,
}
