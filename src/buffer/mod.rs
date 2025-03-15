//! Buffer module - Text storage and manipulation
//!
//! This module implements the buffer system for xvim, which is responsible for
//! storing and manipulating text content. It's based on the rope data structure
//! for efficient editing operations.

mod manager;
mod change;
mod syntax;

pub use manager::{BufferManager, BufferManagerError, BufferManagerResult};
pub use change::{Change, ChangeGroup, ChangeHistory, ChangeType};
pub use syntax::{BufferSyntax, BufferSyntaxExt};

use ropey::Rope;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufReader, Read, Write};
use std::path::{Path, PathBuf};
use std::cmp::min;
use regex::Regex;
use std::collections::HashMap;

// Forward declaration for VisualArea to avoid circular dependency
pub use crate::visual::VisualArea;

// Mark definitions (moved from mark module)
/// A mark is a named position in a buffer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Mark {
    /// The line number (0-based)
    pub line: usize,
    /// The column number (0-based)
    pub column: usize,
}

impl Mark {
    /// Create a new mark
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }
}

/// A collection of marks for a buffer
#[derive(Debug, Clone, Default)]
pub struct MarkMap {
    /// The marks, keyed by their name (a single character)
    marks: HashMap<char, Mark>,
}

impl MarkMap {
    /// Create a new mark map
    pub fn new() -> Self {
        Self {
            marks: HashMap::new(),
        }
    }

    /// Set a mark at the given position
    pub fn set_mark(&mut self, name: char, line: usize, column: usize) {
        self.marks.insert(name, Mark::new(line, column));
    }

    /// Get a mark by name
    pub fn get_mark(&self, name: char) -> Option<&Mark> {
        self.marks.get(&name)
    }

    /// Remove a mark by name
    pub fn remove_mark(&mut self, name: char) -> Option<Mark> {
        self.marks.remove(&name)
    }

    /// Clear all marks
    pub fn clear(&mut self) {
        self.marks.clear();
    }

    /// Get all marks
    pub fn all_marks(&self) -> impl Iterator<Item = (&char, &Mark)> {
        self.marks.iter()
    }
}

/// Errors that can occur during buffer operations
#[derive(Debug)]
pub enum BufferError {
    /// An I/O error occurred
    Io(io::Error),
    /// The buffer is read-only
    ReadOnly,
    /// Invalid position in the buffer
    InvalidPosition,
    /// Other errors
    Other(String),
}

impl std::fmt::Display for BufferError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BufferError::Io(err) => write!(f, "I/O error: {}", err),
            BufferError::ReadOnly => write!(f, "Buffer is read-only"),
            BufferError::InvalidPosition => write!(f, "Invalid position in buffer"),
            BufferError::Other(msg) => write!(f, "{}", msg),
        }
    }
}

impl Error for BufferError {}

impl From<io::Error> for BufferError {
    fn from(err: io::Error) -> Self {
        BufferError::Io(err)
    }
}

/// Result type for buffer operations
pub type BufferResult<T> = Result<T, BufferError>;

/// Buffer state flags
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BufferFlags {
    /// Buffer has never been loaded
    pub never_loaded: bool,
    /// Buffer has been modified since last save
    pub modified: bool,
    /// Buffer had read errors during loading
    pub read_error: bool,
    /// Buffer is a new file (not yet written to disk)
    pub new_file: bool,
}

impl Default for BufferFlags {
    fn default() -> Self {
        Self {
            never_loaded: true,
            modified: false,
            read_error: false,
            new_file: false,
        }
    }
}

/// Buffer type (similar to Vim's 'buftype' option)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BufferType {
    /// Normal buffer (associated with a file)
    Normal,
    /// Buffer not associated with a file
    NoFile,
    /// Quickfix list buffer
    QuickFix,
    /// Help buffer
    Help,
    /// Terminal buffer
    Terminal,
    /// Prompt buffer
    Prompt,
    /// Popup buffer
    Popup,
}

impl Default for BufferType {
    fn default() -> Self {
        Self::Normal
    }
}

