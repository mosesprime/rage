//! Rage Bootstrap
//! Parse Tree

use super::lexeme::LexemeIndex;

#[derive(Debug)]
pub struct Range {
    pub start: LexemeIndex,
    pub end: LexemeIndex,
}

pub type Label<'a> = &'a str;

/// Abstract Syntax Tree
#[derive(Debug)]
pub struct AST<'a> {
    root: ModuleDecl<'a>,
}

#[derive(Debug)]
pub enum Expression<'a> {
    FuncDecl(&'a FuncDecl<'a>),
    CallExpr(&'a CallExpr<'a>),
    ModuleDecl(&'a ModuleDecl<'a>),
    TypeDecl(&'a TypeDecl<'a>),
    LocalDecl(&'a LocalDecl<'a>),

    Symbol,
    UNKNOWN,
}

#[derive(Debug)]
pub struct LocalDecl<'a> {
    attributes: &'a [Attribute],
    range: Range,
    label: Label<'a>,
    // expr
}

#[derive(Debug)]
pub struct TypeDecl<'a> {
    attributes: &'a [Attribute],
    range: Range,
    generics: &'a [GenericField<'a>],
    //fields: &'a [],
}

#[derive(Debug)]
pub struct ModuleDecl<'a> {
    attributes: &'a [Attribute],
    range: Range,
    label: Label<'a>,
    body: &'a [Expression<'a>],
}

#[derive(Debug)]
pub struct CallExpr<'a> {
    range: Range,
    label: Label<'a>,
    //arguments: &'a [],
}

#[derive(Debug)]
pub struct FuncDecl<'a> {
    attributes: &'a [Attribute],
    range: Range,
    label: Label<'a>,
    generics: &'a [GenericField<'a>],
    params: &'a [ParameterField],
    results: &'a [ResultField],
    body: &'a [Expression<'a>],
}

#[derive(Debug)]
pub struct GenericField<'a> {
    attributes: &'a [Attribute],
    range: Range,
    label: Label<'a>,
    // type-path
}

#[derive(Debug)]
pub enum Attribute {
    Mutable,
    Public,
}

#[derive(Debug)]
pub struct DataTypeField<'a> {
    attributes: &'a [Attribute],
    range: Range,
    
}

#[derive(Debug)]
pub struct ResultField {
    range: Range,
    //kind: Typ,
}

#[derive(Debug)]
pub struct ParameterField {}

#[derive(Debug)]
pub struct Path<'a> {
    range: Range,
    segments: &'a [Label<'a>],
}

