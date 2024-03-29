//! Rage Bootstrap
//! Compiler Builder

use std::{fs::{Metadata, ReadDir}, thread::{self, JoinHandle}, sync::mpsc::{self, Sender, Receiver, TryRecvError}, path::PathBuf, collections::VecDeque};

use anyhow::{Context, anyhow};

use crate::parser::{tree::ParseTree, Parser};

use super::{incrimental::Incrimental, source::Source};

pub struct DriverPool {
    queue: VecDeque<BuildTask>,
    drivers: Vec<Driver>,
}

impl DriverPool {
    pub fn new(num_cpus: usize) -> Self {
        let mut drivers = Vec::with_capacity(num_cpus);
        for _ in 0..num_cpus {
            drivers.push(Driver::spawn());
        }
        Self { 
            queue: VecDeque::default(),
            drivers,
        }
    }

    fn assign_to_available(&mut self) {
        let avail = self.drivers.iter_mut().filter_map(|d| d.is_available().then_some(d));
        avail.for_each(|d| {
            if let Some(task) = self.queue.pop_front() {
                d.assign(task).unwrap();
            }
        });
    }

    pub fn add_task(&mut self, task: BuildTask) {
        self.queue.push_back(task);
    }

    pub fn add_priority_task(&mut self, task: BuildTask) {
        self.queue.push_front(task);
        self.assign_to_available();
    }

    pub fn get_events(&mut self) -> Option<BuildEvent> {
        self.assign_to_available();
        // TODO: handle error instead of Result -> Option
        self.drivers.iter_mut().find_map(|d| d.query().ok()?)
    }
}

impl Drop for DriverPool {
    fn drop(&mut self) {
        self.drivers.iter_mut().for_each(|d| d.shutdown())
    }
}

#[derive(Debug)]
pub enum DriverError {
    Busy,
    NoTask,
    Channel(anyhow::Error),
    Closed(anyhow::Result<()>),
}

/// A single compilation unit.
/// Should spawn one per thread if able.
pub struct Driver {
    busy: bool,
    task_tx: Sender<BuildTask>,
    event_rx: Receiver<BuildEvent>,
    handle: JoinHandle<()>,
}

impl Driver {
    /// Spawn a new [Driver] in its own thread.
    pub fn spawn() -> Self {
        let (task_tx, task_rx): (Sender<BuildTask>, Receiver<BuildTask>) = mpsc::channel();
        let (event_tx, event_rx) = mpsc::channel();
        let handle = thread::spawn(move || {
            log::debug!("spinning up driver");
            loop {
                if let Ok(task) = task_rx.try_recv() {
                    if BuildTask::SHUTDOWN == task {
                        break;
                    }
                    event_tx.send(task.execute()).expect("driver channel failed");
                }
            }
            log::debug!("shutting down driver");
        });
        Self { busy: false, task_tx, event_rx, handle }
    }

    pub fn is_available(&self) -> bool {
        !self.busy
    }

    pub fn shutdown(&mut self) {
        self.task_tx.send(BuildTask::SHUTDOWN).expect("driver channel failed");
    }

    pub fn assign(&mut self, task: BuildTask) -> Result<(), DriverError> {
        if !self.busy {
            self.busy = true;
            match self.task_tx.send(task) {
                Ok(()) => return Ok(()),
                Err(e) => return Err(DriverError::Channel(e.into())),
            }
        } else {
            return Err(DriverError::Busy);
        }
    }

    pub fn query(&mut self) -> anyhow::Result<Option<BuildEvent>> {
        if !self.busy {
            return Ok(None);
        }
        match self.event_rx.try_recv() {
            Ok(event) => {
                self.busy = false;
                return Ok(Some(event));
            },
            Err(TryRecvError::Empty) => return Ok(None),
            Err(TryRecvError::Disconnected) => return Err(TryRecvError::Disconnected.into()),
        }
    }
}

/// Product of a task being executed.
pub enum BuildEvent {
    ReadMetadata { metadata: Metadata },
    ReadDir { read_dir: ReadDir },
    ReadFile { 
        source: Source,
    },
    Parsed { 
        parse_tree: ParseTree,
    },
    // TODO: expand errors
    Error(anyhow::Error),
}

/// Task to be executed.
#[derive(PartialEq)]
pub enum BuildTask {
    ReadMetadata { path: PathBuf },
    ReadDir { path: PathBuf },
    ReadFile { path: PathBuf },
    Parse { source: Source },
    SHUTDOWN,
}

impl BuildTask {
    fn execute(self) -> BuildEvent {
        let error = match self {
            BuildTask::ReadMetadata { path } => {
                log::debug!("reading metadata from {}", path.display());
                match std::fs::metadata(path).context("failed to read metadata") {
                    Ok(metadata) => return BuildEvent::ReadMetadata { metadata },
                    Err(e) => e,
                }
            },
            BuildTask::ReadDir { path } => {
                log::debug!("reading directory at {}", path.display());
                match std::fs::read_dir(path).context("failed to read directory") {
                    Ok(read_dir) => return BuildEvent::ReadDir { read_dir },
                    Err(e) => e,
                }
            },
            BuildTask::ReadFile { path } => {
                log::debug!("reading file at {}", path.display());
                match Source::from_path(path).context("failed to read source file") {
                    Ok(source) => return BuildEvent::ReadFile { source },
                    Err(e) => e,
                }
            },
            BuildTask::Parse { source } => {
                if let Incrimental::RawText(text) = source.incrimental {
                    // TODO: remove expect
                    match Parser::new(text.as_str()).start() {
                        Ok(parse_tree) => return BuildEvent::Parsed { parse_tree },
                        Err(e) => e,
                    }
                } else {
                    anyhow!("missing source text")
                }
            },
            BuildTask::SHUTDOWN => unreachable!(),
        };
        return BuildEvent::Error(error);
    }
}

