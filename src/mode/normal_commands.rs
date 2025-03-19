// Normal mode command handler

// use crate::buffer::Buffer;
use crate::cursor::Direction;
use crate::editor::{Editor, EditorResult, TextObjectType};
// use crate::mode::Mode;
use crate::operator::{Operator, OperatorTarget};
use crate::register::RegisterType;
use crate::text_object::TextObjectType as TextObjectTypeExt;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::collections::HashMap;

/// Handler for normal mode commands
pub struct NormalCommandHandler {
    /// Current count prefix (e.g., 5 in 5dd)
    count: Option<usize>,
    /// Current operator (e.g., d in dd)
    operator: Option<Operator>,
    /// Current register (e.g., a in "ayy)
    register: Option<char>,
    /// Command mappings
    command_map: HashMap<char, Box<dyn Fn(&mut Editor) -> EditorResult<()> + Send + Sync>>,
    /// Operator mappings
    operator_map: HashMap<char, Operator>,
    /// Motion mappings
    motion_map: HashMap<char, Direction>,
    /// Text object mappings
    text_object_map: HashMap<char, TextObjectType>,
}

impl NormalCommandHandler {
    /// Create a new normal mode command handler
    pub fn new() -> Self {
        let mut handler = Self {
            count: None,
            operator: None,
            register: None,
            command_map: HashMap::new(),
            operator_map: HashMap::new(),
            motion_map: HashMap::new(),
            text_object_map: HashMap::new(),
        };
        
        handler.register_default_commands();
        handler.register_default_operators();
        handler.register_default_motions();
        handler.register_default_text_objects();
        
        handler
    }
    
