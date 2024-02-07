//! Rage Bootstrap

use crate::syntax::{lexeme::{Lexeme, LexemeKind}, Statement, LiteralExpr};

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
        log::warn!("parse_tree is a work in progress");
        while let Some(peek) = parser.peek_lexeme() {
            match &peek.kind {
                LexemeKind::Whitespace(_) | LexemeKind::Comment(_) => {
                    // ignore whitespace and comments
                    parser.next_lexeme();
                },
                LexemeKind::Literal(_) => {
                    let x = LiteralExpr::parse(parser)?;
                    println!("{x:?}");
                },
                _ => todo!(),
            }
        }
        Ok(ParseTree::new())
    }
}
