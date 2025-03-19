//! Mark command handlers
//!
//! This module implements handlers for mark commands.

use crate::command::{ExCommand, ExCommandError, ExCommandResult, ExCommandRegistry};
use std::sync::Mutex;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::{Arc, Mutex};
use std::sync::Mutex;
use std::sync::Arc;
use std::sync::Mutex;
use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::SystemTime;
use std::sync::Mutex;
use std::sync::Arc;
use std::sync::Mutex;

/// Mark position
#[derive(Debug, Clone, Copy)]
pub struct MarkPosition {
    /// Buffer ID
    pub buffer_id: usize,
    /// Line number
    pub line: usize,
    /// Column number
    pub column: usize,
    /// Timestamp
    pub timestamp: std::time::SystemTime,
}

/// Mark manager
#[derive(Debug)]
pub struct MarkManager {
    /// Marks by name
    pub marks: HashMap<char, MarkPosition>,
    /// Last jump position
    pub last_jump: Option<MarkPosition>,
    /// Jump list
    pub jump_list: Vec<MarkPosition>,
    /// Current position in the jump list
    pub jump_position: usize,
    /// Maximum jump list size
    pub max_jump_list_size: usize,
    /// Automatic marks
    pub auto_marks: bool,
}

impl MarkManager {
    /// Create a new mark manager
    pub fn new() -> Self {
        Self {
            marks: HashMap::new(),
            last_jump: None,
            jump_list: Vec::new(),
            jump_position: 0,
            max_jump_list_size: 100,
            auto_marks: true,
        }
    }

    /// Set a mark
    pub fn set_mark(&mut self, name: char, position: MarkPosition) -> ExCommandResult<()> {
        // Check if the mark name is valid
        if !is_valid_mark_name(name) {
            return Err(ExCommandError::InvalidArgument(format!("Invalid mark name: {}", name)));
        }
        
        // Set the mark
        self.marks.insert(name, position);
        
        Ok(())
    }

    /// Get a mark
    pub fn get_mark(&self, name: char) -> Option<MarkPosition> {
        self.marks.get(&name).copied()
    }

    /// Delete a mark
    pub fn delete_mark(&mut self, name: char) -> bool {
        self.marks.remove(&name).is_some()
    }

    /// Delete all marks
    pub fn delete_all_marks(&mut self) {
        self.marks.clear();
    }

    /// Delete marks in a range
    pub fn delete_marks_in_range(&mut self, start: char, end: char) -> usize {
        let mut count = 0;
        
        // Get all mark names in the range
        let marks_to_delete: Vec<char> = self.marks.keys()
            .filter(|&&name| name >= start && name <= end)
            .copied()
            .collect();
        
        // Delete each mark
        for name in marks_to_delete {
            self.marks.remove(&name);
            count += 1;
        }
        
        count
    }

    /// Delete marks in a buffer
    pub fn delete_marks_in_buffer(&mut self, buffer_id: usize) -> usize {
        let mut count = 0;
        
        // Get all mark names in the buffer
        let marks_to_delete: Vec<char> = self.marks.iter()
            .filter(|(_, &position)| position.buffer_id == buffer_id)
            .map(|(&name, _)| name)
            .collect();
        
        // Delete each mark
        for name in marks_to_delete {
            self.marks.remove(&name);
            count += 1;
        }
        
        count
    }

    /// Delete file marks
    pub fn delete_file_marks(&mut self) -> usize {
        let mut count = 0;
        
        // Get all file mark names
        let marks_to_delete: Vec<char> = self.marks.keys()
            .filter(|&&name| is_file_mark(name))
            .copied()
            .collect();
        
        // Delete each mark
        for name in marks_to_delete {
            self.marks.remove(&name);
            count += 1;
        }
        
        count
    }

    /// Delete local marks
    pub fn delete_local_marks(&mut self, buffer_id: usize) -> usize {
        let mut count = 0;
        
        // Get all local mark names in the buffer
        let marks_to_delete: Vec<char> = self.marks.iter()
            .filter(|(&name, &position)| is_local_mark(name) && position.buffer_id == buffer_id)
            .map(|(&name, _)| name)
            .collect();
        
        // Delete each mark
        for name in marks_to_delete {
            self.marks.remove(&name);
            count += 1;
        }
        
        count
    }

