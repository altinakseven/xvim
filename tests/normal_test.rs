// Normal mode tests

use xvim::buffer::{Buffer, BufferManager};
use xvim::cursor::CursorPosition;
use xvim::editor::Editor;
use xvim::mode::Mode;
use std::path::Path;

#[test]
fn test_normal_mode_basic_movement() {
    // Create a new editor
    let mut editor = Editor::new().unwrap();
    
    // Create a buffer with some text
    let buffer_id = editor.get_buffer_manager_mut().create_buffer().unwrap();
    let buffer = editor.get_buffer_manager_mut().get_buffer_mut(buffer_id).unwrap();
    buffer.insert(0, "Line 1\nLine 2\nLine 3\n").unwrap();
    
    // Set the current buffer
    editor.get_buffer_manager_mut().set_current_buffer(buffer_id).unwrap();
    
    // Set the cursor position to the beginning of the buffer
    editor.get_cursor_manager_mut().set_position(CursorPosition::new(0, 0));
    
    // Test 'j' command (move down)
    editor.mode_manager.handle_key(&mut editor, 'j').unwrap();
    assert_eq!(editor.cursor_position(), CursorPosition::new(1, 0));
    
    // Test 'k' command (move up)
    editor.mode_manager.handle_key(&mut editor, 'k').unwrap();
    assert_eq!(editor.cursor_position(), CursorPosition::new(0, 0));
    
    // Test 'l' command (move right)
    editor.mode_manager.handle_key(&mut editor, 'l').unwrap();
    assert_eq!(editor.cursor_position(), CursorPosition::new(0, 1));
    
    // Test 'h' command (move left)
    editor.mode_manager.handle_key(&mut editor, 'h').unwrap();
    assert_eq!(editor.cursor_position(), CursorPosition::new(0, 0));
    
    // Test '0' command (move to beginning of line)
    editor.mode_manager.handle_key(&mut editor, 'l').unwrap();
    editor.mode_manager.handle_key(&mut editor, 'l').unwrap();
    editor.mode_manager.handle_key(&mut editor, '0').unwrap();
    assert_eq!(editor.cursor_position(), CursorPosition::new(0, 0));
    
    // Test '$' command (move to end of line)
    editor.mode_manager.handle_key(&mut editor, '$').unwrap();
    assert_eq!(editor.cursor_position(), CursorPosition::new(0, 5));
}

#[test]
fn test_normal_mode_word_movement() {
    // Create a new editor
    let mut editor = Editor::new().unwrap();
    
    // Create a buffer with some text
    let buffer_id = editor.get_buffer_manager_mut().create_buffer().unwrap();
    let buffer = editor.get_buffer_manager_mut().get_buffer_mut(buffer_id).unwrap();
    buffer.insert(0, "word1 word2 word3\nword4 word5 word6\n").unwrap();
    
    // Set the current buffer
    editor.get_buffer_manager_mut().set_current_buffer(buffer_id).unwrap();
    
    // Set the cursor position to the beginning of the buffer
    editor.get_cursor_manager_mut().set_position(CursorPosition::new(0, 0));
    
    // Test 'w' command (move to next word)
    editor.mode_manager.handle_key(&mut editor, 'w').unwrap();
    assert_eq!(editor.cursor_position(), CursorPosition::new(0, 6));
    
    // Test 'w' command again
    editor.mode_manager.handle_key(&mut editor, 'w').unwrap();
    assert_eq!(editor.cursor_position(), CursorPosition::new(0, 12));
    
    // Test 'b' command (move to previous word)
    editor.mode_manager.handle_key(&mut editor, 'b').unwrap();
    assert_eq!(editor.cursor_position(), CursorPosition::new(0, 6));
    
    // Test 'e' command (move to end of word)
    editor.mode_manager.handle_key(&mut editor, 'e').unwrap();
    assert_eq!(editor.cursor_position(), CursorPosition::new(0, 10));
}

