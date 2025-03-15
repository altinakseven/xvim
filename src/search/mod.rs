//! Search module - Search functionality for xvim
//!
//! This module implements search functionality for xvim, including
//! forward and backward search, search history, and search highlighting.

use crate::buffer::Buffer;
use crate::editor::Editor;
use crate::cursor::CursorPosition;
use std::collections::VecDeque;

/// Maximum number of search patterns to store in history
const MAX_SEARCH_HISTORY: usize = 50;

/// Search direction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SearchDirection {
    /// Forward search (/)
    Forward,
    /// Backward search (?)
    Backward,
}

/// Search state
#[derive(Debug, Clone)]
pub struct SearchState {
    /// Current search pattern
    pattern: Option<String>,
    /// Current search direction
    direction: SearchDirection,
    /// Search history (most recent first)
    history: VecDeque<String>,
    /// Current position in history when navigating
    history_index: Option<usize>,
    /// Whether search is case-sensitive
    case_sensitive: bool,
    /// Last search results
    last_results: Vec<(usize, usize, String)>,
    /// Current result index
    current_result_index: Option<usize>,
}

impl SearchState {
    /// Create a new search state
    pub fn new() -> Self {
        Self {
            pattern: None,
            direction: SearchDirection::Forward,
            history: VecDeque::new(),
            history_index: None,
            case_sensitive: false,
            last_results: Vec::new(),
            current_result_index: None,
        }
    }

    /// Set the search pattern
    pub fn set_pattern(&mut self, pattern: String) {
        // Add to history if it's a new pattern
        if Some(&pattern) != self.pattern.as_ref() {
            // Remove any existing instances of this pattern
            self.history.retain(|p| p != &pattern);
            
            // Add to the front of the history
            self.history.push_front(pattern.clone());
            
            // Trim history if it's too long
            if self.history.len() > MAX_SEARCH_HISTORY {
                self.history.pop_back();
            }
        }
        
        self.pattern = Some(pattern);
        self.history_index = None;
        self.current_result_index = None;
    }

    /// Get the current search pattern
    pub fn pattern(&self) -> Option<&str> {
        self.pattern.as_deref()
    }

    /// Set the search direction
    pub fn set_direction(&mut self, direction: SearchDirection) {
        self.direction = direction;
    }

    /// Get the current search direction
    pub fn direction(&self) -> SearchDirection {
        self.direction
    }

    /// Set whether search is case-sensitive
    pub fn set_case_sensitive(&mut self, case_sensitive: bool) {
        self.case_sensitive = case_sensitive;
    }

    /// Get whether search is case-sensitive
    pub fn case_sensitive(&self) -> bool {
        self.case_sensitive
    }

    /// Get the next item in search history
    pub fn history_next(&mut self) -> Option<&str> {
        if self.history.is_empty() {
            return None;
        }

        let index = match self.history_index {
            Some(i) if i < self.history.len() - 1 => i + 1,
            _ => 0,
        };

        self.history_index = Some(index);
        self.history.get(index).map(|s| s.as_str())
    }

    /// Get the previous item in search history
    pub fn history_prev(&mut self) -> Option<&str> {
        if self.history.is_empty() {
            return None;
        }

        let index = match self.history_index {
            Some(i) if i > 0 => i - 1,
            Some(_) => self.history.len() - 1,
            None => 0,
        };

        self.history_index = Some(index);
        self.history.get(index).map(|s| s.as_str())
    }

    /// Set the search results
    pub fn set_results(&mut self, results: Vec<(usize, usize, String)>) {
        self.last_results = results;
        self.current_result_index = if self.last_results.is_empty() {
            None
        } else {
            Some(0)
        };
    }

    /// Get the current search results
    pub fn results(&self) -> &[(usize, usize, String)] {
        &self.last_results
    }

    /// Move to the next search result
    pub fn next_result(&mut self) -> Option<(usize, usize, String)> {
        if self.last_results.is_empty() {
            return None;
        }

        let index = match self.current_result_index {
            Some(i) if i < self.last_results.len() - 1 => i + 1,
            _ => 0, // Wrap around to the beginning
        };

        self.current_result_index = Some(index);
        self.last_results.get(index).cloned()
    }

    /// Move to the previous search result
    pub fn prev_result(&mut self) -> Option<(usize, usize, String)> {
        if self.last_results.is_empty() {
            return None;
        }

        let index = match self.current_result_index {
            Some(i) if i > 0 => i - 1,
            _ => self.last_results.len() - 1, // Wrap around to the end
        };

        self.current_result_index = Some(index);
        self.last_results.get(index).cloned()
    }

    /// Get the current result index
    pub fn current_result_index(&self) -> Option<usize> {
        self.current_result_index
    }

    /// Set the current result index
    pub fn set_current_result_index(&mut self, index: Option<usize>) {
        self.current_result_index = index;
    }

    /// Clear the search results
    pub fn clear_results(&mut self) {
        self.last_results.clear();
        self.current_result_index = None;
    }
}

impl Default for SearchState {
    fn default() -> Self {
        Self::new()
    }
}

/// Search functions for the editor
pub trait SearchFunctions {
    /// Start a search in the given direction
    fn start_search(&mut self, direction: SearchDirection) -> Result<(), crate::editor::EditorError>;
    
    /// Execute a search with the given pattern
    fn execute_search(&mut self, pattern: &str) -> Result<bool, crate::editor::EditorError>;
    
    /// Find the next occurrence of the current search pattern
    fn find_next_occurrence(&mut self) -> Result<bool, crate::editor::EditorError>;
    
    /// Find the previous occurrence of the current search pattern
    fn find_prev_occurrence(&mut self) -> Result<bool, crate::editor::EditorError>;
    
    /// Get the search state
    fn search_state(&self) -> &SearchState;
    
    /// Get a mutable reference to the search state
    fn search_state_mut(&mut self) -> &mut SearchState;
}

// The implementation of SearchFunctions for Editor is in editor/mod.rs

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_search_state() {
        let mut state = SearchState::new();
        
        // Test setting pattern
        state.set_pattern("test".to_string());
        assert_eq!(state.pattern(), Some("test"));
        
        // Test setting direction
        state.set_direction(SearchDirection::Backward);
        assert_eq!(state.direction(), SearchDirection::Backward);
        
        // Test setting case sensitivity
        state.set_case_sensitive(true);
        assert_eq!(state.case_sensitive(), true);
        
        // Test history
        state.set_pattern("pattern1".to_string());
        state.set_pattern("pattern2".to_string());
        state.set_pattern("pattern3".to_string());
        
        assert_eq!(state.history_next(), Some("pattern3"));
        assert_eq!(state.history_next(), Some("pattern2"));
        assert_eq!(state.history_next(), Some("pattern1"));
        assert_eq!(state.history_next(), Some("pattern3")); // Wrap around
        
        assert_eq!(state.history_prev(), Some("pattern2"));
        assert_eq!(state.history_prev(), Some("pattern1"));
        assert_eq!(state.history_prev(), Some("pattern3")); // Wrap around
    }
}