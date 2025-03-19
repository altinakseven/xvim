//! Normal mode command integration with the key mapping system
//!
//! This module provides functions to register Normal mode commands as key mappings.

use crate::editor::Editor;
use crate::keymap::{Command, KeyMapping, KeySequence};
use crate::mode::Mode;
use crate::mode::normal_commands::NormalCommandHandler;
use crossterm::event::{KeyCode, KeyModifiers};

/// Register a Normal mode command as a key mapping
pub fn register_normal_command(
    handler: &mut NormalCommandHandler,
    key_map: &mut crate::keymap::KeyMap,
    key: char,
    command_name: &str,
) {
    // Create a key sequence for the key
    let key_event = crossterm::event::KeyEvent::new(
        KeyCode::Char(key),
        KeyModifiers::NONE,
    );
    let key_sequence = KeySequence::from_key(key_event);
    
    // Create a key mapping for the command
    let key_mapping = KeyMapping::new(
        Mode::Normal,
        key_sequence,
        Command::BuiltIn(command_name.to_string()),
        false,
    );
    
    // Add the mapping to the key map
    key_map.add_mapping(key_mapping);
    
    // Register the command with the Normal command handler
    handler.register_command(key, move |editor| {
        // Execute the command
        editor.execute_key_command(Command::BuiltIn(command_name.to_string()))
    });
}

/// Register a Normal mode command with a custom handler
pub fn register_normal_command_with_handler<F>(
    handler: &mut NormalCommandHandler,
    key: char,
    command_handler: F,
)
where
    F: Fn(&mut Editor) -> crate::editor::EditorResult<()> + Send + Sync + 'static,
{
    // Register the command with the Normal command handler
    handler.register_command(key, command_handler);
}

/// Register a Normal mode command with a key sequence
pub fn register_normal_command_with_sequence(
    key_map: &mut crate::keymap::KeyMap,
    sequence: Vec<crossterm::event::KeyEvent>,
    command_name: &str,
) {
    // Create a key sequence for the sequence
    let key_sequence = KeySequence::new(sequence);
    
    // Create a key mapping for the command
    let key_mapping = KeyMapping::new(
        Mode::Normal,
        key_sequence,
        Command::BuiltIn(command_name.to_string()),
        false,
    );
    
    // Add the mapping to the key map
    key_map.add_mapping(key_mapping);
}

/// Register default Normal mode commands as key mappings
pub fn register_default_normal_commands(
    handler: &mut NormalCommandHandler,
    key_map: &mut crate::keymap::KeyMap,
) {
    // Movement commands
    register_normal_command(handler, key_map, 'h', "move_left");
    register_normal_command(handler, key_map, 'j', "move_down");
    register_normal_command(handler, key_map, 'k', "move_up");
    register_normal_command(handler, key_map, 'l', "move_right");
    
    // Word movement
    register_normal_command(handler, key_map, 'w', "word_next");
    register_normal_command(handler, key_map, 'b', "word_prev");
    register_normal_command(handler, key_map, 'e', "word_end");
    
    // Line movement
    register_normal_command(handler, key_map, '0', "line_start");
    register_normal_command(handler, key_map, '$', "line_end");
    register_normal_command(handler, key_map, '^', "line_first_non_whitespace");
    
    // Buffer movement
    register_normal_command(handler, key_map, 'g', "buffer_start");
    register_normal_command(handler, key_map, 'G', "buffer_end");
    
    // Mode switching
    register_normal_command(handler, key_map, 'i', "enter_insert_mode");
    register_normal_command(handler, key_map, 'a', "append");
    register_normal_command(handler, key_map, 'A', "append_end_of_line");
    register_normal_command(handler, key_map, 'I', "insert_start_of_line");
    register_normal_command(handler, key_map, 'o', "open_line_below");
    register_normal_command(handler, key_map, 'O', "open_line_above");
    register_normal_command(handler, key_map, 'v', "enter_visual_char_mode");
    register_normal_command(handler, key_map, 'V', "enter_visual_line_mode");
    register_normal_command(handler, key_map, ':', "enter_command_mode");
    
    // Editing commands
    register_normal_command(handler, key_map, 'p', "paste");
    register_normal_command(handler, key_map, 'P', "paste_before");
    register_normal_command(handler, key_map, 'u', "undo");
    register_normal_command(handler, key_map, 'r', "replace_char");
    register_normal_command(handler, key_map, 'x', "delete_char");
    register_normal_command(handler, key_map, 'X', "delete_char_before");
    register_normal_command(handler, key_map, 'J', "join_lines");
    
    // Search commands
    register_normal_command(handler, key_map, '/', "search_forward");
    register_normal_command(handler, key_map, '?', "search_backward");
    register_normal_command(handler, key_map, 'n', "search_next");
    register_normal_command(handler, key_map, 'N', "search_prev");
    
    // Operator commands
    register_normal_command(handler, key_map, 'd', "delete_operator");
    register_normal_command(handler, key_map, 'c', "change_operator");
    register_normal_command(handler, key_map, 'y', "yank_operator");
    
    // Register multi-key sequences
    let ctrl_v_sequence = vec![
        crossterm::event::KeyEvent::new(
            KeyCode::Char('v'),
            KeyModifiers::CONTROL,
        ),
    ];
    register_normal_command_with_sequence(key_map, ctrl_v_sequence, "enter_visual_block_mode");
    
    let gg_sequence = vec![
        crossterm::event::KeyEvent::new(
            KeyCode::Char('g'),
            KeyModifiers::NONE,
        ),
        crossterm::event::KeyEvent::new(
            KeyCode::Char('g'),
            KeyModifiers::NONE,
        ),
    ];
    register_normal_command_with_sequence(key_map, gg_sequence, "buffer_start");
    
    let gv_sequence = vec![
        crossterm::event::KeyEvent::new(
            KeyCode::Char('g'),
            KeyModifiers::NONE,
        ),
        crossterm::event::KeyEvent::new(
            KeyCode::Char('v'),
            KeyModifiers::NONE,
        ),
    ];
    register_normal_command_with_sequence(key_map, gv_sequence, "reselect_visual");
}

