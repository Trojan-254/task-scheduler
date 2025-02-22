use crate::types::types::*;
use crate::task::task::*;
use tokio::process::Command;
use tokio::sync::mpsc;
use std::process::Output;

pub struct Worker {
    id: WorkerId,
    command_rx: mpsc::Receiver<WorkerCommand>,
    update_tx: mpsc::Sender<WorkerUpdate>,
    is_busy: bool,
}

impl Worker {
    pub fn new(
        id: WorkerId,
        command_rx: mpsc::Receiver<WorkerCommand>,
        update_tx: mpsc::Sender<WorkerUpdate>,
    ) -> Self {
        Worker {
            id,
            command_rx,
            update_tx,
            is_busy: false,
        }
    }

    pub async fn run(&mut self) {
        while let Some(command) = self.command_rx.recv().await {
            match command {
                WorkerCommand::ExecuteTask(task) => {
                    self.is_busy = true;
                    
                    let result = self.execute_task(task.clone()).await;
                    
                    let state = match result {
                        Ok(output) => TaskState::Completed(output),
                        Err(e) => TaskState::Failed(e.to_string()),
                    };
                    
                    let _ = self.update_tx.send(WorkerUpdate {
                        worker_id: self.id.clone(),
                        task_id: task.id,
                        state,
                    }).await;
                    
                    self.is_busy = false;
                }
                WorkerCommand::Stop => break,
            }
        }
    }

    async fn execute_task(&self, task: Task) -> Result<String, TaskError> {
        match task.command {
            TaskCommand::Shell(cmd) => {
                let output = if cfg!(target_os = "windows") {
                    Command::new("cmd")
                        .args(["/C", &cmd])
                        .output()
                        .await
                        .map_err(|e| TaskError::new(e.to_string()))?
                } else {
                    Command::new("sh")
                        .args(["-c", &cmd])
                        .output()
                        .await
                        .map_err(|e| TaskError::new(e.to_string()))?
                };

                self.format_output(output)
            }
            TaskCommand::FileProcess(path) => {
                let contents = tokio::fs::read_to_string(path)
                    .await
                    .map_err(|e| TaskError::new(e.to_string()))?;
                Ok(contents)
            }
        }
    }

    fn format_output(&self, output: Output) -> Result<String, TaskError> {
        let stdout = String::from_utf8(output.stdout)
            .map_err(|e| TaskError::new(e.to_string()))?;
        let stderr = String::from_utf8(output.stderr)
            .map_err(|e| TaskError::new(e.to_string()))?;
        
        if output.status.success() {
            Ok(stdout)
        } else {
            Err(TaskError::new(format!("Command failed: {}", stderr)))
        }
    }
}