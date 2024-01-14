//! Rage Bootstrap
//! Parse Tree

use super::lexeme::LexemeIndex;

// TODO: test if u32 is enough
pub type ExprIndex = u32;
pub type SymbolIndex = u32;
pub type SpanIndex = u32;

#[derive(Debug)]
pub struct Span {
    start: LexemeIndex,
    end: LexemeIndex,
}

#[derive(Debug)]
pub struct ParseTree<'a> {
    attributes: &'a [Attribute],

    // comments: Vec<Comments>,

    expressions: Vec<Expression<'a>>,
    next_expr_index: ExprIndex,

    symbols: Vec<Symbol<'a>>,
    next_symbol_index: SymbolIndex,

    spans: Vec<Span>,
    next_span_index: SpanIndex,
}

impl<'a> Default for ParseTree<'a> {
    fn default() -> Self {
        Self {
            attributes: &[],
            expressions: Vec::default(),
            next_expr_index: 0,
            symbols: Vec::default(),
            next_symbol_index: 0,
            spans: Vec::default(),
            next_span_index: 0
        }
    }
}

impl<'a> ParseTree<'a> {
    pub fn set_attributes(&mut self, attrs: &'a[Attribute]) {
        self.attributes = attrs;
    }

    pub fn add_expression(&mut self, expr: Expression<'a>) -> ExprIndex {
        let index = self.next_expr_index;
        self.expressions.push(expr);
        self.next_expr_index += 1;
        return index;
    }

    pub fn add_symbol(&mut self, symbol: Symbol<'a>) -> SymbolIndex {
        let index = self.next_symbol_index;
        self.symbols.push(symbol);
        self.next_symbol_index += 1;
        return index;
    }

    pub fn add_span(&mut self, span: Span) -> SpanIndex {
        let index = self.next_span_index;
        self.spans.push(span);
        self.next_span_index += 1;
        return index;
    }
}

#[derive(Debug)]
pub enum Expression<'a> {
    Block(&'a Block<'a>),
    FuncDecl(&'a FuncDecl<'a>),
    CallExpr(&'a CallExpr<'a>),
    ModuleDecl(&'a ModuleDecl<'a>),
}

#[derive(Debug)]
pub struct ModuleDecl<'a> {
    attributes: &'a [Attribute],
    label: &'a str,
    block: ExprIndex,
}

#[derive(Debug)]
pub struct CallExpr<'a> {
    label: &'a str,
    arguments: &'a [SymbolIndex],
}

#[derive(Debug)]
pub struct FuncDecl<'a> {
    attributes: &'a [Attribute],
    label: &'a str,
    inputs: &'a [SymbolIndex],
    outputs: &'a [SymbolIndex],
    block: ExprIndex,
}

#[derive(Debug)]
pub struct Block<'a> {
    span: Span,
    inner_tree: &'a ParseTree<'a>,
}

#[derive(Debug)]
pub struct Symbol<'a> {
    label: &'a str,
    kind: SymbolKind,
    size: Option<u32>,
    width: Option<u32>,
}

impl<'a> Symbol<'a> {
    pub fn new(label: &'a str, kind: SymbolKind, size: Option<u32>, width: Option<u32>) -> Self {
        Self { label, kind, size, width }
    }
}

#[derive(Debug)]
pub enum SymbolKind {
    IntLiteral,
    UIntLiteral,
    StringLiteral,
    /// Use label as route to expression.
    ExprPath,
}

#[derive(Debug)]
pub enum Attribute {
    Mutable,
    Public,
}