/// A buffer represents a text document being edited
#[derive(Debug, Clone)]
pub struct Buffer {
    /// Unique identifier for the buffer
    id: usize,
    /// Short name of the buffer (filename without path)
    name: String,
    /// Full path to the file, if any
    file_path: Option<PathBuf>,
    /// The actual text content using a rope data structure
    content: Rope,
    /// Buffer state flags
    flags: BufferFlags,
    /// Buffer type
    buffer_type: BufferType,
    /// Whether the buffer is read-only
    read_only: bool,
    /// Whether the buffer is modifiable
    modifiable: bool,
    /// Number of windows displaying this buffer
    window_count: usize,
    /// Change history for undo/redo
    change_history: ChangeHistory,
    /// Marks in the buffer
    marks: MarkMap,
    /// Syntax highlighting data
    syntax: syntax::BufferSyntax,
    /// Visual area for 'gv' command
    pub(crate) visual_area: Option<crate::visual::VisualArea>,
}

impl Buffer {
    /// Create a new empty buffer
    pub fn new(id: usize) -> Self {
        Self {
            id,
            name: String::from("[No Name]"),
            file_path: None,
            content: Rope::new(),
            flags: BufferFlags {
                never_loaded: true,
                modified: false,
                read_error: false,
                new_file: true,
            },
            buffer_type: BufferType::Normal,
            read_only: false,
            modifiable: true,
            window_count: 0,
            change_history: ChangeHistory::new(),
            marks: MarkMap::new(),
            syntax: syntax::BufferSyntax::new(),
            visual_area: None,
        }
    }

    /// Create a buffer from a file
    pub fn from_file<P: AsRef<Path>>(id: usize, path: P) -> BufferResult<Self> {
        let path = path.as_ref();
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        
        let mut content = String::new();
        reader.read_to_string(&mut content)?;
        
        let file_name = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("[No Name]");
            
        let mut buffer = Self {
            id,
            name: file_name.to_string(),
            file_path: Some(path.to_path_buf()),
            content: Rope::from_str(&content),
            flags: BufferFlags {
                never_loaded: false,
                modified: false,
                read_error: false,
                new_file: false,
            },
            buffer_type: BufferType::Normal,
            read_only: false,
            modifiable: true,
            window_count: 0,
            change_history: ChangeHistory::new(),
            marks: MarkMap::new(),
            syntax: syntax::BufferSyntax::new(),
            visual_area: None,
        };
        
        // Try to auto-detect syntax based on file extension
        if let Ok(registry) = crate::syntax::create_default_registry() {
            buffer.auto_detect_syntax(&registry);
        }
        
        Ok(buffer)
    }
    
    /// Get the buffer ID
    pub fn id(&self) -> usize {
        self.id
    }
    
    /// Get the buffer name
    pub fn name(&self) -> &str {
        &self.name
    }
    
    /// Set the buffer name
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    
    /// Get the file path, if any
    pub fn file_path(&self) -> Option<&Path> {
        self.file_path.as_deref()
    }
    
    /// Get the buffer content as a string
    pub fn content(&self) -> String {
        self.content.to_string()
    }
    
    /// Get the number of lines in the buffer
    pub fn line_count(&self) -> usize {
        self.content.len_lines()
    }
    
    /// Get a specific line from the buffer
    pub fn line(&self, line_idx: usize) -> BufferResult<String> {
        if line_idx >= self.line_count() {
            return Err(BufferError::InvalidPosition);
        }
        
        let line_start = self.content.line_to_char(line_idx);
        let line_end = if line_idx + 1 < self.line_count() {
            self.content.line_to_char(line_idx + 1) - 1 // Exclude newline
        } else {
            self.content.len_chars()
        };
        
        Ok(self.content.slice(line_start..line_end).to_string())
    }
    
    /// Get the length of a line in the buffer
    pub fn line_length(&self, line_idx: usize) -> BufferResult<usize> {
        if line_idx >= self.line_count() {
            return Err(BufferError::InvalidPosition);
        }
        
        let line = self.line(line_idx)?;
        Ok(line.len())
    }
    
