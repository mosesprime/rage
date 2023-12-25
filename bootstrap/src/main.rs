//! Rage Bootstrap
//!
//! Entry point for the bootstrap build system.

use std::{path::PathBuf, time::SystemTime};

use rage_bootstrap::Compiler;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut logger = env_logger::builder();
    #[cfg(debug_assertions)]
    logger.filter_level(log::LevelFilter::Debug).init();
    #[cfg(not(debug_assertions))]
    logger
        .filter(Some("bootstrap"), log::LevelFilter::Info)
        .init();
    let start_time = SystemTime::now();
    let source_path: PathBuf = "./examples/demo.rg".into();
    log::info!("STARTING: {}", &source_path.display());

    let compiler = Compiler::new(source_path)?;
    let (num_errs, num_warn) = compiler.run()?;
    if num_errs == 0 && num_warn == 0 {
        log::info!(
            "SUCCESS! Elapsed {} seconds.",
            start_time.elapsed()?.as_secs_f64()
        );
    } else {
        log::info!(
            "COMPLETE: Elapesd {} seconds, {} errors, {} warnings.",
            start_time.elapsed()?.as_secs_f64(),
            num_errs,
            num_warn
        );
    }
    Ok(())
}