#[test]
fn test_normal_mode_operators() {
    // Create a new editor
    let mut editor = Editor::new().unwrap();
    
    // Create a buffer with some text
    let buffer_id = editor.get_buffer_manager_mut().create_buffer().unwrap();
    let buffer = editor.get_buffer_manager_mut().get_buffer_mut(buffer_id).unwrap();
    buffer.insert(0, "Line 1\nLine 2\nLine 3\n").unwrap();
    
    // Set the current buffer
    editor.get_buffer_manager_mut().set_current_buffer(buffer_id).unwrap();
    
    // Set the cursor position to the beginning of the buffer
    editor.get_cursor_manager_mut().set_position(CursorPosition::new(0, 0));
    
    // Test 'dd' command (delete line)
    editor.mode_manager.handle_key(&mut editor, 'd').unwrap();
    editor.mode_manager.handle_key(&mut editor, 'd').unwrap();
    
    // Check that the line was deleted
    let buffer = editor.get_buffer_manager().get_buffer(buffer_id).unwrap();
    assert_eq!(buffer.line_count(), 3); // Including the final newline
    assert_eq!(buffer.line(0).unwrap(), "Line 2");
    
    // Test 'yy' command (yank line)
    editor.mode_manager.handle_key(&mut editor, 'y').unwrap();
    editor.mode_manager.handle_key(&mut editor, 'y').unwrap();
    
    // Test 'p' command (put yanked text)
    editor.mode_manager.handle_key(&mut editor, 'p').unwrap();
    
    // Check that the line was pasted
    let buffer = editor.get_buffer_manager().get_buffer(buffer_id).unwrap();
    assert_eq!(buffer.line_count(), 4); // Including the final newline
    assert_eq!(buffer.line(0).unwrap(), "Line 2");
    assert_eq!(buffer.line(1).unwrap(), "Line 2");
}

#[test]
fn test_normal_mode_count_prefix() {
    // Create a new editor
    let mut editor = Editor::new().unwrap();
    
    // Create a buffer with some text
    let buffer_id = editor.get_buffer_manager_mut().create_buffer().unwrap();
    let buffer = editor.get_buffer_manager_mut().get_buffer_mut(buffer_id).unwrap();
    buffer.insert(0, "Line 1\nLine 2\nLine 3\nLine 4\nLine 5\n").unwrap();
    
    // Set the current buffer
    editor.get_buffer_manager_mut().set_current_buffer(buffer_id).unwrap();
    
    // Set the cursor position to the beginning of the buffer
    editor.get_cursor_manager_mut().set_position(CursorPosition::new(0, 0));
    
    // Test '3j' command (move down 3 lines)
    editor.mode_manager.handle_key(&mut editor, '3').unwrap();
    editor.mode_manager.handle_key(&mut editor, 'j').unwrap();
    assert_eq!(editor.cursor_position(), CursorPosition::new(3, 0));
    
    // Test '2k' command (move up 2 lines)
    editor.mode_manager.handle_key(&mut editor, '2').unwrap();
    editor.mode_manager.handle_key(&mut editor, 'k').unwrap();
    assert_eq!(editor.cursor_position(), CursorPosition::new(1, 0));
    
    // Test '2dd' command (delete 2 lines)
    editor.mode_manager.handle_key(&mut editor, '2').unwrap();
    editor.mode_manager.handle_key(&mut editor, 'd').unwrap();
    editor.mode_manager.handle_key(&mut editor, 'd').unwrap();
    
    // Check that the lines were deleted
    let buffer = editor.get_buffer_manager().get_buffer(buffer_id).unwrap();
    assert_eq!(buffer.line_count(), 4); // Including the final newline
    assert_eq!(buffer.line(0).unwrap(), "Line 1");
    assert_eq!(buffer.line(1).unwrap(), "Line 4");
}

