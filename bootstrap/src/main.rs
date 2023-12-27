//! Rage Bootstrap
//!
//! Entry point for the bootstrap build system.

use std::{path::PathBuf, time::SystemTime};

use rage_bootstrap::{compiler::Compiler, interpreter::InstructionTree};

fn main() -> anyhow::Result<()> {
    let mut logger = env_logger::builder();
    #[cfg(debug_assertions)]
    logger.filter_level(log::LevelFilter::Debug).init();
    #[cfg(not(debug_assertions))]
    logger.filter_level(log::LevelFilter::Info).init();

    let start_time = SystemTime::now();

    //let root_path: PathBuf = "./examples/".into();
    let root_path: PathBuf = "./".into();

    let mut compiler = Compiler::new(root_path)?;
    let instruction_tree = compiler.run()?;

    log::info!(
        "compiled in {} seconds",
        start_time.elapsed()?.as_secs_f64()
    );

    Ok(())
}
