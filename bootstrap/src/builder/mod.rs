//! Rage Bootstrap Compiler

use std::{fs, path::PathBuf, result::Result::Ok, str::FromStr, collections::HashMap};

use anyhow::{anyhow, Context};

use crate::{interpreter::InstructionTree};

use self::driver::{BuildEvent, Driver, BuildTask};

mod driver;

/// Builds and maintains the [`InstructionTree`].
pub struct Builder {
    /// Path to the project directory being compiled.
    root_path: PathBuf,
    source_files: Vec<PathBuf>,
}

impl Builder {
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
        Ok(Self {
            root_path,
            source_files,
        })
    }

    pub fn run(mut self) -> anyhow::Result<InstructionTree> {
        let mut driver = Driver::spawn();
        /*for path in self.source_files {
            driver.assign(BuildTask::ReadFile { path }).unwrap().unwrap();
            loop {
                if let Some(Ok(event)) = driver.query() {
                    log::info!("{event:?}");
                    break;
                }
            }
        }*/
        let path = self.root_path.join("demo.rg");
        let source = std::fs::read_to_string(path)?;
        driver.assign(BuildTask::Parse { source }).unwrap().unwrap();
        loop {
            if let Some(Ok(BuildEvent::Parsed { tokens })) = driver.query() {
                for token in tokens {
                    println!("{token:?}");
                }
                break;
            }
        }
        Ok(InstructionTree)
    }
}
