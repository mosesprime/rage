//! Rage Bootstrap
//! 
//! Entry point for the bootstrap build system.

use std::{time::SystemTime, path::PathBuf};

use rage_bootstrap::{logging::{TextColor, LogLevel}, Compiler};

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let start_time = SystemTime::now();
    let source_path: PathBuf = "./examples/demo.rg".into();
    LogLevel::Info.println(format!("{} {}", TextColor::wrap_text("STARTING", TextColor::BrightGreen), &source_path.display()));

    let compiler = Compiler::new(source_path)?;
    let (num_errs, num_warn) = compiler.run()?;
    if num_errs == 0 && num_warn == 0 {
        LogLevel::Info.println(format!("{} elapsed {} seconds", TextColor::wrap_text("SUCCESS", TextColor::BrightGreen), start_time.elapsed()?.as_secs_f64()));
    } else {
        LogLevel::Info.println(
            format!(
                "{} elapesd {} seconds, {} errors, {} warnings",
                TextColor::wrap_text("COMPLETE", TextColor::BrightCyan),
                start_time.elapsed()?.as_secs_f64(),
                TextColor::wrap_text(num_errs.to_string(), TextColor::BrightRed),
                TextColor::wrap_text(num_warn.to_string(), TextColor::BrightYellow)
            )
        );
    }
    Ok(())
}
