//! Test for Ex commands
//!
//! This file contains tests for the Ex commands implemented in the xvim editor.

use std::path::Path;
use std::fs;
use std::io::Write;

// Import the necessary modules from the xvim crate
use xvim::command::{ExCommand, ExCommandRegistry, ExCommandParser};
use xvim::editor::Editor;
use xvim::command::handlers;
use xvim::insert::InsertFunctions;

/// Test the :write command
fn test_write_command() {
    println!("Testing :write command...");
    
    // Create a temporary file
    let temp_file = Path::new("test_write.txt");
    
    // Create an editor instance
    let mut editor = Editor::new().unwrap();
    
    // Set the editor reference in the handlers
    handlers::set_editor(&mut editor);
    
    // Create a buffer with some content
    let buffer_id = editor.get_buffer_manager_mut().create_buffer().unwrap();
    editor.get_buffer_manager_mut().set_current_buffer(buffer_id).unwrap();
    editor.insert_text("This is a test file for the :write command.\n").unwrap();
    
    // Create a command registry
    let mut registry = ExCommandRegistry::new();
    handlers::register_handlers(&mut registry, None);
    
    // Create a command parser
    let parser = ExCommandParser::new();
    
    // Execute the :write command
    let cmd = parser.parse("write test_write.txt").unwrap();
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
    let mut editor = Editor::new().unwrap();
    
    // Set the editor reference in the handlers
    handlers::set_editor(&mut editor);
    
    // Create a buffer
    let buffer_id = editor.get_buffer_manager_mut().create_buffer().unwrap();
    editor.get_buffer_manager_mut().set_current_buffer(buffer_id).unwrap();
    
    // Create a command registry
    let mut registry = ExCommandRegistry::new();
    handlers::register_handlers(&mut registry, None);
    
    // Create a command parser
    let parser = ExCommandParser::new();
    
    // Execute the :read command
    let cmd = parser.parse("read test_read.txt").unwrap();
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
    let mut editor = Editor::new().unwrap();
    
    // Set the editor reference in the handlers
    handlers::set_editor(&mut editor);
    
    // Create a buffer with some content
    let buffer_id = editor.get_buffer_manager_mut().create_buffer().unwrap();
    editor.get_buffer_manager_mut().set_current_buffer(buffer_id).unwrap();
    editor.insert_text("Line 1\nLine 2\nLine 3\n").unwrap();
    
    // Create a command registry
    let mut registry = ExCommandRegistry::new();
    handlers::register_handlers(&mut registry, None);
    
    // Create a command parser
    let parser = ExCommandParser::new();
    
    // Execute the :yank command
    let cmd = parser.parse("yank 2").unwrap();
    let result = registry.execute(&cmd);
    
    // Check the result
    assert!(result.is_ok(), "Failed to execute :yank command: {:?}", result);
    
    println!("  :yank command test passed");
}

/// Test the :put command
fn test_put_command() {
    println!("Testing :put command...");
    
    // Create an editor instance
    let mut editor = Editor::new().unwrap();
    
    // Set the editor reference in the handlers
    handlers::set_editor(&mut editor);
    
    // Create a buffer with some content
    let buffer_id = editor.get_buffer_manager_mut().create_buffer().unwrap();
    editor.get_buffer_manager_mut().set_current_buffer(buffer_id).unwrap();
    editor.insert_text("Line 1\nLine 2\nLine 3\n").unwrap();
    
    // Create a command registry
    let mut registry = ExCommandRegistry::new();
    handlers::register_handlers(&mut registry, None);
    
    // Create a command parser
    let parser = ExCommandParser::new();
    
    // First yank some text
    let yank_cmd = parser.parse("yank 2").unwrap();
    let yank_result = registry.execute(&yank_cmd);
    assert!(yank_result.is_ok(), "Failed to execute :yank command: {:?}", yank_result);
    
    // Then execute the :put command
    let put_cmd = parser.parse("put").unwrap();
    let put_result = registry.execute(&put_cmd);
    
    // Check the result
    assert!(put_result.is_ok(), "Failed to execute :put command: {:?}", put_result);
    
    println!("  :put command test passed");
}

