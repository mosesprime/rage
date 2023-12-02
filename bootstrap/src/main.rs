//! Rage Bootstrap
//! 
//! Entry point for the bootstrap build system.

use std::{time::SystemTime, thread, path::PathBuf};

use rage_bootstrap::{lexer::Lexer, TextColor, errors::ErrorManifest, parser::Parser, LogLevel, symbol::SymbolManifest};

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let start_time = SystemTime::now();
    let source_path: PathBuf = "./examples/demo.rg".into();
    LogLevel::Info.println(format!("{} {}", TextColor::wrap_text("STARTING", TextColor::BrightGreen), &source_path.display()));

    let error_manifest = ErrorManifest::new();
    let lexer_err_man = error_manifest.clone();

    let symbol_maifest = SymbolManifest::new();

    let handle = thread::spawn(move || {
        let mut lexer = Lexer::new(source_path).unwrap();
        let mut parser = Parser::new();
        let tokens = lexer.run(lexer_err_man);
        let symbol_table = parser.run(tokens);
        symbol_maifest.lock().unwrap().add_module("test", symbol_table);
    });
    handle.join().unwrap();

    let (num_errors, num_warnings) = error_manifest.lock().unwrap().report();
    if num_errors == 0 && num_warnings == 0 {
        LogLevel::Info.println(
            format!("{} elapsed {} seconds", TextColor::wrap_text("SUCCESS", TextColor::BrightGreen), start_time.elapsed()?.as_secs_f64())
        );
    } else {
        error_manifest.lock().unwrap().print();
        LogLevel::Error.println(
            format!(
                "{} elapsed {} seconds with {} and {}", 
                TextColor::wrap_text("DONE", TextColor::BrightMagenta),
                start_time.elapsed()?.as_secs_f64(),
                TextColor::wrap_text(format!("{} errors", num_errors), TextColor::Red),
                TextColor::wrap_text(format!("{} warnings", num_warnings), TextColor::Yellow)
            )
        );
    }
    Ok(())
}
