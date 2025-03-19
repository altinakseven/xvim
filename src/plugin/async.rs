//! Asynchronous operation support for plugins
//!
//! This module provides support for asynchronous operations in plugins.
//! It allows plugins to perform long-running operations without blocking
//! the editor, and to receive notifications when operations complete.

use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::{Arc, Mutex};
use std::sync::Mutex;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::sync::Mutex;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::{Duration, Instant};
use std::sync::Mutex;
use std::sync::Arc;
use std::sync::Mutex;
use anyhow::{anyhow, Result};
use std::sync::Mutex;
use std::sync::Arc;
use std::sync::Mutex;

/// Task status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskStatus {
    /// Task is pending
    Pending,
    /// Task is running
    Running,
    /// Task is completed successfully
    Completed,
    /// Task failed
    Failed,
    /// Task was cancelled
    Cancelled,
}

/// Task result
#[derive(Debug, Clone)]
pub enum TaskResult {
    /// Task completed successfully with a result
    Success(Vec<u8>),
    /// Task failed with an error
    Error(String),
    /// Task was cancelled
    Cancelled,
}

/// Task information
#[derive(Debug, Clone)]
pub struct TaskInfo {
    /// Task ID
    id: String,
    /// Plugin name
    plugin_name: String,
    /// Task name
    name: String,
    /// Task description
    description: String,
    /// Task status
    status: TaskStatus,
    /// Task result
    result: Option<TaskResult>,
    /// Task creation time
    creation_time: Instant,
    /// Task start time
    start_time: Option<Instant>,
    /// Task completion time
    completion_time: Option<Instant>,
    /// Task progress (0-100)
    progress: u8,
}

impl TaskInfo {
    /// Create a new task info
    pub fn new(id: &str, plugin_name: &str, name: &str, description: &str) -> Self {
        Self {
            id: id.to_string(),
            plugin_name: plugin_name.to_string(),
            name: name.to_string(),
            description: description.to_string(),
            status: TaskStatus::Pending,
            result: None,
            creation_time: Instant::now(),
            start_time: None,
            completion_time: None,
            progress: 0,
        }
    }
    
    /// Get the task ID
    pub fn id(&self) -> &str {
        &self.id
    }
    
    /// Get the plugin name
    pub fn plugin_name(&self) -> &str {
        &self.plugin_name
    }
    
    /// Get the task name
    pub fn name(&self) -> &str {
        &self.name
    }
    
    /// Get the task description
    pub fn description(&self) -> &str {
        &self.description
    }
    
    /// Get the task status
    pub fn status(&self) -> TaskStatus {
        self.status
    }
    
    /// Get the task result
    pub fn result(&self) -> Option<&TaskResult> {
        self.result.as_ref()
    }
    
    /// Get the task creation time
    pub fn creation_time(&self) -> Instant {
        self.creation_time
    }
    
    /// Get the task start time
    pub fn start_time(&self) -> Option<Instant> {
        self.start_time
    }
    
    /// Get the task completion time
    pub fn completion_time(&self) -> Option<Instant> {
        self.completion_time
    }
    
    /// Get the task progress
    pub fn progress(&self) -> u8 {
        self.progress
    }
    
    /// Set the task status
    pub fn set_status(&mut self, status: TaskStatus) {
        self.status = status;
        
        // Update times based on status
        match status {
            TaskStatus::Running => {
                if self.start_time.is_none() {
                    self.start_time = Some(Instant::now());
                }
            }
            TaskStatus::Completed | TaskStatus::Failed | TaskStatus::Cancelled => {
                if self.completion_time.is_none() {
                    self.completion_time = Some(Instant::now());
                }
            }
            _ => {}
        }
    }
    
    /// Set the task result
    pub fn set_result(&mut self, result: TaskResult) {
        self.result = Some(result);
        
        // Update status based on result
        match &self.result {
            Some(TaskResult::Success(_)) => self.set_status(TaskStatus::Completed),
            Some(TaskResult::Error(_)) => self.set_status(TaskStatus::Failed),
            Some(TaskResult::Cancelled) => self.set_status(TaskStatus::Cancelled),
            None => {}
        }
    }
    
    /// Set the task progress
    pub fn set_progress(&mut self, progress: u8) {
        self.progress = progress.min(100);
    }
}

/// Task callback
pub type TaskCallback = Box<dyn FnOnce(TaskResult) + Send + 'static>;

/// Task manager
pub struct TaskManager {
    /// Tasks
    tasks: Arc<Mutex<HashMap<String, TaskInfo>>>,
    /// Next task ID
    next_task_id: u64,
}

impl TaskManager {
    /// Create a new task manager
    pub fn new() -> Self {
        Self {
            tasks: Arc::new(Mutex::new(HashMap::new())),
            next_task_id: 1,
        }
    }
    
    /// Generate a new task ID
    fn generate_task_id(&mut self) -> String {
        let id = format!("task_{}", self.next_task_id);
        self.next_task_id += 1;
        id
    }
    
