//! Rage Bootstrap Compiler

use std::{fs, path::PathBuf, result::Result::Ok, str::FromStr, collections::HashMap};

use anyhow::{anyhow, Context};

use crate::{interpreter::InstructionTree, symbol::SymbolStore, ModuleIndex, parser::{Parser, scanner::Scanner}};

use self::builder::{Builder, BuildResult};

mod builder;

/// Front end of bootstrapper.
/// Builds and maintains the [`InstructionTree`].
pub struct Compiler<'a> {
    /// Path to the project directory being compiled.
    root_path: PathBuf,
    source_files: Vec<PathBuf>,
    symbol_store: SymbolStore<'a>,
}

impl<'a> Compiler<'a> {
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
                            if let Some(ext) = path.extension() {
                                if ext == "rg" {
                                    log::debug!("adding file {}", path.display());
                                    source_files.push(path);
                                } else {
                                    log::debug!("ignoring file {}", path.display());
                                }
                            } else {
                                log::debug!("no extension for {}", path.display());
                            }
                        } else if path.is_dir() {
                            if let Some(path_str) = path.to_str() {
                                if path_str.contains("/.") {
                                    log::debug!("ignoring directory {}", path.display());
                                } else {
                                    log::debug!("searching directory {}", path.display());
                                    unexplored_dirs.push(path);
                                }
                            } else {
                                unimplemented!();
                            }
                        } else {
                            return Err(anyhow!("unexpected file type {}", path.display()));
                        }
                    }
                    Err(e) => return Err(anyhow!(e)),
                }
            }
        }
        let symbol_store = SymbolStore::default();
        Ok(Self {
            root_path,
            source_files,
            symbol_store,
        })
    }

    pub async fn run(mut self) -> anyhow::Result<InstructionTree> {
        for file_path in self.source_files {
            if let Ok(BuildResult::ReadFile { hash, contents }) = Builder::spawn_and_execute(builder::BuildTask::ReadFile { path: file_path.into() }).await {
                log::debug!("hash {}", hash);
                let _ = Builder::spawn_and_execute(builder::BuildTask::Parse { source: contents }).await?;
            }
        }
        Ok(InstructionTree)
    }
}
