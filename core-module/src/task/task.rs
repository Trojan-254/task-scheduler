use crate::types::types::*;
use std::time::SystemTime;

#[derive(Debug, Clone)]
pub enum TaskCommand {
    Shell(String),
    FileProcess(String),
}

#[derive(Debug, Clone)]
pub struct Task {
    pub id: TaskId,
    pub command: TaskCommand,
    pub created_at: SystemTime,
    pub state: TaskState,
}

impl Task {
    pub fn new(id: TaskId, command: TaskCommand) -> Self {
        Task {
            id,
            command,
            created_at: SystemTime::now(),
            state: TaskState::Pending,
        }
    }
}