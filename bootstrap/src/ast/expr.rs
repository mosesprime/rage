//! Rage Bootstrap
//! Expressions

use crate::symbol::{SymbolIndex, Symbol};

pub type ExprIndex = usize;

pub enum SExpr {
    Expr(ExprIndex),
    Symbol(SymbolIndex),
}

pub enum Expression<'a> {
    Operator(OperatorExpr<'a>),
    Literal(LiteralExpr<'a>),
    Function(FunctionExpr<'a>),
    Return(ReturnExpr),
    Block(BlockExpr<'a>),
    UNKNOWN,
}

pub enum OperatorExpr<'a> {
    Plus { lhs: &'a Expression<'a>, rhs: &'a Expression<'a> },
    Minus { lhs: &'a Expression<'a>, rhs: &'a Expression<'a> }, 
    Equal { lhs: &'a Expression<'a>, rhs: &'a Expression<'a> },
}

pub enum LiteralExpr<'a> {
    Numeric(&'a Symbol<'a>),
}

pub struct FunctionExpr<'a> {
    is_public: bool,
    label: &'a str,
    //generics: &'a [ExprIndex],
    block: &'a BlockExpr<'a>,
    output: Option<&'a ReturnExpr>,
}

pub struct ReturnExpr {}

pub struct BlockExpr<'a> (&'a [Expression<'a>]);

pub struct ExpressionStore<'a> {
    next_index: ExprIndex,
    expressions: Vec<Expression<'a>>,
    // PERF: attempt better memory packing?
}

impl<'a> Default for ExpressionStore<'a> {
    fn default() -> Self {
        Self { next_index: 0, expressions: Vec::default() }
    }
}

impl<'a> ExpressionStore<'a> {
    /*pub fn add_expression(&mut self, expr: Expression) -> ExprIndex {
        let index = self.next_index;
        self.next_index += 1;
        self.expressions.push(expr);
        return index;
    }

    pub fn get_expression(&self, index: ExprIndex) -> Option<&Expression> {
        self.expressions.get(index)
    }

    pub fn merge(&mut self, mut other: ExpressionStore, symbol_offset: usize) {
        let i = self.next_index;
        self.next_index += other.next_index;
        self.expressions.append(&mut other.expressions);
    }*/
}
