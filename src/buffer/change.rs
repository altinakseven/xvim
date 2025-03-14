//! Change tracking for undo/redo functionality
//!
//! This module implements change tracking for the buffer system, enabling
//! undo and redo operations. It uses a change history stack to record
//! modifications to the buffer.

use std::time::{Duration, Instant};

/// Types of changes that can be made to a buffer
#[derive(Debug, Clone, PartialEq)]
pub enum ChangeType {
    /// Insert text at a position
    Insert {
        /// Position where text was inserted (character index)
        position: usize,
        /// Text that was inserted
        text: String,
    },
    /// Delete text from a range
    Delete {
        /// Start position of deleted text (character index)
        start: usize,
        /// End position of deleted text (character index)
        end: usize,
        /// Text that was deleted
        text: String,
    },
    /// Replace text in a range
    Replace {
        /// Start position of replaced text (character index)
        start: usize,
        /// End position of replaced text (character index)
        end: usize,
        /// Text that was removed
        old_text: String,
        /// Text that was inserted
        new_text: String,
    },
}

impl ChangeType {
    /// Get the inverse of this change (for undo)
    pub fn inverse(&self) -> Self {
        match self {
            ChangeType::Insert { position, text } => ChangeType::Delete {
                start: *position,
                end: *position + text.chars().count(),
                text: text.clone(),
            },
            ChangeType::Delete { start, end, text } => ChangeType::Insert {
                position: *start,
                text: text.clone(),
            },
            ChangeType::Replace { start, end, old_text, new_text } => ChangeType::Replace {
                start: *start,
                end: *start + new_text.chars().count(),
                old_text: new_text.clone(),
                new_text: old_text.clone(),
            },
        }
    }
}

/// A change to a buffer
#[derive(Debug, Clone)]
pub struct Change {
    /// Type of change
    pub change_type: ChangeType,
    /// Timestamp when the change was made
    pub timestamp: Instant,
}

impl Change {
    /// Create a new change
    pub fn new(change_type: ChangeType) -> Self {
        Self {
            change_type,
            timestamp: Instant::now(),
        }
    }
    
    /// Get the inverse of this change (for undo)
    pub fn inverse(&self) -> Self {
        Self {
            change_type: self.change_type.inverse(),
            timestamp: Instant::now(),
        }
    }
}

/// A group of changes that should be undone/redone together
#[derive(Debug, Clone)]
pub struct ChangeGroup {
    /// Changes in this group
    pub changes: Vec<Change>,
    /// Timestamp when the group was started
    pub timestamp: Instant,
}

impl ChangeGroup {
    /// Create a new change group
    pub fn new() -> Self {
        Self {
            changes: Vec::new(),
            timestamp: Instant::now(),
        }
    }
    
    /// Add a change to this group
    pub fn add_change(&mut self, change: Change) {
        self.changes.push(change);
    }
    
    /// Check if this group is empty
    pub fn is_empty(&self) -> bool {
        self.changes.is_empty()
    }
    
    /// Get the inverse of this change group (for undo)
    pub fn inverse(&self) -> Self {
        let mut inverse = Self::new();
        // Add changes in reverse order
        for change in self.changes.iter().rev() {
            inverse.add_change(change.inverse());
        }
        inverse
    }
}

/// Change history for a buffer
#[derive(Debug, Clone)]
pub struct ChangeHistory {
    /// Undo stack (changes that can be undone)
    undo_stack: Vec<ChangeGroup>,
    /// Redo stack (changes that have been undone)
    redo_stack: Vec<ChangeGroup>,
    /// Current change group being built
    current_group: ChangeGroup,
    /// Maximum time between changes to be considered part of the same group
    group_timeout: Duration,
    /// Whether changes are being recorded
    recording: bool,
}

impl ChangeHistory {
    /// Create a new change history
    pub fn new() -> Self {
        Self {
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            current_group: ChangeGroup::new(),
            group_timeout: Duration::from_millis(500), // 500ms timeout for grouping changes
            recording: true,
        }
    }
    
