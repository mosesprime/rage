//! Rage Bootstrap
//! Syntax Token

use crate::parser::lexeme::LexemeKind;

use super::Span;

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }
}

#[derive(Debug)]
pub enum TokenKind {
    Identifier,
    Operator(OperatorKind),
    Delimiter(DelimiterKind),
    Seperator(SeperatorKind),
    Literal(LiteralKind),
    Comment(CommentKind),
    Terminator(TerminatorKind),
    /// #
    Meta,
    Path,
    Pointer,
    Reference,
    Borrow,
    Member,

    VERBATIM(LexemeKind),
    UNKNOWN,
}

///
#[derive(Debug)]
pub enum TerminatorKind {
    /// ;
    Explicit,
    ///
    Implied,
}

/// Operator.
#[derive(Debug)]
pub enum OperatorKind {
    Arithmetic(ArithmeticOp),
    Relational(RelationalOp),
    Logical(LogicalOp),
    Bitwise(BitwiseOp),
    Assignment(AssignmentOp),
}
    
/// Arithmetic operator.
#[derive(Debug)]
pub enum ArithmeticOp {
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
pub enum RelationalOp {
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
pub enum LogicalOp {
    /// !a 
    LogicNOT,
    /// a && b
    LogicAND,
    /// a || b
    LogicOR,
}

/// Bitwise operator.
#[derive(Debug)]
pub enum BitwiseOp {
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
pub enum AssignmentOp {
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
pub enum SeperatorKind {
    /// a `,` b
    List,
    /// A `|` B
    Variety,
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

