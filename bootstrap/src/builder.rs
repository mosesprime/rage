//! Rage Bootstrap

use std::{
    fs,
    path::PathBuf,
    sync::mpsc::{self, Receiver, Sender},
    thread::{self, JoinHandle},
};

use anyhow::Context;

use crate::{scanner::Scanner, token::Token};

/// A single compilation unit.
pub struct Builder {
    thread: JoinHandle<()>,
    sender: Sender<BuilderJob>,
    receiver: Receiver<BuilderResult>,
}

impl Builder {
    pub fn new() -> Self {
        let (job_sender, job_receiver) = mpsc::channel();
        let (result_sender, result_receiver) = mpsc::channel();
        let thread = thread::spawn(move || {
            log::debug!("starting new builder thread");
            loop {
                let job: BuilderJob = job_receiver.recv().unwrap();
                match job.execute() {
                    BuilderResult::SHUTDOWN => {
                        log::debug!("shutting down builder thread");
                        break;
                    }
                    res => result_sender.send(res).unwrap(),
                }
            }
        });
        Self {
            thread,
            sender: job_sender,
            receiver: result_receiver,
        }
    }

    pub fn send_job(&self, job: BuilderJob) {
        self.sender.send(job).unwrap()
    }

    pub fn recv_result(&self) -> BuilderResult {
        self.receiver.recv().unwrap()
    }
}

pub enum BuilderResult {
    Sourced(anyhow::Result<String>),
    Tokenized(Vec<Token>),

    SHUTDOWN,
}

pub enum BuilderJob {
    Source { path: PathBuf },
    Tokenize { source: String },

    SHUTDOWN,
}

impl BuilderJob {
    fn execute(&self) -> BuilderResult {
        match self {
            BuilderJob::Source { path } => {
                log::debug!("sourcing {}", path.display());
                let result = fs::read_to_string(path).context("failed to read source file");
                return BuilderResult::Sourced(result);
            }
            BuilderJob::Tokenize { source } => {
                log::debug!("tokenizing");
                let result = Scanner::new(source.as_str()).collect();
                return BuilderResult::Tokenized(result);
            }
            BuilderJob::SHUTDOWN => BuilderResult::SHUTDOWN,
        }
    }
}