    /// Register default commands
    fn register_default_commands(&mut self) {
        // Movement commands
        self.register_command('h', |editor| {
            if let Some(buffer_id) = editor.current_buffer_id() {
                let buffer = editor.get_buffer_manager().get_buffer(buffer_id)?;
                editor.get_cursor_manager_mut().move_cursor(Direction::Left, buffer)?;
            }
            Ok(())
        });
        
        self.register_command('j', |editor| {
            if let Some(buffer_id) = editor.current_buffer_id() {
                let buffer = editor.get_buffer_manager().get_buffer(buffer_id)?;
                editor.get_cursor_manager_mut().move_cursor(Direction::Down, buffer)?;
            }
            Ok(())
        });
        
        self.register_command('k', |editor| {
            if let Some(buffer_id) = editor.current_buffer_id() {
                let buffer = editor.get_buffer_manager().get_buffer(buffer_id)?;
                editor.get_cursor_manager_mut().move_cursor(Direction::Up, buffer)?;
            }
            Ok(())
        });
        
        self.register_command('l', |editor| {
            if let Some(buffer_id) = editor.current_buffer_id() {
                let buffer = editor.get_buffer_manager().get_buffer(buffer_id)?;
                editor.get_cursor_manager_mut().move_cursor(Direction::Right, buffer)?;
            }
            Ok(())
        });
        
        // Word movement
        self.register_command('w', |editor| {
            if let Some(buffer_id) = editor.current_buffer_id() {
                let buffer = editor.get_buffer_manager().get_buffer(buffer_id)?;
                editor.get_cursor_manager_mut().move_cursor(Direction::WordNext, buffer)?;
            }
            Ok(())
        });
        
        self.register_command('b', |editor| {
            if let Some(buffer_id) = editor.current_buffer_id() {
                let buffer = editor.get_buffer_manager().get_buffer(buffer_id)?;
                editor.get_cursor_manager_mut().move_cursor(Direction::WordPrev, buffer)?;
            }
            Ok(())
        });
        
        self.register_command('e', |editor| {
            if let Some(buffer_id) = editor.current_buffer_id() {
                let buffer = editor.get_buffer_manager().get_buffer(buffer_id)?;
                editor.get_cursor_manager_mut().move_cursor(Direction::WordEnd, buffer)?;
            }
            Ok(())
        });
        
        // Line movement
        self.register_command('0', |editor| {
            if let Some(buffer_id) = editor.current_buffer_id() {
                let buffer = editor.get_buffer_manager().get_buffer(buffer_id)?;
                editor.get_cursor_manager_mut().move_cursor(Direction::LineStart, buffer)?;
            }
            Ok(())
        });
        
        self.register_command('$', |editor| {
            if let Some(buffer_id) = editor.current_buffer_id() {
                let buffer = editor.get_buffer_manager().get_buffer(buffer_id)?;
                editor.get_cursor_manager_mut().move_cursor(Direction::LineEnd, buffer)?;
            }
            Ok(())
        });
        
        self.register_command('^', |editor| {
            if let Some(buffer_id) = editor.current_buffer_id() {
                let buffer = editor.get_buffer_manager().get_buffer(buffer_id)?;
                editor.get_cursor_manager_mut().move_cursor(Direction::LineFirstNonWhitespace, buffer)?;
            }
            Ok(())
        });
        
        // Buffer movement
        self.register_command('g', |editor| {
            if let Some(buffer_id) = editor.current_buffer_id() {
                let buffer = editor.get_buffer_manager().get_buffer(buffer_id)?;
                editor.get_cursor_manager_mut().move_cursor(Direction::BufferStart, buffer)?;
            }
            Ok(())
        });
        
        self.register_command('G', |editor| {
            if let Some(buffer_id) = editor.current_buffer_id() {
                let buffer = editor.get_buffer_manager().get_buffer(buffer_id)?;
                editor.get_cursor_manager_mut().move_cursor(Direction::BufferEnd, buffer)?;
            }
            Ok(())
        });
        
        // Mode switching
        self.register_command('i', |editor| {
            editor.start_insert_mode(false)?;
            Ok(())
        });
        
        self.register_command('a', |editor| {
            // Move cursor right before entering insert mode
            if let Some(buffer_id) = editor.current_buffer_id() {
                let buffer = editor.get_buffer_manager().get_buffer(buffer_id)?;
                editor.get_cursor_manager_mut().move_cursor(Direction::Right, buffer)?;
            }
            editor.start_insert_mode(false)?;
            Ok(())
        });
        
        self.register_command('A', |editor| {
            // Move cursor to end of line before entering insert mode
            if let Some(buffer_id) = editor.current_buffer_id() {
                let buffer = editor.get_buffer_manager().get_buffer(buffer_id)?;
                editor.get_cursor_manager_mut().move_cursor(Direction::LineEnd, buffer)?;
            }
            editor.start_insert_mode(false)?;
            Ok(())
        });
        
        self.register_command('I', |editor| {
            // Move cursor to first non-whitespace character before entering insert mode
            if let Some(buffer_id) = editor.current_buffer_id() {
                let buffer = editor.get_buffer_manager().get_buffer(buffer_id)?;
                editor.get_cursor_manager_mut().move_cursor(Direction::LineFirstNonWhitespace, buffer)?;
            }
            editor.start_insert_mode(false)?;
            Ok(())
        });
        
        self.register_command('o', |editor| {
            // Open line below
            if let Some(buffer_id) = editor.current_buffer_id() {
                let buffer = editor.get_buffer_manager_mut().get_buffer_mut(buffer_id)?;
                
                // Get cursor position
                let cursor_pos = editor.cursor_position();
                
                // Move to the end of the current line
                editor.get_cursor_manager_mut().move_cursor(Direction::LineEnd, buffer)?;
                
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
            Ok(())
        });
        
        self.register_command('O', |editor| {
            // Open line above
            if let Some(buffer_id) = editor.current_buffer_id() {
                let buffer = editor.get_buffer_manager_mut().get_buffer_mut(buffer_id)?;
                
                // Get cursor position
                let cursor_pos = editor.cursor_position();
                
                // Move to the beginning of the current line
                editor.get_cursor_manager_mut().move_cursor(Direction::LineStart, buffer)?;
                
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
            Ok(())
        });
        
        self.register_command('v', |editor| {
            editor.start_visual_mode(crate::visual::VisualMode::Char)?;
            Ok(())
        });
        
        self.register_command('V', |editor| {
            editor.start_visual_mode(crate::visual::VisualMode::Line)?;
            Ok(())
        });
        
        self.register_command(':', |editor| {
            editor.mode_manager.enter_command_mode();
            Ok(())
        });
        
        // Editing commands
        self.register_command('p', |editor| {
            editor.paste()?;
            Ok(())
        });
        
        self.register_command('P', |editor| {
            // Move cursor left, paste, then move cursor right
            if let Some(buffer_id) = editor.current_buffer_id() {
                let buffer = editor.get_buffer_manager().get_buffer(buffer_id)?;
                editor.get_cursor_manager_mut().move_cursor(Direction::Left, buffer)?;
                
                // Clone the buffer to avoid borrowing issues
                let buffer_id_copy = buffer_id;
                editor.paste()?;
                
                // Get the buffer again after paste
                let buffer = editor.get_buffer_manager().get_buffer(buffer_id_copy)?;
                editor.get_cursor_manager_mut().move_cursor(Direction::Right, buffer)?;
            }
            Ok(())
        });
        
        self.register_command('u', |editor| {
            editor.undo()?;
            Ok(())
        });
        
        self.register_command('r', |editor| {
            // Replace character (requires another key press)
            // This is handled in the handle_key method
            Ok(())
        });
        
        self.register_command('x', |editor| {
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
                        editor.set_register(RegisterType::Unnamed, register_content);
                    }
                    
                    // Now get a mutable reference to the buffer and perform the deletion
                    let buffer = editor.get_buffer_manager_mut().get_buffer_mut(buffer_id)?;
                    
                    // Delete the character at the cursor
                    buffer.delete(cursor_position, cursor_position + 1)?;
                }
            }
            Ok(())
        });
        
