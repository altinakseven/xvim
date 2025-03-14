//! Selection module - Text selection and visual mode
//!
//! This module handles text selection in the editor, including visual mode
//! selection and operations on selected text.

use crate::buffer::Buffer;
use crate::buffer::BufferResult;
use crate::cursor::CursorPosition;

/// Selection types in visual mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectionType {
    /// Character-wise selection (Visual mode)
    Character,
    /// Line-wise selection (Visual Line mode)
    Line,
    /// Block-wise selection (Visual Block mode)
    Block,
}

/// Represents a text selection in the buffer
#[derive(Debug, Clone)]
pub struct Selection {
    /// The type of selection
    pub selection_type: SelectionType,
    /// The start position of the selection (anchor)
    pub start: CursorPosition,
    /// The end position of the selection (cursor)
    pub end: CursorPosition,
}

impl Selection {
    /// Create a new selection
    pub fn new(selection_type: SelectionType, start: CursorPosition) -> Self {
        Self {
            selection_type,
            start,
            end: start,
        }
    }

    /// Check if a position is within the selection
    pub fn contains(&self, position: &CursorPosition) -> bool {
        match self.selection_type {
            SelectionType::Character => {
                // For character selection, check if the position is between start and end
                let (start_line, start_col) = self.normalized_start();
                let (end_line, end_col) = self.normalized_end();

                if position.line < start_line || position.line > end_line {
                    return false;
                }

                if position.line == start_line && position.line == end_line {
                    // Selection is on a single line
                    position.column >= start_col && position.column <= end_col
                } else if position.line == start_line {
                    // First line of multi-line selection
                    position.column >= start_col
                } else if position.line == end_line {
                    // Last line of multi-line selection
                    position.column <= end_col
                } else {
                    // Middle line of multi-line selection
                    true
                }
            }
            SelectionType::Line => {
                // For line selection, check if the line is between start and end lines
                let start_line = self.normalized_start().0;
                let end_line = self.normalized_end().0;
                position.line >= start_line && position.line <= end_line
            }
            SelectionType::Block => {
                // For block selection, check if the position is within the block
                let (start_line, start_col) = self.normalized_start();
                let (end_line, end_col) = self.normalized_end();

                if position.line < start_line || position.line > end_line {
                    return false;
                }

                position.column >= start_col && position.column <= end_col
            }
        }
    }

    /// Get the normalized start position (top-left corner of selection)
    pub fn normalized_start(&self) -> (usize, usize) {
        match self.selection_type {
            SelectionType::Character => {
                if self.start.line < self.end.line || 
                   (self.start.line == self.end.line && self.start.column <= self.end.column) {
                    (self.start.line, self.start.column)
                } else {
                    (self.end.line, self.end.column)
                }
            }
            SelectionType::Line => {
                if self.start.line <= self.end.line {
                    (self.start.line, 0)
                } else {
                    (self.end.line, 0)
                }
            }
            SelectionType::Block => {
                let start_col = std::cmp::min(self.start.column, self.end.column);
                let start_line = std::cmp::min(self.start.line, self.end.line);
                (start_line, start_col)
            }
        }
    }

    /// Get the normalized end position (bottom-right corner of selection)
    pub fn normalized_end(&self) -> (usize, usize) {
        match self.selection_type {
            SelectionType::Character => {
                if self.start.line < self.end.line || 
                   (self.start.line == self.end.line && self.start.column <= self.end.column) {
                    (self.end.line, self.end.column)
                } else {
                    (self.start.line, self.start.column)
                }
            }
            SelectionType::Line => {
                let end_line = if self.start.line <= self.end.line {
                    self.end.line
                } else {
                    self.start.line
                };
                (end_line, usize::MAX) // End of line
            }
            SelectionType::Block => {
                let end_col = std::cmp::max(self.start.column, self.end.column);
                let end_line = std::cmp::max(self.start.line, self.end.line);
                (end_line, end_col)
            }
        }
    }

