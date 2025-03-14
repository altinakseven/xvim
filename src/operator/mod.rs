//! Operator module - Implementation of Vim's operators
//!
//! This module implements Vim's operators, such as delete, change, yank, etc.
//! Operators are commands that operate on text objects or motions.

use crate::text_object::TextObjectType;
use crate::cursor::Direction;

/// The different operators
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
    /// Delete text
    Delete,
    /// Change text (delete and enter insert mode)
    Change,
    /// Yank (copy) text
    Yank,
    /// Indent text
    Indent,
    /// Outdent text
    Outdent,
    /// Format text
    Format,
    /// Fold text
    Fold,
    /// Unfold text
    Unfold,
    /// Convert to uppercase
    ToUpper,
    /// Convert to lowercase
    ToLower,
    /// Swap case
    SwapCase,
    /// Filter through external command
    Filter,
}

impl Operator {
    /// Get the operator name as a string
    pub fn name(&self) -> &'static str {
        match self {
            Operator::Delete => "delete",
            Operator::Change => "change",
            Operator::Yank => "yank",
            Operator::Indent => "indent",
            Operator::Outdent => "outdent",
            Operator::Format => "format",
            Operator::Fold => "fold",
            Operator::Unfold => "unfold",
            Operator::ToUpper => "to_upper",
            Operator::ToLower => "to_lower",
            Operator::SwapCase => "swap_case",
            Operator::Filter => "filter",
        }
    }
    
    /// Get the operator from a character
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            'd' => Some(Operator::Delete),
            'c' => Some(Operator::Change),
            'y' => Some(Operator::Yank),
            '>' => Some(Operator::Indent),
            '<' => Some(Operator::Outdent),
            '=' => Some(Operator::Format),
            'z' => Some(Operator::Fold),
            'Z' => Some(Operator::Unfold),
            'g' => Some(Operator::ToUpper), // This is actually 'gU', but we'll handle the 'U' separately
            'g' => Some(Operator::ToLower), // This is actually 'gu', but we'll handle the 'u' separately
            '~' => Some(Operator::SwapCase),
            '!' => Some(Operator::Filter),
            _ => None,
        }
    }
}

/// The target of an operator
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OperatorTarget {
    /// A text object
    TextObject(TextObjectType, bool), // TextObjectType, include_delimiters
    /// A motion
    Motion(Direction),
    /// A line range
    LineRange(usize, usize), // start_line, end_line
    /// A character range
    CharRange(usize, usize), // start_char, end_char
}

/// The operator state
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperatorState {
    /// The operator
    pub operator: Operator,
    /// The count (number of times to apply the operator)
    pub count: usize,
    /// The target (what to operate on)
    pub target: Option<OperatorTarget>,
}

impl OperatorState {
    /// Create a new operator state
    pub fn new(operator: Operator) -> Self {
        Self {
            operator,
            count: 1,
            target: None,
        }
    }
    
    /// Set the target
    pub fn set_target(&mut self, target: OperatorTarget) {
        self.target = Some(target);
    }
    
    /// Set the count
    pub fn set_count(&mut self, count: usize) {
        self.count = count;
    }
    
    /// Check if the operator state is complete (has a target)
    pub fn is_complete(&self) -> bool {
        self.target.is_some()
    }
}

/// The operator manager
pub struct OperatorManager {
    /// The current operator state
    current_state: Option<OperatorState>,
}

impl OperatorManager {
    /// Create a new operator manager
    pub fn new() -> Self {
        Self {
            current_state: None,
        }
    }
    
    /// Start an operator
    pub fn start_operator(&mut self, operator: Operator) {
        self.current_state = Some(OperatorState::new(operator));
    }
    
    /// Set the target for the current operator
    pub fn set_target(&mut self, target: OperatorTarget) -> Option<OperatorState> {
        if let Some(state) = &mut self.current_state {
            state.set_target(target);
            let result = state.clone();
            self.current_state = None;
            Some(result)
        } else {
            None
        }
    }
    
    /// Set the count for the current operator
    pub fn set_count(&mut self, count: usize) {
        if let Some(state) = &mut self.current_state {
            state.set_count(count);
        }
    }
    
    /// Get the current operator state
    pub fn current_state(&self) -> Option<&OperatorState> {
        self.current_state.as_ref()
    }
    
    /// Cancel the current operator
    pub fn cancel(&mut self) {
        self.current_state = None;
    }
}

impl Default for OperatorManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_operator_from_char() {
        assert_eq!(Operator::from_char('d'), Some(Operator::Delete));
        assert_eq!(Operator::from_char('c'), Some(Operator::Change));
        assert_eq!(Operator::from_char('y'), Some(Operator::Yank));
        assert_eq!(Operator::from_char('x'), None);
    }
    
    #[test]
    fn test_operator_state() {
        let mut state = OperatorState::new(Operator::Delete);
        assert_eq!(state.operator, Operator::Delete);
        assert_eq!(state.count, 1);
        assert_eq!(state.target, None);
        assert!(!state.is_complete());
        
        state.set_count(3);
        assert_eq!(state.count, 3);
        
        state.set_target(OperatorTarget::Motion(Direction::Down));
        assert_eq!(state.target, Some(OperatorTarget::Motion(Direction::Down)));
        assert!(state.is_complete());
    }
    
    #[test]
    fn test_operator_manager() {
        let mut manager = OperatorManager::new();
        assert_eq!(manager.current_state(), None);
        
        manager.start_operator(Operator::Delete);
        assert!(manager.current_state().is_some());
        assert_eq!(manager.current_state().unwrap().operator, Operator::Delete);
        
        manager.set_count(2);
        assert_eq!(manager.current_state().unwrap().count, 2);
        
        let state = manager.set_target(OperatorTarget::Motion(Direction::Down));
        assert!(state.is_some());
        assert_eq!(state.unwrap().operator, Operator::Delete);
        assert_eq!(manager.current_state(), None);
        
        manager.start_operator(Operator::Yank);
        assert!(manager.current_state().is_some());
        
        manager.cancel();
        assert_eq!(manager.current_state(), None);
    }
}