/// Test the :copy command
fn test_copy_command() {
    println!("Testing :copy command...");
    
    // Create an editor instance
    let mut editor = Editor::new().unwrap();
    
    // Set the editor reference in the handlers
    handlers::set_editor(&mut editor);
    
    // Create a buffer with some content
    let buffer_id = editor.get_buffer_manager_mut().create_buffer().unwrap();
    editor.get_buffer_manager_mut().set_current_buffer(buffer_id).unwrap();
    editor.insert_text("Line 1\nLine 2\nLine 3\n").unwrap();
    
    // Create a command registry
    let mut registry = ExCommandRegistry::new();
    handlers::register_handlers(&mut registry, None);
    
    // Create a command parser
    let parser = ExCommandParser::new();
    
    // Execute the :copy command
    let cmd = parser.parse("copy 3").unwrap();
    let result = registry.execute(&cmd);
    
    // Check the result
    assert!(result.is_ok(), "Failed to execute :copy command: {:?}", result);
    
    println!("  :copy command test passed");
}

/// Test the :move command
fn test_move_command() {
    println!("Testing :move command...");
    
    // Create an editor instance
    let mut editor = Editor::new().unwrap();
    
    // Set the editor reference in the handlers
    handlers::set_editor(&mut editor);
    
    // Create a buffer with some content
    let buffer_id = editor.get_buffer_manager_mut().create_buffer().unwrap();
    editor.get_buffer_manager_mut().set_current_buffer(buffer_id).unwrap();
    editor.insert_text("Line 1\nLine 2\nLine 3\n").unwrap();
    
    // Create a command registry
    let mut registry = ExCommandRegistry::new();
    handlers::register_handlers(&mut registry, None);
    
    // Create a command parser
    let parser = ExCommandParser::new();
    
    // Execute the :move command
    let cmd = parser.parse("move 3").unwrap();
    let result = registry.execute(&cmd);
    
    // Check the result
    assert!(result.is_ok(), "Failed to execute :move command: {:?}", result);
    
    println!("  :move command test passed");
}

/// Test the :substitute command
fn test_substitute_command() {
    println!("Testing :substitute command...");
    
    // Create an editor instance
    let mut editor = Editor::new().unwrap();
    
    // Set the editor reference in the handlers
    handlers::set_editor(&mut editor);
    
    // Create a buffer with some content
    let buffer_id = editor.get_buffer_manager_mut().create_buffer().unwrap();
    editor.get_buffer_manager_mut().set_current_buffer(buffer_id).unwrap();
    editor.insert_text("Line 1\nLine 2\nLine 3\n").unwrap();
    
    // Create a command registry
    let mut registry = ExCommandRegistry::new();
    handlers::register_handlers(&mut registry, None);
    
    // Create a command parser
    let parser = ExCommandParser::new();
    
    // Execute the :substitute command
    let cmd = parser.parse("substitute/Line/NewLine/g").unwrap();
    let result = registry.execute(&cmd);
    
    // Check the result
    assert!(result.is_ok(), "Failed to execute :substitute command: {:?}", result);
    
    println!("  :substitute command test passed");
}

/// Test the :global command
fn test_global_command() {
    println!("Testing :global command...");
    
    // Create an editor instance
    let mut editor = Editor::new().unwrap();
    
    // Set the editor reference in the handlers
    handlers::set_editor(&mut editor);
    
    // Create a buffer with some content
    let buffer_id = editor.get_buffer_manager_mut().create_buffer().unwrap();
    editor.get_buffer_manager_mut().set_current_buffer(buffer_id).unwrap();
    editor.insert_text("Line 1\nLine 2\nLine 3\n").unwrap();
    
    // Create a command registry
    let mut registry = ExCommandRegistry::new();
    handlers::register_handlers(&mut registry, None);
    // Create a command parser
    let parser = ExCommandParser::new();
    
    // Execute the :global command
    let cmd = parser.parse("global/Line/delete").unwrap();
    let result = registry.execute(&cmd);
    let result = registry.execute(&cmd);
    
    // Check the result
    assert!(result.is_ok(), "Failed to execute :global command: {:?}", result);
    
    println!("  :global command test passed");
}

