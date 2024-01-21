//! Rage Bootstrap
//! Parse Tree

#[derive(Debug)]
pub struct Range {
    pub start: u32,
    pub end: u32,
}

pub type Label<'a> = &'a str;

#[derive(Debug)]
pub struct ParseTree<'a> {
    pub declarations: Vec<Declaration<'a>>,
    //pub directives: Vec<Directives<'a>>
}

#[derive(Debug)]
pub enum Declaration<'a> {
    LocalDecl(&'a LocalDecl<'a>),
    TypeDecl(&'a TypeDecl<'a>),
    ModuleDecl(&'a ModuleDecl<'a>),
    FuncDecl(&'a FuncDecl<'a>),
}

#[derive(Debug)]
pub enum Expression<'a> {
    CallExpr(&'a CallExpr<'a>),
    ReturnExpr(&'a ReturnExpr<'a>),
    TupleExpr(&'a TupleExpr<'a>),

    Symbol(Label<'a>),
    UNKNOWN,
}

#[derive(Debug)]
pub struct TupleExpr<'a> {
    range: Range,
    inners: &'a [Expression<'a>]
}

#[derive(Debug)]
pub struct ReturnExpr<'a> {
    range: Range,
    expr: &'a Expression<'a>,
}

#[derive(Debug)]
pub struct LocalDecl<'a> {
    attributes: &'a [Attribute],
    range: Range,
    label: Label<'a>,
    rhs: &'a Expression<'a>,
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
    body: Vec<Declaration<'a>>,
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
    qualified: bool,
    segments: &'a [Label<'a>],
}

