//! Plugin events system
//!
//! This module defines the event system for the plugin architecture.
//! It allows plugins to receive notifications about editor events.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Event types that can be sent to plugins
#[derive(Debug, Clone, PartialEq)]
pub enum EventType {
    /// Buffer created
    BufferCreated(usize),
    /// Buffer deleted
    BufferDeleted(usize),
    /// Buffer changed
    BufferChanged(usize),
    /// Mode changed
    ModeChanged(String),
    /// Cursor moved
    CursorMoved(usize, usize, usize), // buffer_id, line, column
    /// Command executed
    CommandExecuted(String),
    /// Custom event
    Custom(String, Vec<u8>),
}

/// Event handler trait
pub trait EventHandler: Send + Sync {
    /// Handle an event
    fn handle_event(&mut self, event: &EventType);
}

/// Event manager
pub struct EventManager {
    /// Event handlers
    handlers: HashMap<String, Vec<Arc<Mutex<dyn EventHandler>>>>,
}

impl EventManager {
    /// Create a new event manager
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }
    
    /// Register an event handler
    pub fn register_handler(&mut self, event_type: &str, handler: Arc<Mutex<dyn EventHandler>>) {
        let handlers = self.handlers.entry(event_type.to_string()).or_insert_with(Vec::new);
        handlers.push(handler);
    }
    
    /// Unregister an event handler
    pub fn unregister_handler(&mut self, event_type: &str, handler: &Arc<Mutex<dyn EventHandler>>) {
        if let Some(handlers) = self.handlers.get_mut(event_type) {
            // Remove the handler by pointer equality
            handlers.retain(|h| !Arc::ptr_eq(h, handler));
        }
    }
    
    /// Dispatch an event to all registered handlers
    pub fn dispatch_event(&mut self, event_type: &str, event: &EventType) {
        if let Some(handlers) = self.handlers.get_mut(event_type) {
            for handler in handlers {
                if let Ok(mut handler) = handler.lock() {
                    handler.handle_event(event);
                }
            }
        }
    }
    
    /// Dispatch a buffer created event
    pub fn buffer_created(&mut self, buffer_id: usize) {
        let event = EventType::BufferCreated(buffer_id);
        self.dispatch_event("buffer_created", &event);
    }
    
    /// Dispatch a buffer deleted event
    pub fn buffer_deleted(&mut self, buffer_id: usize) {
        let event = EventType::BufferDeleted(buffer_id);
        self.dispatch_event("buffer_deleted", &event);
    }
    
    /// Dispatch a buffer changed event
    pub fn buffer_changed(&mut self, buffer_id: usize) {
        let event = EventType::BufferChanged(buffer_id);
        self.dispatch_event("buffer_changed", &event);
    }
    
    /// Dispatch a mode changed event
    pub fn mode_changed(&mut self, mode: &str) {
        let event = EventType::ModeChanged(mode.to_string());
        self.dispatch_event("mode_changed", &event);
    }
    
    /// Dispatch a cursor moved event
    pub fn cursor_moved(&mut self, buffer_id: usize, line: usize, column: usize) {
        let event = EventType::CursorMoved(buffer_id, line, column);
        self.dispatch_event("cursor_moved", &event);
    }
    
    /// Dispatch a command executed event
    pub fn command_executed(&mut self, command: &str) {
        let event = EventType::CommandExecuted(command.to_string());
        self.dispatch_event("command_executed", &event);
    }
    
    /// Dispatch a custom event
    pub fn custom_event(&mut self, name: &str, data: Vec<u8>) {
        let event = EventType::Custom(name.to_string(), data);
        self.dispatch_event("custom", &event);
        self.dispatch_event(name, &event);
    }
}

impl Default for EventManager {
    fn default() -> Self {
        Self::new()
    }
}