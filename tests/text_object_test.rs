//! Test for text_object commands
//!
//! This file contains tests for the text_object commands implemented in the xvim editor.

use std::path::Path;
use std::fs;
use std::io::Write;

// Import the necessary modules from the xvim crate
use xvim::command::{ExCommand, ExCommandRegistry, ExCommandParser};
use xvim::editor::Editor;
use xvim::command::handlers;
use xvim::insert::InsertFunctions;

/// Test the text_object functionality
fn test_text_object_basic() {
    println!("Testing text_object basic functionality...");
    
    // Create an editor instance
    let mut editor = Editor::new().unwrap();
    
    // Set the editor reference in the handlers
    handlers::set_editor(&mut editor);
    
    // Create a buffer with some content
    let buffer_id = editor.get_buffer_manager_mut().create_buffer().unwrap();
    editor.get_buffer_manager_mut().set_current_buffer(buffer_id).unwrap();
    editor.insert_text("Test content for text_object test\n").unwrap();
    
    // Create a command registry
    let mut registry = ExCommandRegistry::new();
    handlers::register_handlers(&mut registry, None);
    
    // Create a command parser
    let parser = ExCommandParser::new();
    
    // Execute the command
    // TODO: Replace with actual command
    let cmd = parser.parse("text_object").unwrap();
    let result = registry.execute(&cmd);
    
    // Check the result
    assert!(result.is_ok(), "Failed to execute text_object command: {:?}", result);
    
    println!("  text_object basic test passed");
}

/// Run all tests
fn main() {
    println!("Running text_object tests...");
    
    test_text_object_basic();
    
    println!("All text_object tests passed!");
}
