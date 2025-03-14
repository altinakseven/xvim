//! Plugin debugging support
//!
//! This module provides enhanced debugging support for plugins.
//! It allows developers to debug their plugins more effectively
//! by providing tools for logging, tracing, and inspecting plugin state.

use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

/// Log level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LogLevel {
    /// Trace level (most verbose)
    Trace,
    /// Debug level
    Debug,
    /// Info level
    Info,
    /// Warning level
    Warn,
    /// Error level
    Error,
    /// Fatal level (least verbose)
    Fatal,
}

impl LogLevel {
    /// Convert to string
    pub fn to_string(&self) -> &'static str {
        match self {
            LogLevel::Trace => "TRACE",
            LogLevel::Debug => "DEBUG",
            LogLevel::Info => "INFO",
            LogLevel::Warn => "WARN",
            LogLevel::Error => "ERROR",
            LogLevel::Fatal => "FATAL",
        }
    }
    
    /// Convert from string
    pub fn from_string(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "TRACE" => Some(LogLevel::Trace),
            "DEBUG" => Some(LogLevel::Debug),
            "INFO" => Some(LogLevel::Info),
            "WARN" => Some(LogLevel::Warn),
            "ERROR" => Some(LogLevel::Error),
            "FATAL" => Some(LogLevel::Fatal),
            _ => None,
        }
    }
    
    /// Check if this level is at least as severe as another level
    pub fn is_at_least(&self, other: LogLevel) -> bool {
        self.to_numeric() >= other.to_numeric()
    }
    
    /// Convert to numeric value (higher is more severe)
    fn to_numeric(&self) -> u8 {
        match self {
            LogLevel::Trace => 0,
            LogLevel::Debug => 1,
            LogLevel::Info => 2,
            LogLevel::Warn => 3,
            LogLevel::Error => 4,
            LogLevel::Fatal => 5,
        }
    }
}

/// Log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    /// Timestamp
    pub timestamp: u64,
    /// Plugin name
    pub plugin_name: String,
    /// Log level
    pub level: LogLevel,
    /// Message
    pub message: String,
    /// Additional data
    pub data: Option<String>,
}

impl LogEntry {
    /// Create a new log entry
    pub fn new(plugin_name: &str, level: LogLevel, message: &str, data: Option<&str>) -> Self {
        Self {
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_else(|_| Duration::from_secs(0))
                .as_secs(),
            plugin_name: plugin_name.to_string(),
            level,
            message: message.to_string(),
            data: data.map(|s| s.to_string()),
        }
    }
    
    /// Format as a string
    pub fn format(&self) -> String {
        let timestamp = chrono::DateTime::<chrono::Utc>::from_timestamp(self.timestamp as i64, 0)
            .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
            .unwrap_or_else(|| self.timestamp.to_string());
        
        let data_str = if let Some(data) = &self.data {
            format!(" - {}", data)
        } else {
            String::new()
        };
        
        format!("[{}] [{}] [{}] {}{}", 
            timestamp, 
            self.plugin_name, 
            self.level.to_string(), 
            self.message,
            data_str)
    }
}

/// Performance trace entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceEntry {
    /// Trace ID
    pub id: String,
    /// Plugin name
    pub plugin_name: String,
    /// Operation name
    pub operation: String,
    /// Start time
    pub start_time: u64,
    /// End time
    pub end_time: Option<u64>,
    /// Duration in milliseconds
    pub duration_ms: Option<u64>,
    /// Additional data
    pub data: Option<String>,
}

impl TraceEntry {
    /// Create a new trace entry
    pub fn new(id: &str, plugin_name: &str, operation: &str, data: Option<&str>) -> Self {
        Self {
            id: id.to_string(),
            plugin_name: plugin_name.to_string(),
            operation: operation.to_string(),
            start_time: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_else(|_| Duration::from_secs(0))
                .as_secs(),
            end_time: None,
            duration_ms: None,
            data: data.map(|s| s.to_string()),
        }
    }
    
    /// Complete the trace entry
    pub fn complete(&mut self) {
        let end_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_else(|_| Duration::from_secs(0))
            .as_secs();
        
        self.end_time = Some(end_time);
        self.duration_ms = Some((end_time - self.start_time) * 1000);
    }
    
