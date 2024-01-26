//! Rage Bootstrap
//! Syntax

use std::{fmt::{Debug, Display}, iter::Peekable};

use crate::{common::Attribute, parser::{lexeme::LexemeKind, scanner::Scanner, Parse, ParseBuffer, ParseError}};

use self::token::Token;

pub mod token;

/// Represents the index of starting and ending chars.
/// # Safety
/// Uses [u32] for [char] indecies so errors will occur if 
/// file contains more the u32::MAX chars (about 4 billion).
#[derive(Debug)]
pub struct Span {
    pub start: u32,
    pub end: u32,
}

impl Span {
    pub fn new(start: u32, end: u32) -> Self {
        Self { start, end }
    }

    pub fn as_range(&self) -> std::ops::Range<usize> {
        (self.start as usize)..(self.end as usize)
    }
}

#[derive(Debug)]
pub enum Statement {
    Declaration(Declaration),
    Expression(Expression),
}

impl Parse for Statement {
    fn parse(buffer: &ParseBuffer) -> Result<Option<Self>, ParseError> {
        todo!()
    }
}

#[derive(Debug)]
pub enum Declaration {
    Function(Box<FuncDecl>),
    //Foreign(Box<ForeignDecl>),
    //Macro(Box<MacroDecl>),
    Module(Box<ModuleDecl>),
    Type(Box<TypeDecl>),
    //Verbatim(Vec<Token>)
}

#[derive(Debug)]
pub enum Expression {
    //Array(Box<ArrayExpr>),
    Assign(Box<AssignExpr>),
    //Async(Box<AsyncExpr>),
    Attribute(Box<AttrExpr>),
    //Await(Box<AwaitExpr>),
    //Binary(Box<BinaryExpr>),
    Block(Box<BlockExpr>),
    Call(Box<CallExpr>),
    //Field(Box<FieldExpr>),
    //Index(Box<IndexExpr>),
    Literal(Box<LiteralExpr>),
    //Macro(Box<MacroExpr>),
    //Member(Box<MethodExpr>),
    //Path(Box<PathExpr>),
    //Return(Box<ReturnExpr>),
    //Tuple(Box<TupleExpr>),
    //Unary(Box<UnaryExpr>),
    //Unsafe(Box<UnsafeExpr>),
    //Verbatim(Vec<Token>),
}


/// Assignment expression: `a += b`
#[derive(Debug)]
pub struct AssignExpr {
    pub lhs: Expression,
    pub op: Operator,
    pub rhs: Expression,
    pub span: Span,
}

/// Block expression: `{ ... }`
#[derive(Debug)]
pub struct BlockExpr {
    pub delimiter: Token,
    pub body: Vec<Statement>,
    pub span: Span,
}

/// Sub-Module declartaion: `my_module { ... }`
#[derive(Debug)]
pub struct ModuleDecl {
    pub attributes: Vec<AttrExpr>,
    pub label: Label,
    pub block: BlockExpr,
    pub span: Span,
}

/// Literal expression: `5`
#[derive(Debug)]
pub struct LiteralExpr {
    // TODO: IDK if I need: pub attributes: Vec<Attribute>,
    pub literal: Token,
    pub value: Box<str>,
}

/// 
#[derive(Debug)]
pub struct TypeDecl {
    pub attributes: Vec<AttrExpr>,
    pub label: Label,

    pub span: Span,
}

/// Function Declaration: `#[inline] my_adder_func(a u32, b u32) u32 { return a + b; }`
#[derive(Debug)]
pub struct FuncDecl {
    pub attributes: Vec<AttrExpr>,
    pub label: Label,
    pub generics: Enclosed<Seperated<GenericExpr>>,
    pub inputs: Enclosed<Seperated<FieldExpr>>,
    pub outputs: Enclosed<Seperated<ReturnType>>,
    pub span: Span,
}

#[derive(Debug)]
pub struct GenericExpr {}

#[derive(Debug)]
pub struct FieldExpr {}

#[derive(Debug)]
pub struct ReturnType {}

/// Call expression: `some_number.add(5)`
#[derive(Debug)]
pub struct CallExpr {
    pub attributes: Vec<AttrExpr>,
    pub call_op: Operator,
    pub func: Label,
    pub args: Enclosed<Seperated<Expression>>,
    pub span: Span,
}

#[derive(Debug)]
pub struct Label {
    pub token: Token,
    pub ident: Box<str>,
}

#[derive(Debug)]
pub struct Enclosed<T> {
    pub delimiter: Token,
    pub element: T,
}

impl<T> Parse for Enclosed<T> {
    fn parse(buffer: &ParseBuffer) -> Result<Option<Self>, ParseError> {
        todo!()
    }
}

#[derive(Debug)]
pub struct Seperated<T> {
    pub elemets: Vec<T>,
    pub seperators: Vec<Token>,
}

impl<T> Parse for Seperated<T> {
    fn parse(buffer: &ParseBuffer) -> Result<Option<Self>, ParseError> {
        todo!()
    }
}

#[derive(Debug)]
pub struct Operator {
    pub token: Token,
}

/// Attribute expression: `#[public, inline(false)]`
#[derive(Debug)]
pub struct AttrExpr {
    pub pound: Token,
    pub elements: Enclosed<Seperated<Attribute>>,
    pub span: Span,
}

impl Parse for AttrExpr {
    fn parse(buffer: &ParseBuffer) -> Result<Option<Self>, ParseError> {
        todo!()
    }
}
