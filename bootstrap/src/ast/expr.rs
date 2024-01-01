//! Rage Bootstrap
//! Expressions

use crate::symbol::SymbolIndex;

pub type ExprIndex = usize;

pub enum Expression {
    OperatorExpr(OperatorExpr),
    LiteralExpr(LiteralExpr),
}

pub enum OperatorExpr {
    Plus { lhs: ExprIndex, rhs: ExprIndex },
    Minus { lhs: ExprIndex, rhs: ExprIndex }, 
}

pub enum LiteralExpr {
    Numeric(SymbolIndex),
}

pub struct ExpressionStore {
    next_index: ExprIndex,
    expressions: Vec<Expression>,
    // PERF: attempt better memory packing?
}

impl Default for ExpressionStore {
    fn default() -> Self {
        Self { next_index: 0, expressions: Vec::default() }
    }
}

impl ExpressionStore {
    pub fn add_expression(&mut self, expr: Expression) -> ExprIndex {
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
    }
}