/// Execute a Normal mode command by name
pub fn execute_normal_command(editor: &mut Editor, command_name: &str) -> crate::editor::EditorResult<()> {
    match command_name {
        "move_left" => {
            if let Some(buffer_id) = editor.current_buffer_id() {
                let buffer = editor.get_buffer_manager().get_buffer(buffer_id)?;
                editor.get_cursor_manager_mut().move_cursor(crate::cursor::Direction::Left, buffer)?;
            }
        },
        "move_down" => {
            if let Some(buffer_id) = editor.current_buffer_id() {
                let buffer = editor.get_buffer_manager().get_buffer(buffer_id)?;
                editor.get_cursor_manager_mut().move_cursor(crate::cursor::Direction::Down, buffer)?;
            }
        },
        "move_up" => {
            if let Some(buffer_id) = editor.current_buffer_id() {
                let buffer = editor.get_buffer_manager().get_buffer(buffer_id)?;
                editor.get_cursor_manager_mut().move_cursor(crate::cursor::Direction::Up, buffer)?;
            }
        },
        "move_right" => {
            if let Some(buffer_id) = editor.current_buffer_id() {
                let buffer = editor.get_buffer_manager().get_buffer(buffer_id)?;
                editor.get_cursor_manager_mut().move_cursor(crate::cursor::Direction::Right, buffer)?;
            }
        },
        "word_next" => {
            if let Some(buffer_id) = editor.current_buffer_id() {
                let buffer = editor.get_buffer_manager().get_buffer(buffer_id)?;
                editor.get_cursor_manager_mut().move_cursor(crate::cursor::Direction::WordNext, buffer)?;
            }
        },
        "word_prev" => {
            if let Some(buffer_id) = editor.current_buffer_id() {
                let buffer = editor.get_buffer_manager().get_buffer(buffer_id)?;
                editor.get_cursor_manager_mut().move_cursor(crate::cursor::Direction::WordPrev, buffer)?;
            }
        },
        "word_end" => {
            if let Some(buffer_id) = editor.current_buffer_id() {
                let buffer = editor.get_buffer_manager().get_buffer(buffer_id)?;
                editor.get_cursor_manager_mut().move_cursor(crate::cursor::Direction::WordEnd, buffer)?;
            }
        },
        "line_start" => {
            if let Some(buffer_id) = editor.current_buffer_id() {
                let buffer = editor.get_buffer_manager().get_buffer(buffer_id)?;
                editor.get_cursor_manager_mut().move_cursor(crate::cursor::Direction::LineStart, buffer)?;
            }
        },
        "line_end" => {
            if let Some(buffer_id) = editor.current_buffer_id() {
                let buffer = editor.get_buffer_manager().get_buffer(buffer_id)?;
                editor.get_cursor_manager_mut().move_cursor(crate::cursor::Direction::LineEnd, buffer)?;
            }
        },
        "line_first_non_whitespace" => {
            if let Some(buffer_id) = editor.current_buffer_id() {
                let buffer = editor.get_buffer_manager().get_buffer(buffer_id)?;
                editor.get_cursor_manager_mut().move_cursor(crate::cursor::Direction::LineFirstNonWhitespace, buffer)?;
            }
        },
        "buffer_start" => {
            if let Some(buffer_id) = editor.current_buffer_id() {
                let buffer = editor.get_buffer_manager().get_buffer(buffer_id)?;
                editor.get_cursor_manager_mut().move_cursor(crate::cursor::Direction::BufferStart, buffer)?;
            }
        },
        "buffer_end" => {
            if let Some(buffer_id) = editor.current_buffer_id() {
                let buffer = editor.get_buffer_manager().get_buffer(buffer_id)?;
                editor.get_cursor_manager_mut().move_cursor(crate::cursor::Direction::BufferEnd, buffer)?;
            }
        },
        "enter_insert_mode" => {
            editor.start_insert_mode(false)?;
        },
        "append" => {
            // Move cursor right before entering insert mode
            if let Some(buffer_id) = editor.current_buffer_id() {
                let buffer = editor.get_buffer_manager().get_buffer(buffer_id)?;
                editor.get_cursor_manager_mut().move_cursor(crate::cursor::Direction::Right, buffer)?;
            }
            editor.start_insert_mode(false)?;
        },
        "append_end_of_line" => {
            // Move cursor to end of line before entering insert mode
            if let Some(buffer_id) = editor.current_buffer_id() {
                let buffer = editor.get_buffer_manager().get_buffer(buffer_id)?;
                editor.get_cursor_manager_mut().move_cursor(crate::cursor::Direction::LineEnd, buffer)?;
            }
            editor.start_insert_mode(false)?;
        },
        "insert_start_of_line" => {
            // Move cursor to first non-whitespace character before entering insert mode
            if let Some(buffer_id) = editor.current_buffer_id() {
                let buffer = editor.get_buffer_manager().get_buffer(buffer_id)?;
                editor.get_cursor_manager_mut().move_cursor(crate::cursor::Direction::LineFirstNonWhitespace, buffer)?;
            }
            editor.start_insert_mode(false)?;
        },
        "open_line_below" => {
            // Open line below
            if let Some(buffer_id) = editor.current_buffer_id() {
                let buffer = editor.get_buffer_manager_mut().get_buffer_mut(buffer_id)?;
                
                // Get cursor position
                let cursor_pos = editor.cursor_position();
                
                // Move to the end of the current line
                editor.get_cursor_manager_mut().move_cursor(crate::cursor::Direction::LineEnd, buffer)?;
                
                // Get the updated cursor position
                let end_pos = editor.cursor_position();
                
                // Convert cursor position to character index
                let cursor_position = buffer.position_to_char_idx(end_pos.line, end_pos.column)?;
                
                // Insert a newline at the cursor position
                buffer.insert(cursor_position, "\n")?;
                
                // Update cursor position to the beginning of the new line
                let new_position = buffer.char_idx_to_position(cursor_position + 1)?;
                editor.get_cursor_manager_mut().set_position(new_position);
                
                // Enter insert mode
                editor.start_insert_mode(false)?;
            }
        },
        "open_line_above" => {
            // Open line above
            if let Some(buffer_id) = editor.current_buffer_id() {
                let buffer = editor.get_buffer_manager_mut().get_buffer_mut(buffer_id)?;
                
                // Get cursor position
                let cursor_pos = editor.cursor_position();
                
                // Move to the beginning of the current line
                editor.get_cursor_manager_mut().move_cursor(crate::cursor::Direction::LineStart, buffer)?;
                
                // Get the updated cursor position
                let start_pos = editor.cursor_position();
                
                // Convert cursor position to character index
                let cursor_position = buffer.position_to_char_idx(start_pos.line, start_pos.column)?;
                
                // Insert a newline at the cursor position
                buffer.insert(cursor_position, "\n")?;
                
                // Update cursor position to the beginning of the original line
                let new_position = buffer.char_idx_to_position(cursor_position)?;
                editor.get_cursor_manager_mut().set_position(new_position);
                
                // Enter insert mode
                editor.start_insert_mode(false)?;
            }
        },
        "enter_visual_char_mode" => {
            editor.start_visual_mode(crate::visual::VisualMode::Char)?;
        },
        "enter_visual_line_mode" => {
            editor.start_visual_mode(crate::visual::VisualMode::Line)?;
        },
        "enter_visual_block_mode" => {
            editor.start_visual_mode(crate::visual::VisualMode::Block)?;
        },
        "enter_command_mode" => {
            editor.mode_manager.enter_command_mode();
        },
        "paste" => {
            editor.paste()?;
        },
        "paste_before" => {
            // Move cursor left, paste, then move cursor right
            if let Some(buffer_id) = editor.current_buffer_id() {
                let buffer = editor.get_buffer_manager().get_buffer(buffer_id)?;
                editor.get_cursor_manager_mut().move_cursor(crate::cursor::Direction::Left, buffer)?;
                
                // Clone the buffer to avoid borrowing issues
                let buffer_id_copy = buffer_id;
                editor.paste()?;
                
                // Get the buffer again after paste
                let buffer = editor.get_buffer_manager().get_buffer(buffer_id_copy)?;
                editor.get_cursor_manager_mut().move_cursor(crate::cursor::Direction::Right, buffer)?;
            }
        },
        "undo" => {
            editor.undo()?;
        },
        "replace_char" => {
            // This is handled in the handle_key method
        },
        "delete_char" => {
            // Delete character at cursor
            if let Some(buffer_id) = editor.current_buffer_id() {
                let buffer = editor.get_buffer_manager().get_buffer(buffer_id)?;
                
                // Get cursor position
                let cursor_pos = editor.cursor_position();
                
                // Convert cursor position to character index
                let cursor_position = buffer.position_to_char_idx(cursor_pos.line, cursor_pos.column)?;
                
                // Get the content length
                let content_len = buffer.content().len();
                
                // Only delete if we're not at the end of the buffer
                if cursor_position < content_len {
                    // Get the character to be deleted
                    let content = buffer.content();
                    let char_to_delete = if cursor_position < content.len() {
                        content[cursor_position..cursor_position + 1].to_string()
                    } else {
                        String::new()
                    };
                    
                    // Store the text in the unnamed register
                    if !char_to_delete.is_empty() {
                        let register_content = crate::register::RegisterContent::character_wise(&char_to_delete);
                        editor.set_register(crate::register::RegisterType::Unnamed, register_content);
                    }
                    
                    // Now get a mutable reference to the buffer and perform the deletion
                    let buffer = editor.get_buffer_manager_mut().get_buffer_mut(buffer_id)?;
                    
                    // Delete the character at the cursor
                    buffer.delete(cursor_position, cursor_position + 1)?;
                }
            }
        },
        "delete_char_before" => {
            // Delete character before cursor
            if let Some(buffer_id) = editor.current_buffer_id() {
                let buffer = editor.get_buffer_manager().get_buffer(buffer_id)?;
                
                // Get cursor position
                let cursor_pos = editor.cursor_position();
                
                // Convert cursor position to character index
                let cursor_position = buffer.position_to_char_idx(cursor_pos.line, cursor_pos.column)?;
                
                // Only delete if we're not at the beginning of the buffer
                if cursor_position > 0 {
                    // Get the character to be deleted
                    let content = buffer.content();
                    let char_to_delete = if cursor_position <= content.len() {
                        content[cursor_position - 1..cursor_position].to_string()
                    } else {
                        String::new()
                    };
                    
                    // Store the text in the unnamed register
                    if !char_to_delete.is_empty() {
                        let register_content = crate::register::RegisterContent::character_wise(&char_to_delete);
                        editor.set_register(crate::register::RegisterType::Unnamed, register_content);
                    }
                    
                    // Now get a mutable reference to the buffer and perform the deletion
                    let buffer = editor.get_buffer_manager_mut().get_buffer_mut(buffer_id)?;
                    
                    // Delete the character before the cursor
                    buffer.delete(cursor_position - 1, cursor_position)?;
                    
                    // Update cursor position
                    let new_position = buffer.char_idx_to_position(cursor_position - 1)?;
                    editor.get_cursor_manager_mut().set_position(new_position);
                }
            }
        },
        "join_lines" => {
            // Join lines
            if let Some(buffer_id) = editor.current_buffer_id() {
                let cursor_pos = editor.cursor_position();
                editor.join_lines(buffer_id, cursor_pos.line, cursor_pos.line + 1)?;
            }
        },
        "search_forward" => {
            editor.start_search(crate::search::SearchDirection::Forward)?;
        },
        "search_backward" => {
            editor.start_search(crate::search::SearchDirection::Backward)?;
        },
        "search_next" => {
            editor.repeat_search(false)?;
        },
        "search_prev" => {
            editor.repeat_search(true)?;
        },
        "delete_operator" => {
            // This is handled in the handle_key method
        },
        "change_operator" => {
            // This is handled in the handle_key method
        },
        "yank_operator" => {
            // This is handled in the handle_key method
        },
        "reselect_visual" => {
            editor.reselect_visual_area()?;
        },
        _ => {
            return Err(crate::editor::EditorError::Other(format!("Unknown command: {}", command_name)));
        }
    }
    
    Ok(())
}