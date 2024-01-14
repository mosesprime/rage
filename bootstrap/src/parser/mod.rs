//! Rage Bootstrap
//! Parser

use self::{scanner::Scanner, lexeme::{LexemeKind, Lexeme}};

pub mod lexeme;
pub mod scanner;
pub mod tree;

/// 
pub struct Parser<'a> {
    source: &'a str,
    scanner: Scanner<'a>,
    errors: Vec<anyhow::Error>,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            scanner: Scanner::new(source),
            errors: Vec::default(),
        }
    }
    
    /// Gets a slice of the source if able.
    pub fn get_value(&self, index: usize, length: usize) -> Option<&str> {
        self.source.get(index..(index + length))
    }

    /// Get the line of source code where the index is located.
    pub fn get_line_from_index(&self, index: usize) -> Option<&str> {
        self.source.lines().find(|line| {
            let mut indcies = line.char_indices();
            if let Some((n, _)) = indcies.next() {
                n == index
            } else {
                false
            }
        })
    }

    /// Gets a single line of the source if able.
    pub fn get_line_number(&self, line_num: usize) -> Option<&str> {
        self.source.lines().nth(line_num)
    }

    pub fn run( self) -> Vec<Lexeme> {
        /*while let Some(lexeme) = self.scanner.next() {
            let v = self.get_value(lexeme.index as usize, lexeme.length as usize);
            log::debug!("{lexeme:?} {v:?}");
        }*/
        self.scanner.collect()
    }
}
