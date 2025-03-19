//! Insert mode module for xvim
//!
//! This module implements Insert mode functionality, including:
//! - Basic text insertion
//! - Auto-indentation
//! - Special key handling (backspace, enter, etc.)
//! - Insert mode commands (Ctrl-key combinations)
//! - Completion

use crate::buffer::Buffer;
use crate::command::completion_handlers::{CompletionContext, CompletionItem, CompletionType};
use crate::cursor::CursorPosition;
use crate::editor::{Editor, EditorError, EditorResult};
// use crate::mode::Mode;
use std::cmp::{max, min};
// use regex::Regex;

/// Insert mode state
#[derive(Debug, Clone)]
pub struct InsertState {
    /// Whether insert mode is active
    pub active: bool,
    /// Position where insert mode was started
    pub start_position: CursorPosition,
    /// Whether we're in replace mode rather than insert mode
    pub replace: bool,
    /// Whether we're in virtual edit mode
    pub virtual_edit: bool,
    /// Whether auto-indent is enabled
    pub auto_indent: bool,
    /// Text inserted during this insert mode session (for undo)
    pub inserted_text: String,
    /// Number of lines inserted during this insert mode session
    pub lines_inserted: usize,
    /// Whether completion is active
    pub completion_active: bool,
    /// Completion type
    pub completion_type: Option<CompletionType>,
    /// Base word for completion
    pub completion_base: String,
    /// Start position of the word being completed
    pub completion_start_pos: Option<CursorPosition>,
}

impl Default for InsertState {
    fn default() -> Self {
        Self {
            active: false,
            start_position: CursorPosition::new(0, 0),
            replace: false,
            virtual_edit: false,
            auto_indent: true,
            inserted_text: String::new(),
            lines_inserted: 0,
            completion_active: false,
            completion_type: None,
            completion_base: String::new(),
            completion_start_pos: None,
        }
    }
}

impl InsertState {
    /// Create a new insert state
    pub fn new() -> Self {
        Self::default()
    }

    /// Start insert mode
    pub fn start(&mut self, cursor: CursorPosition, replace: bool) {
        self.active = true;
        self.start_position = cursor;
        self.replace = replace;
        self.inserted_text.clear();
        self.lines_inserted = 0;
    }

    /// End insert mode
    pub fn end(&mut self) {
        self.active = false;
        self.inserted_text.clear();
        self.lines_inserted = 0;
    }

    /// Record inserted text
    pub fn record_insert(&mut self, text: &str) {
        self.inserted_text.push_str(text);
        self.lines_inserted += text.matches('\n').count();
    }

    /// Record deleted text (for backspace)
    pub fn record_delete(&mut self, _text: &str) {
        // For a complete implementation, we would record deletions
        // to properly support undo, but we'll keep it simple for now
    }
}

/// Insert mode functions for the editor
pub trait InsertFunctions {
    /// Start insert mode
    fn start_insert_mode(&mut self, replace: bool) -> EditorResult<()>;
    
    /// End insert mode
    fn end_insert_mode(&mut self) -> EditorResult<()>;
    
    /// Insert text at the cursor position
    fn insert_text(&mut self, text: &str) -> EditorResult<()>;
    
    /// Delete character before cursor (backspace)
    fn delete_char_before_cursor(&mut self) -> EditorResult<()>;
    
    /// Delete character at cursor (delete)
    fn delete_char_at_cursor(&mut self) -> EditorResult<()>;
    
    /// Insert a newline
    fn insert_newline(&mut self) -> EditorResult<()>;
    
    /// Get the insert state
    fn insert_state(&self) -> &InsertState;
    
    /// Get a mutable reference to the insert state
    fn insert_state_mut(&mut self) -> &mut InsertState;
    
    /// Trigger completion
    fn trigger_completion(&mut self, completion_type: CompletionType) -> EditorResult<()>;
    
    /// Move to the next completion
    fn next_completion(&mut self) -> EditorResult<()>;
    
    /// Move to the previous completion
    fn prev_completion(&mut self) -> EditorResult<()>;
    
    /// Accept the current completion
    fn accept_completion(&mut self) -> EditorResult<()>;
    
    /// Cancel the current completion
    fn cancel_completion(&mut self) -> EditorResult<()>;
    
    /// Update completion based on current text
    fn update_completion(&mut self) -> EditorResult<()>;
}

/// Extension trait for Buffer to handle auto-indentation
pub trait BufferInsertExt {
    /// Get the indentation for a line
    fn get_line_indent(&self, line: usize) -> String;
    
    /// Get the appropriate indentation for a new line after the given line
    fn get_auto_indent(&self, line: usize) -> String;
}

impl BufferInsertExt for Buffer {
    fn get_line_indent(&self, line: usize) -> String {
        if line >= self.line_count() {
            return String::new();
        }
        
        let line_text = match self.line(line) {
            Ok(text) => text,
            Err(_) => String::new(),
        };
        let indent = line_text.chars()
            .take_while(|&c| c == ' ' || c == '\t')
            .collect::<String>();
        
        indent
    }
    
    fn get_auto_indent(&self, line: usize) -> String {
        // Simple auto-indent: just copy the indentation from the previous line
        self.get_line_indent(line)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_insert_state() {
        let mut state = InsertState::new();
        assert_eq!(state.active, false);
        
        let cursor = CursorPosition::new(1, 0);
        state.start(cursor, false);
        assert_eq!(state.active, true);
        assert_eq!(state.start_position, cursor);
        assert_eq!(state.replace, false);
        
        state.record_insert("hello");
        assert_eq!(state.inserted_text, "hello");
        assert_eq!(state.lines_inserted, 0);
        
        state.record_insert("\nworld");
        assert_eq!(state.inserted_text, "hello\nworld");
        assert_eq!(state.lines_inserted, 1);
        
        state.end();
        assert_eq!(state.active, false);
        assert_eq!(state.inserted_text, "");
    }
}