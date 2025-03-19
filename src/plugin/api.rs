//! Plugin API for the xvim editor
//!
//! This module defines the API that plugins can use to interact with the editor.
//! It provides access to editor functionality in a controlled and sandboxed way.

use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::{Arc, Mutex};
use std::sync::Mutex;
use std::sync::Arc;
use std::sync::Mutex;
use crate::buffer::BufferManager;
use std::sync::Mutex;
use std::sync::Arc;
use std::sync::Mutex;
use crate::command::ExCommandRegistry;
use std::sync::Mutex;
use std::sync::Arc;
use std::sync::Mutex;
use crate::mode::ModeManager;
use std::sync::Mutex;
use std::sync::Arc;
use std::sync::Mutex;

/// Plugin context
///
/// This struct provides access to editor functionality for plugins.
/// It is shared between the editor and all plugins.
pub struct PluginContext {
    /// Buffer manager reference
    buffer_manager: Option<Arc<Mutex<BufferManager>>>,
    /// Mode manager reference
    mode_manager: Option<Arc<Mutex<ModeManager>>>,
    /// Command registry reference
    command_registry: Option<Arc<Mutex<ExCommandRegistry>>>,
    /// Terminal UI reference
    terminal_ui: Option<Arc<Mutex<crate::ui::TerminalUi>>>,
    /// Custom data storage for plugins
    custom_data: HashMap<String, Vec<u8>>,
}

impl PluginContext {
    /// Create a new plugin context
    pub fn new() -> Self {
        Self {
            buffer_manager: None,
            mode_manager: None,
            command_registry: None,
            terminal_ui: None,
            custom_data: HashMap::new(),
        }
    }
    
    /// Set the buffer manager reference
    pub fn set_buffer_manager(&mut self, buffer_manager: Arc<Mutex<BufferManager>>) {
        self.buffer_manager = Some(buffer_manager);
    }
    
    /// Set the mode manager reference
    pub fn set_mode_manager(&mut self, mode_manager: Arc<Mutex<ModeManager>>) {
        self.mode_manager = Some(mode_manager);
    }
    
    /// Set the command registry reference
    pub fn set_command_registry(&mut self, command_registry: Arc<Mutex<ExCommandRegistry>>) {
        self.command_registry = Some(command_registry);
    }
    
    /// Set the terminal UI reference
    pub fn set_terminal_ui(&mut self, terminal_ui: Arc<Mutex<crate::ui::TerminalUi>>) {
        self.terminal_ui = Some(terminal_ui);
    }
    
    /// Get the buffer manager reference
    pub fn buffer_manager(&self) -> Option<Arc<Mutex<BufferManager>>> {
        self.buffer_manager.clone()
    }
    
    /// Get the mode manager reference
    pub fn mode_manager(&self) -> Option<Arc<Mutex<ModeManager>>> {
        self.mode_manager.clone()
    }
    
    /// Get the command registry reference
    pub fn command_registry(&self) -> Option<Arc<Mutex<ExCommandRegistry>>> {
        self.command_registry.clone()
    }
    
    /// Get the terminal UI reference
    pub fn terminal_ui(&self) -> Option<Arc<Mutex<crate::ui::TerminalUi>>> {
        self.terminal_ui.clone()
    }
    
    /// Set custom data
    pub fn set_custom_data(&mut self, key: &str, data: Vec<u8>) {
        self.custom_data.insert(key.to_string(), data);
    }
    
    /// Get custom data
    pub fn get_custom_data(&self, key: &str) -> Option<&Vec<u8>> {
        self.custom_data.get(key)
    }
    
    /// Remove custom data
    pub fn remove_custom_data(&mut self, key: &str) -> Option<Vec<u8>> {
        self.custom_data.remove(key)
    }
}

impl Default for PluginContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Plugin API functions
///
/// These functions are exposed to plugins through the WASM interface.
/// They provide a safe way for plugins to interact with the editor.
pub mod functions {
    use super::*;
    
    /// Get the current buffer ID
    pub fn get_current_buffer_id(context: &PluginContext) -> Option<usize> {
        if let Some(buffer_manager) = &context.buffer_manager() {
            if let Ok(buffer_manager) = buffer_manager.lock() {
                return buffer_manager.current_buffer_id();
            }
        }
        None
    }
    
    /// Get the current mode
    pub fn get_current_mode(context: &PluginContext) -> Option<String> {
        if let Some(mode_manager) = &context.mode_manager() {
            if let Ok(mode_manager) = mode_manager.lock() {
                return Some(format!("{:?}", mode_manager.current_mode()));
            }
        }
        None
    }
    
    /// Get buffer content
    pub fn get_buffer_content(context: &PluginContext, buffer_id: usize) -> Option<String> {
        if let Some(buffer_manager) = &context.buffer_manager() {
            if let Ok(buffer_manager) = buffer_manager.lock() {
                if let Ok(buffer) = buffer_manager.get_buffer(buffer_id) {
                    return Some(buffer.content().to_string());
                }
            }
        }
        None
    }
    
    /// Set buffer content
    pub fn set_buffer_content(context: &PluginContext, buffer_id: usize, content: &str) -> bool {
        if let Some(buffer_manager) = &context.buffer_manager() {
            if let Ok(mut buffer_manager) = buffer_manager.lock() {
                if let Ok(buffer) = buffer_manager.get_buffer_mut(buffer_id) {
                    // Clear the buffer and insert the new content
                    let buffer_len = buffer.content().len();
                    if buffer_len > 0 {
                        if let Err(_) = buffer.delete(0, buffer_len) {
                            return false;
                        }
                    }
                    if let Ok(_) = buffer.insert(0, content) {
                        return true;
                    }
                }
            }
        }
        false
    }
    
    /// Register a command
    pub fn register_command(context: &PluginContext, name: &str, handler: Box<dyn Fn(&[&str]) -> Result<(), String> + Send + Sync + 'static>) -> bool {
        if let Some(command_registry) = &context.command_registry() {
            if let Ok(mut command_registry) = command_registry.lock() {
                // Create a wrapper function that converts the plugin command handler to an ExCommand handler
                let wrapper = move |ex_cmd: &crate::command::ExCommand| -> crate::command::ExCommandResult<()> {
                    // Convert ExCommand args to string slices
                    let args: Vec<&str> = ex_cmd.args.iter().map(|s| s.as_str()).collect();
                    
                    // Call the plugin handler
                    match handler(&args) {
                        Ok(_) => Ok(()),
                        Err(err) => Err(crate::command::ExCommandError::InvalidCommand(err)),
                    }
                };
                
                // Register the command with the command registry
                command_registry.register(name, wrapper);
                return true;
            }
        }
        false
    }
}