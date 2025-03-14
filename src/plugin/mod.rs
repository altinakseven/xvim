//! Plugin module - WASM plugin system
//!
//! This module implements the WebAssembly-based plugin system for xvim.
//! It provides a way to extend the editor with plugins written in any
//! language that can compile to WebAssembly.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use anyhow::{anyhow, Result};
use crate::ui::TerminalUi;

// Re-export submodules
pub mod api;
pub mod r#async;
pub mod commands;
pub mod debug;
pub mod dependency;
pub mod events;
pub mod network;
pub mod ui;
pub mod wasm;

use api::PluginContext;
use r#async::TaskManager;
use debug::DebugManager;
use dependency::DependencyManager;
use events::EventManager;
use network::NetworkManager;
use ui::UiManager;
use wasm::WasmRuntime;

/// Plugin information
#[derive(Debug, Clone)]
pub struct PluginInfo {
    /// Plugin name
    pub name: String,
    /// Plugin version
    pub version: String,
    /// Plugin description
    pub description: String,
    /// Plugin author
    pub author: String,
    /// Plugin path
    pub path: PathBuf,
}

/// Plugin manager
pub struct PluginManager {
    /// WASM runtime
    runtime: WasmRuntime,
    /// Event manager
    event_manager: EventManager,
    /// UI manager
    ui_manager: Option<UiManager>,
    /// Network manager
    network_manager: NetworkManager,
    /// Task manager
    task_manager: TaskManager,
    /// Dependency manager
    dependency_manager: DependencyManager,
    /// Debug manager
    debug_manager: DebugManager,
    /// Plugin context
    context: Arc<Mutex<PluginContext>>,
    /// Plugin information
    plugins: HashMap<String, PluginInfo>,
    /// Plugin directory
    plugin_dir: PathBuf,
}

impl PluginManager {
    /// Create a new plugin manager
    pub fn new() -> Self {
        // Create a new WASM runtime
        let runtime = WasmRuntime::new();
        
        // Create a new event manager
        let event_manager = EventManager::new();
        
        // Create a new network manager
        let network_manager = NetworkManager::new();
        
        // Create a new task manager
        let task_manager = TaskManager::new();
        
        // Create a new dependency manager
        let dependency_manager = DependencyManager::new();
        
        // Create a new debug manager
        let debug_manager = DebugManager::new();
        
        // Create a shared plugin context
        let context = Arc::new(Mutex::new(PluginContext::new()));
        
        // Default plugin directory
        let plugin_dir = PathBuf::from("plugins");
        
        Self {
            runtime,
            event_manager,
            ui_manager: None,
            network_manager,
            task_manager,
            dependency_manager,
            debug_manager,
            context,
            plugins: HashMap::new(),
            plugin_dir,
        }
    }
    
    /// Set the terminal UI reference
    pub fn set_terminal_ui(&mut self, terminal: Arc<Mutex<TerminalUi>>) {
        // Create a new UI manager
        let ui_manager = UiManager::new(terminal);
        
        // Set the UI manager
        self.ui_manager = Some(ui_manager);
    }
    
    /// Initialize the plugin system
    pub fn init(&mut self) -> Result<()> {
        // Initialize the WASM runtime
        self.runtime.init()?;
        
        // Initialize the debug manager
        self.debug_manager.init()?;
        
        // Create the plugin directory if it doesn't exist
        if !self.plugin_dir.exists() {
            std::fs::create_dir_all(&self.plugin_dir)?;
        }
        
        Ok(())
    }
    
    /// Set the plugin directory
    pub fn set_plugin_dir<P: AsRef<Path>>(&mut self, path: P) {
        self.plugin_dir = path.as_ref().to_path_buf();
    }
    
    /// Get the plugin directory
    pub fn plugin_dir(&self) -> &Path {
        &self.plugin_dir
    }
    
