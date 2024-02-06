//! Rage Bootstrap

use crate::syntax::Statement;

use super::Parse;

/// Suggared abstract syntax tree (AST) of a given source file.
pub struct ParseTree {
    ///
    stmts: Vec<Statement>,
}

impl ParseTree {
    pub fn new() -> Self {
        Self { 
            stmts: Vec::default()
        }
    }

}

impl Parse for ParseTree {
    fn parse(parser: &mut super::Parser<'_>) -> Result<Self, anyhow::Error> {
        todo!()
    }
}
