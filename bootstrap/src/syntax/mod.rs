//! Rage Bootstrap
//! Syntax

use std::{fmt::{Debug, Display}, iter::Peekable, str::FromStr};

use crate::{common::{span::Span, Attribute}, parser::{lexeme::{Lexeme, LexemeKind}, scanner::Scanner, Parse, ParseError}};

use self::{token::{Delimiter, DelimiterKind, Literal, Operator, OperatorKind, Seperator, SeperatorKind, Terminator}};

pub mod token;

#[derive(Debug)]
pub enum Statement {
    Declaration(Declaration),
    Expression(Expression),
}

#[derive(Debug)]
pub enum Declaration {
    Function(Box<FuncDecl>),
    //Foreign(Box<ForeignDecl>),
    Local(Box<LocalDecl>),
    Macro(Box<MacroDecl>),
    Module(Box<ModuleDecl>),
    //Type(Box<TypeDecl>),
    //Verbatim(Vec<LexemeKind>)
}

#[derive(Debug)]
pub enum Expression {
    //Array(Box<ArrayExpr>),
    //Assign(Box<AssignExpr>),
    //Async(Box<AsyncExpr>),
    Attribute(Box<AttrExpr>),
    //Await(Box<AwaitExpr>),
    Binary(Box<BinaryExpr>),
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
    Unary(Box<UnaryExpr>),
    //Unsafe(Box<UnsafeExpr>),
    //Verbatim(Vec<Token>),
}

#[derive(Debug)]
pub struct MacroDecl {
    pub attrs: Vec<AttrExpr>,
    pub label: Label,
    pub inner: Macro, 
}

#[derive(Debug)]
pub struct MacroExpr {
    pub attrs: Vec<AttrExpr>,
    pub inner: Macro,
}

#[derive(Debug)]
pub struct Macro {
    pub delimiter: Option<Delimiter>,
    // TODO: pub inner: Vec<Token>,
}

#[derive(Debug)]
pub struct LocalDecl {
    pub attrs: Vec<AttrExpr>,
    pub label: Label,
    pub init: Option<LocalInit>,
    pub terminator: Terminator,
}

#[derive(Debug)]
pub struct LocalInit {
    pub op: Operator,
    pub expr: Box<Expression>,
}

/// Unary expression: `!a`
#[derive(Debug)]
pub struct UnaryExpr {
    pub attrs: Vec<Attribute>,
    pub op: Operator,
    pub expr: Box<Expression>,
    pub span: Span,
}

/// Assignment expression: `a += b`
#[derive(Debug)]
pub struct BinaryExpr {
    pub lhs: Box<Expression>,
    pub op: Operator,
    pub rhs: Box<Expression>,
    pub span: Span,
}

/// Block expression: `{ ... }`
#[derive(Debug)]
pub struct BlockExpr {
    pub delimiter: Delimiter,
    pub body: Vec<Statement>,
    pub span: Span,
}

/// Sub-Module declartaion: `my_module { ... }`
#[derive(Debug)]
pub struct ModuleDecl {
    pub attrs: Vec<AttrExpr>,
    pub label: Label,
    pub block: Box<BlockExpr>,
    pub span: Span,
}

/// Literal expression: `5`
#[derive(Debug)]
pub struct LiteralExpr {
    pub literal: Literal,
    pub span: Span,
}

/// Function Declaration: `#[inline] my_func(a u32, b u32) u32 { return a + b; }`
#[derive(Debug)]
pub struct FuncDecl {
    pub attributes: Vec<AttrExpr>,
    pub label: Label,
    pub generics: Option<Enclosed<Seperated<GenericExpr>>>,
    pub inputs: Enclosed<Seperated<Parameter>>,
    pub outputs: Option<Enclosed<Seperated<ReturnType>>>,
    pub body: Box<BlockExpr>,
    pub span: Span,
}

/// Parameter: my_func(`a mut *u32`) { ... }
#[derive(Debug)]
pub struct Parameter {
    pub attrs: Vec<AttrExpr>,
    pub label: Label,
    pub kind: Box<TypeExpr>,
}

#[derive(Debug)]
pub struct TypeExpr {
    pub attrs: Vec<AttrExpr>,
    pub inner: Type,
    pub span: Span,
}

#[derive(Debug)]
pub enum Type {
    Borrow(BorrowType),
    Path(PathType),
    Pointer(PointerType),
    Reference(ReferenceType),
    Tuple(TupleType),
}

#[derive(Debug)]
pub struct PathType(Box<PathExpr>);

#[derive(Debug)]
pub struct BorrowType {
    pub borrow_op: Operator,
    pub element: Box<Type>,
}

#[derive(Debug)]
pub struct PointerType {
    pub ptr_op: Operator,
    pub element: Box<Type>,
}

#[derive(Debug)]
pub struct ReferenceType {
    pub ref_op: Operator,
    pub element: Box<Type>,
}

#[derive(Debug)]
pub struct TupleType {
    pub elements: Seperated<Type>
}

#[derive(Debug)]
pub struct GenericExpr {}

#[derive(Debug)]
pub struct FieldExpr {
    pub attributes: Vec<Attribute>,

}

#[derive(Debug)]
pub struct ReturnType {}

/// Call expression: `invoke(a, b)`
#[derive(Debug)]
pub struct CallExpr {
    pub attrs: Vec<AttrExpr>,
    pub func: Label,
    pub args: Enclosed<Seperated<Expression>>,
    pub span: Span,
}

#[derive(Debug)]
pub struct Enclosed<T> {
    pub delimiter: Delimiter,
    pub inner: T,
    pub span : Span,
}


#[derive(Debug)]
pub struct Seperated<T> {
    pub elemets: Vec<T>,
    pub seperators: Vec<Seperator>,
}

/// Attribute expression: `#[public, inline(false)]`
#[derive(Debug)]
pub struct AttrExpr {
    pub pound: Span,
    pub elements: Enclosed<Seperated<Attribute>>,
    pub span: Span,
}

#[derive(Debug)]
pub struct Label {
    pub ident: Identifier,
    pub span: Span,
}

#[derive(Debug)]
pub struct Identifier(pub Box<str>);

impl From<&str> for Identifier {
    fn from(value: &str) -> Self {
        Self(value.into())
    }
}

#[derive(Debug)]
pub struct PathExpr {
    pub attributes: Vec<Attribute>,
    pub segments: Seperated<PathSegment>,
    pub span: Span,
}

#[derive(Debug)]
pub struct PathSegment {
    pub ident: Identifier,
    pub span: Span,
}