    /// Convert a line and column position to a character index
    pub fn position_to_char_idx(&self, line: usize, column: usize) -> BufferResult<usize> {
        if line >= self.line_count() {
            return Err(BufferError::InvalidPosition);
        }
        
        let line_content = self.line(line)?;
        if column > line_content.len() {
            return Err(BufferError::InvalidPosition);
        }
        
        let line_start = self.content.line_to_char(line);
        Ok(line_start + column)
    }
    
    /// Convert a character index to a cursor position
    pub fn char_idx_to_position(&self, idx: usize) -> BufferResult<crate::cursor::CursorPosition> {
        let (line, column) = self.char_idx_to_line_col(idx)?;
        Ok(crate::cursor::CursorPosition::new(line, column))
    }
    
    /// Convert a character index to a line and column
    pub fn char_idx_to_line_col(&self, idx: usize) -> BufferResult<(usize, usize)> {
        if idx > self.content.len_chars() {
            return Err(BufferError::InvalidPosition);
        }
        
        let mut line = 0;
        let mut col = 0;
        let mut current_idx = 0;
        
        for c in self.content.chars() {
            if current_idx == idx {
                break;
            }
            
            if c == '\n' {
                line += 1;
                col = 0;
            } else {
                col += 1;
            }
            
            current_idx += 1;
        }
        
        Ok((line, col))
    }
    
    /// Insert text at the specified position
    pub fn insert(&mut self, char_idx: usize, text: &str) -> BufferResult<()> {
        if !self.modifiable {
            return Err(BufferError::ReadOnly);
        }
        
        if char_idx > self.content.len_chars() {
            return Err(BufferError::InvalidPosition);
        }
        
        // Record the change for undo
        self.change_history.record_change(ChangeType::Insert {
            position: char_idx,
            text: text.to_string(),
        });
        
        self.content.insert(char_idx, text);
        self.flags.modified = true;
        
        Ok(())
    }
    
    /// Delete text in the specified range
    pub fn delete(&mut self, start: usize, end: usize) -> BufferResult<()> {
        if !self.modifiable {
            return Err(BufferError::ReadOnly);
        }
        
        if start > end || end > self.content.len_chars() {
            return Err(BufferError::InvalidPosition);
        }
        
        // Get the text being deleted for undo
        let deleted_text = self.content.slice(start..end).to_string();
        
        // Record the change for undo
        self.change_history.record_change(ChangeType::Delete {
            start,
            end,
            text: deleted_text,
        });
        
        self.content.remove(start..end);
        self.flags.modified = true;
        
        Ok(())
    }
    
    /// Check if the buffer has been modified
    pub fn is_modified(&self) -> bool {
        self.flags.modified
    }
    
    /// Set the modified flag
    pub fn set_modified(&mut self, modified: bool) {
        self.flags.modified = modified;
    }
    
    /// Check if the buffer is read-only
    pub fn is_read_only(&self) -> bool {
        self.read_only
    }
    
    /// Set the read-only flag
    pub fn set_read_only(&mut self, read_only: bool) {
        self.read_only = read_only;
        if read_only {
            self.modifiable = false;
        }
    }
    
    /// Check if the buffer is modifiable
    pub fn is_modifiable(&self) -> bool {
        self.modifiable
    }
    
    /// Set the modifiable flag
    pub fn set_modifiable(&mut self, modifiable: bool) {
        self.modifiable = modifiable;
        if !modifiable {
            self.read_only = true;
        }
    }
    
    /// Increment the window count
    pub fn increment_window_count(&mut self) {
        self.window_count += 1;
    }
    
    /// Decrement the window count
    pub fn decrement_window_count(&mut self) {
        if self.window_count > 0 {
            self.window_count -= 1;
        }
    }
    
    /// Get the window count
    pub fn window_count(&self) -> usize {
        self.window_count
    }
    
