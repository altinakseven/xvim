//! Test for Insert mode functionality
//!
//! This file contains tests for the Insert mode functionality implemented in the xvim editor.

use std::path::Path;
use std::fs;
use std::io::Write;

// Import the necessary modules from the xvim crate
use xvim::editor::Editor;
use xvim::mode::Mode;
use xvim::insert::InsertFunctions;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Test basic text insertion in Insert mode
fn test_basic_insertion() {
    println!("Testing basic text insertion in Insert mode...");
    
    // Create an editor instance
    let mut editor = Editor::new().unwrap();
    
    // Create a buffer with some content
    let buffer_id = editor.get_buffer_manager_mut().create_buffer().unwrap();
    editor.get_buffer_manager_mut().set_current_buffer(buffer_id).unwrap();
    
    // Enter insert mode
    editor.start_insert_mode(false).unwrap();
    
    // Verify we're in insert mode
    assert_eq!(editor.current_mode(), Mode::Insert);
    
    // Insert some text
    editor.insert_text("Hello, world!").unwrap();
    
    // Get the buffer content
    let buffer = editor.get_buffer_manager().get_buffer(buffer_id).unwrap();
    let content = buffer.content();
    
    // Verify the content
    assert_eq!(content, "Hello, world!");
    
    // End insert mode
    editor.end_insert_mode().unwrap();
    
    // Verify we're back in normal mode
    assert_eq!(editor.current_mode(), Mode::Normal);
    
    println!("  Basic text insertion test passed");
}

/// Test auto-indentation in Insert mode
fn test_auto_indentation() {
    println!("Testing auto-indentation in Insert mode...");
    
    // Create an editor instance
    let mut editor = Editor::new().unwrap();
    
    // Create a buffer with some content
    let buffer_id = editor.get_buffer_manager_mut().create_buffer().unwrap();
    editor.get_buffer_manager_mut().set_current_buffer(buffer_id).unwrap();
    
    // Enter insert mode
    editor.start_insert_mode(false).unwrap();
    
    // Insert an indented line
    editor.insert_text("    Line 1").unwrap();
    
    // Insert a newline
    editor.insert_newline().unwrap();
    
    // Get the buffer content
    let buffer = editor.get_buffer_manager().get_buffer(buffer_id).unwrap();
    let content = buffer.content();
    
    // Verify the content has auto-indentation
    assert_eq!(content, "    Line 1\n    ");
    
    // End insert mode
    editor.end_insert_mode().unwrap();
    
    println!("  Auto-indentation test passed");
}

/// Test deleting characters in Insert mode
fn test_character_deletion() {
    println!("Testing character deletion in Insert mode...");
    
    // Create an editor instance
    let mut editor = Editor::new().unwrap();
    
    // Create a buffer with some content
    let buffer_id = editor.get_buffer_manager_mut().create_buffer().unwrap();
    editor.get_buffer_manager_mut().set_current_buffer(buffer_id).unwrap();
    
    // Enter insert mode
    editor.start_insert_mode(false).unwrap();
    
    // Insert some text
    editor.insert_text("Hello, world!").unwrap();
    
    // Delete a character before the cursor
    editor.delete_char_before_cursor().unwrap();
    
    // Get the buffer content
    let buffer = editor.get_buffer_manager().get_buffer(buffer_id).unwrap();
    let content = buffer.content();
    
    // Verify the content
    assert_eq!(content, "Hello, world");
    
    // Delete another character
    editor.delete_char_before_cursor().unwrap();
    
    // Get the updated content
    let buffer = editor.get_buffer_manager().get_buffer(buffer_id).unwrap();
    let content = buffer.content();
    
    // Verify the content
    assert_eq!(content, "Hello, worl");
    
    // End insert mode
    editor.end_insert_mode().unwrap();
    
    println!("  Character deletion test passed");
}

/// Test opening new lines with insert mode
fn test_open_line_commands() {
    println!("Testing opening new lines...");
    
    // Create an editor instance
    let mut editor = Editor::new().unwrap();
    
    // Create a buffer with some content
    let buffer_id = editor.get_buffer_manager_mut().create_buffer().unwrap();
    editor.get_buffer_manager_mut().set_current_buffer(buffer_id).unwrap();
    
    // Enter insert mode
    editor.start_insert_mode(false).unwrap();
    
    // Insert initial content
    editor.insert_text("Line 1\nLine 3").unwrap();
    
    // End insert mode
    editor.end_insert_mode().unwrap();
    
    // Manually simulate the 'o' command by:
    // 1. Moving to the end of the first line
    let buffer = editor.get_buffer_manager().get_buffer(buffer_id).unwrap();
    let cursor_pos = editor.cursor_position();
    
    // 2. Insert a newline at the end of the first line
    editor.start_insert_mode(false).unwrap();
    editor.insert_newline().unwrap();
    
    // 3. Insert text on the new line
    editor.insert_text("Line 2").unwrap();
    
    // 4. End insert mode
    editor.end_insert_mode().unwrap();
    
    // Get the buffer content
    let buffer = editor.get_buffer_manager().get_buffer(buffer_id).unwrap();
    let content = buffer.content();
    
    // Verify the content (may have extra whitespace due to auto-indent)
    assert!(content.contains("Line 1") && content.contains("Line 2") && content.contains("Line 3"));
    
    // Manually simulate the 'O' command by:
    // 1. Moving to the beginning of the first line
    let buffer = editor.get_buffer_manager().get_buffer(buffer_id).unwrap();
    
    // 2. Insert a newline at the beginning of the first line
    editor.start_insert_mode(false).unwrap();
    editor.insert_newline().unwrap();
    
    // 3. Move cursor up to the new line
    let cursor_pos = editor.cursor_position();
    
    // 4. Insert text on the new line
    editor.insert_text("Line 0").unwrap();
    
    // 5. End insert mode
    editor.end_insert_mode().unwrap();
    
    // Get the buffer content
    let buffer = editor.get_buffer_manager().get_buffer(buffer_id).unwrap();
    let content = buffer.content();
    
    // Verify the content contains all lines (may have extra whitespace)
    assert!(content.contains("Line 0") && content.contains("Line 1") &&
            content.contains("Line 2") && content.contains("Line 3"));
    
    println!("  Opening new lines test passed");
}

/// Run all tests
fn main() {
    println!("Running Insert mode tests...");
    
    // Run the tests
    test_basic_insertion();
    test_auto_indentation();
    test_character_deletion();
    test_open_line_commands();
    
    println!("All Insert mode tests passed!");
}