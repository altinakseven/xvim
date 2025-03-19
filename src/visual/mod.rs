//! Visual mode module for xvim
//!
//! This module implements Visual mode functionality, including:
//! - Character-wise visual mode ('v')
//! - Line-wise visual mode ('V')
//! - Block-wise visual mode (Ctrl-V)
//! - Visual mode operations

use crate::buffer::Buffer;
use crate::cursor::CursorPosition;
use crate::editor::{Editor, EditorError, EditorResult};
// // use crate::mode::Mode;
use crate::selection::{SelectionManager, SelectionType};
use std::cmp::{max, min};

/// Visual mode type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VisualMode {
    /// Character-wise visual mode ('v')
    Char,
    /// Line-wise visual mode ('V')
    Line,
    /// Block-wise visual mode (Ctrl-V)
    Block,
}

impl VisualMode {
    /// Convert a character to a visual mode
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            'v' => Some(VisualMode::Char),
            'V' => Some(VisualMode::Line),
            '\u{16}' => Some(VisualMode::Block), // Ctrl-V
            _ => None,
        }
    }

    /// Convert visual mode to a character
    pub fn to_char(&self) -> char {
        match self {
            VisualMode::Char => 'v',
            VisualMode::Line => 'V',
            VisualMode::Block => '\u{16}', // Ctrl-V
        }
    }

    /// Convert visual mode to selection type
    pub fn to_selection_type(&self) -> SelectionType {
        match self {
            VisualMode::Char => SelectionType::Character,
            VisualMode::Line => SelectionType::Line,
            VisualMode::Block => SelectionType::Block,
        }
    }
}

/// Visual mode state
#[derive(Debug, Clone)]
pub struct VisualState {
    /// Whether visual mode is active
    pub active: bool,
    /// Current visual mode
    pub mode: VisualMode,
    /// Original visual mode (used for restoring after temporary changes)
    pub mode_orig: Option<VisualMode>,
    /// Start position of visual selection
    pub start: CursorPosition,
    /// Whether to reselect the previous visual area with 'gv'
    pub reselect: bool,
    /// Whether we're in Select mode rather than Visual mode
    pub select: bool,
    /// Register to use for Select mode
    pub select_reg: Option<char>,
    /// Whether the selection is exclusive (controlled by 'selection' option)
    pub is_exclusive: bool,
}

impl Default for VisualState {
    fn default() -> Self {
        Self {
            active: false,
            mode: VisualMode::Char,
            mode_orig: None,
            start: CursorPosition::new(0, 0),
            reselect: false,
            select: false,
            select_reg: None,
            is_exclusive: false,
        }
    }
}

impl VisualState {
    /// Create a new visual state
    pub fn new() -> Self {
        Self::default()
    }

    /// Start visual mode
    pub fn start(&mut self, mode: VisualMode, cursor: CursorPosition) {
        self.active = true;
        self.mode = mode;
        self.start = cursor;
        self.reselect = true;
    }

    /// End visual mode
    pub fn end(&mut self) {
        self.active = false;
        self.select = false;
        self.select_reg = None;
    }

    /// Toggle between visual modes
    pub fn toggle_mode(&mut self, mode: VisualMode) {
        if self.mode == mode {
            self.end();
        } else {
            self.mode = mode;
        }
    }

    /// Save the current visual area for 'gv' command
    pub fn save_visual_area(&self, buffer_id: usize, cursor: CursorPosition) -> VisualArea {
        VisualArea {
            buffer_id,
            mode: self.mode,
            start: self.start,
            end: cursor,
            curswant: cursor.column,
        }
    }
}

/// Saved visual area for 'gv' command
#[derive(Debug, Clone)]
pub struct VisualArea {
    /// Buffer ID
    pub buffer_id: usize,
    /// Visual mode
    pub mode: VisualMode,
    /// Start position
    pub start: CursorPosition,
    /// End position
    pub end: CursorPosition,
    /// Desired column position
    pub curswant: usize,
}

/// Visual mode functions for the editor
pub trait VisualFunctions {
    /// Start visual mode
    fn start_visual_mode(&mut self, mode: VisualMode) -> EditorResult<()>;
    
    /// End visual mode
    fn end_visual_mode(&mut self) -> EditorResult<()>;
    
    /// Toggle visual mode
    fn toggle_visual_mode(&mut self, mode: VisualMode) -> EditorResult<()>;
    
    /// Reselect previous visual area (gv command)
    fn reselect_visual_area(&mut self) -> EditorResult<()>;
    
    /// Swap start and end of visual area (o/O command)
    fn swap_visual_corners(&mut self, upper: bool) -> EditorResult<()>;
    
    /// Get the visual state
    fn visual_state(&self) -> &VisualState;
    
    /// Get a mutable reference to the visual state
    fn visual_state_mut(&mut self) -> &mut VisualState;
}

// Note: The implementation of VisualFunctions for Editor is now in src/editor/mod.rs

/// Extension trait for Buffer to handle visual areas
pub trait BufferVisualExt {
    /// Get the visual area for this buffer
    fn visual_area(&self) -> Option<&VisualArea>;
    
    /// Set the visual area for this buffer
    fn set_visual_area(&mut self, visual_area: VisualArea);
}

impl BufferVisualExt for Buffer {
    fn visual_area(&self) -> Option<&VisualArea> {
        self.visual_area.as_ref()
    }
    
    fn set_visual_area(&mut self, visual_area: VisualArea) {
        self.visual_area = Some(visual_area);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_visual_mode_conversion() {
        assert_eq!(VisualMode::from_char('v'), Some(VisualMode::Char));
        assert_eq!(VisualMode::from_char('V'), Some(VisualMode::Line));
        assert_eq!(VisualMode::from_char('\u{16}'), Some(VisualMode::Block));
        assert_eq!(VisualMode::from_char('x'), None);
        
        assert_eq!(VisualMode::Char.to_char(), 'v');
        assert_eq!(VisualMode::Line.to_char(), 'V');
        assert_eq!(VisualMode::Block.to_char(), '\u{16}');
    }
    
    #[test]
    fn test_visual_state() {
        let mut state = VisualState::new();
        assert_eq!(state.active, false);
        
        let cursor = CursorPosition::new(1, 0);
        state.start(VisualMode::Char, cursor);
        assert_eq!(state.active, true);
        assert_eq!(state.mode, VisualMode::Char);
        assert_eq!(state.start, cursor);
        
        state.toggle_mode(VisualMode::Line);
        assert_eq!(state.mode, VisualMode::Line);
        
        state.toggle_mode(VisualMode::Line);
        assert_eq!(state.active, false);
        
        state.start(VisualMode::Block, cursor);
        assert_eq!(state.active, true);
        assert_eq!(state.mode, VisualMode::Block);
        
        state.end();
        assert_eq!(state.active, false);
    }
}