    /// Check if the buffer is hidden (loaded but not displayed)
    pub fn is_hidden(&self) -> bool {
        !self.flags.never_loaded && self.window_count == 0
    }
    
    /// Get the buffer type
    pub fn buffer_type(&self) -> BufferType {
        self.buffer_type
    }
    
    /// Set a mark at the current cursor position
    pub fn set_mark(&mut self, name: char, line: usize, column: usize) -> BufferResult<()> {
        if line >= self.line_count() {
            return Err(BufferError::InvalidPosition);
        }
        
        // Get the line to check column validity
        let line_content = self.line(line)?;
        if column > line_content.len() {
            return Err(BufferError::InvalidPosition);
        }
        
        self.marks.set_mark(name, line, column);
        Ok(())
    }
    
    /// Get a mark by name
    pub fn get_mark(&self, name: char) -> Option<&Mark> {
        self.marks.get_mark(name)
    }
    
    /// Remove a mark by name
    pub fn remove_mark(&mut self, name: char) -> Option<Mark> {
        self.marks.remove_mark(name)
    }
    
    /// Get all marks in the buffer
    pub fn get_all_marks(&self) -> impl Iterator<Item = (&char, &Mark)> {
        self.marks.all_marks()
    }
    
    /// Convert a mark position to a character index
    pub fn mark_to_char_idx(&self, mark: &Mark) -> BufferResult<usize> {
        if mark.line >= self.line_count() {
            return Err(BufferError::InvalidPosition);
        }
        
        let line_start = self.content.line_to_char(mark.line);
        let line_content = self.line(mark.line)?;
        
        if mark.column > line_content.len() {
            return Err(BufferError::InvalidPosition);
        }
        
        Ok(line_start + mark.column)
    }
    
    /// Jump to a mark position
    pub fn jump_to_mark(&self, name: char) -> BufferResult<Option<(usize, usize)>> {
        if let Some(mark) = self.get_mark(name) {
            if mark.line >= self.line_count() {
                return Err(BufferError::InvalidPosition);
            }
            
            let line_content = self.line(mark.line)?;
            if mark.column > line_content.len() {
                return Err(BufferError::InvalidPosition);
            }
            
            Ok(Some((mark.line, mark.column)))
        } else {
            Ok(None)
        }
    }
    
    /// Undo the last change
    pub fn undo(&mut self) -> BufferResult<bool> {
        if !self.modifiable {
            return Err(BufferError::ReadOnly);
        }
        
        if let Some(change_group) = self.change_history.undo() {
            // Apply each change in the group
            for change in &change_group.changes {
                match &change.change_type {
                    ChangeType::Insert { position, text } => {
                        // For undo, an Insert becomes a Delete
                        let end = *position + text.chars().count();
                        if *position < self.content.len_chars() && end <= self.content.len_chars() {
                            self.content.remove(*position..end);
                        }
                    },
                    ChangeType::Delete { start, end: _, text } => {
                        // For undo, a Delete becomes an Insert
                        if *start <= self.content.len_chars() {
                            self.content.insert(*start, text);
                        }
                    },
                    ChangeType::Replace { start, end: _, old_text, new_text: _ } => {
                        // For undo, a Replace reverts to the old text
                        let end = *start + old_text.chars().count();
                        if *start < self.content.len_chars() && end <= self.content.len_chars() {
                            self.content.remove(*start..end);
                            self.content.insert(*start, old_text);
                        }
                    },
                }
            }
            
            // Mark the buffer as modified
            self.flags.modified = true;
            
            Ok(true)
        } else {
            // No changes to undo
            Ok(false)
        }
    }
    