    /// Add a jump
    pub fn add_jump(&mut self, position: MarkPosition) -> ExCommandResult<()> {
        // Set the last jump position
        self.last_jump = Some(position);
        
        // Add the jump to the jump list
        if self.jump_position < self.jump_list.len() {
            // Remove jumps after the current position
            self.jump_list.truncate(self.jump_position);
        }
        
        // Add the new jump
        self.jump_list.push(position);
        
        // Update the jump position
        self.jump_position = self.jump_list.len();
        
        // Limit the jump list size
        if self.jump_list.len() > self.max_jump_list_size {
            // Remove the oldest jump
            self.jump_list.remove(0);
            
            // Update the jump position
            self.jump_position -= 1;
        }
        
        Ok(())
    }

    /// Jump to a position
    pub fn jump_to(&mut self, position: MarkPosition) -> ExCommandResult<()> {
        // Add the current position to the jump list
        self.add_jump(position)?;
        
        Ok(())
    }

    /// Jump back
    pub fn jump_back(&mut self) -> Option<MarkPosition> {
        if self.jump_position > 0 {
            // Update the jump position
            self.jump_position -= 1;
            
            // Get the jump position
            self.jump_list.get(self.jump_position).copied()
        } else {
            None
        }
    }

    /// Jump forward
    pub fn jump_forward(&mut self) -> Option<MarkPosition> {
        if self.jump_position < self.jump_list.len() - 1 {
            // Update the jump position
            self.jump_position += 1;
            
            // Get the jump position
            self.jump_list.get(self.jump_position).copied()
        } else {
            None
        }
    }

    /// Get the jump list
    pub fn get_jump_list(&self) -> &[MarkPosition] {
        &self.jump_list
    }

    /// Get the current jump position
    pub fn get_jump_position(&self) -> usize {
        self.jump_position
    }

    /// Set the maximum jump list size
    pub fn set_max_jump_list_size(&mut self, size: usize) {
        self.max_jump_list_size = size;
        
        // Limit the jump list size
        if self.jump_list.len() > self.max_jump_list_size {
            // Remove the oldest jumps
            let to_remove = self.jump_list.len() - self.max_jump_list_size;
            self.jump_list.drain(0..to_remove);
            
            // Update the jump position
            self.jump_position = self.jump_position.saturating_sub(to_remove);
        }
    }

    /// Set automatic marks
    pub fn set_auto_marks(&mut self, enabled: bool) {
        self.auto_marks = enabled;
    }

    /// Auto-mark a position
    pub fn auto_mark(&mut self, position: MarkPosition) -> ExCommandResult<()> {
        if self.auto_marks {
            // Set the ' mark (last jump position)
            self.set_mark('\'', position)?;
            
            // Set the ` mark (last jump position)
            self.set_mark('`', position)?;
        }
        
        Ok(())
    }

    /// Get all marks
    pub fn get_all_marks(&self) -> &HashMap<char, MarkPosition> {
        &self.marks
    }

    /// Get all marks in a buffer
    pub fn get_marks_in_buffer(&self, buffer_id: usize) -> Vec<(char, MarkPosition)> {
        self.marks.iter()
            .filter(|(_, &position)| position.buffer_id == buffer_id)
            .map(|(&name, &position)| (name, position))
            .collect()
    }

    /// Get all file marks
    pub fn get_file_marks(&self) -> Vec<(char, MarkPosition)> {
        self.marks.iter()
            .filter(|(&name, _)| is_file_mark(name))
            .map(|(&name, &position)| (name, position))
            .collect()
    }

    /// Get all local marks
    pub fn get_local_marks(&self, buffer_id: usize) -> Vec<(char, MarkPosition)> {
        self.marks.iter()
            .filter(|(&name, &position)| is_local_mark(name) && position.buffer_id == buffer_id)
            .map(|(&name, &position)| (name, position))
            .collect()
    }

    /// Get all numbered marks
    pub fn get_numbered_marks(&self) -> Vec<(char, MarkPosition)> {
        self.marks.iter()
            .filter(|(&name, _)| is_numbered_mark(name))
            .map(|(&name, &position)| (name, position))
            .collect()
    }

    /// Get all named marks
    pub fn get_named_marks(&self) -> Vec<(char, MarkPosition)> {
        self.marks.iter()
            .filter(|(&name, _)| is_named_mark(name))
            .map(|(&name, &position)| (name, position))
            .collect()
    }
}

/// Check if a mark name is valid
fn is_valid_mark_name(name: char) -> bool {
    is_local_mark(name) || is_file_mark(name) || is_special_mark(name)
}

