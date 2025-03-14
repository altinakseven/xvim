//! Asynchronous task management for plugins
//!
//! This module provides functionality for managing asynchronous tasks
//! in plugins. It allows plugins to run operations in the background
//! without blocking the main thread.

use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use anyhow::{anyhow, Result};

/// Task status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskStatus {
    /// Task is pending execution
    Pending,
    /// Task is running
    Running,
    /// Task has completed successfully
    Completed,
    /// Task has failed
    Failed,
    /// Task has been cancelled
    Cancelled,
}

/// Task progress
#[derive(Debug, Clone)]
pub struct TaskProgress {
    /// Current progress value (0-100)
    pub value: u8,
    /// Optional message describing the current progress
    pub message: Option<String>,
}

impl TaskProgress {
    /// Create a new task progress
    pub fn new(value: u8, message: Option<String>) -> Self {
        Self {
            value: value.min(100),
            message,
        }
    }
    
    /// Create a new task progress with a value only
    pub fn with_value(value: u8) -> Self {
        Self::new(value, None)
    }
    
    /// Create a new task progress with a message only
    pub fn with_message(message: &str) -> Self {
        Self::new(0, Some(message.to_string()))
    }
}

/// Task result
#[derive(Debug, Clone)]
pub enum TaskResult {
    /// Task completed successfully with a result
    Success(String),
    /// Task failed with an error
    Error(String),
    /// Task was cancelled
    Cancelled,
}

/// Task information
#[derive(Debug, Clone)]
pub struct TaskInfo {
    /// Task ID
    pub id: String,
    /// Task name
    pub name: String,
    /// Task description
    pub description: Option<String>,
    /// Task status
    pub status: TaskStatus,
    /// Task progress
    pub progress: TaskProgress,
    /// Task result
    pub result: Option<TaskResult>,
    /// Task creation time
    pub created_at: Instant,
    /// Task start time
    pub started_at: Option<Instant>,
    /// Task completion time
    pub completed_at: Option<Instant>,
}

/// Task callback
pub type TaskCallback = Box<dyn FnOnce(TaskResult) + Send + 'static>;

/// Task manager
pub struct TaskManager {
    /// Tasks
    tasks: HashMap<String, TaskInfo>,
    /// Task queue
    queue: VecDeque<String>,
    /// Task callbacks
    callbacks: HashMap<String, TaskCallback>,
    /// Next task ID
    next_id: u64,
}

impl TaskManager {
    /// Create a new task manager
    pub fn new() -> Self {
        Self {
            tasks: HashMap::new(),
            queue: VecDeque::new(),
            callbacks: HashMap::new(),
            next_id: 1,
        }
    }
    
    /// Create a new task
    pub fn create_task(&mut self, name: &str, description: Option<&str>) -> String {
        // Generate a task ID
        let id = format!("task_{}", self.next_id);
        self.next_id += 1;
        
        // Create task info
        let task = TaskInfo {
            id: id.clone(),
            name: name.to_string(),
            description: description.map(|s| s.to_string()),
            status: TaskStatus::Pending,
            progress: TaskProgress::with_value(0),
            result: None,
            created_at: Instant::now(),
            started_at: None,
            completed_at: None,
        };
        
        // Add the task to the map and queue
        self.tasks.insert(id.clone(), task);
        self.queue.push_back(id.clone());
        
        id
    }
    
    /// Get a task by ID
    pub fn get_task(&self, id: &str) -> Option<&TaskInfo> {
        self.tasks.get(id)
    }
    
    /// Get all tasks
    pub fn get_tasks(&self) -> Vec<&TaskInfo> {
        self.tasks.values().collect()
    }
    
    /// Start a task
    pub fn start_task(&mut self, id: &str) -> Result<()> {
        // Get the task
        let task = self.tasks.get_mut(id).ok_or_else(|| anyhow!("Task not found"))?;
        
        // Check if the task is pending
        if task.status != TaskStatus::Pending {
            return Err(anyhow!("Task is not pending"));
        }
        
        // Update the task status
        task.status = TaskStatus::Running;
        task.started_at = Some(Instant::now());
        
        Ok(())
    }
    