    /// Redo the last undone change
    pub fn redo(&mut self) -> BufferResult<bool> {
        if !self.modifiable {
            return Err(BufferError::ReadOnly);
        }
        
        if let Some(change_group) = self.change_history.redo() {
            // Apply each change in the group
            for change in &change_group.changes {
                match &change.change_type {
                    ChangeType::Insert { position, text } => {
                        // For redo, an Insert is applied as is
                        self.content.insert(*position, text);
                    },
                    ChangeType::Delete { start, end, text: _ } => {
                        // For redo, a Delete is applied as is
                        if *start < self.content.len_chars() && *end <= self.content.len_chars() {
                            self.content.remove(*start..*end);
                        }
                    },
                    ChangeType::Replace { start, end, old_text: _, new_text } => {
                        // For redo, a Replace applies the new text
                        if *start < self.content.len_chars() && *end <= self.content.len_chars() {
                            self.content.remove(*start..*end);
                            self.content.insert(*start, new_text);
                        }
                    },
                }
            }
            
            // Mark the buffer as modified
            self.flags.modified = true;
            
            Ok(true)
        } else {
            // No changes to redo
            Ok(false)
        }
    }
    
    /// Check if undo is available
    pub fn can_undo(&self) -> bool {
        self.change_history.can_undo()
    }
    
    /// Check if redo is available
    pub fn can_redo(&self) -> bool {
        self.change_history.can_redo()
    }
    
    /// Save the buffer to its file
    pub fn save(&mut self) -> BufferResult<()> {
        if let Some(path) = &self.file_path {
            let content = self.content.to_string();
            let mut file = File::create(path)?;
            file.write_all(content.as_bytes())?;
            
            // Reset modified flag
            self.flags.modified = false;
            
            // Commit the current change group
            self.change_history.commit_current_group();
            
            Ok(())
        } else {
            Err(BufferError::Other("No file path associated with buffer".to_string()))
        }
    }
    
    /// Save the buffer to a specific file
    pub fn save_as<P: AsRef<Path>>(&mut self, path: P) -> BufferResult<()> {
        let path_buf = path.as_ref().to_path_buf();
        let content = self.content.to_string();
        let mut file = File::create(&path_buf)?;
        file.write_all(content.as_bytes())?;
        
        // Update file path and name
        self.file_path = Some(path_buf.clone());
        if let Some(file_name) = path_buf.file_name().and_then(|n| n.to_str()) {
            self.name = file_name.to_string();
        }
        
        // Reset modified flag
        self.flags.modified = false;
        
        // Commit the current change group
        self.change_history.commit_current_group();
        
        Ok(())
    }
    
    /// Search for a pattern in the buffer
    /// Returns a vector of (line_number, column_number, match_text) tuples
    pub fn search(&self, pattern: &str, case_sensitive: bool) -> BufferResult<Vec<(usize, usize, String)>> {
        let regex_pattern = if case_sensitive {
            pattern.to_string()
        } else {
            format!("(?i){}", pattern)
        };
        
        let regex = match Regex::new(&regex_pattern) {
            Ok(r) => r,
            Err(_) => return Err(BufferError::Other("Invalid search pattern".to_string())),
        };
        
        let mut results = Vec::new();
        let content = self.content.to_string();
        
        for (line_idx, line) in content.lines().enumerate() {
            for mat in regex.find_iter(line) {
                let match_text = mat.as_str().to_string();
                results.push((line_idx, mat.start(), match_text));
            }
        }
        
        Ok(results)
    }
    
    /// Search and replace text in the buffer
    pub fn search_and_replace(&mut self, pattern: &str, replacement: &str, case_sensitive: bool) -> BufferResult<usize> {
        if !self.modifiable {
            return Err(BufferError::ReadOnly);
        }
        
        let regex_pattern = if case_sensitive {
            pattern.to_string()
        } else {
            format!("(?i){}", pattern)
        };
        
        let regex = match Regex::new(&regex_pattern) {
            Ok(r) => r,
            Err(_) => return Err(BufferError::Other("Invalid search pattern".to_string())),
        };
        
        let content = self.content.to_string();
        
        // Count the number of replacements
        let count = regex.find_iter(&content).count();
        
        if count > 0 {
            // Create the new content with replacements
            let new_content = regex.replace_all(&content, replacement).to_string();
            
            // Record the change for undo
            self.change_history.record_change(ChangeType::Replace {
                start: 0,
                end: self.content.len_chars(),
                old_text: content,
                new_text: new_content.clone(),
            });
            
            // Update the content
            self.content = Rope::from_str(&new_content);
            self.flags.modified = true;
        }
        
        Ok(count)
    }
    
