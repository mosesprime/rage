//! Rage Bootstrap
//!
//! Entry point for the bootstrap build system.

use std::{path::PathBuf, time::SystemTime};

use rage_bootstrap::compile;

fn main() -> anyhow::Result<()> {
    let mut logger = env_logger::builder();
    #[cfg(debug_assertions)]
    logger.filter_level(log::LevelFilter::Debug).init();
    #[cfg(not(debug_assertions))]
    logger
        .filter(Some("bootstrap"), log::LevelFilter::Info)
        .init();

    let start_time = SystemTime::now();

    let root_source_path: PathBuf = "./examples/demo.rg".into();

    let instruction_tree = compile(root_source_path)?;

    log::info!(
        "compiled in {} seconds",
        start_time.elapsed().unwrap().as_secs_f64()
    );

    //run(instruction_tree)?
    Ok(())
}
