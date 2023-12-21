//! Rage Bootstrap
//!
//! Entry point for the bootstrap build system.

use std::{path::PathBuf, time::SystemTime};

use log::LevelFilter;
use rage_bootstrap::{
    logging::{self, LogLevel, TextColor},
    Compiler,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start_time = SystemTime::now();
    let source_path: PathBuf = "./examples/demo.rg".into();
    LogLevel::Info.println(format!("STARTING: {}", &source_path.display()));

    let compiler = Compiler::new(source_path)?;
    let (num_errs, num_warn) = compiler.run()?;
    if num_errs == 0 && num_warn == 0 {
        LogLevel::Info.println(format!(
            "SUCCESS! Elapsed {} seconds.",
            start_time.elapsed()?.as_secs_f64()
        ));
    } else {
        LogLevel::Info.println(format!(
            "COMPLETE: Elapesd {} seconds, {} errors, {} warnings.",
            start_time.elapsed()?.as_secs_f64(),
            TextColor::wrap_text(num_errs.to_string(), TextColor::BrightRed),
            TextColor::wrap_text(num_warn.to_string(), TextColor::BrightYellow)
        ));
    }
    Ok(())
}