/// Test the :vglobal command
fn test_vglobal_command() {
    println!("Testing :vglobal command...");
    
    // Create an editor instance
    let mut editor = Editor::new().unwrap();
    
    // Set the editor reference in the handlers
    handlers::set_editor(&mut editor);
    
    // Create a buffer with some content
    let buffer_id = editor.get_buffer_manager_mut().create_buffer().unwrap();
    editor.get_buffer_manager_mut().set_current_buffer(buffer_id).unwrap();
    editor.insert_text("Line 1\nLine 2\nLine 3\n").unwrap();
    
    // Create a command registry
    let mut registry = ExCommandRegistry::new();
    handlers::register_handlers(&mut registry, None);
    
    // Create a command parser
    let parser = ExCommandParser::new();
    
    // Execute the :vglobal command to delete lines that don't contain "Line 2"
    let cmd = parser.parse("vglobal/Line 2/delete").unwrap();
    let result = registry.execute(&cmd);
    
    // Check the result
    assert!(result.is_ok(), "Failed to execute :vglobal command: {:?}", result);
    
    // Verify that only Line 2 remains in the buffer (plus possibly an empty line at the end)
    let buffer = editor.get_buffer_manager().get_buffer(buffer_id).unwrap();
    
    // The buffer might have 1 or 2 lines (if there's a trailing newline)
    assert!(buffer.line_count() <= 2, "Expected 1-2 lines to remain, but found {}", buffer.line_count());
    
    // Check that the first line is "Line 2"
    assert_eq!(buffer.line(0).unwrap(), "Line 2", "Expected 'Line 2' to remain, but found '{}'", buffer.line(0).unwrap());
    
    // If there's a second line, it should be empty (just a newline)
    if buffer.line_count() > 1 {
        assert_eq!(buffer.line(1).unwrap(), "", "Expected empty line, but found '{}'", buffer.line(1).unwrap());
    }
    
    println!("  :vglobal command test passed");
}

/// Test the :set command
fn test_set_command() {
    println!("Testing :set command...");
    
    // Create an editor instance
    let mut editor = Editor::new().unwrap();
    
    // Set the editor reference in the handlers
    handlers::set_editor(&mut editor);
    
    // Create a command registry
    let mut registry = ExCommandRegistry::new();
    handlers::register_handlers(&mut registry, None);
    
    // Create a command parser
    let parser = ExCommandParser::new();
    
    // Execute the :set command
    let cmd = parser.parse("set number").unwrap();
    let result = registry.execute(&cmd);
    
    // Check the result
    assert!(result.is_ok(), "Failed to execute :set command: {:?}", result);
    
    println!("  :set command test passed");
}

/// Test the :map command
fn test_map_command() {
    println!("Testing :map command...");
    
    // Create an editor instance
    let mut editor = Editor::new().unwrap();
    
    // Set the editor reference in the handlers
    handlers::set_editor(&mut editor);
    
    // Create a command registry
    let mut registry = ExCommandRegistry::new();
    handlers::register_handlers(&mut registry, None);
    
    // Create a command parser
    let parser = ExCommandParser::new();
    
    // Execute the :map command
    let cmd = parser.parse("map j gj").unwrap();
    let result = registry.execute(&cmd);
    
    // Check the result
    assert!(result.is_ok(), "Failed to execute :map command: {:?}", result);
    
    println!("  :map command test passed");
}

/// Test the :unmap command
fn test_unmap_command() {
    println!("Testing :unmap command...");
    
    // Create an editor instance
    let mut editor = Editor::new().unwrap();
    
    // Set the editor reference in the handlers
    handlers::set_editor(&mut editor);
    
    // Create a command registry
    let mut registry = ExCommandRegistry::new();
    handlers::register_handlers(&mut registry, None);
    
    // Create a command parser
    let parser = ExCommandParser::new();
    
    // First map a key
    let map_cmd = parser.parse("map j gj").unwrap();
    let map_result = registry.execute(&map_cmd);
    assert!(map_result.is_ok(), "Failed to execute :map command: {:?}", map_result);
    
    // Then execute the :unmap command
    let unmap_cmd = parser.parse("unmap j").unwrap();
    let unmap_result = registry.execute(&unmap_cmd);
    
    // Check the result
    assert!(unmap_result.is_ok(), "Failed to execute :unmap command: {:?}", unmap_result);
    
    println!("  :unmap command test passed");
}

