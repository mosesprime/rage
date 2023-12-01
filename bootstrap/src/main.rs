//! Rage Bootstrap
//! 
//! Entry point for the bootstrap build system.

use std::{time::SystemTime, thread, path::PathBuf};

use rage_bootstrap::{lexer::Lexer, TextColor, errors::ErrorManifest};

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let start_time = SystemTime::now();
    let source_path: PathBuf = "./examples/demo.rg".into();
    println!(
        "[{}] {}",
        TextColor::wrap_text("STARTING".to_string(), TextColor::Green),
        &source_path.display()
    );
    let error_manifest = ErrorManifest::new();
    let lexer_err_man = error_manifest.clone();

    let handle = thread::spawn(move || {
        let mut lexer = Lexer::new(source_path).unwrap();
        lexer.run(lexer_err_man).unwrap();
        let tokens = lexer.report();
        let mut cursor = 0;
        for token in tokens {
            let value = lexer.get_value(cursor, token.length).unwrap();
            println!("{token:?} {value:?}");
            cursor += token.length;
        }     
    });

    handle.join().unwrap();

    let (num_errors, num_warnings) = error_manifest.lock().unwrap().report();
    if num_errors == 0 && num_warnings == 0 {
        println!(
            "[{}] elapsed {} seconds",
            TextColor::wrap_text("SUCCESS".to_string(), TextColor::BrightGreen),
            start_time.elapsed()?.as_secs_f64()
        );
    } else {
        error_manifest.lock().unwrap().print();
        eprintln!(
            "[DONE] elapsed {} seconds with {} errors and {} warnings", 
            start_time.elapsed()?.as_secs_f64(),
            TextColor::wrap_text(num_errors.to_string(), TextColor::Red),
            TextColor::wrap_text(num_warnings.to_string(), TextColor::Yellow)
        );
    }

    Ok(())
}