#[test]
fn test_normal_mode_text_objects() {
    // Create a new editor
    let mut editor = Editor::new().unwrap();
    
    // Create a buffer with some text
    let buffer_id = editor.get_buffer_manager_mut().create_buffer().unwrap();
    let buffer = editor.get_buffer_manager_mut().get_buffer_mut(buffer_id).unwrap();
    buffer.insert(0, "This is a sentence. This is another sentence.\nThis is a paragraph.\n\nThis is another paragraph.\n").unwrap();
    
    // Set the current buffer
    editor.get_buffer_manager_mut().set_current_buffer(buffer_id).unwrap();
    
    // Set the cursor position to the beginning of the buffer
    editor.get_cursor_manager_mut().set_position(CursorPosition::new(0, 0));
    
    // Test 'dw' command (delete word)
    editor.mode_manager.handle_key(&mut editor, 'd').unwrap();
    editor.mode_manager.handle_key(&mut editor, 'w').unwrap();
    
    // Check that the word was deleted
    let buffer = editor.get_buffer_manager().get_buffer(buffer_id).unwrap();
    assert_eq!(buffer.line(0).unwrap(), "is a sentence. This is another sentence.");
    
    // Test 'cw' command (change word)
    editor.mode_manager.handle_key(&mut editor, 'c').unwrap();
    editor.mode_manager.handle_key(&mut editor, 'w').unwrap();
    
    // Check that we're in insert mode
    assert_eq!(editor.current_mode(), Mode::Insert);
    
    // Insert some text
    editor.insert_text("was").unwrap();
    
    // Return to normal mode
    editor.mode_manager.enter_normal_mode();
    
    // Check that the word was changed
    let buffer = editor.get_buffer_manager().get_buffer(buffer_id).unwrap();
    assert_eq!(buffer.line(0).unwrap(), "was a sentence. This is another sentence.");
}

#[test]
fn test_normal_mode_mode_switching() {
    // Create a new editor
    let mut editor = Editor::new().unwrap();
    
    // Create a buffer with some text
    let buffer_id = editor.get_buffer_manager_mut().create_buffer().unwrap();
    let buffer = editor.get_buffer_manager_mut().get_buffer_mut(buffer_id).unwrap();
    buffer.insert(0, "Line 1\nLine 2\nLine 3\n").unwrap();
    
    // Set the current buffer
    editor.get_buffer_manager_mut().set_current_buffer(buffer_id).unwrap();
    
    // Set the cursor position to the beginning of the buffer
    editor.get_cursor_manager_mut().set_position(CursorPosition::new(0, 0));
    
    // Test 'i' command (enter insert mode)
    editor.mode_manager.handle_key(&mut editor, 'i').unwrap();
    assert_eq!(editor.current_mode(), Mode::Insert);
    
    // Return to normal mode
    editor.mode_manager.enter_normal_mode();
    
    // Test 'a' command (enter insert mode after cursor)
    editor.mode_manager.handle_key(&mut editor, 'a').unwrap();
    assert_eq!(editor.current_mode(), Mode::Insert);
    assert_eq!(editor.cursor_position(), CursorPosition::new(0, 1));
    
    // Return to normal mode
    editor.mode_manager.enter_normal_mode();
    
    // Test 'A' command (enter insert mode at end of line)
    editor.mode_manager.handle_key(&mut editor, 'A').unwrap();
    assert_eq!(editor.current_mode(), Mode::Insert);
    assert_eq!(editor.cursor_position(), CursorPosition::new(0, 6));
    
    // Return to normal mode
    editor.mode_manager.enter_normal_mode();
    
    // Test 'o' command (open line below)
    editor.mode_manager.handle_key(&mut editor, 'o').unwrap();
    assert_eq!(editor.current_mode(), Mode::Insert);
    assert_eq!(editor.cursor_position(), CursorPosition::new(1, 0));
    
    // Return to normal mode
    editor.mode_manager.enter_normal_mode();
    
    // Test 'O' command (open line above)
    editor.mode_manager.handle_key(&mut editor, 'O').unwrap();
    assert_eq!(editor.current_mode(), Mode::Insert);
    assert_eq!(editor.cursor_position(), CursorPosition::new(1, 0));
    
    // Return to normal mode
    editor.mode_manager.enter_normal_mode();
    
    // Test 'v' command (enter visual mode)
    editor.mode_manager.handle_key(&mut editor, 'v').unwrap();
    assert_eq!(editor.current_mode(), Mode::Visual);
    
    // Return to normal mode
    editor.mode_manager.enter_normal_mode();
    
    // Test 'V' command (enter visual line mode)
    editor.mode_manager.handle_key(&mut editor, 'V').unwrap();
    assert_eq!(editor.current_mode(), Mode::VisualLine);
    
    // Return to normal mode
    editor.mode_manager.enter_normal_mode();
    
    // Test ':' command (enter command mode)
    editor.mode_manager.handle_key(&mut editor, ':').unwrap();
    assert_eq!(editor.current_mode(), Mode::Command);
}