    /// Format as a string
    pub fn format(&self) -> String {
        let start_time = chrono::DateTime::<chrono::Utc>::from_timestamp(self.start_time as i64, 0)
            .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
            .unwrap_or_else(|| self.start_time.to_string());
        
        let duration_str = if let Some(duration_ms) = self.duration_ms {
            format!(" - Duration: {}ms", duration_ms)
        } else {
            String::new()
        };
        
        let data_str = if let Some(data) = &self.data {
            format!(" - {}", data)
        } else {
            String::new()
        };
        
        format!("[{}] [{}] [TRACE] {} - {}{}{}", 
            start_time, 
            self.plugin_name, 
            self.id,
            self.operation,
            duration_str,
            data_str)
    }
}

/// Plugin state snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginStateSnapshot {
    /// Plugin name
    pub plugin_name: String,
    /// Timestamp
    pub timestamp: u64,
    /// Memory usage in bytes
    pub memory_usage: u64,
    /// Number of active tasks
    pub active_tasks: u32,
    /// Custom state variables
    pub variables: HashMap<String, String>,
}

impl PluginStateSnapshot {
    /// Create a new plugin state snapshot
    pub fn new(plugin_name: &str) -> Self {
        Self {
            plugin_name: plugin_name.to_string(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_else(|_| Duration::from_secs(0))
                .as_secs(),
            memory_usage: 0,
            active_tasks: 0,
            variables: HashMap::new(),
        }
    }
    
    /// Format as a string
    pub fn format(&self) -> String {
        let timestamp = chrono::DateTime::<chrono::Utc>::from_timestamp(self.timestamp as i64, 0)
            .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
            .unwrap_or_else(|| self.timestamp.to_string());
        
        let mut result = format!("[{}] [{}] [STATE] Memory: {} bytes, Active Tasks: {}\n", 
            timestamp, 
            self.plugin_name, 
            self.memory_usage,
            self.active_tasks);
        
        if !self.variables.is_empty() {
            result.push_str("Variables:\n");
            for (key, value) in &self.variables {
                result.push_str(&format!("  {}: {}\n", key, value));
            }
        }
        
        result
    }
}

/// Debug configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugConfig {
    /// Enable debugging
    pub enabled: bool,
    /// Log level
    pub log_level: LogLevel,
    /// Log file path
    pub log_file: Option<String>,
    /// Enable performance tracing
    pub enable_tracing: bool,
    /// Enable state snapshots
    pub enable_snapshots: bool,
    /// Maximum log file size in bytes
    pub max_log_size: u64,
    /// Maximum number of log files to keep
    pub max_log_files: u32,
}

impl DebugConfig {
    /// Create a new debug configuration
    pub fn new() -> Self {
        Self {
            enabled: false,
            log_level: LogLevel::Info,
            log_file: None,
            enable_tracing: false,
            enable_snapshots: false,
            max_log_size: 10 * 1024 * 1024, // 10 MB
            max_log_files: 5,
        }
    }
    
    /// Enable debugging
    pub fn enable(&mut self) {
        self.enabled = true;
    }
    
    /// Disable debugging
    pub fn disable(&mut self) {
        self.enabled = false;
    }
    
    /// Set the log level
    pub fn set_log_level(&mut self, level: LogLevel) {
        self.log_level = level;
    }
    
    /// Set the log file path
    pub fn set_log_file<P: AsRef<Path>>(&mut self, path: Option<P>) {
        self.log_file = path.map(|p| p.as_ref().to_string_lossy().to_string());
    }
    
    /// Enable performance tracing
    pub fn enable_tracing(&mut self) {
        self.enable_tracing = true;
    }
    
    /// Disable performance tracing
    pub fn disable_tracing(&mut self) {
        self.enable_tracing = false;
    }
    
    /// Enable state snapshots
    pub fn enable_snapshots(&mut self) {
        self.enable_snapshots = true;
    }
    
    /// Disable state snapshots
    pub fn disable_snapshots(&mut self) {
        self.enable_snapshots = false;
    }
    
    /// Set the maximum log file size
    pub fn set_max_log_size(&mut self, size: u64) {
        self.max_log_size = size;
    }
    
    /// Set the maximum number of log files
    pub fn set_max_log_files(&mut self, count: u32) {
        self.max_log_files = count;
    }
    
    /// Load from a file
    pub fn load_from_file<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        let content = std::fs::read_to_string(path)?;
        let config: DebugConfig = serde_json::from_str(&content)?;
        *self = config;
        Ok(())
    }
    
    /// Save to a file
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }
}

