//! Rage Bootstrap

use std::{path::PathBuf, thread::{JoinHandle, self}, sync::{mpsc::{Sender, Receiver, self}, Arc, Mutex}, error::Error, fs, usize};

use builder::Builder;
use errors::{CompError, ErrorManifest};
use lexer::Lexer;
use parser::Parser;
use token::Token;

pub mod builder;
pub mod errors;
pub mod lexer;
pub mod logging;
pub mod parser;
pub mod symbol;
pub mod token;

pub struct Compiler {
    num_builders: usize,
    error_manifest: Arc<Mutex<ErrorManifest>>,
    source_root: PathBuf,
    unclaimed_source_files: Arc<Mutex<Vec<PathBuf>>>,
}

impl Compiler {
    pub fn new(source_root: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let num_builders = std::thread::available_parallelism()?.get();
        let start = vec![source_root.clone()];
        Ok(Self {
            num_builders,
            error_manifest: ErrorManifest::new(),
            source_root,
            unclaimed_source_files: Arc::new(Mutex::new(start)),
        })
    } 

    pub fn run(mut self) -> Result<(usize, usize), Box<dyn std::error::Error>> {
        // TODO: add other files to unclaimed source files
        let mut handles: Vec<JoinHandle<()>> = vec![];
        for _ in 0..self.num_builders {
            let available_source_files = Arc::clone(&self.unclaimed_source_files);
            let errs = self.error_manifest.clone();
            let handle: JoinHandle<_> = thread::Builder::new().spawn(move || {
                if let Some(path) = available_source_files.lock().unwrap().pop() {
                    let mut builder = Builder::new(errs.to_owned(), path).unwrap();
                    builder.run();
                }
            }).unwrap();
            handles.push(handle);
        }
        for handle in handles {
            handle.join().unwrap();
        }
        let err_report = self.error_manifest.lock().unwrap().report();
        Ok(err_report)
    }
}

