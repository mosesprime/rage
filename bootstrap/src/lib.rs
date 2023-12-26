//! Rage Bootstrap

use std::{path::PathBuf, thread};

use anyhow::Context;
use builder::Builder;

use crate::builder::{BuilderJob, BuilderResult};

pub mod builder;
pub mod scanner;
pub mod token;

pub fn compile(root_source_path: PathBuf) -> anyhow::Result<()> {
    //-> anyhow::Result<InstructionTree> {
    log::info!("starting compilation: {}", root_source_path.display());
    let num_cpus = thread::available_parallelism()
        .context("unable to get number of available threads")?
        .get();
    let builder = Builder::new();
    builder.send_job(BuilderJob::Source {
        path: root_source_path,
    });
    if let BuilderResult::Sourced(source) = builder.recv_result() {
        builder.send_job(BuilderJob::Tokenize { source: source? });
        if let BuilderResult::Tokenized(tokens) = builder.recv_result() {
            for token in tokens {
                println!("{token:?}");
            }
            builder.send_job(BuilderJob::SHUTDOWN);
        }
    }
    Ok(())
}

//pub fn run(instruction_tree: InstructionTree) -> anyhow::Result<()> {
//Interpreter::new(instruction_tree).execute()?
//}