impl Default for DebugConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Active trace
struct ActiveTrace {
    /// Trace entry
    entry: TraceEntry,
    /// Start time
    start_time: Instant,
}

/// Debug manager
pub struct DebugManager {
    /// Debug configuration
    config: DebugConfig,
    /// Log file
    log_file: Option<File>,
    /// Log entries
    logs: Vec<LogEntry>,
    /// Active traces
    active_traces: HashMap<String, ActiveTrace>,
    /// Completed traces
    completed_traces: Vec<TraceEntry>,
    /// Plugin state snapshots
    snapshots: HashMap<String, Vec<PluginStateSnapshot>>,
    /// Log directory
    log_dir: PathBuf,
}

impl DebugManager {
    /// Create a new debug manager
    pub fn new() -> Self {
        Self {
            config: DebugConfig::new(),
            log_file: None,
            logs: Vec::new(),
            active_traces: HashMap::new(),
            completed_traces: Vec::new(),
            snapshots: HashMap::new(),
            log_dir: PathBuf::from("logs"),
        }
    }
    
    /// Initialize the debug manager
    pub fn init(&mut self) -> Result<()> {
        // Create the log directory if it doesn't exist
        if !self.log_dir.exists() {
            std::fs::create_dir_all(&self.log_dir)?;
        }
        
        // Open the log file if configured
        if let Some(log_file) = &self.config.log_file {
            let path = Path::new(log_file);
            let file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(path)?;
            
            self.log_file = Some(file);
        }
        
        Ok(())
    }
    
    /// Set the debug configuration
    pub fn set_config(&mut self, config: DebugConfig) -> Result<()> {
        self.config = config;
        
        // Re-initialize with the new configuration
        self.init()
    }
    
    /// Get the debug configuration
    pub fn config(&self) -> &DebugConfig {
        &self.config
    }
    
    /// Get a mutable reference to the debug configuration
    pub fn config_mut(&mut self) -> &mut DebugConfig {
        &mut self.config
    }
    
    /// Set the log directory
    pub fn set_log_dir<P: AsRef<Path>>(&mut self, path: P) {
        self.log_dir = path.as_ref().to_path_buf();
    }
    
    /// Log a message
    pub fn log(&mut self, plugin_name: &str, level: LogLevel, message: &str, data: Option<&str>) -> Result<()> {
        // Check if debugging is enabled
        if !self.config.enabled {
            return Ok(());
        }
        
        // Check if the log level is sufficient
        if !level.is_at_least(self.config.log_level) {
            return Ok(());
        }
        
        // Create a log entry
        let entry = LogEntry::new(plugin_name, level, message, data);
        
        // Add to the in-memory log
        self.logs.push(entry.clone());
        
        // Write to the log file if available
        if let Some(file) = &mut self.log_file {
            writeln!(file, "{}", entry.format())?;
            file.flush()?;
            
            // Check if we need to rotate the log file
            if let Ok(metadata) = file.metadata() {
                if metadata.len() > self.config.max_log_size {
                    self.rotate_logs()?;
                }
            }
        }
        
        Ok(())
    }
    
    /// Start a performance trace
    pub fn start_trace(&mut self, plugin_name: &str, operation: &str, data: Option<&str>) -> Result<String> {
        // Check if debugging and tracing are enabled
        if !self.config.enabled || !self.config.enable_tracing {
            return Ok(String::new());
        }
        
        // Generate a trace ID
        let id = format!("trace_{}", uuid::Uuid::new_v4());
        
        // Create a trace entry
        let entry = TraceEntry::new(&id, plugin_name, operation, data);
        
        // Add to active traces
        self.active_traces.insert(id.clone(), ActiveTrace {
            entry,
            start_time: Instant::now(),
        });
        
        Ok(id)
    }
    
    /// End a performance trace
    pub fn end_trace(&mut self, trace_id: &str) -> Result<()> {
        // Check if debugging and tracing are enabled
        if !self.config.enabled || !self.config.enable_tracing {
            return Ok(());
        }
        
        // Check if the trace exists
        if let Some(mut active_trace) = self.active_traces.remove(trace_id) {
            // Complete the trace
            active_trace.entry.complete();
            
            // Add to completed traces
            self.completed_traces.push(active_trace.entry.clone());
            
            // Log the trace
            self.log(
                &active_trace.entry.plugin_name,
                LogLevel::Debug,
                &format!("Trace completed: {} - {}", trace_id, active_trace.entry.operation),
                active_trace.entry.duration_ms.map(|d| d.to_string()).as_deref(),
            )?;
        }
        
        Ok(())
    }
    
