//! Rage Bootstrap Compiler

use std::{fs, path::PathBuf, result::Result::Ok, sync::mpsc::Receiver};

use anyhow::{anyhow, Context};

use crate::{interpreter::InstructionTree};

use self::driver::{BuildResult, Driver, BuildTask, JobHandler};

mod driver;
pub mod source;

/// Builds and maintains the [`InstructionTree`].
pub struct Builder {
    /// Path to the file being compiled.
    path: PathBuf,
    num_cpus: usize,
    job_handler: JobHandler,
    drivers: Vec<Driver>,
    sources: Vec<PathBuf>,
}

impl Builder {
    pub fn new(path: PathBuf, num_cpus: usize) -> anyhow::Result<Self> {
        if !path.is_file() {
            return Err(anyhow!("path must point to a file"));
        }
        let job_handler = JobHandler::default();
        let mut drivers = Vec::with_capacity(num_cpus);
        for _ in 0..num_cpus {
            drivers.push(Driver::spawn(&job_handler));
        }
        Ok(Self {
            path: path.clone(),
            num_cpus,
            job_handler,
            drivers,
            sources: vec![path],
        })
    }

    pub fn run(mut self) -> anyhow::Result<InstructionTree> {
        // TODO: replace test driver with real drivers
        let source = std::fs::read_to_string(self.path)?;
        if let BuildResult::Parsed { lexemes } = self.job_handler.push_priority(BuildTask::Parse { source }) {
            for lexeme in lexemes {
                println!("{lexeme:?}");
            }
        }
        for mut driver in self.drivers {
            driver.shutdown()
        }
        Ok(InstructionTree)
    }
}
