use core_module::{
    scheduler::scheduler::Scheduler,
    worker::worker::Worker,
    types::types::WorkerId,
    task::task::{Task, TaskCommand},
};

#[tokio::main]
async fn main() {
    let mut scheduler = Scheduler::new();
    
    // Create a worker
    let worker_id = WorkerId::from("worker1");
    let command_rx = scheduler.add_worker(worker_id.clone()).await;
    let mut worker = Worker::new(
        worker_id,
        command_rx,
        scheduler.get_update_tx(),
    );

    // Spawn worker task
    tokio::spawn(async move {
        worker.run().await;
    });

    // Submit example tasks
    let tasks = vec![
        Task::new(
            "list-files".to_string(),
            TaskCommand::Shell("ls -la".to_string()),
        ),
        Task::new(
            "memory-usage".to_string(),
            TaskCommand::Shell("free -h".to_string()),
        ),
    ];

    for task in tasks {
        scheduler.submit_task(task).await;
    }

    // Handle updates in the background
    scheduler.handle_updates().await;
}
