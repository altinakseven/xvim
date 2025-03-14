# Async Plugin for xvim

This is an example plugin for the xvim editor that demonstrates the asynchronous operation support in the plugin API.

## Features

- Adds commands to create and manage asynchronous tasks
- Demonstrates progress tracking and task status monitoring
- Shows how to handle task completion and cancellation
- Provides a simple example of background processing

## Commands

The plugin adds the following commands to xvim:

- `:async_run [duration]` - Run an asynchronous task that takes the specified duration in seconds (default: 5)
- `:async_status [task_id]` - Show the status of all tasks or a specific task
- `:async_cancel <task_id>` - Cancel a running task

## Building

To build the plugin, you need to have Rust and the WebAssembly target installed:

```bash
# Install the WebAssembly target
rustup target add wasm32-wasi

# Build the plugin
cargo build --target wasm32-wasi --release
```

The compiled WebAssembly module will be located at `target/wasm32-wasi/release/async_plugin.wasm`.

## Installation

To install the plugin, copy the WebAssembly module to the xvim plugins directory:

```bash
mkdir -p ~/.config/xvim/plugins
cp target/wasm32-wasi/release/async_plugin.wasm ~/.config/xvim/plugins/
```

## Usage

Once the plugin is installed, you can use it in xvim:

1. Start xvim
2. Run the `:async_run 10` command to start a task that runs for 10 seconds
3. Run the `:async_status` command to see the status of all tasks
4. Run the `:async_status <task_id>` command to see the status of a specific task
5. Run the `:async_cancel <task_id>` command to cancel a running task

## Example Session

```
:async_run 10
Started task with ID: task_1

:async_status
Task Status:
  task_1 - Async Task (10s) (This task runs for 10 seconds): 45%, Status: Running

:async_status task_1
Task task_1 - Async Task (10s) (This task runs for 10 seconds)
  Status: Running
  Progress: 45%

:async_cancel task_1
Task 'task_1' cancelled

:async_status task_1
Task task_1 - Async Task (10s) (This task runs for 10 seconds)
  Status: Cancelled
  Result: Task was cancelled
```

## Asynchronous API

This plugin demonstrates the following asynchronous API functions:

- `create_task`: Create and run an asynchronous task
- `update_task_progress`: Update the progress of a task
- `is_task_cancelled`: Check if a task has been cancelled
- `get_task_info`: Get information about a task
- `list_tasks`: List all tasks
- `cancel_task`: Cancel a running task

## License

This plugin is licensed under the MIT License. See the LICENSE file for details.