    /// Record a change
    pub fn record_change(&mut self, change_type: ChangeType) {
        if !self.recording {
            return;
        }
        
        let change = Change::new(change_type);
        
        // Check if we should start a new group
        if self.current_group.is_empty() || 
           change.timestamp - self.current_group.timestamp > self.group_timeout {
            // Commit the current group if it's not empty
            if !self.current_group.is_empty() {
                let group = std::mem::replace(&mut self.current_group, ChangeGroup::new());
                self.undo_stack.push(group);
            }
            
            // Start a new group
            self.current_group = ChangeGroup::new();
        }
        
        // Add the change to the current group
        self.current_group.add_change(change);
        
        // Clear the redo stack when a new change is made
        self.redo_stack.clear();
    }
    
    /// Commit the current change group
    pub fn commit_current_group(&mut self) {
        if !self.current_group.is_empty() {
            let group = std::mem::replace(&mut self.current_group, ChangeGroup::new());
            self.undo_stack.push(group);
        }
    }
    
    /// Check if undo is available
    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty() || !self.current_group.is_empty()
    }
    
    /// Check if redo is available
    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }
    
    /// Get the next undo group
    pub fn undo(&mut self) -> Option<ChangeGroup> {
        // Commit the current group if it's not empty
        self.commit_current_group();
        
        // Pop the last change group from the undo stack
        if let Some(group) = self.undo_stack.pop() {
            // Push to the redo stack
            self.redo_stack.push(group.clone());
            
            // Return the original group for applying undo operations
            Some(group)
        } else {
            None
        }
    }
    
    /// Get the next redo group
    pub fn redo(&mut self) -> Option<ChangeGroup> {
        // Pop the last change group from the redo stack
        if let Some(group) = self.redo_stack.pop() {
            // Push to the undo stack
            self.undo_stack.push(group.clone());
            
            // Return the original group for applying redo operations
            Some(group)
        } else {
            None
        }
    }
    
    /// Start recording changes
    pub fn start_recording(&mut self) {
        self.recording = true;
    }
    
    /// Stop recording changes
    pub fn stop_recording(&mut self) {
        self.recording = false;
    }
    
    /// Clear the change history
    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
        self.current_group = ChangeGroup::new();
    }
    
    /// Set the group timeout
    pub fn set_group_timeout(&mut self, timeout: Duration) {
        self.group_timeout = timeout;
    }
}

impl Default for ChangeHistory {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_change_inverse() {
        // Test Insert inverse
        let insert = ChangeType::Insert {
            position: 5,
            text: "hello".to_string(),
        };
        let inverse = insert.inverse();
        assert_eq!(inverse, ChangeType::Delete {
            start: 5,
            end: 10,
            text: "hello".to_string(),
        });
        
        // Test Delete inverse
        let delete = ChangeType::Delete {
            start: 5,
            end: 10,
            text: "hello".to_string(),
        };
        let inverse = delete.inverse();
        assert_eq!(inverse, ChangeType::Insert {
            position: 5,
            text: "hello".to_string(),
        });
        
        // Test Replace inverse
        let replace = ChangeType::Replace {
            start: 5,
            end: 10,
            old_text: "hello".to_string(),
            new_text: "world".to_string(),
        };
        let inverse = replace.inverse();
        assert_eq!(inverse, ChangeType::Replace {
            start: 5,
            end: 10,
            old_text: "world".to_string(),
            new_text: "hello".to_string(),
        });
    }
    
    #[test]
    fn test_change_history_record() {
        let mut history = ChangeHistory::new();
        
        // Record a change
        history.record_change(ChangeType::Insert {
            position: 0,
            text: "hello".to_string(),
        });
        
        assert!(history.can_undo());
        assert!(!history.can_redo());
    }
    
    #[test]
    fn test_change_history_undo_redo() {
        let mut history = ChangeHistory::new();
        
        // Record some changes
        history.record_change(ChangeType::Insert {
            position: 0,
            text: "hello".to_string(),
        });
        
        // Force a new group
        std::thread::sleep(Duration::from_millis(600));
        
        history.record_change(ChangeType::Insert {
            position: 5,
            text: " world".to_string(),
        });
        
        // Undo the last change - with our new approach, this returns the original change
        let undo_group = history.undo().unwrap();
        assert_eq!(undo_group.changes.len(), 1);
        assert_eq!(undo_group.changes[0].change_type, ChangeType::Insert {
            position: 5,
            text: " world".to_string(),
        });
        
        // Redo the change
        let redo_group = history.redo().unwrap();
        assert_eq!(redo_group.changes.len(), 1);
        assert_eq!(redo_group.changes[0].change_type, ChangeType::Insert {
            position: 5,
            text: " world".to_string(),
        });
    }
}