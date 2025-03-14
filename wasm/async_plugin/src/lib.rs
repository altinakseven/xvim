//! Async Plugin for xvim
//!
//! This is an example plugin that demonstrates the asynchronous operation support
//! in the xvim plugin API. It adds commands to create and manage asynchronous tasks.

// Import the xvim plugin API
use xvim_plugin_api::*;

// Plugin entry point
#[no_mangle]
pub extern "C" fn init() -> i32 {
    // Register the plugin with the editor
    register_plugin("async_plugin", "0.1.0", "Async Plugin", "xvim Team");
    
    // Register commands
    register_command("async_run", async_run_command);
    register_command("async_status", async_status_command);
    register_command("async_cancel", async_cancel_command);
    
    // Return success
    0
}

// Command handler for :async_run
fn async_run_command(args: &[&str]) -> Result<(), String> {
    let duration = if args.is_empty() {
        5 // Default duration in seconds
    } else {
        args[0].parse::<u64>().map_err(|e| format!("Invalid duration: {}", e))?
    };
    
    // Create a task name
    let task_name = format!("Async Task ({}s)", duration);
    
    // Create and run an asynchronous task
    let task_id = create_task(&task_name, &format!("This task runs for {} seconds", duration), move || {
        // This code runs in a background thread
        
        // Calculate the sleep interval for updating progress
        let total_millis = duration * 1000;
        let steps = 100;
        let interval = total_millis / steps;
        
        // Simulate a long-running operation
        for i in 0..=steps {
            // Update progress
            update_task_progress(i as u8);
            
            // Sleep for a bit
            std::thread::sleep(std::time::Duration::from_millis(interval));
            
            // Check if the task was cancelled
            if is_task_cancelled() {
                return Err("Task was cancelled".into());
            }
        }
        
        // Return the result
        Ok(format!("Task completed after {} seconds", duration).into_bytes())
    }, Some(Box::new(|result| {
        // This callback is called when the task completes
        match result {
            TaskResult::Success(data) => {
                // Handle successful completion
                let message = String::from_utf8_lossy(&data);
                editor_message(&format!("Task completed: {}", message));
            }
            TaskResult::Error(err) => {
                // Handle error
                editor_message(&format!("Task failed: {}", err));
            }
            TaskResult::Cancelled => {
                // Handle cancellation
                editor_message("Task was cancelled");
            }
        }
    })));
    
    // Display the task ID
    editor_message(&format!("Started task with ID: {}", task_id));
    
    Ok(())
}

// Command handler for :async_status
fn async_status_command(args: &[&str]) -> Result<(), String> {
    if args.is_empty() {
        // List all tasks
        let tasks = list_tasks();
        
        if tasks.is_empty() {
            editor_message("No tasks found");
            return Ok(());
        }
        
        // Display task information
        editor_message("Task Status:");
        for task in tasks {
            let status_str = match task.status {
                TaskStatus::Pending => "Pending",
                TaskStatus::Running => "Running",
                TaskStatus::Completed => "Completed",
                TaskStatus::Failed => "Failed",
                TaskStatus::Cancelled => "Cancelled",
            };
            
            editor_message(&format!("  {} - {} ({}): {}%, Status: {}", 
                task.id, task.name, task.description, task.progress, status_str));
        }
    } else {
        // Get status of a specific task
        let task_id = args[0];
        
        if let Some(task) = get_task_info(task_id) {
            let status_str = match task.status {
                TaskStatus::Pending => "Pending",
                TaskStatus::Running => "Running",
                TaskStatus::Completed => "Completed",
                TaskStatus::Failed => "Failed",
                TaskStatus::Cancelled => "Cancelled",
            };
            
            editor_message(&format!("Task {} - {} ({})", task.id, task.name, task.description));
            editor_message(&format!("  Status: {}", status_str));
            editor_message(&format!("  Progress: {}%", task.progress));
            
            if let Some(result) = task.result {
                match result {
                    TaskResult::Success(data) => {
                        let message = String::from_utf8_lossy(&data);
                        editor_message(&format!("  Result: {}", message));
                    }
                    TaskResult::Error(err) => {
                        editor_message(&format!("  Error: {}", err));
                    }
                    TaskResult::Cancelled => {
                        editor_message("  Result: Task was cancelled");
                    }
                }
            }
        } else {
            editor_message(&format!("Task '{}' not found", task_id));
        }
    }
    
    Ok(())
}

// Command handler for :async_cancel
fn async_cancel_command(args: &[&str]) -> Result<(), String> {
    if args.is_empty() {
        return Err("Task ID required".into());
    }
    
    let task_id = args[0];
    
    // Cancel the task
    if cancel_task(task_id) {
        editor_message(&format!("Task '{}' cancelled", task_id));
    } else {
        editor_message(&format!("Failed to cancel task '{}'", task_id));
    }
    
    Ok(())
}

// Plugin API mock for the example
mod xvim_plugin_api {
    // Plugin registration
    pub fn register_plugin(name: &str, version: &str, description: &str, author: &str) {
        // This would be implemented by the xvim plugin API
    }
    
    // Command registration
    pub fn register_command(name: &str, handler: fn(&[&str]) -> Result<(), String>) {
        // This would be implemented by the xvim plugin API
    }
    
    // Display a message in the editor
    pub fn editor_message(message: &str) {
        // This would be implemented by the xvim plugin API
    }
    
    // Task status
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum TaskStatus {
        Pending,
        Running,
        Completed,
        Failed,
        Cancelled,
    }
    
    // Task result
    #[derive(Debug, Clone)]
    pub enum TaskResult {
        Success(Vec<u8>),
        Error(String),
        Cancelled,
    }
    
    // Task information
    #[derive(Debug, Clone)]
    pub struct TaskInfo {
        pub id: String,
        pub name: String,
        pub description: String,
        pub status: TaskStatus,
        pub progress: u8,
        pub result: Option<TaskResult>,
    }
    
    // Create a task
    pub fn create_task<F, T>(name: &str, description: &str, f: F, callback: Option<Box<dyn FnOnce(TaskResult) + Send + 'static>>) -> String
    where
        F: FnOnce() -> Result<T, String> + Send + 'static,
        T: Into<Vec<u8>> + Send + 'static,
    {
        // This would be implemented by the xvim plugin API
        "task_1".to_string()
    }
    
    // Update task progress
    pub fn update_task_progress(progress: u8) {
        // This would be implemented by the xvim plugin API
    }
    
    // Check if a task is cancelled
    pub fn is_task_cancelled() -> bool {
        // This would be implemented by the xvim plugin API
        false
    }
    
    // Get task information
    pub fn get_task_info(task_id: &str) -> Option<TaskInfo> {
        // This would be implemented by the xvim plugin API
        None
    }
    
    // List all tasks
    pub fn list_tasks() -> Vec<TaskInfo> {
        // This would be implemented by the xvim plugin API
        Vec::new()
    }
    
    // Cancel a task
    pub fn cancel_task(task_id: &str) -> bool {
        // This would be implemented by the xvim plugin API
        true
    }
}