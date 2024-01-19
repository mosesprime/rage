//! Rage Bootstrap
//! Compiler Builder

use std::{fs::{Metadata, ReadDir}, thread::{self, JoinHandle}, sync::{mpsc::{self, Sender, Receiver, SyncSender}, Arc, Mutex}, path::PathBuf, time::Duration, collections::{VecDeque, HashMap}, ops::DerefMut};

use blake3::Hash;

use crate::{parser::{Parser, lexeme::Lexeme}, syntax::token::Token};

use super::source::Source;

/// A single compilation unit.
/// Should spawn one per thread if able.
pub struct Driver {
    shutdown_tx: Sender<()>,
    handle: JoinHandle<()>,
}

impl Driver {
    /// Spawn a new [Driver] in its own thread.
    pub fn spawn(job_handler: &JobHandler) -> Self {
        let (shutdown_tx, shutdown_rx) = mpsc::channel();
        let mut job_queue = job_handler.queue();
        let handle = thread::spawn(move || {
            log::debug!("spinning up driver");
            loop {
                if let Ok(_) = shutdown_rx.try_recv() {
                    log::debug!("shutting down driver");
                    break;
                }
                if let Some((task, sender)) = job_queue.pop_front() {
                    sender.send(task.execute()).expect("channel closed");
                }
            }
        });
        Self { shutdown_tx, handle }
    }

    pub fn shutdown(&mut self) {
        self.shutdown_tx.send(()).expect("channel closed");
    }
}

/// Product of a task being executed.
#[derive(Debug)]
pub enum BuildResult {
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
        lexemes: Vec<Lexeme>,
    },
    Error(anyhow::Error)
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
    fn execute(self) -> BuildResult {
        // TODO: add error handling instead of .expect()
        match self {
            BuildTask::ReadMetadata { path } => {
                let metadata = std::fs::metadata(path).expect("failed to read metadata");
                return BuildResult::ReadMetadata { metadata }
            },
            BuildTask::ReadDir { path } => {
                let read_dir = std::fs::read_dir(path).expect("failed to read directory");
                return BuildResult::ReadDir { read_dir };
            },
            BuildTask::ReadFile { path } => {
                let contents = std::fs::read_to_string(path).expect("failed to read file");
                let hash = blake3::hash(contents.as_bytes());
                return BuildResult::ReadFile { hash, contents };
            },
            BuildTask::Parse { source } => {
                let mut parser = Parser::new(source.as_str()).run();
                return BuildResult::Parsed { lexemes: parser };
            },
            BuildTask::SHUTDOWN => unreachable!(),
        }
    }
}

pub type JobId = usize;

pub struct JobHandler {
    next_id: JobId,
    queue: JobQueue,
    results: HashMap<JobId, Receiver<BuildResult>>,
}

#[derive(Clone)]
struct JobQueue(Arc<Mutex<VecDeque<(BuildTask, Sender<BuildResult>)>>>);

impl JobQueue {
    fn push_back(&mut self, task: BuildTask, sender: Sender<BuildResult>) {
        let mut inner = self.0.lock().expect("failed to aquire lock");
        inner.push_back((task, sender));
    }

    fn push_front(&mut self, task: BuildTask, sender: Sender<BuildResult>) {
        let mut inner = self.0.lock().expect("failed to aquire lock");
        inner.push_front((task, sender));
    }

    fn pop_front(&mut self) -> Option<(BuildTask, Sender<BuildResult>)> {
        let mut inner = self.0.lock().expect("failed to aquire lock");
        inner.pop_front()
    }
}

impl Default for JobHandler {
    fn default() -> Self {
        Self{
            next_id: 0,
            queue: JobQueue(Arc::new(Mutex::new(VecDeque::default()))),
            results: HashMap::default(),
        }
    }
}

impl JobHandler {
    /// Push task to back of queue.
    pub fn push_normal(&mut self, task: BuildTask) -> JobId {
        let (tx, rx) = mpsc::channel();
        let id = self.next_id;
        self.next_id += 1;
        self.queue.push_back(task, tx);
        self.results.insert(id, rx);
        id
    }

    /// Push task to front of queue. Blocks the thread until task is complete.
    pub fn push_priority(&mut self, task: BuildTask) -> BuildResult {
        let (tx, rx) = mpsc::channel();
        self.queue.push_front(task, tx);
        rx.recv().expect("channel hung up")
    }

    /// Get a clone of the internal queue.
    pub fn queue(&self) -> JobQueue {
        self.queue.clone()
    }

    /// Query the handler to see if a particular task is complete.
    pub fn query(&mut self, id: JobId) -> Option<BuildResult> {
        if let Some(recv) = self.results.get(&id) {
            if let Ok(res) = recv.try_recv() {
                self.results.remove(&id);
                return Some(res);
            }
        }
        None
    }

    /// Number of pending jobs, both in queue and awaiting results.
    pub fn pending(&mut self) -> usize {
        let mut n = self.queue.0.lock().expect("failed to aquire lock").len();
        n += self.results.len();
        n
    }
}

