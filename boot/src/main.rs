use std::{iter::Peekable, path::Path};

use lexer::Lexer;

mod lexer;

fn main() {
    env_logger::init();
    let path = Path::new("./run.rg");
    let input = std::fs::read_to_string(path).unwrap();
    let lexer = Lexer::new(input.as_str());
    for lexeme in lexer.into_iter() {
        println!("{:?}", lexeme);
    }
}

trait Parse: Sized {
    fn parse(parser: &mut Parser) -> Result<Self, &'static str>;
}

struct Parser<'a> {
    lexemes: Peekable<Lexer<'a>>,
}

impl<'a> Parser<'a> {
    fn new(lexer: Lexer<'a>) -> Self {
        Self {
            lexemes: lexer.peekable(),
        }
    }
}