    /// Get the selected text from the buffer
    pub fn get_text(&self, buffer: &Buffer) -> BufferResult<String> {
        match self.selection_type {
            SelectionType::Character => {
                let (start_line, start_col) = self.normalized_start();
                let (end_line, end_col) = self.normalized_end();

                if start_line == end_line {
                    // Selection is on a single line
                    let line = buffer.line(start_line)?;
                    if start_col >= line.len() {
                        return Ok(String::new());
                    }
                    let end_col = std::cmp::min(end_col, line.len());
                    Ok(line[start_col..end_col].to_string())
                } else {
                    // Multi-line selection
                    let mut result = String::new();

                    // First line
                    let first_line = buffer.line(start_line)?;
                    if start_col < first_line.len() {
                        result.push_str(&first_line[start_col..]);
                    }
                    result.push('\n');

                    // Middle lines
                    for line_idx in start_line + 1..end_line {
                        result.push_str(&buffer.line(line_idx)?);
                        result.push('\n');
                    }

                    // Last line
                    let last_line = buffer.line(end_line)?;
                    let end_col = std::cmp::min(end_col, last_line.len());
                    result.push_str(&last_line[..end_col]);

                    Ok(result)
                }
            }
            SelectionType::Line => {
                let (start_line, _) = self.normalized_start();
                let (end_line, _) = self.normalized_end();

                let mut result = String::new();
                for line_idx in start_line..=end_line {
                    result.push_str(&buffer.line(line_idx)?);
                    result.push('\n');
                }

                Ok(result)
            }
            SelectionType::Block => {
                let (start_line, start_col) = self.normalized_start();
                let (end_line, end_col) = self.normalized_end();

                let mut result = String::new();
                for line_idx in start_line..=end_line {
                    let line = buffer.line(line_idx)?;
                    if start_col < line.len() {
                        let end_col = std::cmp::min(end_col, line.len());
                        result.push_str(&line[start_col..end_col]);
                    }
                    if line_idx < end_line {
                        result.push('\n');
                    }
                }

                Ok(result)
            }
        }
    }

    /// Update the end position of the selection
    pub fn update_end(&mut self, end: CursorPosition) {
        self.end = end;
    }
}

/// Manager for handling selections
pub struct SelectionManager {
    /// The current selection, if any
    current_selection: Option<Selection>,
}

impl SelectionManager {
    /// Create a new selection manager
    pub fn new() -> Self {
        Self {
            current_selection: None,
        }
    }

    /// Start a new selection
    pub fn start_selection(&mut self, selection_type: SelectionType, start: CursorPosition) {
        self.current_selection = Some(Selection::new(selection_type, start));
    }

    /// Update the current selection
    pub fn update_selection(&mut self, end: CursorPosition) {
        if let Some(selection) = &mut self.current_selection {
            selection.update_end(end);
        }
    }

    /// End the current selection
    pub fn end_selection(&mut self) -> Option<Selection> {
        self.current_selection.take()
    }

    /// Get the current selection
    pub fn current_selection(&self) -> Option<&Selection> {
        self.current_selection.as_ref()
    }

    /// Check if there is an active selection
    pub fn has_selection(&self) -> bool {
        self.current_selection.is_some()
    }

    /// Check if a position is within the current selection
    pub fn is_position_selected(&self, position: &CursorPosition) -> bool {
        if let Some(selection) = &self.current_selection {
            selection.contains(position)
        } else {
            false
        }
    }
}

impl Default for SelectionManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selection_character() {
        let start = CursorPosition::new(1, 2);
        let end = CursorPosition::new(1, 5);
        
        let mut selection = Selection::new(SelectionType::Character, start);
        selection.update_end(end);
        
        // Test normalized positions
        assert_eq!(selection.normalized_start(), (1, 2));
        assert_eq!(selection.normalized_end(), (1, 5));
        
        // Test contains
        assert!(selection.contains(&CursorPosition::new(1, 3)));
        assert!(selection.contains(&CursorPosition::new(1, 2)));
        assert!(selection.contains(&CursorPosition::new(1, 5)));
        assert!(!selection.contains(&CursorPosition::new(1, 1)));
        assert!(!selection.contains(&CursorPosition::new(1, 6)));
        assert!(!selection.contains(&CursorPosition::new(0, 3)));
        assert!(!selection.contains(&CursorPosition::new(2, 3)));
        