    /// Load a plugin from a WASM file
    pub fn load_plugin<P: AsRef<Path>>(&mut self, path: P, name: &str) -> Result<()> {
        // Check if the plugin is already loaded
        if self.plugins.contains_key(name) {
            return Err(anyhow!("Plugin '{}' is already loaded", name));
        }
        
        // Get the absolute path
        let path = path.as_ref().to_path_buf();
        
        // Check if the file exists
        if !path.exists() {
            return Err(anyhow!("Plugin file '{}' does not exist", path.display()));
        }
        
        // Check for a dependency file
        let dep_path = path.with_extension("dep.json");
        if dep_path.exists() {
            // Load the dependency information
            self.dependency_manager.load_from_file(&dep_path)?;
            
            // Check if all dependencies are satisfied
            self.dependency_manager.check_dependencies()?;
        }
        
        // Log plugin loading
        self.debug_manager.log(
            "plugin_manager",
            debug::LogLevel::Info,
            &format!("Loading plugin: {}", name),
            Some(&format!("Path: {}", path.display())),
        )?;
        
        // Load the plugin into the WASM runtime
        self.runtime.load_plugin(&path, name)?;
        
        // Create plugin info
        let info = PluginInfo {
            name: name.to_string(),
            version: "0.1.0".to_string(), // TODO: Get from plugin metadata
            description: "".to_string(),  // TODO: Get from plugin metadata
            author: "".to_string(),       // TODO: Get from plugin metadata
            path,
        };
        
        // Add the plugin to the map
        self.plugins.insert(name.to_string(), info);
        
        // If there was no dependency file, register the plugin with default dependency info
        if !dep_path.exists() {
            let dep_info = dependency::PluginDependencyInfo {
                name: name.to_string(),
                version: "0.1.0".to_string(),
                dependencies: Vec::new(),
            };
            
            self.dependency_manager.register_plugin(dep_info)?;
        }
        
        Ok(())
    }
    
    /// Unload a plugin
    pub fn unload_plugin(&mut self, name: &str) -> Result<()> {
        // Check if the plugin is loaded
        if !self.plugins.contains_key(name) {
            return Err(anyhow!("Plugin '{}' is not loaded", name));
        }
        
        // Log plugin unloading
        self.debug_manager.log(
            "plugin_manager",
            debug::LogLevel::Info,
            &format!("Unloading plugin: {}", name),
            None,
        )?;
        
        // Unload the plugin from the WASM runtime
        self.runtime.unload_plugin(name)?;
        
        // Unregister the plugin from the dependency manager
        self.dependency_manager.unregister_plugin(name)?;
        
        // Remove the plugin from the map
        self.plugins.remove(name);
        
        Ok(())
    }
    
    /// List loaded plugins
    pub fn list_plugins(&self) -> Vec<PluginInfo> {
        self.plugins.values().cloned().collect()
    }
    
    /// Get the event manager
    pub fn event_manager(&mut self) -> &mut EventManager {
        &mut self.event_manager
    }
    
    /// Get the UI manager
    pub fn ui_manager(&mut self) -> Option<&mut UiManager> {
        self.ui_manager.as_mut()
    }
    
    /// Get the network manager
    pub fn network_manager(&mut self) -> &mut NetworkManager {
        &mut self.network_manager
    }
    
    /// Get the task manager
    pub fn task_manager(&mut self) -> &mut TaskManager {
        &mut self.task_manager
    }
    
    /// Get the dependency manager
    pub fn dependency_manager(&mut self) -> &mut DependencyManager {
        &mut self.dependency_manager
    }
    
    /// Get the debug manager
    pub fn debug_manager(&mut self) -> &mut DebugManager {
        &mut self.debug_manager
    }
    
    /// Get the plugin context
    pub fn context(&self) -> Arc<Mutex<PluginContext>> {
        self.context.clone()
    }
    
