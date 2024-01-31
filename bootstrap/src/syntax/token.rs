//! Rage Bootstrap
//! Syntax Token

use crate::{common::span::Span, parser::lexeme::{Lexeme, LexemeKind}};

use super::Identifier;

pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }
}

pub enum TokenKind {
    Identifier(Identifier),
    Terminator(TerminatorKind),
    Operator(OperatorKind),
    Delimiter(DelimiterKind),
    UnpairedDelimiter(UnpairedDelimiter),
    Seperator(SeperatorKind),
    Literal(LiteralRepr),
    Verbatim(LexemeKind),
    UNKNOWN,
}

#[derive(Debug)]
pub struct Terminator {
    pub kind: TerminatorKind,
    pub span: Span,
}

///
#[derive(Debug)]
pub enum TerminatorKind {
    /// ;
    Explicit,
    /// implied terminator
    Implied,
}

#[derive(Debug)]
pub struct Operator {
    pub kind: OperatorKind,
    pub span: Span,
}

/// Operator.
#[derive(Debug)]
pub enum OperatorKind {
    Arithmetic(ArithmeticOpKind),
    Relational(RelationalOpKind),
    Logical(LogicalOpKind),
    Bitwise(BitwiseOpKind),
    Assignment(AssignmentOpKind),
    /// $a
    Borrow,
    /// *a
    Pointer,
    /// &a
    Reference,
}
    
/// Arithmetic operator.
#[derive(Debug)]
pub enum ArithmeticOpKind {
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
}

/// Relational operator.
#[derive(Debug)]
pub enum RelationalOpKind {
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
}

/// Logical operator.
#[derive(Debug)]
pub enum LogicalOpKind {
    /// !a 
    LogicNOT,
    /// a && b
    LogicAND,
    /// a || b
    LogicOR,
}

/// Bitwise operator.
#[derive(Debug)]
pub enum BitwiseOpKind {
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
}

/// Assignment operator.
#[derive(Debug)]
pub enum AssignmentOpKind {
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
}

#[derive(Debug)]
pub struct Delimiter {
    pub kind: DelimiterKind,
    pub span: Span,
}

pub struct UnpairedDelimiter {
    pub kind: DelimiterKind,
    pub side: DelimiterSide,
}

pub enum DelimiterSide {
    Open, 
    Close,
}

#[derive(Debug)]
pub enum DelimiterKind {
    /// ( ... )
    Paren,
    /// { ... }
    Curly,
    /// [ ... ]
    Square,
    /// < ... >
    Angle,
    /// 
    Infered,
}

#[derive(Debug)]
pub struct Seperator {
    pub kind: SeperatorKind,
    pub span: Span
}

#[derive(Debug)]
pub enum SeperatorKind {
    /// a `,` b
    List,
    /// A`:`B 
    Path,
    /// A `|` B
    Variety,
}

#[derive(Debug)]
pub struct Literal {
    pub repr: LiteralRepr,
    pub span: Span,
}

#[derive(Debug)]
pub enum LiteralRepr {
    Bool(bool),
    Char(char),
    Hex(Box<str>),
    Octal(Box<str>),
    Binary(Box<str>),
    Integer(Box<str>),
    Float(Box<str>),
    String(Box<str>),
}

#[derive(Debug)]
pub enum LiteralKind {
    Bool,
    Char,
    Hex,
    Octal,
    Binary,
    Integer,
    Float,
    String,
}

#[derive(Debug)]
pub struct Comment {
    pub kind: CommentKind,
    pub raw: Box<str>,
    pub span: Span,
}

#[derive(Debug)]
pub enum CommentKind {
    /// ```
    /// // Hi Mom!
    /// ```
    Line,
    /// ```rage
    /// //*
    /// * Hi Mom!
    /// */
    /// ```
    Block,
    /// ```
    /// /// Hi Mom!
    /// ```
    Documentation,
}


