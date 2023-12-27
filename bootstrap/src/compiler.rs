//! Rage Bootstrap Compiler

use std::{fs, path::PathBuf, result::Result::Ok};

use anyhow::{anyhow, Context};

use crate::{interpreter::InstructionTree, token::Tokenizer};

/// Front end of bootstrapper.
pub struct Compiler {
    /// Path to the project directory being compiled.
    root_path: PathBuf,
    source_files: Vec<PathBuf>,
}

impl Compiler {
    pub fn new(root_path: PathBuf) -> anyhow::Result<Self> {
        if !root_path.is_dir() {
            return Err(anyhow!("root path must be a directory"));
        }
        // turn root path into a list of source files
        let mut source_files: Vec<PathBuf> = vec![];
        let mut unexplored_dirs: Vec<PathBuf> = vec![];
        unexplored_dirs.push(root_path.clone());
        while let Some(dir_path) = unexplored_dirs.pop() {
            for entry in dir_path.read_dir().context("failed to read directory")? {
                match entry {
                    Ok(entry) => {
                        let path = entry.path();
                        if path.is_file() {
                            source_files.push(path);
                        } else if path.is_dir() {
                            unexplored_dirs.push(path);
                        } else {
                            return Err(anyhow!("unexpected file type {}", path.display()));
                        }
                    }
                    Err(e) => return Err(anyhow!(e)),
                }
            }
        }

        Ok(Self {
            root_path,
            source_files,
        })
    }

    pub fn run(mut self) -> anyhow::Result<InstructionTree> {
        for file_path in self.source_files {
            let source = fs::read_to_string(file_path.clone())?;
            let tokenizer = Tokenizer::new(source.as_str());
            for token in tokenizer {
                println!("{token:?}");
            }
        }
        Ok(InstructionTree)
    }
}