    /// Create a new task
    pub fn create_task(&mut self, plugin_name: &str, name: &str, description: &str) -> String {
        // Generate a new task ID
        let id = self.generate_task_id();
        
        // Create task info
        let task_info = TaskInfo::new(&id, plugin_name, name, description);
        
        // Add the task to the map
        if let Ok(mut tasks) = self.tasks.lock() {
            tasks.insert(id.clone(), task_info);
        }
        
        id
    }
    
    /// Get task info
    pub fn get_task_info(&self, task_id: &str) -> Option<TaskInfo> {
        if let Ok(tasks) = self.tasks.lock() {
            tasks.get(task_id).cloned()
        } else {
            None
        }
    }
    
    /// List all tasks
    pub fn list_tasks(&self) -> Vec<TaskInfo> {
        if let Ok(tasks) = self.tasks.lock() {
            tasks.values().cloned().collect()
        } else {
            Vec::new()
        }
    }
    
    /// List tasks for a plugin
    pub fn list_plugin_tasks(&self, plugin_name: &str) -> Vec<TaskInfo> {
        if let Ok(tasks) = self.tasks.lock() {
            tasks.values()
                .filter(|task| task.plugin_name() == plugin_name)
                .cloned()
                .collect()
        } else {
            Vec::new()
        }
    }
    
    /// Update task status
    pub fn update_task_status(&self, task_id: &str, status: TaskStatus) -> Result<()> {
        if let Ok(mut tasks) = self.tasks.lock() {
            if let Some(task) = tasks.get_mut(task_id) {
                task.set_status(status);
                Ok(())
            } else {
                Err(anyhow!("Task '{}' not found", task_id))
            }
        } else {
            Err(anyhow!("Failed to lock tasks"))
        }
    }
    
    /// Update task progress
    pub fn update_task_progress(&self, task_id: &str, progress: u8) -> Result<()> {
        if let Ok(mut tasks) = self.tasks.lock() {
            if let Some(task) = tasks.get_mut(task_id) {
                task.set_progress(progress);
                Ok(())
            } else {
                Err(anyhow!("Task '{}' not found", task_id))
            }
        } else {
            Err(anyhow!("Failed to lock tasks"))
        }
    }
    
    /// Complete a task with a result
    pub fn complete_task(&self, task_id: &str, result: TaskResult) -> Result<()> {
        if let Ok(mut tasks) = self.tasks.lock() {
            if let Some(task) = tasks.get_mut(task_id) {
                task.set_result(result);
                Ok(())
            } else {
                Err(anyhow!("Task '{}' not found", task_id))
            }
        } else {
            Err(anyhow!("Failed to lock tasks"))
        }
    }
    
    /// Cancel a task
    pub fn cancel_task(&self, task_id: &str) -> Result<()> {
        if let Ok(mut tasks) = self.tasks.lock() {
            if let Some(task) = tasks.get_mut(task_id) {
                task.set_result(TaskResult::Cancelled);
                Ok(())
            } else {
                Err(anyhow!("Task '{}' not found", task_id))
            }
        } else {
            Err(anyhow!("Failed to lock tasks"))
        }
    }
    
    /// Run a task asynchronously
    pub fn run_task<F, T>(&mut self, plugin_name: &str, name: &str, description: &str, f: F, callback: Option<TaskCallback>) -> String
    where
        F: FnOnce() -> Result<T> + Send + 'static,
        T: Into<Vec<u8>> + Send + 'static,
    {
        // Create a new task
        let task_id = self.create_task(plugin_name, name, description);
        
        // Clone the task ID for the thread
        let task_id_clone = task_id.clone();
        
        // Clone the tasks map for the thread
        let tasks = self.tasks.clone();
        
        // Spawn a new thread to run the task
        thread::spawn(move || {
            // Update task status to running
            if let Ok(mut tasks) = tasks.lock() {
                if let Some(task) = tasks.get_mut(&task_id_clone) {
                    task.set_status(TaskStatus::Running);
                }
            }
            
            // Run the task
            let result = match f() {
                Ok(value) => TaskResult::Success(value.into()),
                Err(err) => TaskResult::Error(err.to_string()),
            };
            
            // Update task result
            if let Ok(mut tasks) = tasks.lock() {
                if let Some(task) = tasks.get_mut(&task_id_clone) {
                    task.set_result(result.clone());
                }
            }
            
            // Call the callback if provided
            if let Some(callback) = callback {
                callback(result);
            }
        });
        
        task_id
    }
    
    /// Clean up completed tasks
    pub fn cleanup_tasks(&mut self, max_age: Duration) {
        if let Ok(mut tasks) = self.tasks.lock() {
            let now = Instant::now();
            
            // Remove completed tasks that are older than max_age
            tasks.retain(|_, task| {
                if let Some(completion_time) = task.completion_time() {
                    now.duration_since(completion_time) < max_age
                } else {
                    true
                }
            });
        }
    }
}

impl Default for TaskManager {
    fn default() -> Self {
        Self::new()
    }
}