//! Rage Bootstrap
//! 
//! Entry point for the bootstrap build system.

use std::{fs, time::SystemTime};

use rage_bootstrap::lexer::Lexer;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let start_now = SystemTime::now();
    let input = fs::read_to_string("./examples/hello.rg")?;
    let lexer = Lexer::new(input.as_str());
    let lexemes = lexer.tokenize();
    for t in lexemes {
        println!("{t:?}");
    }
    println!("[DONE] elapsed {} seconds", start_now.elapsed()?.as_secs_f64());
    Ok(())
}