/// Check if a mark is a local mark
fn is_local_mark(name: char) -> bool {
    (name >= 'a' && name <= 'z') || (name >= '0' && name <= '9')
}

/// Check if a mark is a file mark
fn is_file_mark(name: char) -> bool {
    name >= 'A' && name <= 'Z'
}

/// Check if a mark is a special mark
fn is_special_mark(name: char) -> bool {
    matches!(name, '\'' | '`' | '[' | ']' | '<' | '>' | '^' | '.')
}

/// Check if a mark is a numbered mark
fn is_numbered_mark(name: char) -> bool {
    name >= '0' && name <= '9'
}

/// Check if a mark is a named mark
fn is_named_mark(name: char) -> bool {
    (name >= 'a' && name <= 'z') || (name >= 'A' && name <= 'Z')
}

// Global mark manager
static mut MARK_MANAGER: Option<MarkManager> = None;

/// Initialize the mark manager
pub fn init_mark_manager() {
    unsafe {
        if MARK_MANAGER.is_none() {
            MARK_MANAGER = Some(MarkManager::new());
        }
    }
}

/// Register mark command handlers
pub fn register_mark_handlers(registry: &mut ExCommandRegistry) {
    // Initialize the mark manager
    init_mark_manager();
    
    // Register mark commands
    registry.register("mark", handle_mark);
    registry.register("ma", handle_mark);
    registry.register("marks", handle_marks);
    registry.register("delmarks", handle_delmarks);
    registry.register("delm", handle_delmarks);
    registry.register("jumps", handle_jumps);
    registry.register("ju", handle_jumps);
    registry.register("clearjumps", handle_clearjumps);
    registry.register("cle", handle_clearjumps);
}

/// Handle the :mark command
fn handle_mark(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match crate::command::handlers::EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Get the mark manager
    let mark_manager = unsafe {
        match &mut MARK_MANAGER {
            Some(manager) => manager,
            None => {
                init_mark_manager();
                match &mut MARK_MANAGER {
                    Some(manager) => manager,
                    None => return Err(ExCommandError::Other("Failed to initialize mark manager".to_string())),
                }
            }
        }
    };
    
    // Parse the command arguments
    let args = cmd.args_str();
    
    if args.is_empty() {
        return Err(ExCommandError::MissingArgument("Mark name required".to_string()));
    }
    
    // Get the mark name
    let mark_name = args.chars().next().unwrap();
    
    // Check if the mark name is valid
    if !is_valid_mark_name(mark_name) {
        return Err(ExCommandError::InvalidArgument(format!("Invalid mark name: {}", mark_name)));
    }
    
    // Get the current buffer ID
    let buffer_id = match editor.current_buffer_id() {
        Some(id) => id,
        None => return Err(ExCommandError::InvalidCommand("No buffer to set mark in".to_string())),
    };
    
    // Get the current cursor position
    let cursor_pos = editor.cursor_position();
    
    // Create the mark position
    let position = MarkPosition {
        buffer_id,
        line: cursor_pos.line,
        column: cursor_pos.column,
        timestamp: std::time::SystemTime::now(),
    };
    
    // Set the mark
    mark_manager.set_mark(mark_name, position)?;
    
    println!("Mark {} set", mark_name);
    Ok(())
}

/// Handle the :marks command
fn handle_marks(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match crate::command::handlers::EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Get the mark manager
    let mark_manager = unsafe {
        match &MARK_MANAGER {
            Some(manager) => manager,
            None => {
                init_mark_manager();
                match &MARK_MANAGER {
                    Some(manager) => manager,
                    None => return Err(ExCommandError::Other("Failed to initialize mark manager".to_string())),
                }
            }
        }
    };
    
    // Parse the command arguments
    let args = cmd.args_str();
    
    // Get the current buffer ID
    let buffer_id = match editor.current_buffer_id() {
        Some(id) => id,
        None => return Err(ExCommandError::InvalidCommand("No buffer to display marks for".to_string())),
    };
    
    // Display the marks
    println!("--- Marks ---");
    println!("mark line  col file/text");
    
    if args.is_empty() {
        // Display all marks
        for (name, position) in mark_manager.get_all_marks() {
            // Get the buffer name
            let buffer_name = match editor.get_buffer_name(position.buffer_id) {
                Some(name) => name,
                None => "[No Name]".to_string(),
            };
            
            println!(" {}   {:4} {:3}  {}", name, position.line, position.column, buffer_name);
        }
    } else {
        // Display specific marks
        for mark_name in args.chars() {
            if let Some(position) = mark_manager.get_mark(mark_name) {
                // Get the buffer name
                let buffer_name = match editor.get_buffer_name(position.buffer_id) {
                    Some(name) => name,
                    None => "[No Name]".to_string(),
                };
                
                println!(" {}   {:4} {:3}  {}", mark_name, position.line, position.column, buffer_name);
            }
        }
    }
    
    Ok(())
}

