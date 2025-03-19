//! Editor module - Core functionality for the xvim editor
//!
//! This module contains the main Editor struct which coordinates all other components
//! and manages the overall state of the editor.

pub mod init;

use std::error::Error;
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Mutex;
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Mutex;
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
// use std::fs::File;
use std::sync::{Arc, Mutex};
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Mutex;
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Mutex;
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use std::path::{Path, PathBuf};
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Mutex;
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Mutex;
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
// use std::fmt;

// Import all modules from crate
use crate::buffer::{BufferManager, BufferManagerError, BufferResult};
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Mutex;
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Mutex;
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use crate::command::{Command, CommandParser, ExCommandRegistry, register_handlers};
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Mutex;
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Mutex;
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use crate::command::completion_handlers::{CompletionType, CompletionItem};
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Mutex;
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Mutex;
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use crate::r#macro::{MacroRecorder, MacroPlayer, MacroRecorderState};
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Mutex;
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Mutex;
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use crate::text_object::TextObjectType as TextObjectTypeExt;
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Mutex;
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Mutex;
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use crate::operator::{Operator, OperatorTarget, OperatorState, OperatorManager};
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Mutex;
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Mutex;
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use crate::config::ConfigManager;
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Mutex;
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Mutex;
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use crate::cursor::{CursorManager, Direction, CursorPosition};
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Mutex;
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Mutex;
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use crate::keymap::{KeyHandler, KeyMapping, KeySequence, Command as KeyCommand};
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Mutex;
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Mutex;
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use crate::mode::{ModeManager, Mode};
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Mutex;
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Mutex;
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use crate::plugin::PluginManager;
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Mutex;
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Mutex;
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use crate::register::{RegisterManager, RegisterType, RegisterContent};
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Mutex;
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Mutex;
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use crate::selection::{SelectionManager, SelectionType};
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Mutex;
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Mutex;
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use crate::syntax::{SyntaxRegistry, Theme, create_default_registry, create_default_theme, ColorSchemeRegistry, create_default_colorscheme_registry};
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Mutex;
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Mutex;
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use crate::ui::{TerminalUi, UiError};
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Mutex;
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Mutex;
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use crate::search::{SearchState, SearchDirection, SearchFunctions};
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Mutex;
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Mutex;
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use crate::visual::{VisualState, VisualFunctions, BufferVisualExt};
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Mutex;
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Mutex;
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use crate::insert::{InsertState, InsertFunctions, BufferInsertExt};
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Mutex;
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Mutex;
use std::path::PathBuf;
use std::path::Path;
use std::path::PathBuf;
// use crossterm::event::KeyEvent;
// use regex::Regex;

// Forward declarations for text objects
pub struct TextObject {
    pub object_type: TextObjectType,
    pub start: usize,
    pub end: usize,
    pub include_delimiters: bool,
}

pub enum TextObjectType {
    Word,
    BigWord,
    Sentence,
    Paragraph,
    ParenBlock,
    BraceBlock,
    BracketBlock,
    AngleBlock,
    SingleQuoteBlock,
    DoubleQuoteBlock,
    BacktickBlock,
    TagBlock,
}

/// Main editor struct that coordinates all components
pub struct Editor {
    /// Buffer manager
    pub buffer_manager: BufferManager,
    /// Command registry
    pub command_registry: Arc<Mutex<ExCommandRegistry>>,
    /// Ex command registry
    pub ex_command_registry: ExCommandRegistry,
    /// Cursor manager
    pub cursor_manager: CursorManager,
    /// Mode manager
    pub mode_manager: ModeManager,
    /// Register manager
    pub register_manager: RegisterManager,
    /// Selection manager
    pub selection_manager: SelectionManager,
    /// Operator manager
    pub operator_manager: OperatorManager,
    /// Macro recorder
    pub macro_recorder: MacroRecorder,
    /// Macro player
    pub macro_player: MacroPlayer,
    /// Search state
    pub search_state: SearchState,
    /// Visual state
    pub visual_state: VisualState,
    /// Insert state
    pub insert_state: InsertState,
    /// UI
    pub ui: TerminalUi,
    /// Config manager
    pub config_manager: ConfigManager,
    /// Plugin manager
    pub plugin_manager: PluginManager,
    /// Syntax registry
    pub syntax_registry: Arc<SyntaxRegistry>,
    /// Theme
    pub theme: Arc<Theme>,
    /// Color scheme registry
    pub color_scheme_registry: Arc<ColorSchemeRegistry>,
}

