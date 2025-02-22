use crate::types::types::*;
use crate::task::task::*;
use std::collections::HashMap;
use tokio::sync::mpsc;

pub struct Scheduler {
    workers: HashMap<WorkerId, mpsc::Sender<WorkerCommand>>,
    tasks: HashMap<TaskId, Task>,
    update_rx: mpsc::Receiver<WorkerUpdate>,
    update_tx: mpsc::Sender<WorkerUpdate>,
}

impl Scheduler {
    pub fn new() -> Self {
        let (update_tx, update_rx) = mpsc::channel(100);
        Scheduler {
            workers: HashMap::new(),
            tasks: HashMap::new(),
            update_rx,
            update_tx,
        }
    }

    pub fn get_update_tx(&self) -> mpsc::Sender<WorkerUpdate> {
        self.update_tx.clone()
    }

    pub async fn add_worker(&mut self, worker_id: WorkerId) -> mpsc::Receiver<WorkerCommand> {
        let (tx, rx) = mpsc::channel(10);
        self.workers.insert(worker_id, tx);
        rx
    }

    pub async fn submit_task(&mut self, task: Task) {
        println!("Submitting task: {:?}", task);
        self.tasks.insert(task.id.clone(), task);
        self.try_schedule_tasks().await;
    }

    async fn try_schedule_tasks(&mut self) {
        let pending_tasks: Vec<_> = self.tasks
            .values()
            .filter(|task| matches!(task.state, TaskState::Pending))
            .cloned()
            .collect();

        for task in pending_tasks {
            if let Some(worker_tx) = self.find_available_worker() {
                println!("Scheduling task {} to worker", task.id);
                let _ = worker_tx.send(WorkerCommand::ExecuteTask(task)).await;
            }
        }
    }

    fn find_available_worker(&self) -> Option<&mpsc::Sender<WorkerCommand>> {
        self.workers.values().next()
    }

    pub async fn handle_updates(&mut self) {
        while let Some(update) = self.update_rx.recv().await {
            if let Some(task) = self.tasks.get_mut(&update.task_id) {
                task.state = update.state.clone();
                match &update.state {
                    TaskState::Completed(output) => {
                        println!("Task {} completed. Output: {}", task.id, output);
                    }
                    TaskState::Failed(error) => {
                        println!("Task {} failed: {}", task.id, error);
                    }
                    _ => {
                        println!("Task {} status updated to: {:?}", task.id, task.state);
                    }
                }
            }
            self.try_schedule_tasks().await;
        }
    }
}