/// Handle the :delmarks command
fn handle_delmarks(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match crate::command::handlers::EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Get the mark manager
    let mark_manager = unsafe {
        match &mut MARK_MANAGER {
            Some(manager) => manager,
            None => {
                init_mark_manager();
                match &mut MARK_MANAGER {
                    Some(manager) => manager,
                    None => return Err(ExCommandError::Other("Failed to initialize mark manager".to_string())),
                }
            }
        }
    };
    
    // Parse the command arguments
    let args = cmd.args_str();
    
    if args.is_empty() {
        return Err(ExCommandError::MissingArgument("Mark names required".to_string()));
    }
    
    // Get the current buffer ID
    let buffer_id = match editor.current_buffer_id() {
        Some(id) => id,
        None => return Err(ExCommandError::InvalidCommand("No buffer to delete marks from".to_string())),
    };
    
    // Check for special cases
    if args == "!" {
        // Delete all marks except for file marks
        let count = mark_manager.delete_local_marks(buffer_id);
        println!("Deleted {} local marks", count);
        return Ok(());
    } else if args == "*" {
        // Delete all file marks
        let count = mark_manager.delete_file_marks();
        println!("Deleted {} file marks", count);
        return Ok(());
    }
    
    // Parse the arguments
    let mut count = 0;
    let mut i = 0;
    
    while i < args.len() {
        let c = args.chars().nth(i).unwrap();
        
        if c == '-' && i + 1 < args.len() {
            // Range of marks
            let start = args.chars().nth(i - 1).unwrap_or(' ');
            let end = args.chars().nth(i + 1).unwrap_or(' ');
            
            if is_valid_mark_name(start) && is_valid_mark_name(end) {
                // Delete marks in the range
                count += mark_manager.delete_marks_in_range(start, end);
            }
            
            i += 2;
        } else if is_valid_mark_name(c) {
            // Single mark
            if mark_manager.delete_mark(c) {
                count += 1;
            }
            
            i += 1;
        } else {
            // Skip invalid characters
            i += 1;
        }
    }
    
    println!("Deleted {} marks", count);
    Ok(())
}

/// Handle the :jumps command
fn handle_jumps(_cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match crate::command::handlers::EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Get the mark manager
    let mark_manager = unsafe {
        match &MARK_MANAGER {
            Some(manager) => manager,
            None => {
                init_mark_manager();
                match &MARK_MANAGER {
                    Some(manager) => manager,
                    None => return Err(ExCommandError::Other("Failed to initialize mark manager".to_string())),
                }
            }
        }
    };
    
    // Get the jump list
    let jump_list = mark_manager.get_jump_list();
    let jump_position = mark_manager.get_jump_position();
    
    if jump_list.is_empty() {
        println!("No jumps");
        return Ok(());
    }
    
    println!("--- Jump list ---");
    println!("jump line  col file/text");
    
    // Display the jump list
    for (i, jump) in jump_list.iter().enumerate() {
        let current_marker = if i == jump_position { ">" } else { " " };
        
        // Get the buffer name
        let buffer_name = match editor.get_buffer_name(jump.buffer_id) {
            Some(name) => name,
            None => "[No Name]".to_string(),
        };
        
        println!("{} {:3} {:4} {:3}  {}",
            current_marker,
            i,
            jump.line,
            jump.column,
            buffer_name
        );
    }
    
    Ok(())
}

/// Handle the :clearjumps command
fn handle_clearjumps(_cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the mark manager
    let mark_manager = unsafe {
        match &mut MARK_MANAGER {
            Some(manager) => manager,
            None => {
                init_mark_manager();
                match &mut MARK_MANAGER {
                    Some(manager) => manager,
                    None => return Err(ExCommandError::Other("Failed to initialize mark manager".to_string())),
                }
            }
        }
    };
    
    // Clear the jump list
    mark_manager.jump_list.clear();
    mark_manager.jump_position = 0;
    
    println!("Jump list cleared");
    Ok(())
}