impl Editor {
    /// Create a new editor
    pub fn new() -> EditorResult<Self> {
        // Create the command registry
        let command_registry = Arc::new(Mutex::new(ExCommandRegistry::new()));
        
        // Create the ex command registry
        let ex_command_registry = ExCommandRegistry::new();
        
        // Create the buffer manager
        let buffer_manager = BufferManager::new();
        
        // Create the cursor manager
        let cursor_manager = CursorManager::new();
        
        // Create the mode manager
        let mode_manager = ModeManager::new();
        
        // Create the register manager
        let register_manager = RegisterManager::new();
        
        // Create the selection manager
        let selection_manager = SelectionManager::new();
        
        // Create the operator manager
        let operator_manager = OperatorManager::new();
        
        // Create the macro recorder
        let macro_recorder = MacroRecorder::new();
        
        // Create the macro player
        let macro_player = MacroPlayer::new();
        
        // Create the search state
        let search_state = SearchState::new();
        
        // Create the visual state
        let visual_state = VisualState::new();
        
        // Create the insert state
        let insert_state = InsertState::new();
        
        // Create the UI
        let ui = TerminalUi::new()?;
        
        // Create the config manager
        let config_manager = ConfigManager::new();
        
        // Create the plugin manager
        let plugin_manager = PluginManager::new();
        
        // Create the syntax registry
        let syntax_registry = Arc::new(create_default_registry()?);
        
        // Create the theme
        let theme = Arc::new(create_default_theme());
        
        // Create the color scheme registry
        let color_scheme_registry = Arc::new(create_default_colorscheme_registry());
        
        // Register command handlers
        register_handlers(&command_registry);
        
        Ok(Self {
            buffer_manager,
            command_registry,
            ex_command_registry,
            cursor_manager,
            mode_manager,
            register_manager,
            selection_manager,
            operator_manager,
            macro_recorder,
            macro_player,
            search_state,
            visual_state,
            insert_state,
            ui,
            config_manager,
            plugin_manager,
            syntax_registry,
            theme,
            color_scheme_registry,
        })
    }
    
    /// Set the theme
    pub fn set_theme(&mut self, theme: Arc<Theme>) {
        self.theme = theme;
    }
    
    /// Redraw the editor
    pub fn redraw(&mut self) -> EditorResult<()> {
        // Get the current buffer
        let current_buffer_id = self.buffer_manager.current_buffer_id();
        if let Some(buffer_id) = current_buffer_id {
            // Get the buffer
            let buffer = self.buffer_manager.get_buffer(buffer_id)?;
            
            // Redraw the UI
            self.ui.redraw(&[buffer], self.mode_manager.current_mode(), "")?;
        }
        
        Ok(())
    }
}

/// Errors that can occur during editor operations
#[derive(Debug)]
pub enum EditorError {
    /// Buffer-related error
    Buffer(BufferManagerError),
    /// UI-related error
    Ui(UiError),
    /// I/O error
    Io(std::io::Error),
    /// Other errors
    Other(String),
}

impl std::fmt::Display for EditorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EditorError::Buffer(err) => write!(f, "Buffer error: {}", err),
            EditorError::Ui(err) => write!(f, "UI error: {}", err),
            EditorError::Io(err) => write!(f, "I/O error: {}", err),
            EditorError::Other(msg) => write!(f, "{}", msg),
        }
    }
}

impl Error for EditorError {}

impl From<BufferManagerError> for EditorError {
    fn from(err: BufferManagerError) -> Self {
        EditorError::Buffer(err)
    }
}

impl From<UiError> for EditorError {
    fn from(err: UiError) -> Self {
        EditorError::Ui(err)
    }
}

impl From<std::io::Error> for EditorError {
    fn from(err: std::io::Error) -> Self {
        EditorError::Io(err)
    }
}

impl From<crate::buffer::BufferError> for EditorError {
    fn from(err: crate::buffer::BufferError) -> Self {
        EditorError::Buffer(crate::buffer::BufferManagerError::BufferError(err))
    }
}

impl From<anyhow::Error> for EditorError {
    fn from(err: anyhow::Error) -> Self {
        EditorError::Other(format!("{}", err))
    }
}

/// Result type used throughout the editor
pub type EditorResult<T> = Result<T, EditorError>;