        // Test with reversed selection
        let start = CursorPosition::new(1, 5);
        let end = CursorPosition::new(1, 2);
        
        let mut selection = Selection::new(SelectionType::Character, start);
        selection.update_end(end);
        
        // Test normalized positions
        assert_eq!(selection.normalized_start(), (1, 2));
        assert_eq!(selection.normalized_end(), (1, 5));
    }
    
    #[test]
    fn test_selection_line() {
        let start = CursorPosition::new(1, 0);
        let end = CursorPosition::new(3, 0);
        
        let mut selection = Selection::new(SelectionType::Line, start);
        selection.update_end(end);
        
        // Test normalized positions
        assert_eq!(selection.normalized_start(), (1, 0));
        assert_eq!(selection.normalized_end(), (3, usize::MAX));
        
        // Test contains
        assert!(selection.contains(&CursorPosition::new(1, 0)));
        assert!(selection.contains(&CursorPosition::new(1, 10)));
        assert!(selection.contains(&CursorPosition::new(2, 5)));
        assert!(selection.contains(&CursorPosition::new(3, 0)));
        assert!(selection.contains(&CursorPosition::new(3, 20)));
        assert!(!selection.contains(&CursorPosition::new(0, 0)));
        assert!(!selection.contains(&CursorPosition::new(4, 0)));
        
        // Test with reversed selection
        let start = CursorPosition::new(3, 0);
        let end = CursorPosition::new(1, 0);
        
        let mut selection = Selection::new(SelectionType::Line, start);
        selection.update_end(end);
        
        // Test normalized positions
        assert_eq!(selection.normalized_start(), (1, 0));
        assert_eq!(selection.normalized_end(), (3, usize::MAX));
    }
    
    #[test]
    fn test_selection_block() {
        let start = CursorPosition::new(1, 2);
        let end = CursorPosition::new(3, 5);
        
        let mut selection = Selection::new(SelectionType::Block, start);
        selection.update_end(end);
        
        // Test normalized positions
        assert_eq!(selection.normalized_start(), (1, 2));
        assert_eq!(selection.normalized_end(), (3, 5));
        
        // Test contains
        assert!(selection.contains(&CursorPosition::new(1, 2)));
        assert!(selection.contains(&CursorPosition::new(1, 5)));
        assert!(selection.contains(&CursorPosition::new(2, 3)));
        assert!(selection.contains(&CursorPosition::new(3, 2)));
        assert!(selection.contains(&CursorPosition::new(3, 5)));
        assert!(!selection.contains(&CursorPosition::new(1, 1)));
        assert!(!selection.contains(&CursorPosition::new(1, 6)));
        assert!(!selection.contains(&CursorPosition::new(0, 3)));
        assert!(!selection.contains(&CursorPosition::new(4, 3)));
        
        // Test with reversed selection
        let start = CursorPosition::new(3, 5);
        let end = CursorPosition::new(1, 2);
        
        let mut selection = Selection::new(SelectionType::Block, start);
        selection.update_end(end);
        
        // Test normalized positions
        assert_eq!(selection.normalized_start(), (1, 2));
        assert_eq!(selection.normalized_end(), (3, 5));
    }
    
    #[test]
    fn test_selection_manager() {
        let mut manager = SelectionManager::new();
        
        // Test initial state
        assert!(!manager.has_selection());
        assert!(manager.current_selection().is_none());
        
        // Test starting a selection
        let start = CursorPosition::new(1, 2);
        manager.start_selection(SelectionType::Character, start);
        
        assert!(manager.has_selection());
        assert!(manager.current_selection().is_some());
        
        // Test updating a selection
        let end = CursorPosition::new(1, 5);
        manager.update_selection(end);
        
        let selection = manager.current_selection().unwrap();
        assert_eq!(selection.start, start);
        assert_eq!(selection.end, end);
        
        // Test ending a selection
        let selection = manager.end_selection().unwrap();
        assert_eq!(selection.start, start);
        assert_eq!(selection.end, end);
        
        assert!(!manager.has_selection());
        assert!(manager.current_selection().is_none());
    }
}