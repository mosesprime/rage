//! Rage Bootstrap Parser
//! Code block scoping.

use crate::token::{Token, TokenKind, symbol::Symbol};

/// Tokens between '{' and '}'
#[derive(Debug)]
pub struct Block {
    pub start_index: usize,
    pub end_index: usize,
}

impl Block {
    pub fn scan(tokens: &Vec<Token>) -> Result<Vec<Block>, Vec<usize>> {
        let mut index = 0;
        let mut blocks: Vec<Block> = vec![];
        let mut opens: Vec<usize> = vec![];
        let mut issues: Vec<usize> = vec![];
        tokens.iter().for_each(|t| {
            match t.kind {
                TokenKind::Symbol(Symbol::LCurly) => {
                    opens.push(index)
                },
                TokenKind::Symbol(Symbol::RCurly) => {
                    if let Some(open) = opens.pop() {
                        blocks.push(Block { start_index: open, end_index: index })
                    } else {
                        issues.push(index)
                    }
                },
                _ => {},
            }
            index += 1;
        });
        if issues.is_empty() {
            return Ok(blocks);
        }
        return Err(issues);
    }
}
