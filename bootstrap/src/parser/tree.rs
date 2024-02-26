//! Rage Bootstrap

use crate::syntax::{lexeme::{Lexeme, LexemeKind}, Statement, LiteralExpr};

use super::Parse;

/// Suggared abstract syntax tree (AST) of a given source file.
#[derive(Debug)]
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

    pub fn add(&mut self, stmt: Statement) {
        log::debug!("adding statement: {:?}", stmt); // TODO: remove
        self.stmts.push(stmt)
    }
}

