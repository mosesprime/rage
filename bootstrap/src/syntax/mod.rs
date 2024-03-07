//! Rage Bootstrap
//! Syntax
//! Reference: https://github.com/dtolnay/syn

use anyhow::{bail, Context, Ok};

use crate::{common::{Attribute, Attributes, Mutability, Visability}, parser::{span::Span, Parse, Parser}};

use self::{keywords::KeywordKind, lexeme::LexemeKind, symbol::SymbolKind};

pub mod keywords;
pub mod lexeme;
pub mod symbol;

#[derive(Debug)]
pub enum Statement {
    Declaration(Declaration),
    Expression(Expression),
    Return(ReturnStmt),
}

impl Parse for Statement {
    fn parse(parser: &mut Parser<'_>) -> Result<Self, anyhow::Error> {
        let first = parser.peek_lexeme().context("missing lexeme for statement")?;
        match first.kind {
            LexemeKind::Keyword(keywords::KeywordKind::Return) => {
                Ok(Statement::Return(ReturnStmt::parse(parser)?))
            }, 
            LexemeKind::Term 
                | LexemeKind::Keyword(KeywordKind::Mut)
                | LexemeKind::Keyword(KeywordKind::Pub) => {
                Ok(Statement::Declaration(Declaration::parse(parser)?))
            },
            _ => bail!("Invalid start to a new statement"),
        }
    }
}

#[derive(Debug)]
pub enum Expression {
    Binary(),
    Unary(),
    Literal(LiteralExpr),
}

impl Parse for Expression {
    fn parse(parser: &mut Parser<'_>) -> Result<Self, anyhow::Error> {
        if let Some(peek) = parser.peek_lexeme() {
            match peek.kind {
                LexemeKind::Whitespace(_)
                | LexemeKind::Comment(_) => {
                    parser.consume_lexeme();
                    return Expression::parse(parser);
                },
                LexemeKind::Term => todo!(),
                LexemeKind::Literal(_) => Ok(Expression::Literal(LiteralExpr::parse(parser)?)),
                _ => todo!(),
            }
        } else {
            bail!("missing lexeme for expression")
        }
    }
}

#[derive(Debug)]
pub enum Declaration {
    Loacl(LocalDecl),
    Func(FuncDecl),
    Type(TypeDecl),
}

impl Parse for Declaration {
    fn parse(parser: &mut Parser<'_>) -> Result<Self, anyhow::Error> {
        let start = parser.get_cursor();
        let first = parser.next_lexeme().context("missing lexeme for declaration")?;
        let attrs = Attributes::parse(parser)?;
        let mutable = Mutability::parse(parser)?;
        let visable = Visability::parse(parser)?;
        match first.kind {
            // 'MyType' struct {}
            LexemeKind::Term => {
                parser.push_buffer(first, start);
                let second = parser.peek_lexeme().context("missing lexeme")?;
                match &second.kind {
                    // MyType 'struct' {}
                    LexemeKind::Term => todo!(),  
                    LexemeKind::Symbol(s) => match s {
                        // my_func'(') {}
                        SymbolKind::LParen => Ok(Declaration::Func(FuncDecl::parse(parser)?)),
                        // my_func'<'>() {}
                        SymbolKind::Lesser => todo!(),
                        // num '=' 5
                        SymbolKind::Equal => todo!(),
                        _ => bail!("Invalid declaration."),
                    },
                    _ => bail!("Invalid declaration."),
                }
            },
            _ => bail!("Invalid declaration.")
        }
    }
}

#[derive(Debug)]
pub struct ConstDecl {
    pub vis: Visability,
    pub label: Box<str>,
    pub ty: Box<str>,
    pub inner: Expression,
}

impl Parse for ConstDecl {
    fn parse(parser: &mut Parser<'_>) -> Result<Self, anyhow::Error> {
        todo!()       
    }
}


#[derive(Debug)]
pub struct TypeDecl {
    // TODO:
}

#[derive(Debug)]
pub struct LocalDecl {
    // TODO:
}

#[derive(Debug)]
pub struct ReturnStmt {
    pub inner: Expression,
    pub span: Span,
}

impl Parse for ReturnStmt {
    fn parse(parser: &mut Parser<'_>) -> Result<Self, anyhow::Error> {
        let start = parser.get_cursor();
        let _ = parser.next_lexeme().context("missing lexeme for return statement")?;
        let expr = Expression::parse(parser)?;
        let end = parser.get_cursor();
        Ok(Self { inner: expr, span: Span::new(start, end) })
    }
}


#[derive(Debug)]
pub struct FuncDecl {
    label: Box<str>,
    // TODO:
}

impl Parse for FuncDecl {
    fn parse(parser: &mut Parser<'_>) -> Result<Self, anyhow::Error> {
        let buffered = parser.get_buffer();
        parser.clear_buffer();
        todo!()
    }
}


#[derive(Debug)]
pub struct LiteralExpr {
    pub kind: LiteralKind,
    pub value: Box<str>,
    pub span: Span,
}

impl Parse for LiteralExpr {
    fn parse(parser: &mut Parser<'_>) -> Result<Self, anyhow::Error> {
        let start = parser.get_cursor();
        let lexeme = parser.next_lexeme().context("missing lexeme for literal expression")?;
        let kind = match lexeme.kind {
            LexemeKind::Literal(lit) => lit,
            _ => bail!("Can not parse non-literal into literal expression."),
        };
        Ok(Self { 
            kind,
            value: lexeme.value().context("missing literal value")?.into(),
            span: Span::new(start, parser.get_cursor()),
        })
    }
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

#[derive(Debug, Clone, Copy, PartialEq)]
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


#[derive(Debug, Clone, Copy, PartialEq)]
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

#[derive(Debug, Clone, Copy, PartialEq)]
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