/// Test the :registers command
fn test_registers_command() {
    println!("Testing :registers command...");
    
    // Create an editor instance
    let mut editor = Editor::new().unwrap();
    
    // Set the editor reference in the handlers
    handlers::set_editor(&mut editor);
    
    // Create a command registry
    let mut registry = ExCommandRegistry::new();
    handlers::register_handlers(&mut registry, None);
    
    // Create a command parser
    let parser = ExCommandParser::new();
    
    // Execute the :registers command
    let cmd = parser.parse("registers").unwrap();
    let result = registry.execute(&cmd);
    
    // Check the result
    assert!(result.is_ok(), "Failed to execute :registers command: {:?}", result);
    
    println!("  :registers command test passed");
}

/// Test the :normal command
fn test_normal_command() {
    println!("Testing :normal command...");
    
    // Create an editor instance
    let mut editor = Editor::new().unwrap();
    
    // Set the editor reference in the handlers
    handlers::set_editor(&mut editor);
    
    // Create a buffer with some content
    let buffer_id = editor.get_buffer_manager_mut().create_buffer().unwrap();
    editor.get_buffer_manager_mut().set_current_buffer(buffer_id).unwrap();
    editor.insert_text("Line 1\nLine 2\nLine 3\n").unwrap();
    
    // Move cursor to the beginning
    editor.get_cursor_manager_mut().set_position(xvim::cursor::CursorPosition::new(0, 0));
    
    // Create a command registry
    let mut registry = ExCommandRegistry::new();
    handlers::register_handlers(&mut registry, None);
    
    // Create a command parser
    let parser = ExCommandParser::new();
    
    // Execute the :normal command to move down and delete a line
    let cmd = parser.parse("normal jdd").unwrap();
    let result = registry.execute(&cmd);
    
    // Check the result
    assert!(result.is_ok(), "Failed to execute :normal command: {:?}", result);
    
    // Verify the buffer content
    let buffer = editor.get_buffer_manager().get_buffer(buffer_id).unwrap();
    assert_eq!(buffer.content(), "Line 1\nLine 3\n");
    
    println!("  :normal command test passed");
}

/// Test the :sort command
fn test_sort_command() {
    println!("Testing :sort command...");
    
    // Create an editor instance
    let mut editor = Editor::new().unwrap();
    
    // Set the editor reference in the handlers
    handlers::set_editor(&mut editor);
    
    // Create a buffer with some unsorted content
    let buffer_id = editor.get_buffer_manager_mut().create_buffer().unwrap();
    editor.get_buffer_manager_mut().set_current_buffer(buffer_id).unwrap();
    editor.insert_text("c\na\nb\n").unwrap();
    
    // Create a command registry
    let mut registry = ExCommandRegistry::new();
    handlers::register_handlers(&mut registry, None);
    
    // Create a command parser
    let parser = ExCommandParser::new();
    
    // Execute the :sort command
    let cmd = parser.parse("sort").unwrap();
    let result = registry.execute(&cmd);
    
    // Check the result
    assert!(result.is_ok(), "Failed to execute :sort command: {:?}", result);
    
    // Verify the buffer content
    let buffer = editor.get_buffer_manager().get_buffer(buffer_id).unwrap();
    assert_eq!(buffer.content(), "a\nb\nc\n");
    
    println!("  :sort command test passed");
}

/// Test the :cd command
fn test_cd_command() {
    println!("Testing :cd command...");
    
    // Create an editor instance
    let mut editor = Editor::new().unwrap();
    
    // Set the editor reference in the handlers
    handlers::set_editor(&mut editor);
    
    // Create a command registry
    let mut registry = ExCommandRegistry::new();
    handlers::register_handlers(&mut registry, None);
    
    // Create a command parser
    let parser = ExCommandParser::new();
    
    // Get the current directory
    let current_dir = std::env::current_dir().unwrap();
    
    // Execute the :cd command to change to the parent directory
    let cmd = parser.parse("cd ..").unwrap();
    let result = registry.execute(&cmd);
    
    // Check the result
    assert!(result.is_ok(), "Failed to execute :cd command: {:?}", result);
    
    // Check that the directory was changed
    let new_dir = std::env::current_dir().unwrap();
    assert_ne!(current_dir, new_dir);
    
    // Change back to the original directory
    std::env::set_current_dir(current_dir).unwrap();
    
    println!("  :cd command test passed");
}

