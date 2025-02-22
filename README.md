# Rust Task Scheduler

![Task Scheduler image](https://github.com/user-attachments/assets/6547218e-8dc1-4631-8a71-22c25d0614b8)



A robust, asynchronous task scheduling system built in Rust using Tokio. This scheduler provides a flexible framework for distributing and executing tasks across multiple workers, with support for both shell commands and file processing operations.

## Features

- **Asynchronous Execution**: Built on Tokio for efficient concurrent task processing
- **Multiple Task Types**: Support for shell commands and file processing operations
- **Worker Management**: Dynamic worker allocation and task distribution
- **Status Tracking**: Real-time task state monitoring and updates
- **Cross-Platform**: Compatible with both Windows and Unix-based systems
- **Error Handling**: Robust error management with detailed feedback

## Architecture

The system consists of four main components:

### 1. Scheduler
- Manages task distribution and worker coordination
- Maintains task and worker state
- Handles task submissions and updates
- Implements task queuing and scheduling logic

### 2. Worker
- Executes assigned tasks asynchronously
- Handles both shell commands and file operations
- Provides detailed execution feedback
- Implements error handling and recovery

### 3. Task
- Represents individual work units
- Supports multiple command types
- Tracks execution state and progress
- Contains metadata like creation time and ID

### 4. Types
- Defines core system types and states
- Implements error handling structures
- Provides common type definitions and enums

## Installation

1. Add the dependency to your `Cargo.toml`:
```toml
[dependencies]
task-scheduler = { git = "https://github.com/Trojan-254/task-scheduler" }
```

2. Install required dependencies:
```bash
cargo build
```

## Usage

### Basic Example

```rust
use task_scheduler::{
    task::{Task, TaskCommand},
    types::WorkerId,
    worker::Worker,
    scheduler::Scheduler,
};

#[tokio::main]
async fn main() {
    // Initialize the scheduler
    let mut scheduler = Scheduler::new();
    
    // Create a worker
    let worker_id = WorkerId::from("worker1");
    let command_rx = scheduler.add_worker(worker_id.clone()).await;
    let worker = Worker::new(
        worker_id,
        command_rx,
        scheduler.get_update_tx(),
    );

    // Spawn worker
    tokio::spawn(async move {
        worker.run().await;
    });

    // Create and submit a task
    let task = Task::new(
        "list-files".to_string(),
        TaskCommand::Shell("ls -la".to_string()),
    );
    
    scheduler.submit_task(task).await;

    // Handle updates
    scheduler.handle_updates().await;
}
```

### Task Types

1. Shell Commands:
```rust
let shell_task = Task::new(
    "memory-check".to_string(),
    TaskCommand::Shell("free -h".to_string()),
);
```

2. File Processing:
```rust
let file_task = Task::new(
    "read-log".to_string(),
    TaskCommand::FileProcess("/var/log/system.log".to_string()),
);
```

## Task States

Tasks can be in one of four states:

- `Pending`: Task has been submitted but not yet started
- `Running`: Task is currently being executed
- `Completed(String)`: Task completed successfully with output
- `Failed(String)`: Task failed with error message

## Error Handling

The system implements a custom `TaskError` type that is thread-safe and provides detailed error information. All errors are properly propagated and handled throughout the execution chain.

## Best Practices

1. **Worker Management**
   - Create workers based on system resources
   - Monitor worker health and restart if necessary
   - Implement proper shutdown procedures

2. **Task Submission**
   - Include meaningful task IDs
   - Set appropriate timeouts
   - Handle task dependencies if needed

3. **Error Handling**
   - Always check task results
   - Implement retry logic for transient failures
   - Log errors appropriately

## Contributing

Contributions are welcome! Please follow these steps:

1. Fork the repository
2. Create a feature branch
3. Commit your changes
4. Push to the branch
5. Create a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Testing

Run the test suite:

```bash
cargo test
```

Run with specific features:

```bash
cargo test --features "extended-logging"
```

## Known Limitations

- No built-in task prioritization
- Limited retry mechanisms
- No persistent storage of task states
- Single-thread execution per worker

## Future Improvements

- [ ] Task priority queues
- [ ] Persistent task storage
- [ ] Worker load balancing
- [ ] Task dependency management
- [ ] Advanced scheduling policies
- [ ] Web interface for monitoring
- [ ] Metrics and telemetry
- [ ] Docker support

## Support

For issues and feature requests, please use the GitHub issue tracker.
