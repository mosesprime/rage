//! Rage Bootstrap
//!
//! Entry point for the bootstrap build system.

use std::{path::PathBuf, time::SystemTime};

use anyhow::Context;
use rage_bootstrap::builder::Builder;

fn main() -> anyhow::Result<()> {
    let mut logger = env_logger::builder();
    #[cfg(debug_assertions)]
    logger.filter_level(log::LevelFilter::Debug).init();
    #[cfg(not(debug_assertions))]
    logger.filter_level(log::LevelFilter::Info).init();

    let start_time = SystemTime::now();

    let root_path: PathBuf = "./examples/demo.rg".into();

    let num_cpus = std::thread::available_parallelism().context("unable to get number of available threads")?;
    let mut builder = Builder::new(root_path, num_cpus.into())?;
    let instruction_tree = builder.run()?;

    log::info!(
        "compiled in {} seconds",
        start_time.elapsed()?.as_secs_f64()
    );

    Ok(())
}

