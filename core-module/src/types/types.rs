use std::error::Error;
use std::fmt;


pub type TaskId = String;
pub type WorkerId = String;

#[derive(Debug, Clone, PartialEq)]
pub enum TaskState {
    Pending,
    Running,
    Completed(String),
    Failed(String),
}

#[derive(Debug)]
pub enum WorkerCommand {
    ExecuteTask(crate::task::task::Task),
    Stop,
}

#[derive(Debug)]
pub struct WorkerUpdate {
    pub worker_id: WorkerId,
    pub task_id: TaskId,
    pub state: TaskState,
}

#[derive(Debug)]
pub struct TaskError {
    message: String,
}

impl TaskError {
    pub fn new<T: Into<String>>(message: T) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for TaskError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for TaskError {}