    /// Call a function in a plugin
    pub fn call_function(&mut self, plugin_name: &str, function_name: &str, args: &[u8]) -> Result<Vec<u8>> {
        // Start a trace for the function call
        let trace_id = self.debug_manager.start_trace(
            plugin_name,
            &format!("call_function: {}", function_name),
            Some(&format!("args_size: {}", args.len())),
        )?;
        
        // Call the function
        let result = self.runtime.call_function(plugin_name, function_name, args);
        
        // End the trace
        self.debug_manager.end_trace(&trace_id)?;
        
        // Log the result
        match &result {
            Ok(data) => {
                self.debug_manager.log(
                    plugin_name,
                    debug::LogLevel::Debug,
                    &format!("Function call succeeded: {}", function_name),
                    Some(&format!("result_size: {}", data.len())),
                )?;
            }
            Err(err) => {
                self.debug_manager.log(
                    plugin_name,
                    debug::LogLevel::Error,
                    &format!("Function call failed: {}", function_name),
                    Some(&format!("error: {}", err)),
                )?;
            }
        }
        
        result
    }
    
    /// Execute a command in a plugin
    pub fn execute_command(&mut self, command: &str, args: &[&str]) -> Result<()> {
        // Add debug output
        println!("DEBUG: plugin_manager.execute_command called with command: {}, args: {:?}", command, args);
        
        // Log the command execution
        self.debug_manager.log(
            "plugin_manager",
            debug::LogLevel::Info,
            &format!("Executing command: {}", command),
            Some(&format!("args: {:?}", args)),
        )?;
        
        // Start a trace for the command execution
        let trace_id = self.debug_manager.start_trace(
            "plugin_manager",
            &format!("execute_command: {}", command),
            None,
        )?;
        
        // Find the plugin that registered this command
        // For now, we'll just try to call the command handler in each plugin
        // In a real implementation, we would have a registry of commands
        let mut success = false;
        
        println!("DEBUG: Searching for plugin to handle command: {}", command);
        for plugin_name in self.plugins.keys() {
            println!("DEBUG: Trying plugin: {} for command: {}", plugin_name, command);
            
            // Try to call the command handler in this plugin
            let result = self.runtime.call_command(plugin_name, command, args);
            
            match &result {
                Ok(handled) => {
                    println!("DEBUG: Plugin {} returned handled={} for command: {}", plugin_name, handled, command);
                },
                Err(err) => {
                    println!("DEBUG: Plugin {} returned error for command {}: {}", plugin_name, command, err);
                }
            }
            
            if let Ok(true) = result {
                // The plugin handled the command
                success = true;
                println!("DEBUG: Plugin {} successfully handled command: {}", plugin_name, command);
                break;
            }
        }
        
        // End the trace
        self.debug_manager.end_trace(&trace_id)?;
        
        if success {
            println!("DEBUG: Command {} executed successfully", command);
            Ok(())
        } else {
            println!("DEBUG: Command {} not found", command);
            Err(anyhow!("Command not found: {}", command))
        }
    }
    
    /// Send an event to all plugins
    pub fn send_event(&mut self, event: &events::EventType) -> Result<()> {
        // Get the event type as a string
        let event_type = match event {
            events::EventType::BufferCreated(_) => "buffer_created",
            events::EventType::BufferDeleted(_) => "buffer_deleted",
            events::EventType::BufferChanged(_) => "buffer_changed",
            events::EventType::ModeChanged(_) => "mode_changed",
            events::EventType::CursorMoved(_, _, _) => "cursor_moved",
            events::EventType::CommandExecuted(_) => "command_executed",
            events::EventType::Custom(name, _) => name,
        };
        
        // Log the event
        self.debug_manager.log(
            "plugin_manager",
            debug::LogLevel::Debug,
            &format!("Sending event: {}", event_type),
            None,
        )?;
        
        // Start a trace for the event dispatch
        let trace_id = self.debug_manager.start_trace(
            "plugin_manager",
            &format!("send_event: {}", event_type),
            None,
        )?;
        
        // Dispatch the event to all registered handlers
        self.event_manager.dispatch_event(event_type, event);
        
        // Send the event to all plugins
        self.runtime.send_event(event)?;
        
        // End the trace
        self.debug_manager.end_trace(&trace_id)?;
        
        Ok(())
    }
}

impl Default for PluginManager {
    fn default() -> Self {
        Self::new()
    }
}