    /// Take a plugin state snapshot
    pub fn take_snapshot(&mut self, plugin_name: &str, memory_usage: u64, active_tasks: u32, variables: HashMap<String, String>) -> Result<()> {
        // Check if debugging and snapshots are enabled
        if !self.config.enabled || !self.config.enable_snapshots {
            return Ok(());
        }
        
        // Create a snapshot
        let mut snapshot = PluginStateSnapshot::new(plugin_name);
        snapshot.memory_usage = memory_usage;
        snapshot.active_tasks = active_tasks;
        snapshot.variables = variables;
        
        // Add to snapshots
        let snapshots = self.snapshots.entry(plugin_name.to_string()).or_insert_with(Vec::new);
        snapshots.push(snapshot.clone());
        
        // Log the snapshot
        self.log(
            plugin_name,
            LogLevel::Debug,
            &format!("State snapshot taken: Memory: {} bytes, Active Tasks: {}", memory_usage, active_tasks),
            None,
        )?;
        
        Ok(())
    }
    
    /// Get logs for a plugin
    pub fn get_logs(&self, plugin_name: &str) -> Vec<LogEntry> {
        self.logs.iter()
            .filter(|entry| entry.plugin_name == plugin_name)
            .cloned()
            .collect()
    }
    
    /// Get traces for a plugin
    pub fn get_traces(&self, plugin_name: &str) -> Vec<TraceEntry> {
        self.completed_traces.iter()
            .filter(|entry| entry.plugin_name == plugin_name)
            .cloned()
            .collect()
    }
    
    /// Get snapshots for a plugin
    pub fn get_snapshots(&self, plugin_name: &str) -> Vec<PluginStateSnapshot> {
        self.snapshots.get(plugin_name)
            .map(|snapshots| snapshots.clone())
            .unwrap_or_else(Vec::new)
    }
    
    /// Clear logs for a plugin
    pub fn clear_logs(&mut self, plugin_name: &str) {
        self.logs.retain(|entry| entry.plugin_name != plugin_name);
    }
    
    /// Clear traces for a plugin
    pub fn clear_traces(&mut self, plugin_name: &str) {
        self.completed_traces.retain(|entry| entry.plugin_name != plugin_name);
    }
    
    /// Clear snapshots for a plugin
    pub fn clear_snapshots(&mut self, plugin_name: &str) {
        self.snapshots.remove(plugin_name);
    }
    
    /// Export logs to a file
    pub fn export_logs<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let content = serde_json::to_string_pretty(&self.logs)?;
        std::fs::write(path, content)?;
        Ok(())
    }
    
    /// Export traces to a file
    pub fn export_traces<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let content = serde_json::to_string_pretty(&self.completed_traces)?;
        std::fs::write(path, content)?;
        Ok(())
    }
    
    /// Export snapshots to a file
    pub fn export_snapshots<P: AsRef<Path>>(&self, plugin_name: &str, path: P) -> Result<()> {
        let snapshots = self.get_snapshots(plugin_name);
        let content = serde_json::to_string_pretty(&snapshots)?;
        std::fs::write(path, content)?;
        Ok(())
    }
    
    /// Rotate log files
    fn rotate_logs(&mut self) -> Result<()> {
        // Close the current log file
        self.log_file = None;
        
        // Get the log file path
        let log_file = self.config.log_file.as_ref()
            .ok_or_else(|| anyhow!("No log file configured"))?;
        
        // Rotate the log files
        for i in (1..self.config.max_log_files).rev() {
            let src = format!("{}.{}", log_file, i);
            let dst = format!("{}.{}", log_file, i + 1);
            
            if Path::new(&src).exists() {
                std::fs::rename(&src, &dst)?;
            }
        }
        
        // Rename the current log file
        let backup = format!("{}.1", log_file);
        if Path::new(log_file).exists() {
            std::fs::rename(log_file, &backup)?;
        }
        
        // Open a new log file
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_file)?;
        
        self.log_file = Some(file);
        
        Ok(())
    }
}

impl Default for DebugManager {
    fn default() -> Self {
        Self::new()
    }
}