    /// Find the next occurrence of a pattern starting from a position
    pub fn find_next(&self, pattern: &str, start_line: usize, start_col: usize, case_sensitive: bool) -> BufferResult<Option<(usize, usize, String)>> {
        let search_results = self.search(pattern, case_sensitive)?;
        
        if search_results.is_empty() {
            return Ok(None);
        }
        
        // If we're at the beginning of the first match, we should find the next one
        if start_line == 0 && start_col == 0 && !search_results.is_empty() && search_results[0].0 == 0 && search_results[0].1 == 0 {
            if search_results.len() > 1 {
                // Return the second match
                let (line, col, text) = search_results[1].clone();
                return Ok(Some((line, col, text)));
            } else {
                // If there's only one match, return it (wrap around)
                let (line, col, text) = search_results[0].clone();
                return Ok(Some((line, col, text)));
            }
        }
        
        // Find the first result that is after the current position
        for &(line, col, ref text) in &search_results {
            if line > start_line || (line == start_line && col > start_col) {
                return Ok(Some((line, col, text.clone())));
            }
        }
        
        // If we didn't find anything after the current position, wrap around to the beginning
        if !search_results.is_empty() {
            let (line, col, text) = search_results[0].clone();
            return Ok(Some((line, col, text)));
        }
        
        Ok(None)
    }
    
