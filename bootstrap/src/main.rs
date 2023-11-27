//! Rage Bootstrap
//! 
//! Entry point for the bootstrap build system.

use std::fs;

use rage_bootstrap::lexer::Lexer;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let lexer = Lexer::new();
    let input = fs::read_to_string("./examples/hello.rg")?;
    let lexemes = lexer.tokenize(input.as_str());
    for t in lexemes {
        println!("{t:?}");
    }
    Ok(())
}
