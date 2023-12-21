//! Rage Bootstrap

use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
    usize,
};

use builder::Builder;
use logging::LogLevel;

pub mod builder;
pub mod errors;
pub mod lexer;
pub mod logging;
pub mod parser;
pub mod symbol;
pub mod token;

pub struct Compiler {
    num_builders: usize,
    source_root: PathBuf,
    unclaimed_source_files: Arc<Mutex<Vec<PathBuf>>>,
}

impl Compiler {
    pub fn new(source_root: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let num_builders = std::thread::available_parallelism()?.get();
        LogLevel::Info.println(format!("spawning {} builders", num_builders));
        let start = vec![source_root.clone()];
        Ok(Self {
            num_builders,
            source_root,
            unclaimed_source_files: Arc::new(Mutex::new(start)),
        })
    }

    pub fn run(self) -> Result<(usize, usize), Box<dyn std::error::Error>> {
        // TODO: add other files to unclaimed source files
        let mut handles: Vec<JoinHandle<()>> = vec![];
        for _ in 0..self.num_builders {
            let available_source_files = Arc::clone(&self.unclaimed_source_files);
            let handle: JoinHandle<_> = thread::Builder::new()
                .spawn(move || {
                    if let Some(path) = available_source_files.lock().unwrap().pop() {
                        let mut builder = Builder::default();
                        builder.source(path).unwrap();
                        builder.run();
                    }
                })
                .unwrap();
            handles.push(handle);
        }
        for handle in handles {
            handle.join().unwrap();
        }
        let err_report = todo!();
        Ok(err_report)
    }
}
