//! Test for Ex commands
//!
//! This file contains tests for the Ex commands implemented in the xvim editor.

use std::path::Path;
use std::fs;
use std::io::Write;

// Import the necessary modules from the xvim crate
use xvim::command::{ExCommand, ExCommandRegistry};
use xvim::editor::Editor;
use xvim::command::handlers;

/// Test the :write command
fn test_write_command() {
    println!("Testing :write command...");
    
    // Create a temporary file
    let temp_file = Path::new("test_write.txt");
    
    // Create an editor instance
    let mut editor = Editor::new();
    
    // Set the editor reference in the handlers
    handlers::set_editor(&mut editor);
    
    // Create a buffer with some content
    let buffer_id = editor.create_buffer().unwrap();
    editor.set_current_buffer(buffer_id);
    editor.insert_text("This is a test file for the :write command.\n").unwrap();
    
    // Create a command registry
    let mut registry = ExCommandRegistry::new();
    handlers::register_handlers(&mut registry, None);
    
    // Execute the :write command
    let cmd = ExCommand::parse("write test_write.txt").unwrap();
    let result = registry.execute(&cmd);
    
    // Check the result
    assert!(result.is_ok(), "Failed to execute :write command: {:?}", result);
    
    // Check if the file was created
    assert!(temp_file.exists(), "File was not created");
    
    // Clean up
    fs::remove_file(temp_file).unwrap();
    
    println!("  :write command test passed");
}

/// Test the :read command
fn test_read_command() {
    println!("Testing :read command...");
    
    // Create a temporary file with some content
    let temp_file = Path::new("test_read.txt");
    let mut file = fs::File::create(temp_file).unwrap();
    writeln!(file, "This is a test file for the :read command.").unwrap();
    file.flush().unwrap();
    
    // Create an editor instance
    let mut editor = Editor::new();
    
    // Set the editor reference in the handlers
    handlers::set_editor(&mut editor);
    
    // Create a buffer
    let buffer_id = editor.create_buffer().unwrap();
    editor.set_current_buffer(buffer_id);
    
    // Create a command registry
    let mut registry = ExCommandRegistry::new();
    handlers::register_handlers(&mut registry, None);
    
    // Execute the :read command
    let cmd = ExCommand::parse("read test_read.txt").unwrap();
    let result = registry.execute(&cmd);
    
    // Check the result
    assert!(result.is_ok(), "Failed to execute :read command: {:?}", result);
    
    // Clean up
    fs::remove_file(temp_file).unwrap();
    
    println!("  :read command test passed");
}

/// Test the :yank command
fn test_yank_command() {
    println!("Testing :yank command...");
    
    // Create an editor instance
    let mut editor = Editor::new();
    
    // Set the editor reference in the handlers
    handlers::set_editor(&mut editor);
    
    // Create a buffer with some content
    let buffer_id = editor.create_buffer().unwrap();
    editor.set_current_buffer(buffer_id);
    editor.insert_text("Line 1\nLine 2\nLine 3\n").unwrap();
    
    // Create a command registry
    let mut registry = ExCommandRegistry::new();
    handlers::register_handlers(&mut registry, None);
    
    // Execute the :yank command
    let cmd = ExCommand::parse("yank 2").unwrap();
    let result = registry.execute(&cmd);
    
    // Check the result
    assert!(result.is_ok(), "Failed to execute :yank command: {:?}", result);
    
    println!("  :yank command test passed");
}

/// Test the :put command
fn test_put_command() {
    println!("Testing :put command...");
    
    // Create an editor instance
    let mut editor = Editor::new();
    
    // Set the editor reference in the handlers
    handlers::set_editor(&mut editor);
    
    // Create a buffer with some content
    let buffer_id = editor.create_buffer().unwrap();
    editor.set_current_buffer(buffer_id);
    editor.insert_text("Line 1\nLine 2\nLine 3\n").unwrap();
    
    // Create a command registry
    let mut registry = ExCommandRegistry::new();
    handlers::register_handlers(&mut registry, None);
    
    // First yank some text
    let yank_cmd = ExCommand::parse("yank 2").unwrap();
    let yank_result = registry.execute(&yank_cmd);
    assert!(yank_result.is_ok(), "Failed to execute :yank command: {:?}", yank_result);
    
    // Then execute the :put command
    let put_cmd = ExCommand::parse("put").unwrap();
    let put_result = registry.execute(&put_cmd);
    
    // Check the result
    assert!(put_result.is_ok(), "Failed to execute :put command: {:?}", put_result);
    
    println!("  :put command test passed");
}