/// Set a mark
pub fn set_mark(name: char, position: MarkPosition) -> ExCommandResult<()> {
    // Get the mark manager
    let mark_manager = unsafe {
        match &mut MARK_MANAGER {
            Some(manager) => manager,
            None => {
                init_mark_manager();
                match &mut MARK_MANAGER {
                    Some(manager) => manager,
                    None => return Err(ExCommandError::Other("Failed to initialize mark manager".to_string())),
                }
            }
        }
    };
    
    // Set the mark
    mark_manager.set_mark(name, position)
}

/// Get a mark
pub fn get_mark(name: char) -> Option<MarkPosition> {
    // Get the mark manager
    let mark_manager = unsafe {
        match &MARK_MANAGER {
            Some(manager) => manager,
            None => {
                init_mark_manager();
                match &MARK_MANAGER {
                    Some(manager) => manager,
                    None => return None,
                }
            }
        }
    };
    
    // Get the mark
    mark_manager.get_mark(name)
}

/// Add a jump
pub fn add_jump(position: MarkPosition) -> ExCommandResult<()> {
    // Get the mark manager
    let mark_manager = unsafe {
        match &mut MARK_MANAGER {
            Some(manager) => manager,
            None => {
                init_mark_manager();
                match &mut MARK_MANAGER {
                    Some(manager) => manager,
                    None => return Err(ExCommandError::Other("Failed to initialize mark manager".to_string())),
                }
            }
        }
    };
    
    // Add the jump
    mark_manager.add_jump(position)
}

/// Jump to a position
pub fn jump_to(position: MarkPosition) -> ExCommandResult<()> {
    // Get the mark manager
    let mark_manager = unsafe {
        match &mut MARK_MANAGER {
            Some(manager) => manager,
            None => {
                init_mark_manager();
                match &mut MARK_MANAGER {
                    Some(manager) => manager,
                    None => return Err(ExCommandError::Other("Failed to initialize mark manager".to_string())),
                }
            }
        }
    };
    
    // Jump to the position
    mark_manager.jump_to(position)
}

/// Jump back
pub fn jump_back() -> Option<MarkPosition> {
    // Get the mark manager
    let mark_manager = unsafe {
        match &mut MARK_MANAGER {
            Some(manager) => manager,
            None => {
                init_mark_manager();
                match &mut MARK_MANAGER {
                    Some(manager) => manager,
                    None => return None,
                }
            }
        }
    };
    
    // Jump back
    mark_manager.jump_back()
}

/// Jump forward
pub fn jump_forward() -> Option<MarkPosition> {
    // Get the mark manager
    let mark_manager = unsafe {
        match &mut MARK_MANAGER {
            Some(manager) => manager,
            None => {
                init_mark_manager();
                match &mut MARK_MANAGER {
                    Some(manager) => manager,
                    None => return None,
                }
            }
        }
    };
    
    // Jump forward
    mark_manager.jump_forward()
}

/// Auto-mark a position
pub fn auto_mark(position: MarkPosition) -> ExCommandResult<()> {
    // Get the mark manager
    let mark_manager = unsafe {
        match &mut MARK_MANAGER {
            Some(manager) => manager,
            None => {
                init_mark_manager();
                match &mut MARK_MANAGER {
                    Some(manager) => manager,
                    None => return Err(ExCommandError::Other("Failed to initialize mark manager".to_string())),
                }
            }
        }
    };
    
    // Auto-mark the position
    mark_manager.auto_mark(position)
}

/// Get the jump list
pub fn get_jump_list() -> Vec<MarkPosition> {
    // Get the mark manager
    let mark_manager = unsafe {
        match &MARK_MANAGER {
            Some(manager) => manager,
            None => {
                init_mark_manager();
                match &MARK_MANAGER {
                    Some(manager) => manager,
                    None => return Vec::new(),
                }
            }
        }
    };
    
    // Get the jump list
    mark_manager.get_jump_list().to_vec()
}

/// Get the current jump position
pub fn get_jump_position() -> usize {
    // Get the mark manager
    let mark_manager = unsafe {
        match &MARK_MANAGER {
            Some(manager) => manager,
            None => {
                init_mark_manager();
                match &MARK_MANAGER {
                    Some(manager) => manager,
                    None => return 0,
                }
            }
        }
    };
    
    // Get the jump position
    mark_manager.get_jump_position()
}