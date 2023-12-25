//! Rage Bootstrap

use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
    usize,
};

use builder::Builder;

pub mod api;
pub mod builder;
pub mod errors;
pub mod lexer;
pub mod parser;
pub mod symbol;
pub mod token;

pub struct Compiler {
    threads: usize,
    handles: Vec<JoinHandle<()>>,
    source_root: PathBuf,
    unclaimed_source_files: Arc<Mutex<Vec<PathBuf>>>,
}

impl Compiler {
    pub fn new(source_root: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let start = vec![source_root.clone()];
        Ok(Self {
            threads: std::thread::available_parallelism()?.get(),
            handles: Default::default(),
            source_root,
            unclaimed_source_files: Arc::new(Mutex::new(start)),
        })
    }

    pub fn run(mut self) -> Result<(usize, usize), Box<dyn std::error::Error>> {
        // TODO: add other files to unclaimed source files
        for _ in 0..self.threads {
            let available_source_files = Arc::clone(&self.unclaimed_source_files);
            let handle: JoinHandle<_> = thread::Builder::new()
                .spawn(move || {
                    if let Some(path) = available_source_files.lock().unwrap().pop() {
                        let mut builder = Builder::default();
                        builder
                            .source(path)
                            .map_err(|e| {
                                log::error!("failed to load source: {}", e);
                            })
                            .unwrap();
                        builder.run();
                    }
                })
                .unwrap();
            self.handles.push(handle);
        }
        for handle in self.handles {
            handle.join().unwrap();
        }
        //let err_report = todo!();
        //Ok(err_report)
        Ok((0, 0))
    }
}
