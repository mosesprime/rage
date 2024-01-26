//! Rage Bootstrap

use std::fmt::Display;

use crate::{builder::source::SourceId, common::Attribute, syntax::{AttrExpr, Statement}};

use super::{Parse, ParseBuffer};

/// Suggared abstract syntax tree (AST) of a given source.
pub struct ParseTree {
    /// [SourceId] of the file.
    source_id: SourceId,
    /// [Attribute]s to apply to the file.
    attributes: Vec<Attribute>,
    /// Parsed [Statement]s in the file.
    statements: Vec<Statement>,
    /// 
    unresolved: Vec<usize>, // TODO: unresolved parsing
}

impl Display for ParseTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SourceId: {}\nStatements: {:#?}\nUnresolved: {:#?}", self.source_id.to_hex(), self.statements, self.unresolved)
    }
}

impl Parse for ParseTree {
    fn parse(buffer: &ParseBuffer) -> Result<Option<Self>, super::ParseError> {
        todo!()
    }
}

impl ParseTree {
    pub fn new(source_id: &SourceId) -> Self {
        Self { 
            source_id: source_id.clone(),
            attributes: Vec::default(),
            statements: Vec::default(),
            unresolved: Vec::default(),
        }
    }

    pub fn id(&self) -> &SourceId {
        &self.source_id
    }
}

