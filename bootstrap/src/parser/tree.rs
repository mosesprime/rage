//! Rage Bootstrap

use std::fmt::Display;

use crate::{builder::source::SourceId, common::Attribute, syntax::{AttrExpr, Statement, UnaryExpr}};


/// Suggared abstract syntax tree (AST) of a given source.
pub struct ParseTree<'a> {
    /// [SourceId] of the file.
    source_id: &'a SourceId,
    /// [Attribute]s to apply to the file.
    attributes: Vec<Attribute>,
    /// Parsed [Statement]s in the file.
    statements: Vec<Statement>,
    /// 
    unresolved: Vec<usize>, // TODO: unresolved parsing
}

impl<'a> Display for ParseTree<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SourceId: {}\nStatements: {:#?}\nUnresolved: {:#?}", self.source_id.to_hex(), self.statements, self.unresolved)
    }
}

impl<'a> ParseTree<'a> {
    pub fn new(source_id: &'a SourceId) -> Self {
        Self { 
            source_id,
            attributes: Vec::default(),
            statements: Vec::default(),
            unresolved: Vec::default(),
        }
    }

    pub fn id(&self) -> &SourceId {
        &self.source_id
    }

    pub fn push(&mut self, stmt: Statement) {
        self.statements.push(stmt)
    }

    pub fn push_unary_expr(&mut self, expr: UnaryExpr) { todo!() }
}

