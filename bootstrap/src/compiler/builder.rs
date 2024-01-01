//! Rage Bootstrap
//! Compiler Builder

use std::fs::Metadata;

use anyhow::Context;
use async_std::{task::{JoinHandle, self}, path::PathBuf, fs::{ReadDir, self}};
use blake3::Hash;

use crate::{symbol::SymbolStore, ast::expr::ExpressionStore, ModuleIndex, parser::{scanner::Scanner, Parser}};

/// Task to be run async.
pub enum BuildTask {
    ReadMetadata { path: PathBuf },
    ReadDir { path: PathBuf },
    ReadFile { path: PathBuf },
    Parse { source: String },
}

pub enum BuildResult {
    ReadMetadata { metadata: Metadata },
    ReadDir { read_dir: ReadDir },
    ReadFile { 
        hash: Hash,
        contents: String,
    },
    Parsed { 
        symbol_store: SymbolStore<'static>,
        expression_store: ExpressionStore,
    },
    TEST,
}

/// A single compilation unit.
pub struct Builder {
    task_handle: JoinHandle<anyhow::Result<BuildResult>>,
}

impl Builder {
    pub async fn spawn_and_execute(task: BuildTask) -> anyhow::Result<BuildResult> {
        task::spawn(async move {
            log::debug!("starting task");
            match task {
                BuildTask::ReadMetadata { path } => {
                    let metadata = fs::metadata(path).await.context("failed to read metadata")?;
                    return Ok(BuildResult::ReadMetadata { metadata });
                },
                BuildTask::ReadDir { path } => {
                    let read_dir = fs::read_dir(path).await.context("failed to read directory")?;
                    return Ok(BuildResult::ReadDir { read_dir });
                },
                BuildTask::ReadFile { path } => {
                    let contents = fs::read_to_string(path).await.context("failed to read file")?;
                    let hash = blake3::hash(contents.as_bytes());
                    return Ok(BuildResult::ReadFile { hash, contents });
                },
                BuildTask::Parse { source } => {
                    let mut parser = Parser::new(source.as_str());
                    parser.run();
                    // TODO: finish
                    return Ok(BuildResult::TEST);
                },
            }
        }).await
    }
}