    /// Update task progress
    pub fn update_progress(&mut self, id: &str, progress: TaskProgress) -> Result<()> {
        // Get the task
        let task = self.tasks.get_mut(id).ok_or_else(|| anyhow!("Task not found"))?;
        
        // Check if the task is running
        if task.status != TaskStatus::Running {
            return Err(anyhow!("Task is not running"));
        }
        
        // Update the task progress
        task.progress = progress;
        
        Ok(())
    }
    
    /// Complete a task
    pub fn complete_task(&mut self, id: &str, result: TaskResult) -> Result<()> {
        // Get the task
        let task = self.tasks.get_mut(id).ok_or_else(|| anyhow!("Task not found"))?;
        
        // Check if the task is running
        if task.status != TaskStatus::Running {
            return Err(anyhow!("Task is not running"));
        }
        
        // Update the task status
        match result {
            TaskResult::Success(_) => task.status = TaskStatus::Completed,
            TaskResult::Error(_) => task.status = TaskStatus::Failed,
            TaskResult::Cancelled => task.status = TaskStatus::Cancelled,
        }
        
        task.result = Some(result.clone());
        task.completed_at = Some(Instant::now());
        
        // Call the callback if one exists
        if let Some(callback) = self.callbacks.remove(id) {
            callback(result);
        }
        
        Ok(())
    }
    
    /// Cancel a task
    pub fn cancel_task(&mut self, id: &str) -> Result<()> {
        // Get the task
        let task = self.tasks.get_mut(id).ok_or_else(|| anyhow!("Task not found"))?;
        
        // Check if the task is pending or running
        if task.status != TaskStatus::Pending && task.status != TaskStatus::Running {
            return Err(anyhow!("Task is not pending or running"));
        }
        
        // Update the task status
        task.status = TaskStatus::Cancelled;
        task.result = Some(TaskResult::Cancelled);
        task.completed_at = Some(Instant::now());
        
        // Call the callback if one exists
        if let Some(callback) = self.callbacks.remove(id) {
            callback(TaskResult::Cancelled);
        }
        
        Ok(())
    }
    
    /// Set a callback for a task
    pub fn set_callback(&mut self, id: &str, callback: TaskCallback) -> Result<()> {
        // Check if the task exists
        if !self.tasks.contains_key(id) {
            return Err(anyhow!("Task not found"));
        }
        
        // Set the callback
        self.callbacks.insert(id.to_string(), callback);
        
        Ok(())
    }
    
    /// Clean up completed tasks
    pub fn cleanup(&mut self, max_age: Duration) {
        // Get the current time
        let now = Instant::now();
        
        // Remove completed tasks that are older than max_age
        self.tasks.retain(|_, task| {
            if let Some(completed_at) = task.completed_at {
                if now.duration_since(completed_at) > max_age {
                    return false;
                }
            }
            true
        });
    }
    
    /// Get the next task from the queue
    pub fn next_task(&mut self) -> Option<String> {
        self.queue.pop_front()
    }
    
    /// Add a task to the queue
    pub fn queue_task(&mut self, id: &str) -> Result<()> {
        // Check if the task exists
        if !self.tasks.contains_key(id) {
            return Err(anyhow!("Task not found"));
        }
        
        // Add the task to the queue
        self.queue.push_back(id.to_string());
        
        Ok(())
    }
    
    /// Get the number of tasks in the queue
    pub fn queue_size(&self) -> usize {
        self.queue.len()
    }
    
    /// Get the number of running tasks
    pub fn running_tasks(&self) -> usize {
        self.tasks.values().filter(|task| task.status == TaskStatus::Running).count()
    }
    
    /// Get the number of pending tasks
    pub fn pending_tasks(&self) -> usize {
        self.tasks.values().filter(|task| task.status == TaskStatus::Pending).count()
    }
    
    /// Get the number of completed tasks
    pub fn completed_tasks(&self) -> usize {
        self.tasks.values().filter(|task| task.status == TaskStatus::Completed).count()
    }
    
    /// Get the number of failed tasks
    pub fn failed_tasks(&self) -> usize {
        self.tasks.values().filter(|task| task.status == TaskStatus::Failed).count()
    }
    
    /// Get the number of cancelled tasks
    pub fn cancelled_tasks(&self) -> usize {
        self.tasks.values().filter(|task| task.status == TaskStatus::Cancelled).count()
    }
}

impl Default for TaskManager {
    fn default() -> Self {
        Self::new()
    }
}