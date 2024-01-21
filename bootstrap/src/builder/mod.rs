//! Rage Bootstrap Compiler

use std::{path::PathBuf, result::Result::Ok};

use anyhow::anyhow;

use crate::interpreter::InstructionTree;

use self::{driver::{BuildTask, DriverPool, BuildEvent}, source::SourceRecord};

mod driver;
pub mod source;

/// Builds and maintains the [`InstructionTree`].
pub struct Builder {
    /// Path to the file being compiled.
    path: PathBuf,
    num_cpus: usize,
    driver_pool: DriverPool,
    sources: Vec<PathBuf>,
}

impl Builder {
    pub fn new(path: PathBuf, num_cpus: usize) -> anyhow::Result<Self> {
        if !path.is_file() {
            return Err(anyhow!("path must point to a file"));
        }
        let driver_pool = DriverPool::new(num_cpus);
        Ok(Self {
            path: path.clone(),
            num_cpus,
            driver_pool,
            sources: vec![path],
        })
    }

    pub fn run(mut self) -> anyhow::Result<InstructionTree> {
        let source = std::fs::read_to_string(self.path.clone())?;
        //let record = SourceRecord { path: self.path, hash: blake3::hash(source.as_bytes()) };
        self.driver_pool.add_priority_task(BuildTask::Parse { source });
        loop {
            let results = self.driver_pool.get_events();
            if results.is_some() {
                for result in results {
                    match result {
                        BuildEvent::Parsed { parse_tree } => {
                            println!("{parse_tree:?}");
                        },
                        _ => unimplemented!(),
                    }
                }
                break;
            }
        }
        Ok(InstructionTree)
    }
}
