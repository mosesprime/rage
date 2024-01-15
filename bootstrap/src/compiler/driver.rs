//! Rage Bootstrap
//! Compiler Builder

use std::{fs::{Metadata, ReadDir}, thread::{self, JoinHandle}, sync::{mpsc::{self, Sender, Receiver, SyncSender}, Arc, Mutex}, path::PathBuf, time::Duration};

use anyhow::{Context, anyhow};
use blake3::Hash;

use crate::{parser::{Parser, tree::ParseTree}, syntax::token::Token};

/// A single compilation unit.
/// Should spawn one per thread if able.
pub struct Driver {
    is_busy: bool,
    task_tx: SyncSender<BuildTask>,
    event_rx: Receiver<BuildEvent>,
    handle: JoinHandle<()>,
}

impl Driver {
    /// Spawn a new [Driver] in its own thread.
    pub fn spawn() -> Self {
        let (task_tx, task_rx): (SyncSender<BuildTask>, Receiver<BuildTask>) = mpsc::sync_channel(0);
        let (event_tx, event_rx) = mpsc::sync_channel(0);
        let handle = thread::spawn(move || {
            log::debug!("spinning up driver");
            loop {
                if let Ok(task) = task_rx.recv() {
                    if task == BuildTask::SHUTDOWN {
                        log::debug!("shutting down driver");
                        break;
                    }
                    let res = task.execute();
                    event_tx.send(res).expect("failed to send build event");
                }
            }
        });
        Self { is_busy: false, task_tx, event_rx, handle }
    }

    /// Attempts to gracefully shutdown the [Driver].
    pub fn shutdown(&mut self) -> anyhow::Result<()> {
        self.task_tx.send(BuildTask::SHUTDOWN)?;
        match self.event_rx.recv_timeout(Duration::from_secs(1)).context("failed shutdown") {
            Ok(BuildEvent::SHUTDOWN) => return Ok(()),
            Ok(_) => return Err(anyhow!("unexpected shutdown response")),
            Err(e) => return Err(e),
        }
    }

    /// Checks if the [Driver] is occupied with a [BuildTask].
    pub fn is_busy(&self) -> bool {
        self.is_busy
    }

    /// Send a [BuildTask] to the [Driver].
    /// # Return 
    /// - `Some(Ok(()))` if task was assigned successfully.
    /// - `Some(anyhow::Error)` if task failed to be sent through channel.
    /// - `None` if the driver was unable to accept the task.
    pub fn assign(&mut self, task: BuildTask) -> Option<anyhow::Result<()>> {
        if self.is_busy {
            return None;
        }
        self.is_busy = true;
        match self.task_tx.send(task).context("failed to send build task") {
            Ok(()) => return Some(Ok(())),
            Err(e) => return Some(Err(e)),
        }
    }

    /// Query the [Driver] for a [BuildEvent].
    /// # Return 
    /// - `Some(Ok(BuildEvent))` if the assigned [BuildTask] was completed.
    /// - `Some(anyhow::Error)` if the event channel failed.
    /// - `None` if the task has not completed yet.
    pub fn query(&mut self) -> Option<anyhow::Result<BuildEvent>> {
        match self.event_rx.try_recv() {
            Ok(event) => {
                self.is_busy = false;
                return Some(Ok(event));
            },
            Err(e) => match e {
                mpsc::TryRecvError::Empty => return None,
                mpsc::TryRecvError::Disconnected => return Some(Err(anyhow!("driver disconnected"))),
            },
        }       
    }
}

/// Product of a task being executed.
#[derive(Debug)]
pub enum BuildEvent {
    ReadMetadata { metadata: Metadata },
    ReadDir { read_dir: ReadDir },
    ReadFile { 
        hash: Hash,
        contents: String,
    },
    Parsed { 
        // TODO: test if this lifetime causes extra memory use or 
        // if it is correctly coerced into a shorter lifetime
        //parse_tree: ParseTree<'static>
        tokens: Vec<Token>,
    },
    SHUTDOWN,
}

/// Task to be executed.
#[derive(Debug, PartialEq)]
pub enum BuildTask {
    ReadMetadata { path: PathBuf },
    ReadDir { path: PathBuf },
    ReadFile { path: PathBuf },
    Parse { source: String },
    SHUTDOWN,
}

impl BuildTask {
    fn execute(self) -> BuildEvent {
        // TODO: add error handling instead of .expect()
        match self {
            BuildTask::ReadMetadata { path } => {
                let metadata = std::fs::metadata(path).expect("failed to read metadata");
                return BuildEvent::ReadMetadata { metadata }
            },
            BuildTask::ReadDir { path } => {
                let read_dir = std::fs::read_dir(path).expect("failed to read directory");
                return BuildEvent::ReadDir { read_dir };
            },
            BuildTask::ReadFile { path } => {
                let contents = std::fs::read_to_string(path).expect("failed to read file");
                let hash = blake3::hash(contents.as_bytes());
                return BuildEvent::ReadFile { hash, contents };
            },
            BuildTask::Parse { source } => {
                let mut parser = Parser::new(source.as_str()).run();
                let parse_tree = ParseTree::default();
                return BuildEvent::Parsed { tokens: parser };
            },
            BuildTask::SHUTDOWN => unreachable!(),
        }
    }
}

