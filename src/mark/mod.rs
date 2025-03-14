//! Mark module
//!
//! This module implements marks for xvim, which are named positions in a buffer
//! that allow users to quickly jump to specific locations.

use std::collections::HashMap;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mark_creation() {
        let mark = Mark::new(10, 5);
        assert_eq!(mark.line, 10);
        assert_eq!(mark.column, 5);
    }

    #[test]
    fn test_mark_map() {
        let mut mark_map = MarkMap::new();
        
        // Set some marks
        mark_map.set_mark('a', 10, 5);
        mark_map.set_mark('b', 20, 10);
        
        // Get marks
        let mark_a = mark_map.get_mark('a').unwrap();
        assert_eq!(mark_a.line, 10);
        assert_eq!(mark_a.column, 5);
        
        let mark_b = mark_map.get_mark('b').unwrap();
        assert_eq!(mark_b.line, 20);
        assert_eq!(mark_b.column, 10);
        
        // Non-existent mark
        assert!(mark_map.get_mark('c').is_none());
        
        // Remove a mark
        let removed = mark_map.remove_mark('a').unwrap();
        assert_eq!(removed.line, 10);
        assert_eq!(removed.column, 5);
        assert!(mark_map.get_mark('a').is_none());
        
        // Clear all marks
        mark_map.clear();
        assert!(mark_map.get_mark('b').is_none());
    }
}