//! Rage Bootstrap
//! Syntax
//! Reference: https://github.com/dtolnay/syn

use anyhow::{bail, Context, Ok};

use crate::parser::Parse;

use self::lexeme::LexemeKind;

pub mod keywords;
pub mod lexeme;

#[derive(Debug)]
pub enum Statement {
    Declaration(Declaration),
    Expression(),
    //Return(),
}

impl Parse for Statement {
    fn parse(parser: &mut crate::parser::Parser<'_>) -> Result<Self, anyhow::Error> {
        let first = parser.next_lexeme().context("missing lexeme")?;
        let err = match first.kind {
            LexemeKind::Term => {
                let second = parser.next_lexeme().context(format!("Standalone term is unparsable: {:?}", parser.span()));
                return Ok(Statement::Declaration(Declaration::Func(FuncDecl { label: first.value().expect("todo").into() }))); // TODO:
            },
            _ => {
                bail!("Invalid start to a new statement: {:?}", parser.span());}
            ,
        };
    }
}

#[derive(Debug)]
pub enum Declaration {
    Loacl(),
    Func(FuncDecl),
    Struct(),
}

#[derive(Debug)]
pub struct FuncDecl {
    label: Box<str>,
    // TODO:
}

pub enum Expression {
    Binary(),
    Unary(),
    Return(),
}

#[derive(Debug)]
pub struct LiteralExpr {
    pub kind: LiteralKind,
    pub value: Box<str>,
    // pub span: Span,
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

#[derive(Debug, Clone, Copy)]
pub enum WhitespaceKind {
    /// space or tab
    Blank,
    /// new line
    NewLine,
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
    /// A`:`B 
    Path,
    /// A `|` B
    Variety,
}


#[derive(Debug, Clone, Copy)]
pub enum LiteralKind {
    /// `0x55AA`
    Hex,
    /// `true` or `false`
    Bool,
    /// `'a'`
    Char,
    // TODO: finish octal literal impl
    Octal,
    /// `0b01010011
    Binary,
    /// `420`
    Integer,
    /// `3.14`
    Float,
    /// `"the quick brown fox jumps over the lazy dog"`
    String,
}

#[derive(Debug, Clone, Copy)]
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