/// Test the :copy command
fn test_copy_command() {
    println!("Testing :copy command...");
    
    // Create an editor instance
    let mut editor = Editor::new();
    
    // Set the editor reference in the handlers
    handlers::set_editor(&mut editor);
    
    // Create a buffer with some content
    let buffer_id = editor.create_buffer().unwrap();
    editor.set_current_buffer(buffer_id);
    editor.insert_text("Line 1\nLine 2\nLine 3\n").unwrap();
    
    // Create a command registry
    let mut registry = ExCommandRegistry::new();
    handlers::register_handlers(&mut registry, None);
    
    // Execute the :copy command
    let cmd = ExCommand::parse("copy 3").unwrap();
    let result = registry.execute(&cmd);
    
    // Check the result
    assert!(result.is_ok(), "Failed to execute :copy command: {:?}", result);
    
    println!("  :copy command test passed");
}

/// Test the :move command
fn test_move_command() {
    println!("Testing :move command...");
    
    // Create an editor instance
    let mut editor = Editor::new();
    
    // Set the editor reference in the handlers
    handlers::set_editor(&mut editor);
    
    // Create a buffer with some content
    let buffer_id = editor.create_buffer().unwrap();
    editor.set_current_buffer(buffer_id);
    editor.insert_text("Line 1\nLine 2\nLine 3\n").unwrap();
    
    // Create a command registry
    let mut registry = ExCommandRegistry::new();
    handlers::register_handlers(&mut registry, None);
    
    // Execute the :move command
    let cmd = ExCommand::parse("move 3").unwrap();
    let result = registry.execute(&cmd);
    
    // Check the result
    assert!(result.is_ok(), "Failed to execute :move command: {:?}", result);
    
    println!("  :move command test passed");
}

/// Test the :substitute command
fn test_substitute_command() {
    println!("Testing :substitute command...");
    
    // Create an editor instance
    let mut editor = Editor::new();
    
    // Set the editor reference in the handlers
    handlers::set_editor(&mut editor);
    
    // Create a buffer with some content
    let buffer_id = editor.create_buffer().unwrap();
    editor.set_current_buffer(buffer_id);
    editor.insert_text("Line 1\nLine 2\nLine 3\n").unwrap();
    
    // Create a command registry
    let mut registry = ExCommandRegistry::new();
    handlers::register_handlers(&mut registry, None);
    
    // Execute the :substitute command
    let cmd = ExCommand::parse("substitute/Line/NewLine/g").unwrap();
    let result = registry.execute(&cmd);
    
    // Check the result
    assert!(result.is_ok(), "Failed to execute :substitute command: {:?}", result);
    
    println!("  :substitute command test passed");
}

/// Test the :global command
fn test_global_command() {
    println!("Testing :global command...");
    
    // Create an editor instance
    let mut editor = Editor::new();
    
    // Set the editor reference in the handlers
    handlers::set_editor(&mut editor);
    
    // Create a buffer with some content
    let buffer_id = editor.create_buffer().unwrap();
    editor.set_current_buffer(buffer_id);
    editor.insert_text("Line 1\nLine 2\nLine 3\n").unwrap();
    
    // Create a command registry
    let mut registry = ExCommandRegistry::new();
    handlers::register_handlers(&mut registry, None);
    
    // Execute the :global command
    let cmd = ExCommand::parse("global/Line/delete").unwrap();
    let result = registry.execute(&cmd);
    
    // Check the result
    assert!(result.is_ok(), "Failed to execute :global command: {:?}", result);
    
    println!("  :global command test passed");
}

/// Test the :vglobal command
fn test_vglobal_command() {
    println!("Testing :vglobal command...");
    
    // Create an editor instance
    let mut editor = Editor::new();
    
    // Set the editor reference in the handlers
    handlers::set_editor(&mut editor);
    
    // Create a buffer with some content
    let buffer_id = editor.create_buffer().unwrap();
    editor.set_current_buffer(buffer_id);
    editor.insert_text("Line 1\nLine 2\nLine 3\n").unwrap();
    
    // Create a command registry
    let mut registry = ExCommandRegistry::new();
    handlers::register_handlers(&mut registry, None);
    
    // Execute the :vglobal command
    let cmd = ExCommand::parse("vglobal/Line 2/delete").unwrap();
    let result = registry.execute(&cmd);
    
    // Check the result
    assert!(result.is_ok(), "Failed to execute :vglobal command: {:?}", result);
    
    println!("  :vglobal command test passed");
}

