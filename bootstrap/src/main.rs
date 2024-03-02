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

    //let root_path: PathBuf = "./examples/demo.rg".into();
    //let root_path: PathBuf = "./examples/hello.rg".into();
    let root_path: PathBuf = "./examples/true.rg".into();
    //let num_cpus = std::thread::available_parallelism().context("unable to get number of available threads")?.into();
    let num_cpus = 1;
    let mut builder = Builder::new(root_path, num_cpus)?;

    let instruction_tree = match builder.run() {
        Ok(i) => i,
        Err(e) => {
            log::error!("{e}");
            return Ok(());
        },
    };

    log::info!(
        "compiled in {} seconds",
        start_time.elapsed()?.as_secs_f64()
    );

    Ok(())
}