        self.register_command('X', |editor| {
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
                        editor.set_register(RegisterType::Unnamed, register_content);
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
            Ok(())
        });
        
        self.register_command('J', |editor| {
            // Join lines
            if let Some(buffer_id) = editor.current_buffer_id() {
                let cursor_pos = editor.cursor_position();
                editor.join_lines(buffer_id, cursor_pos.line, cursor_pos.line + 1)?;
            }
            Ok(())
        });
        
        // Search commands
        self.register_command('/', |editor| {
            editor.start_search(crate::search::SearchDirection::Forward)?;
            Ok(())
        });
        
        self.register_command('?', |editor| {
            editor.start_search(crate::search::SearchDirection::Backward)?;
            Ok(())
        });
        
        self.register_command('n', |editor| {
            editor.repeat_search(false)?;
            Ok(())
        });
        
        self.register_command('N', |editor| {
            editor.repeat_search(true)?;
            Ok(())
        });
    }
    
    /// Register default operators
    fn register_default_operators(&mut self) {
        self.operator_map.insert('d', Operator::Delete);
        self.operator_map.insert('c', Operator::Change);
        self.operator_map.insert('y', Operator::Yank);
    }
    
    /// Register default motions
    fn register_default_motions(&mut self) {
        self.motion_map.insert('h', Direction::Left);
        self.motion_map.insert('j', Direction::Down);
        self.motion_map.insert('k', Direction::Up);
        self.motion_map.insert('l', Direction::Right);
        self.motion_map.insert('w', Direction::WordNext);
        self.motion_map.insert('b', Direction::WordPrev);
        self.motion_map.insert('e', Direction::WordEnd);
        self.motion_map.insert('0', Direction::LineStart);
        self.motion_map.insert('$', Direction::LineEnd);
        self.motion_map.insert('^', Direction::LineFirstNonWhitespace);
        self.motion_map.insert('g', Direction::BufferStart);
        self.motion_map.insert('G', Direction::BufferEnd);
    }
    
    /// Register default text objects
    fn register_default_text_objects(&mut self) {
        self.text_object_map.insert('w', TextObjectType::Word);
        self.text_object_map.insert('W', TextObjectType::BigWord);
        self.text_object_map.insert('s', TextObjectType::Sentence);
        self.text_object_map.insert('p', TextObjectType::Paragraph);
        self.text_object_map.insert('(', TextObjectType::ParenBlock);
        self.text_object_map.insert(')', TextObjectType::ParenBlock);
        self.text_object_map.insert('{', TextObjectType::BraceBlock);
        self.text_object_map.insert('}', TextObjectType::BraceBlock);
        self.text_object_map.insert('[', TextObjectType::BracketBlock);
        self.text_object_map.insert(']', TextObjectType::BracketBlock);
        self.text_object_map.insert('<', TextObjectType::AngleBlock);
        self.text_object_map.insert('>', TextObjectType::AngleBlock);
        self.text_object_map.insert('\'', TextObjectType::SingleQuoteBlock);
        self.text_object_map.insert('"', TextObjectType::DoubleQuoteBlock);
        self.text_object_map.insert('`', TextObjectType::BacktickBlock);
        self.text_object_map.insert('t', TextObjectType::TagBlock);
    }
    
    /// Register a command
    pub fn register_command<F>(&mut self, key: char, handler: F)
    where
        F: Fn(&mut Editor) -> EditorResult<()> + Send + Sync + 'static,
    {
        self.command_map.insert(key, Box::new(handler));
    }
    
    /// Handle a key press in normal mode
    pub fn handle_key(&mut self, editor: &mut Editor, key: char) -> EditorResult<()> {
        // Check if we're in operator-pending mode
        if let Some(operator) = self.operator {
            // Check if the key is a motion
            if let Some(motion) = self.motion_map.get(&key) {
                // Execute the operator with the motion
                match operator {
                    Operator::Delete => editor.delete_to_motion(*motion)?,
                    Operator::Change => editor.change_to_motion(*motion)?,
                    Operator::Yank => { editor.yank_to_motion(*motion)?; },
                    _ => return Ok(()),
                }
                
                // Reset operator state
                self.operator = None;
                self.count = None;
                self.register = None;
                
                return Ok(());
            }
            
            // Check if the key is a text object
            if let Some(text_object) = self.text_object_map.get(&key) {
                // Execute the operator with the text object
                match operator {
                    Operator::Delete => editor.delete_text_object(*text_object, false)?,
                    Operator::Change => editor.change_text_object(*text_object, false)?,
                    Operator::Yank => { editor.yank_text_object(*text_object, false)?; },
                    _ => return Ok(()),
                }
                
                // Reset operator state
                self.operator = None;
                self.count = None;
                self.register = None;
                
                return Ok(());
            }
            
            // Check if the key is the same as the operator (e.g., dd, yy, cc)
            if let Some(op) = self.operator_map.get(&key) {
                if *op == operator {
                    // Get the current line
                    let cursor_pos = editor.cursor_position();
                    let line = cursor_pos.line;
                    
                    // Get the count (default to 1)
                    let count = self.count.unwrap_or(1);
                    
                    // Execute the operator on the line(s)
                    match operator {
                        Operator::Delete => editor.delete_lines(line, line + count - 1)?,
                        Operator::Change => editor.change_lines(line, line + count - 1)?,
                        Operator::Yank => { editor.yank_lines(line, line + count - 1)?; },
                        _ => return Ok(()),
                    }
                    
                    // Reset operator state
                    self.operator = None;
                    self.count = None;
                    self.register = None;
                    
                    return Ok(());
                }
            }
            
            // If we get here, the key is not a valid operator target
            // Reset operator state
            self.operator = None;
            self.count = None;
            self.register = None;
            
            return Ok(());
        }
        
        // Check if the key is a digit (for count prefix)
        if key.is_ascii_digit() {
            // If the key is '0' and we don't have a count yet, it's a movement command
            if key == '0' && self.count.is_none() {
                if let Some(handler) = self.command_map.get(&key) {
                    handler(editor)?;
                }
                return Ok(());
            }
            
            // Otherwise, update the count
            let digit = key.to_digit(10).unwrap() as usize;
            self.count = Some(self.count.unwrap_or(0) * 10 + digit);
            return Ok(());
        }
        
        // Check if the key is a register selector (")
        if key == '"' {
            // Wait for the next key to select the register
            // This is handled in the mode manager
            return Ok(());
        }
        
        // Check if the key is an operator
        if let Some(operator) = self.operator_map.get(&key) {
            self.operator = Some(*operator);
            return Ok(());
        }
        
        // Check if the key is a command
        if let Some(handler) = self.command_map.get(&key) {
            // Get the count (default to 1)
            let count = self.count.unwrap_or(1);
            
            // Execute the command count times
            for _ in 0..count {
                handler(editor)?;
            }
            
            // Reset count
            self.count = None;
            
            return Ok(());
        }
        
        // If we get here, the key is not a valid command
        Ok(())
    }
    
    /// Set the current register
    pub fn set_register(&mut self, register: char) {
        self.register = Some(register);
    }
    
    /// Get the current register
    pub fn get_register(&self) -> Option<char> {
        self.register
    }
    
    /// Reset the command state
    pub fn reset(&mut self) {
        self.count = None;
        self.operator = None;
        self.register = None;
    }
}