/// Test the :buffers command
fn test_buffers_command() {
    println!("Testing :buffers command...");
    
    // Create an editor instance
    let mut editor = Editor::new().unwrap();
    
    // Set the editor reference in the handlers
    handlers::set_editor(&mut editor);
    
    // Create a buffer
    let buffer_id = editor.get_buffer_manager_mut().create_buffer().unwrap();
    editor.get_buffer_manager_mut().set_current_buffer(buffer_id).unwrap();
    
    // Create a command registry
    let mut registry = ExCommandRegistry::new();
    handlers::register_handlers(&mut registry, None);
    
    // Create a command parser
    let parser = ExCommandParser::new();
    
    // Execute the :buffers command
    let cmd = parser.parse("buffers").unwrap();
    let result = registry.execute(&cmd);
    
    // Check the result
    assert!(result.is_ok(), "Failed to execute :buffers command: {:?}", result);
    
    println!("  :buffers command test passed");
}

/// Test the :undo command
fn test_undo_command() {
    println!("Testing :undo command...");
    
    // Create an editor instance
    let mut editor = Editor::new().unwrap();
    
    // Set the editor reference in the handlers
    handlers::set_editor(&mut editor);
    
    // Create a buffer with some content
    let buffer_id = editor.get_buffer_manager_mut().create_buffer().unwrap();
    editor.get_buffer_manager_mut().set_current_buffer(buffer_id).unwrap();
    
    // Insert the initial text
    editor.insert_text("Initial text").unwrap();
    
    // Sleep for a bit to force a new change group (the change history groups changes that occur within 500ms)
    std::thread::sleep(std::time::Duration::from_millis(600));
    
    // Make a change to the buffer
    editor.insert_text(" with more content").unwrap();
    
    // Create a command registry
    let mut registry = ExCommandRegistry::new();
    handlers::register_handlers(&mut registry, None);
    
    // Create a command parser
    let parser = ExCommandParser::new();
    
    // Execute the :undo command
    let cmd = parser.parse("undo").unwrap();
    let result = registry.execute(&cmd);
    
    // Check the result
    assert!(result.is_ok(), "Failed to execute :undo command: {:?}", result);
    
    // Verify the buffer content
    let buffer = editor.get_buffer_manager().get_buffer(buffer_id).unwrap();
    assert_eq!(buffer.content(), "Initial text", "Expected 'Initial text', but got '{}'", buffer.content());
    
    println!("  :undo command test passed");
}

/// Test the :redo command
fn test_redo_command() {
    println!("Testing :redo command...");
    
    // Create an editor instance
    let mut editor = Editor::new().unwrap();
    
    // Set the editor reference in the handlers
    handlers::set_editor(&mut editor);
    
    // Create a buffer with some content
    let buffer_id = editor.get_buffer_manager_mut().create_buffer().unwrap();
    editor.get_buffer_manager_mut().set_current_buffer(buffer_id).unwrap();
    
    // Insert the initial text
    editor.insert_text("Initial text").unwrap();
    
    // Sleep for a bit to force a new change group (the change history groups changes that occur within 500ms)
    std::thread::sleep(std::time::Duration::from_millis(600));
    
    // Make a change to the buffer
    editor.insert_text(" with more content").unwrap();
    
    // Create a command registry
    let mut registry = ExCommandRegistry::new();
    handlers::register_handlers(&mut registry, None);
    
    // Create a command parser
    let parser = ExCommandParser::new();
    
    // First undo the change
    let undo_cmd = parser.parse("undo").unwrap();
    let undo_result = registry.execute(&undo_cmd);
    assert!(undo_result.is_ok(), "Failed to execute :undo command: {:?}", undo_result);
    
    // Then execute the :redo command
    let redo_cmd = parser.parse("redo").unwrap();
    let redo_result = registry.execute(&redo_cmd);
    
    // Check the result
    assert!(redo_result.is_ok(), "Failed to execute :redo command: {:?}", redo_result);
    
    // Verify the buffer content
    let buffer = editor.get_buffer_manager().get_buffer(buffer_id).unwrap();
    assert_eq!(buffer.content(), "Initial text with more content");
    
    println!("  :redo command test passed");
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
    
    // Test undo/redo operations
    test_undo_command();
    test_redo_command();
    
    // Test other operations
    test_set_command();
    test_map_command();
    test_unmap_command();
    test_registers_command();
    test_buffers_command();
    
    // Test our new commands
    test_normal_command();
    test_sort_command();
    test_cd_command();
    
    println!("All tests passed!");
}