//! Rage Bootstrap
//! 
//! Entry point for the bootstrap build system.

use std::{time::SystemTime, sync::mpsc, thread};

use rage_bootstrap::lexer::Lexer;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let start_now = SystemTime::now();
    let (err_tx, err_rx) = mpsc::channel();
    thread::spawn(move || {
        let mut lexer = Lexer::new("./examples/demo.rg".into()).unwrap();
        lexer.run(err_tx.clone()).unwrap();
        let tokens = lexer.report();
        let mut cursor = 0;
        for token in tokens {
            let value = lexer.get_value(cursor, token.length).unwrap();
            println!("{token:?} {value:?}");
            cursor += token.length;
        }     
    });
    while let Ok(err) = err_rx.recv() {
        eprintln!("{}", err);
    }
    println!("[DONE] elapsed {} seconds", start_now.elapsed()?.as_secs_f64());
    Ok(())
}