/// Test the :set command
fn test_set_command() {
    println!("Testing :set command...");
    
    // Create an editor instance
    let mut editor = Editor::new();
    
    // Set the editor reference in the handlers
    handlers::set_editor(&mut editor);
    
    // Create a command registry
    let mut registry = ExCommandRegistry::new();
    handlers::register_handlers(&mut registry, None);
    
    // Execute the :set command
    let cmd = ExCommand::parse("set number").unwrap();
    let result = registry.execute(&cmd);
    
    // Check the result
    assert!(result.is_ok(), "Failed to execute :set command: {:?}", result);
    
    println!("  :set command test passed");
}

/// Test the :map command
fn test_map_command() {
    println!("Testing :map command...");
    
    // Create an editor instance
    let mut editor = Editor::new();
    
    // Set the editor reference in the handlers
    handlers::set_editor(&mut editor);
    
    // Create a command registry
    let mut registry = ExCommandRegistry::new();
    handlers::register_handlers(&mut registry, None);
    
    // Execute the :map command
    let cmd = ExCommand::parse("map j gj").unwrap();
    let result = registry.execute(&cmd);
    
    // Check the result
    assert!(result.is_ok(), "Failed to execute :map command: {:?}", result);
    
    println!("  :map command test passed");
}

/// Test the :unmap command
fn test_unmap_command() {
    println!("Testing :unmap command...");
    
    // Create an editor instance
    let mut editor = Editor::new();
    
    // Set the editor reference in the handlers
    handlers::set_editor(&mut editor);
    
    // Create a command registry
    let mut registry = ExCommandRegistry::new();
    handlers::register_handlers(&mut registry, None);
    
    // First map a key
    let map_cmd = ExCommand::parse("map j gj").unwrap();
    let map_result = registry.execute(&map_cmd);
    assert!(map_result.is_ok(), "Failed to execute :map command: {:?}", map_result);
    
    // Then execute the :unmap command
    let unmap_cmd = ExCommand::parse("unmap j").unwrap();
    let unmap_result = registry.execute(&unmap_cmd);
    
    // Check the result
    assert!(unmap_result.is_ok(), "Failed to execute :unmap command: {:?}", unmap_result);
    
    println!("  :unmap command test passed");
}

/// Test the :registers command
fn test_registers_command() {
    println!("Testing :registers command...");
    
    // Create an editor instance
    let mut editor = Editor::new();
    
    // Set the editor reference in the handlers
    handlers::set_editor(&mut editor);
    
    // Create a command registry
    let mut registry = ExCommandRegistry::new();
    handlers::register_handlers(&mut registry, None);
    
    // Execute the :registers command
    let cmd = ExCommand::parse("registers").unwrap();
    let result = registry.execute(&cmd);
    
    // Check the result
    assert!(result.is_ok(), "Failed to execute :registers command: {:?}", result);
    
    println!("  :registers command test passed");
}

/// Test the :buffers command
fn test_buffers_command() {
    println!("Testing :buffers command...");
    
    // Create an editor instance
    let mut editor = Editor::new();
    
    // Set the editor reference in the handlers
    handlers::set_editor(&mut editor);
    
    // Create a buffer
    let buffer_id = editor.create_buffer().unwrap();
    editor.set_current_buffer(buffer_id);
    
    // Create a command registry
    let mut registry = ExCommandRegistry::new();
    handlers::register_handlers(&mut registry, None);
    
    // Execute the :buffers command
    let cmd = ExCommand::parse("buffers").unwrap();
    let result = registry.execute(&cmd);
    
    // Check the result
    assert!(result.is_ok(), "Failed to execute :buffers command: {:?}", result);
    
    println!("  :buffers command test passed");
}

/// Run all tests
fn main() {
    println!("Running Ex command tests...");
    
    // Test file operations
    test_write_command();
    test_read_command();
    
    // Test editing operations
    test_yank_command();
    test_put_command();
    test_copy_command();
    test_move_command();
    test_substitute_command();
    test_global_command();
    test_vglobal_command();
    
    // Test other operations
    test_set_command();
    test_map_command();
    test_unmap_command();
    test_registers_command();
    test_buffers_command();
    
    println!("All tests passed!");
}