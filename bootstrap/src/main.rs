//! Rage Bootstrap
//!
//! Entry point for the bootstrap build system.

use std::{path::PathBuf, time::SystemTime};

use rage_bootstrap::{
    log_info,
    logging::{LogLevel, TextColor},
    Compiler,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start_time = SystemTime::now();
    let source_path: PathBuf = "./examples/demo.rg".into();
    log_info!("STARTING: {}", &source_path.display());

    let compiler = Compiler::new(source_path)?;
    let (num_errs, num_warn) = compiler.run()?;
    if num_errs == 0 && num_warn == 0 {
        log_info!(
            "SUCCESS! Elapsed {} seconds.",
            start_time.elapsed()?.as_secs_f64()
        );
    } else {
        log_info!(
            "COMPLETE: Elapesd {} seconds, {} errors, {} warnings.",
            start_time.elapsed()?.as_secs_f64(),
            TextColor::BrightRed.wrap_text(num_errs.to_string()),
            TextColor::BrightYellow.wrap_text(num_warn.to_string())
        );
    }
    Ok(())
}
