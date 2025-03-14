//! Mode module - Implementation of Vim's modal editing system
//!
//! This module implements the state machine for Vim's modal editing system,
//! including normal, insert, visual, and command modes.

use std::fmt;

/// The different editor modes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Mode {
    /// Normal mode - For navigation and commands
    Normal,
    /// Insert mode - For inserting text
    Insert,
    /// Visual mode - For selecting text
    Visual,
    /// Visual Line mode - For selecting whole lines
    VisualLine,
    /// Visual Block mode - For selecting rectangular blocks
    VisualBlock,
    /// Command mode - For entering ex commands
    Command,
    /// Replace mode - For replacing text
    Replace,
    /// Terminal mode - For terminal buffers
    Terminal,
    /// Operator Pending mode - Waiting for a motion after an operator
    OperatorPending,
}

impl Mode {
    /// Check if the mode is a visual mode (Visual, VisualLine, or VisualBlock)
    pub fn is_visual(&self) -> bool {
        matches!(self, Mode::Visual | Mode::VisualLine | Mode::VisualBlock)
    }
    
    /// Check if the mode allows inserting text (Insert or Replace)
    pub fn is_insert(&self) -> bool {
        matches!(self, Mode::Insert | Mode::Replace)
    }
    
    /// Get the corresponding selection type for a visual mode
    pub fn to_selection_type(&self) -> Option<crate::selection::SelectionType> {
        match self {
            Mode::Visual => Some(crate::selection::SelectionType::Character),
            Mode::VisualLine => Some(crate::selection::SelectionType::Line),
            Mode::VisualBlock => Some(crate::selection::SelectionType::Block),
            _ => None,
        }
    }
    
    /// Get the mode name as a string
    pub fn name(&self) -> &'static str {
        match self {
            Mode::Normal => "NORMAL",
            Mode::Insert => "INSERT",
            Mode::Visual => "VISUAL",
            Mode::VisualLine => "VISUAL LINE",
            Mode::VisualBlock => "VISUAL BLOCK",
            Mode::Command => "COMMAND",
            Mode::Replace => "REPLACE",
            Mode::Terminal => "TERMINAL",
            Mode::OperatorPending => "OPERATOR-PENDING",
        }
    }
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Normal
    }
}

/// The mode manager handles mode transitions and state
#[derive(Clone)]
pub struct ModeManager {
    /// The current mode
    current_mode: Mode,
    /// The previous mode (for returning from Command mode)
    previous_mode: Mode,
}

impl ModeManager {
    /// Create a new mode manager
    pub fn new() -> Self {
        Self {
            current_mode: Mode::Normal,
            previous_mode: Mode::Normal,
        }
    }
    
    /// Get the current mode
    pub fn current_mode(&self) -> Mode {
        self.current_mode
    }
    
    /// Get the previous mode
    pub fn previous_mode(&self) -> Mode {
        self.previous_mode
    }
    
    /// Change to a new mode
    pub fn change_mode(&mut self, mode: Mode) {
        if self.current_mode != mode {
            self.previous_mode = self.current_mode;
            self.current_mode = mode;
        }
    }
    
    /// Return to the previous mode
    pub fn return_to_previous_mode(&mut self) {
        let temp = self.current_mode;
        self.current_mode = self.previous_mode;
        self.previous_mode = temp;
    }
    
    /// Enter normal mode
    pub fn enter_normal_mode(&mut self) {
        self.change_mode(Mode::Normal);
    }
    
    /// Enter insert mode
    pub fn enter_insert_mode(&mut self) {
        self.change_mode(Mode::Insert);
    }
    
    /// Enter visual mode
    pub fn enter_visual_mode(&mut self) {
        self.change_mode(Mode::Visual);
    }
    
    /// Enter visual line mode
    pub fn enter_visual_line_mode(&mut self) {
        self.change_mode(Mode::VisualLine);
    }
    
    /// Enter visual block mode
    pub fn enter_visual_block_mode(&mut self) {
        self.change_mode(Mode::VisualBlock);
    }
    
    /// Enter command mode
    pub fn enter_command_mode(&mut self) {
        self.change_mode(Mode::Command);
    }
    
    /// Enter replace mode
    pub fn enter_replace_mode(&mut self) {
        self.change_mode(Mode::Replace);
    }
    
    /// Enter terminal mode
    pub fn enter_terminal_mode(&mut self) {
        self.change_mode(Mode::Terminal);
    }
    
    /// Enter operator-pending mode
    pub fn enter_operator_pending_mode(&mut self) {
        self.change_mode(Mode::OperatorPending);
    }
}

impl Default for ModeManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_mode_transitions() {
        let mut manager = ModeManager::new();
        
        // Default mode should be Normal
        assert_eq!(manager.current_mode(), Mode::Normal);
        
        // Test mode changes
        manager.enter_insert_mode();
        assert_eq!(manager.current_mode(), Mode::Insert);
        assert_eq!(manager.previous_mode(), Mode::Normal);
        
        manager.enter_visual_mode();
        assert_eq!(manager.current_mode(), Mode::Visual);
        assert_eq!(manager.previous_mode(), Mode::Insert);
        
        // Test return to previous mode
        manager.return_to_previous_mode();
        assert_eq!(manager.current_mode(), Mode::Insert);
        assert_eq!(manager.previous_mode(), Mode::Visual);
    }
    
    #[test]
    fn test_mode_properties() {
        assert!(Mode::Visual.is_visual());
        assert!(Mode::VisualLine.is_visual());
        assert!(Mode::VisualBlock.is_visual());
        assert!(!Mode::Normal.is_visual());
        
        assert!(Mode::Insert.is_insert());
        assert!(Mode::Replace.is_insert());
        assert!(!Mode::Normal.is_insert());
    }
}