    /// Find the previous occurrence of a pattern starting from a position
    pub fn find_prev(&self, pattern: &str, start_line: usize, start_col: usize, case_sensitive: bool) -> BufferResult<Option<(usize, usize, String)>> {
        let search_results = self.search(pattern, case_sensitive)?;
        
        if search_results.is_empty() {
            return Ok(None);
        }
        
        // Special case for the beginning of the first match
        if start_line == 0 && start_col == 0 && !search_results.is_empty() && search_results[0].0 == 0 && search_results[0].1 == 0 {
            // Wrap around to the last match
            let (line, col, text) = search_results.last().unwrap().clone();
            return Ok(Some((line, col, text)));
        }
        
        // Find the last result that is before the current position
        let mut prev_match = None;
        for &(line, col, ref text) in &search_results {
            if line < start_line || (line == start_line && col < start_col) {
                prev_match = Some((line, col, text.clone()));
            } else {
                break;
            }
        }
        
        // If we didn't find anything before the current position, wrap around to the end
        if prev_match.is_none() {
            let (line, col, text) = search_results.last().unwrap().clone();
            prev_match = Some((line, col, text));
        }
        
        Ok(prev_match)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_new_buffer() {
        let buffer = Buffer::new(1);
        assert_eq!(buffer.id(), 1);
        assert_eq!(buffer.name(), "[No Name]");
        assert_eq!(buffer.line_count(), 1); // Empty buffer has one line
        assert!(buffer.is_modified() == false);
    }
    
    #[test]
    fn test_buffer_insert() {
        let mut buffer = Buffer::new(1);
        buffer.insert(0, "Hello, world!").unwrap();
        assert_eq!(buffer.content(), "Hello, world!");
        assert_eq!(buffer.line_count(), 1);
        assert!(buffer.is_modified());
    }
    
    #[test]
    fn test_buffer_delete() {
        let mut buffer = Buffer::new(1);
        buffer.insert(0, "Hello, world!").unwrap();
        buffer.delete(0, 7).unwrap();
        assert_eq!(buffer.content(), "world!");
        assert!(buffer.is_modified());
    }
    
    #[test]
    fn test_buffer_lines() {
        let mut buffer = Buffer::new(1);
        buffer.insert(0, "Line 1\nLine 2\nLine 3").unwrap();
        assert_eq!(buffer.line_count(), 3);
        assert_eq!(buffer.line(0).unwrap(), "Line 1");
        assert_eq!(buffer.line(1).unwrap(), "Line 2");
        assert_eq!(buffer.line(2).unwrap(), "Line 3");
    }
    
    #[test]
    fn test_buffer_undo_redo() {
        let mut buffer = Buffer::new(1);
        
        // Insert some text
        buffer.insert(0, "Hello").unwrap();
        assert_eq!(buffer.content(), "Hello");
        
        // Commit the first change group to ensure it's separate from the next one
        buffer.change_history.commit_current_group();
        
        // Insert more text
        buffer.insert(5, ", world!").unwrap();
        assert_eq!(buffer.content(), "Hello, world!");
        
        // Commit the second change group
        buffer.change_history.commit_current_group();
        
        // Undo the second insert
        let result = buffer.undo().unwrap();
        assert!(result);
        assert_eq!(buffer.content(), "Hello");
        
        // Redo the second insert
        let result = buffer.redo().unwrap();
        assert!(result);
        assert_eq!(buffer.content(), "Hello, world!");
        
        // Test can_undo and can_redo
        assert!(buffer.can_undo());
        assert!(!buffer.can_redo());
        
        // Clear history and verify no more undo/redo available
        buffer.change_history.clear();
        assert!(!buffer.can_undo());
        assert!(!buffer.can_redo());
        
        // Verify undo returns false when nothing to undo
        let result = buffer.undo().unwrap();
        assert!(!result);
    }
    
    #[test]
    fn test_buffer_save() {
        use tempfile::NamedTempFile;
        
        // Create a temporary file
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().to_path_buf();
        
        // Create a buffer and add content
        let mut buffer = Buffer::new(1);
        buffer.insert(0, "Test content").unwrap();
        
        // Save the buffer to the file
        buffer.save_as(&path).unwrap();
        
        // Check that the file contains the content
        let content = std::fs::read_to_string(&path).unwrap();
        assert_eq!(content, "Test content");
        
        // Check that the buffer is no longer marked as modified
        assert!(!buffer.is_modified());
        
        // Modify the buffer
        buffer.insert(12, " updated").unwrap();
        assert!(buffer.is_modified());
        
        // Save the buffer again
        buffer.save().unwrap();
        
        // Check that the file contains the updated content
        let content = std::fs::read_to_string(&path).unwrap();
        assert_eq!(content, "Test content updated");
    }
    
    #[test]
    fn test_buffer_search() {
        let mut buffer = Buffer::new(1);
        buffer.insert(0, "Hello, world!\nThis is a test.\nHello again!").unwrap();
        
        // Test case-sensitive search
        let results = buffer.search("Hello", true).unwrap();
        assert_eq!(results.len(), 2);
        assert_eq!(results[0], (0, 0, "Hello".to_string()));
        assert_eq!(results[1], (2, 0, "Hello".to_string()));
        
        // Test case-insensitive search
        let results = buffer.search("hello", false).unwrap();
        assert_eq!(results.len(), 2);
        
        // Test search with no matches
        let results = buffer.search("xyz", true).unwrap();
        assert_eq!(results.len(), 0);
    }
    
    #[test]
    fn test_buffer_search_and_replace() {
        let mut buffer = Buffer::new(1);
        buffer.insert(0, "Hello, world!\nThis is a test.\nHello again!").unwrap();
        
        // Test case-sensitive search and replace
        let count = buffer.search_and_replace("Hello", "Hi", true).unwrap();
        assert_eq!(count, 2);
        assert_eq!(buffer.content(), "Hi, world!\nThis is a test.\nHi again!");
        
        // Test case-insensitive search and replace
        buffer = Buffer::new(1);
        buffer.insert(0, "Hello, world!\nThis is a test.\nHELLO again!").unwrap();
        let count = buffer.search_and_replace("hello", "Hi", false).unwrap();
        assert_eq!(count, 2);
        assert_eq!(buffer.content(), "Hi, world!\nThis is a test.\nHi again!");
        
        // Test search and replace with no matches
        let count = buffer.search_and_replace("xyz", "abc", true).unwrap();
        assert_eq!(count, 0);
        assert_eq!(buffer.content(), "Hi, world!\nThis is a test.\nHi again!");
    }
    
    #[test]
    fn test_buffer_marks() {
        let mut buffer = Buffer::new(1);
        buffer.insert(0, "Line 1\nLine 2\nLine 3").unwrap();
        
        // Set some marks
        buffer.set_mark('a', 0, 2).unwrap(); // Line 1, column 2
        buffer.set_mark('b', 1, 3).unwrap(); // Line 2, column 3
        buffer.set_mark('c', 2, 4).unwrap(); // Line 3, column 4
        
        // Get marks
        let mark_a = buffer.get_mark('a').unwrap();
        assert_eq!(mark_a.line, 0);
        assert_eq!(mark_a.column, 2);
        
        let mark_b = buffer.get_mark('b').unwrap();
        assert_eq!(mark_b.line, 1);
        assert_eq!(mark_b.column, 3);
        
        // Jump to mark
        let pos = buffer.jump_to_mark('c').unwrap().unwrap();
        assert_eq!(pos, (2, 4));
        
        // Convert mark to character index
        let char_idx = buffer.mark_to_char_idx(mark_a).unwrap();
        assert_eq!(char_idx, 2); // "Li|ne 1" (| is position 2)
        
        let char_idx = buffer.mark_to_char_idx(mark_b).unwrap();
        assert_eq!(char_idx, 10); // "Line 1\nLin|e 2" (| is position 10)
        
        // Remove a mark
        let removed = buffer.remove_mark('a').unwrap();
        assert_eq!(removed.line, 0);
        assert_eq!(removed.column, 2);
        assert!(buffer.get_mark('a').is_none());
        
        // Test invalid mark positions
        assert!(buffer.set_mark('d', 5, 0).is_err()); // Invalid line
        assert!(buffer.set_mark('d', 0, 10).is_err()); // Invalid column
    }
    
    #[test]
    #[ignore] // TODO: Fix this test
    fn test_buffer_find_next_prev() {
        let mut buffer = Buffer::new(1);
        buffer.insert(0, "Hello, world!\nThis is a test.\nHello again!").unwrap();
        
        // Get all matches first to understand the order
        let results = buffer.search("Hello", true).unwrap();
        assert_eq!(results.len(), 2);
        assert_eq!(results[0], (0, 0, "Hello".to_string())); // First match at line 0, col 0
        assert_eq!(results[1], (2, 0, "Hello".to_string())); // Second match at line 2, col 0
        
        // Test find_next from beginning
        let result = buffer.find_next("Hello", 0, 0, true).unwrap();
        assert!(result.is_some());
        let (line, col, text) = result.unwrap();
        // The next match after line 0, col 0 should be line 2, col 0
        assert_eq!(line, 2);
        assert_eq!(col, 0);
        assert_eq!(text, "Hello");
        
        // Test find_next with wrap-around
        let result = buffer.find_next("Hello", 2, 1, true).unwrap();
        assert!(result.is_some());
        let (line, col, text) = result.unwrap();
        // After the last match, it should wrap to the first match
        assert_eq!(line, 0);
        assert_eq!(col, 0);
        assert_eq!(text, "Hello");
        
        // Test find_prev from middle
        let result = buffer.find_prev("Hello", 2, 1, true).unwrap();
        assert!(result.is_some());
        let (line, col, text) = result.unwrap();
        // The previous match before line 2, col 1 should be line 0, col 0
        assert_eq!(line, 0);
        assert_eq!(col, 0);
        assert_eq!(text, "Hello");
        
        // Test find_prev with wrap-around
        // We need to use a position that's not at the beginning of the first match
        let result = buffer.find_prev("Hello", 0, 1, true).unwrap();
        assert!(result.is_some());
        let (line, col, text) = result.unwrap();
        // Before the first match, it should wrap to the last match
        assert_eq!(line, 2);
        assert_eq!(col, 0);
        assert_eq!(text, "Hello");
    }
}