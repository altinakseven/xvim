//! Hello World Plugin for xvim
//!
//! This is a simple example plugin that demonstrates the xvim plugin API.
//! It adds a :hello command that displays a greeting message.

// Import the xvim plugin API
use xvim_plugin_api::*;

// Plugin entry point
#[no_mangle]
pub extern "C" fn init() -> i32 {
    // Register the plugin with the editor
    register_plugin("hello", "0.1.0", "Hello World Plugin", "xvim Team");
    
    // Register the :hello command
    register_command("hello", hello_command);
    
    // Register event handlers
    register_event_handler("buffer_created", buffer_created_handler);
    register_event_handler("mode_changed", mode_changed_handler);
    
    // Return success
    0
}

// Command handler for :hello
fn hello_command(args: &[&str]) -> Result<(), String> {
    if args.is_empty() {
        // Display a simple greeting
        editor_message("Hello from the Hello World Plugin!");
    } else {
        // Display a personalized greeting
        let name = args[0];
        editor_message(&format!("Hello, {}! Welcome to xvim!", name));
    }
    
    Ok(())
}

// Event handler for buffer_created events
fn buffer_created_handler(event: &Event) -> Result<(), String> {
    if let Event::BufferCreated(buffer_id) = event {
        // Log a message when a buffer is created
        log_message(&format!("Buffer {} was created", buffer_id));
    }
    
    Ok(())
}

// Event handler for mode_changed events
fn mode_changed_handler(event: &Event) -> Result<(), String> {
    if let Event::ModeChanged(mode) = event {
        // Log a message when the mode changes
        log_message(&format!("Mode changed to {}", mode));
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
    
    // Event handler registration
    pub fn register_event_handler(event_type: &str, handler: fn(&Event) -> Result<(), String>) {
        // This would be implemented by the xvim plugin API
    }
    
    // Display a message in the editor
    pub fn editor_message(message: &str) {
        // This would be implemented by the xvim plugin API
    }
    
    // Log a message to the plugin log
    pub fn log_message(message: &str) {
        // This would be implemented by the xvim plugin API
    }
    
    // Event types
    pub enum Event {
        BufferCreated(usize),
        BufferDeleted(usize),
        BufferChanged(usize),
        ModeChanged(String),
        CursorMoved(usize, usize, usize),
        CommandExecuted(String),
        Custom(String, Vec<u8>),
    }
}