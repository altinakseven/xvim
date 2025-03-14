//! Editor module - Core functionality for the xvim editor
//!
//! This module contains the main Editor struct which coordinates all other components
//! and manages the overall state of the editor.

use std::error::Error;
use std::path::Path;

// Import all modules from crate
use crate::buffer::{BufferManager, BufferManagerError, BufferResult};
use crate::cursor::CursorPosition;
use crate::command::{Command, CommandParser, ExCommandRegistry, register_handlers};
use crate::r#macro::{MacroRecorder, MacroPlayer, MacroRecorderState};
use crate::text_object::TextObjectType as TextObjectTypeExt;
use crate::operator::{Operator, OperatorTarget, OperatorState, OperatorManager};
use crate::config::ConfigManager;
use crate::cursor::{CursorManager, Direction};
use crate::keymap::{KeyHandler, KeyMapping, KeySequence, Command as KeyCommand};
use crate::mode::ModeManager;
use crate::plugin::PluginManager;
use crate::register::{RegisterManager, RegisterType, RegisterContent};
use crate::selection::{SelectionManager, SelectionType};
use crate::syntax::{SyntaxRegistry, Theme, create_default_registry, create_default_theme};
use crate::ui::{TerminalUi, UiError};
use crossterm::event::KeyEvent;
use std::sync::{Arc, Mutex};

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

/// Result type used throughout the editor
pub type EditorResult<T> = Result<T, EditorError>;
/// The main editor struct that coordinates all components
pub struct Editor {
    /// Buffer manager
    buffer_manager: BufferManager,
    /// Mode manager
    mode_manager: ModeManager,
    /// Terminal UI
    terminal: TerminalUi,
    /// Key handler for key mappings
    key_handler: KeyHandler,
    /// Configuration manager
    config_manager: ConfigManager,
    /// Syntax registry
    syntax_registry: Arc<SyntaxRegistry>,
    /// Current theme
    theme: Arc<Theme>,
    /// Cursor manager
    cursor_manager: CursorManager,
    /// Selection manager
    selection_manager: SelectionManager,
    /// Register manager
    register_manager: RegisterManager,
    /// Command parser
    command_parser: CommandParser,
    /// Ex command registry
    ex_command_registry: ExCommandRegistry,
    /// Macro recorder
    macro_recorder: MacroRecorder,
    /// Macro player
    macro_player: MacroPlayer,
    /// Operator manager
    operator_manager: OperatorManager,
    /// Plugin manager
    plugin_manager: PluginManager,
    /// Current view position (top line)
    view_position: usize,
    /// Whether the editor is running
    running: bool,
    /// Command buffer for storing command text
    command_buffer: String,
}

impl Editor {
    /// Create a new editor instance
    pub fn new() -> EditorResult<Self> {
        // Initialize buffer manager
        let buffer_manager = BufferManager::new();
        
        // Initialize mode manager
        let mode_manager = ModeManager::new();
        
        // Initialize terminal UI
        let terminal = TerminalUi::new()?;
        
        // Initialize key handler
        let key_handler = KeyHandler::new();
        
        // Initialize configuration manager
        let mut config_manager = ConfigManager::new();
        
        // Try to load configuration
        if let Err(err) = config_manager.load() {
            // If there's an error loading the config, just use the default
            // and log a warning
            eprintln!("Warning: Failed to load configuration: {}", err);
        }
        
        // Initialize command parser
        let command_parser = CommandParser::new();
        
        // Initialize ex command registry
        let mut ex_command_registry = ExCommandRegistry::new();
        
        // Register command handlers
        register_handlers(&mut ex_command_registry);
        
        // Initialize syntax registry
        let syntax_registry = match create_default_registry() {
            Ok(registry) => Arc::new(registry),
            Err(err) => {
                eprintln!("Warning: Failed to create syntax registry: {}", err);
                Arc::new(SyntaxRegistry::new())
            }
        };
        
        // Initialize theme
        let theme = Arc::new(create_default_theme());
        
        // Initialize cursor manager
        let cursor_manager = CursorManager::new();
        
        // Initialize selection manager
        let selection_manager = SelectionManager::new();
        
        // Initialize register manager
        let register_manager = RegisterManager::new();
        
        // Initialize macro recorder and player
        let macro_recorder = MacroRecorder::new(register_manager.clone());
        let macro_player = MacroPlayer::new(register_manager.clone());
        
        // Initialize operator manager
        let operator_manager = OperatorManager::new();
        
        // Initialize plugin manager
        let mut plugin_manager = PluginManager::new();
        
        // Initialize the plugin system
        if let Err(err) = plugin_manager.init() {
            eprintln!("Warning: Failed to initialize plugin system: {}", err);
        }
        
        // Create an empty buffer if none exists
        let mut editor = Self {
            buffer_manager,
            mode_manager,
            terminal,
            key_handler,
            config_manager,
            syntax_registry,
            theme,
            cursor_manager,
            selection_manager,
            register_manager,
            command_parser,
            ex_command_registry,
            macro_recorder,
            macro_player,
            operator_manager,
            plugin_manager,
            view_position: 0,
            running: false,
            command_buffer: String::new(),
        };
        
        // Create an initial empty buffer
        let buffer_id = editor.buffer_manager.create_buffer()?;
        
        // Initialize the tabs in the terminal UI
        editor.terminal.init_tabs(buffer_id)?;
        
        // Set up default key mappings
        editor.setup_default_key_mappings();
        
        // Apply configuration settings
        editor.apply_config();
        
        // Load the noxvim plugin
        editor.load_noxvim_plugin();
        
        Ok(editor)
    }
    
    /// Set up default key mappings
    fn setup_default_key_mappings(&mut self) {
        use crossterm::event::{KeyCode, KeyModifiers};
        use crate::mode::Mode;
        
        // Normal mode mappings
        
        // Quit with Ctrl+Q
        let quit_mapping = KeyMapping::new(
            Mode::Normal,
            KeySequence::from_key(KeyEvent::new(KeyCode::Char('q'), KeyModifiers::CONTROL)),
            KeyCommand::BuiltIn("quit".to_string()),
            false
        );
        self.key_handler.key_map_mut().add_mapping(quit_mapping);
        
        // Enter insert mode with 'i'
        let insert_mapping = KeyMapping::new(
            Mode::Normal,
            KeySequence::from_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE)),
            KeyCommand::BuiltIn("enter_insert_mode".to_string()),
            false
        );
        self.key_handler.key_map_mut().add_mapping(insert_mapping);
        
        // Enter command mode with ':'
        let command_mapping = KeyMapping::new(
            Mode::Normal,
            KeySequence::from_key(KeyEvent::new(KeyCode::Char(':'), KeyModifiers::NONE)),
            KeyCommand::BuiltIn("enter_command_mode".to_string()),
            false
        );
        self.key_handler.key_map_mut().add_mapping(command_mapping);
        
        // Undo with 'u'
        let undo_mapping = KeyMapping::new(
            Mode::Normal,
            KeySequence::from_key(KeyEvent::new(KeyCode::Char('u'), KeyModifiers::NONE)),
            KeyCommand::BuiltIn("undo".to_string()),
            false
        );
        self.key_handler.key_map_mut().add_mapping(undo_mapping);
        
        // Redo with Ctrl+R
        let redo_mapping = KeyMapping::new(
            Mode::Normal,
            KeySequence::from_key(KeyEvent::new(KeyCode::Char('r'), KeyModifiers::CONTROL)),
            KeyCommand::BuiltIn("redo".to_string()),
            false
        );
        self.key_handler.key_map_mut().add_mapping(redo_mapping);
        
        // Paste after cursor with 'p'
        let paste_mapping = KeyMapping::new(
            Mode::Normal,
            KeySequence::from_key(KeyEvent::new(KeyCode::Char('p'), KeyModifiers::NONE)),
            KeyCommand::BuiltIn("paste".to_string()),
            false
        );
        self.key_handler.key_map_mut().add_mapping(paste_mapping);
        // Paste before cursor with 'P'
        let paste_before_mapping = KeyMapping::new(
            Mode::Normal,
            KeySequence::from_key(KeyEvent::new(KeyCode::Char('P'), KeyModifiers::NONE)),
            KeyCommand::BuiltIn("paste_before".to_string()),
            false
        );
        self.key_handler.key_map_mut().add_mapping(paste_before_mapping.clone());
        
        // Tab navigation with gt and gT
        let next_tab_mapping = KeyMapping::new(
            Mode::Normal,
            KeySequence::new(vec![
                KeyEvent::new(KeyCode::Char('g'), KeyModifiers::NONE),
                KeyEvent::new(KeyCode::Char('t'), KeyModifiers::NONE),
            ]),
            KeyCommand::BuiltIn("next_tab".to_string()),
            false
        );
        self.key_handler.key_map_mut().add_mapping(next_tab_mapping);
        
        let prev_tab_mapping = KeyMapping::new(
            Mode::Normal,
            KeySequence::new(vec![
                KeyEvent::new(KeyCode::Char('g'), KeyModifiers::NONE),
                KeyEvent::new(KeyCode::Char('T'), KeyModifiers::NONE),
            ]),
            KeyCommand::BuiltIn("prev_tab".to_string()),
            false
        );
        self.key_handler.key_map_mut().add_mapping(prev_tab_mapping);
        self.key_handler.key_map_mut().add_mapping(paste_before_mapping);
        
        // Macro recording with q
        let start_recording_mapping = KeyMapping::new(
            Mode::Normal,
            KeySequence::from_key(KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE)),
            KeyCommand::BuiltIn("start_recording".to_string()),
            false
        );
        self.key_handler.key_map_mut().add_mapping(start_recording_mapping);
        
        // Macro playback with @
        let play_macro_mapping = KeyMapping::new(
            Mode::Normal,
            KeySequence::from_key(KeyEvent::new(KeyCode::Char('@'), KeyModifiers::NONE)),
            KeyCommand::BuiltIn("play_macro".to_string()),
            false
        );
        self.key_handler.key_map_mut().add_mapping(play_macro_mapping);
        
        // All modes: Escape to return to normal mode
        let escape_mapping = KeyMapping::new(
            Mode::Insert,
            KeySequence::from_key(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE)),
            KeyCommand::BuiltIn("enter_normal_mode".to_string()),
            false
        );
        self.key_handler.key_map_mut().add_mapping(escape_mapping);
        
        let escape_mapping = KeyMapping::new(
            Mode::Command,
            KeySequence::from_key(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE)),
            KeyCommand::BuiltIn("enter_normal_mode".to_string()),
            false
        );
        self.key_handler.key_map_mut().add_mapping(escape_mapping);
    }
    
    /// Get the current mode
    pub fn current_mode(&self) -> crate::mode::Mode {
        self.mode_manager.current_mode()
    }
    
    /// Get the current cursor position
    pub fn cursor_position(&self) -> crate::cursor::CursorPosition {
        self.cursor_manager.position()
    }

    /// Open a file in the editor
    pub fn open_file<P: AsRef<Path>>(&mut self, path: P) -> EditorResult<()> {
        // Open the file in the buffer manager
        let buffer_id = self.buffer_manager.open_file(&path)?;
        
        // Set it as the current buffer
        self.buffer_manager.set_current_buffer(buffer_id)?;
        
        // Create a new tab for the file
        let file_name = path.as_ref().file_name()
            .map(|name| name.to_string_lossy().to_string())
            .unwrap_or_else(|| format!("Buffer {}", buffer_id));
        
        self.terminal.create_tab(buffer_id, Some(file_name))?;
        
        // Reset view position
        self.view_position = 0;
        
        Ok(())
    }
    
    /// Get the current buffer ID
    pub fn current_buffer_id(&self) -> Option<usize> {
        self.buffer_manager.current_buffer_id()
    }
    
    /// Save the current buffer
    pub fn save_current_buffer(&mut self) -> EditorResult<()> {
        // Get the current buffer ID
        let buffer_id = match self.current_buffer_id() {
            Some(id) => id,
            None => return Err(EditorError::Other("No buffer to save".to_string())),
        };
        
        // Get the buffer from the buffer manager
        let buffer = match self.buffer_manager.get_buffer_mut(buffer_id) {
            Ok(buffer) => buffer,
            Err(err) => return Err(EditorError::Buffer(err)),
        };
        
        // Check if the buffer has a file path
        if buffer.file_path().is_none() {
            return Err(EditorError::Other("No file name".to_string()));
        }
        
        // Save the buffer
        match buffer.save() {
            Ok(_) => {
                if let Some(path) = buffer.file_path() {
                    println!("\"{}\" written", path.display());
                } else {
                    println!("Buffer written");
                }
                Ok(())
            },
            Err(err) => Err(EditorError::Buffer(err.into())),
        }
    }
    
    /// Save the current buffer to a specific file
    pub fn save_current_buffer_as<P: AsRef<Path>>(&mut self, path: P) -> EditorResult<()> {
        // Get the current buffer ID
        let buffer_id = match self.current_buffer_id() {
            Some(id) => id,
            None => return Err(EditorError::Other("No buffer to save".to_string())),
        };
        
        // Get the buffer from the buffer manager
        let buffer = match self.buffer_manager.get_buffer_mut(buffer_id) {
            Ok(buffer) => buffer,
            Err(err) => return Err(EditorError::Buffer(err)),
        };
        
        // Save the buffer to the specified file
        match buffer.save_as(path.as_ref()) {
            Ok(_) => {
                println!("\"{}\" written", path.as_ref().display());
                Ok(())
            },
            Err(err) => Err(EditorError::Buffer(err.into())),
        }
    }
    
    /// Delete lines from a buffer starting at the cursor position
    pub fn delete_lines_from_cursor(&mut self, buffer_id: usize, start_line: usize, end_line: usize) -> EditorResult<()> {
        // Get the buffer from the buffer manager
        let buffer = match self.buffer_manager.get_buffer(buffer_id) {
            Ok(buffer) => buffer,
            Err(err) => return Err(EditorError::Buffer(err)),
        };
        
        // Get the start and end positions
        let start_idx = match buffer.position_to_char_idx(start_line, 0) {
            Ok(idx) => idx,
            Err(err) => return Err(EditorError::Buffer(err.into())),
        };
        
        let end_idx = if end_line + 1 < buffer.line_count() {
            match buffer.position_to_char_idx(end_line + 1, 0) {
                Ok(idx) => idx,
                Err(err) => return Err(EditorError::Buffer(err.into())),
            }
        } else {
            buffer.content().len()
        };
        
        // Get the text from the buffer before deleting it
        let content = buffer.content();
        if start_idx < content.len() && end_idx <= content.len() {
            let text = content[start_idx..end_idx].to_string();
            
            // Store the text in the unnamed register
            let lines: Vec<&str> = text.lines().collect();
            let register_content = RegisterContent::line_wise(&lines);
            self.register_manager.set_register(RegisterType::Unnamed, register_content);
        }
        
        // Get a mutable reference to the buffer and delete the lines
        let buffer = match self.buffer_manager.get_buffer_mut(buffer_id) {
            Ok(buffer) => buffer,
            Err(err) => return Err(EditorError::Buffer(err)),
        };
        
        match buffer.delete(start_idx, end_idx) {
            Ok(_) => {
                // Move cursor to the start of the next line or the start of the file if we deleted the last line
                let new_pos = if start_line < buffer.line_count() {
                    CursorPosition::new(start_line, 0)
                } else if buffer.line_count() > 0 {
                    CursorPosition::new(buffer.line_count() - 1, 0)
                } else {
                    CursorPosition::new(0, 0)
                };
                self.cursor_manager.set_position(new_pos);
                Ok(())
            },
            Err(err) => Err(EditorError::Buffer(err.into())),
        }
    }
    
    /// Render the current state
    fn render(&mut self) -> EditorResult<()> {
        // Get all buffers
        let mut buffers = Vec::new();
        for buffer_id in self.buffer_manager.buffer_ids() {
            if let Ok(buffer) = self.buffer_manager.get_buffer(buffer_id) {
                buffers.push(buffer);
            }
        }
        
        // Get references to the buffers
        let buffer_refs: Vec<&crate::buffer::Buffer> = buffers.clone();
        
        // Update the cursor position in the current window
        if let Some(buffer_id) = self.current_buffer_id() {
            if let Some(window) = self.terminal.current_window_mut() {
                if window.buffer_id == buffer_id {
                    // Set the cursor position in the window
                    window.cursor = self.cursor_manager.position();
                    
                    // Ensure the cursor is visible
                    if let Some(buffer) = buffers.iter().find(|b| b.id() == buffer_id) {
                        let _ = window.ensure_cursor_visible(buffer);
                    }
                }
            }
        }
        
        // Render the current tab
        self.terminal.render_current_tab(&buffer_refs, self.current_mode(), &self.command_buffer)?;
        
        Ok(())
    }
    
    /// Get the command buffer
    pub fn command_buffer(&self) -> &str {
        &self.command_buffer
    }
    
    /// Process a key event
    fn process_key(&mut self, key: KeyEvent) -> EditorResult<()> {
        // Check if we're playing back a macro
        if self.macro_player.is_playing() {
            // Get the next key from the macro
            if let Some(macro_key) = self.macro_player.next_key() {
                // Process the macro key
                return self.process_key_internal(macro_key);
            }
        }
        
        // Check if we're recording a macro
        if let MacroRecorderState::Recording(_) = self.macro_recorder.state() {
            // Record the key
            self.macro_recorder.record_key(key);
        }
        
        // Process the key normally
        self.process_key_internal(key)
    }
    
    /// Handle an operator in visual mode
    fn handle_visual_operator(&mut self, operator: Operator) -> EditorResult<()> {
        use crate::mode::Mode;
        
        // Get the current selection
        if let Some(selection) = self.selection_manager.current_selection() {
            if let Some(buffer_id) = self.current_buffer_id() {
                let buffer = self.buffer_manager.get_buffer(buffer_id)?;
                
                // Get the start and end positions
                let start_pos = selection.start;
                let end_pos = selection.end;
                
                // Convert positions to character indices
                let start_idx = buffer.position_to_char_idx(start_pos.line, start_pos.column)?;
                let end_idx = buffer.position_to_char_idx(end_pos.line, end_pos.column)?;
                
                // Determine the range to operate on
                let (start, end) = if start_idx <= end_idx {
                    (start_idx, end_idx)
                } else {
                    (end_idx, start_idx)
                };
                
                // Get the text from the buffer
                let content = buffer.content();
                if start < content.len() && end <= content.len() {
                    let text = content[start..end].to_string();
                    
                    // Store the text in the unnamed register
                    let register_content = match self.current_mode() {
                        Mode::VisualLine => {
                            let lines: Vec<&str> = text.lines().collect();
                            RegisterContent::line_wise(&lines)
                        },
                        Mode::VisualBlock => {
                            let lines: Vec<&str> = text.lines().collect();
                            RegisterContent::block_wise(&lines)
                        },
                        _ => RegisterContent::character_wise(&text),
                    };
                    self.register_manager.set_register(RegisterType::Unnamed, register_content);
                    
                    // Perform the operation
                    match operator {
                        Operator::Delete => {
                            let buffer = self.buffer_manager.get_buffer_mut(buffer_id)?;
                            buffer.delete(start, end)?;
                        },
                        Operator::Change => {
                            let buffer = self.buffer_manager.get_buffer_mut(buffer_id)?;
                            buffer.delete(start, end)?;
                            self.mode_manager.enter_insert_mode();
                        },
                        Operator::Yank => {
                            // Just yank, no need to modify the buffer
                        },
                        _ => {
                            // Other operators not implemented yet
                            return Err(EditorError::Other(format!("Operator {:?} not implemented for visual mode", operator)));
                        }
                    }
                }
                
                // End the selection and return to normal mode
                self.selection_manager.end_selection();
                self.mode_manager.enter_normal_mode();
                
                return Ok(());
            }
        }
        
        // If we get here, there was no selection or buffer
        Err(EditorError::Other("No selection or buffer".to_string()))
    }
    
    /// Internal key processing method
    fn process_key_internal(&mut self, key: KeyEvent) -> EditorResult<()> {
        use crossterm::event::{KeyCode, KeyModifiers};
        use crate::mode::Mode;
        
        // Check for macro recording commands
        if self.current_mode() == Mode::Normal {
            match key.code {
                KeyCode::Char('q') if key.modifiers == KeyModifiers::NONE => {
                    // Check if we're already recording
                    if let MacroRecorderState::Recording(_) = self.macro_recorder.state() {
                        // Stop recording
                        self.macro_recorder.stop_recording();
                        return Ok(());
                    } else {
                        // We need to get the register to record to
                        // This would normally be handled by getting the next key press,
                        // but for now we'll just use a fixed register ('a')
                        self.macro_recorder.start_recording('a');
                        return Ok(());
                    }
                },
                KeyCode::Char('@') if key.modifiers == KeyModifiers::NONE => {
                    // Play back a macro
                    // This would normally be handled by getting the next key press,
                    // but for now we'll just use a fixed register ('a')
                    self.macro_player.start_playback('a');
                    return Ok(());
                },
                // Check for operator commands
                KeyCode::Char('d') if key.modifiers == KeyModifiers::NONE => {
                    // Delete operator
                    return self.handle_operator(Operator::Delete);
                },
                KeyCode::Char('c') if key.modifiers == KeyModifiers::NONE => {
                    // Change operator
                    return self.handle_operator(Operator::Change);
                },
                KeyCode::Char('y') if key.modifiers == KeyModifiers::NONE => {
                    // Yank operator
                    return self.handle_operator(Operator::Yank);
                },
                _ => {}
            }
        } else if self.current_mode() == Mode::OperatorPending {
            // Handle keys in operator-pending mode
            return self.handle_operator_pending_key(key);
        }
        // Handle operator keys in visual mode
        if self.current_mode().is_visual() {
            match key.code {
                KeyCode::Char('d') if key.modifiers == KeyModifiers::NONE => {
                    // Delete operator in visual mode
                    return self.handle_visual_operator(Operator::Delete);
                },
                KeyCode::Char('c') if key.modifiers == KeyModifiers::NONE => {
                    // Change operator in visual mode
                    return self.handle_visual_operator(Operator::Change);
                },
                KeyCode::Char('y') if key.modifiers == KeyModifiers::NONE => {
                    // Yank operator in visual mode
                    return self.handle_visual_operator(Operator::Yank);
                },
                _ => {}
            }
        }
        
        // First, try to handle the key with the key handler
        let mode = self.current_mode();
        if let Some(command) = self.key_handler.process_key(key, mode) {
            return self.execute_key_command(command);
        }
        
        // If the key handler didn't handle it, use the legacy key handling
        self.process_key_legacy(key)
    }
    
    /// Handle an operator command
    fn handle_operator(&mut self, operator: Operator) -> EditorResult<()> {
        // Start the operator
        self.operator_manager.start_operator(operator);
        
        // Enter operator-pending mode
        self.mode_manager.enter_operator_pending_mode();
        
        Ok(())
    }
    
    /// Handle a key in operator-pending mode
    fn handle_operator_pending_key(&mut self, key: KeyEvent) -> EditorResult<()> {
        use crossterm::event::{KeyCode, KeyModifiers};
        
        // Get the current operator state
        if let Some(state) = self.operator_manager.current_state() {
            match key.code {
                // Handle escape to cancel the operator
                KeyCode::Esc => {
                    self.operator_manager.cancel();
                    self.mode_manager.enter_normal_mode();
                    return Ok(());
                },
                
                // Handle text object keys
                KeyCode::Char('w') => {
                    // Word text object
                    let target = OperatorTarget::TextObject(TextObjectTypeExt::Word, false);
                    if let Some(completed_state) = self.operator_manager.set_target(target) {
                        return self.execute_operator(completed_state);
                    }
                },
                KeyCode::Char('W') => {
                    // WORD text object (non-whitespace)
                    let target = OperatorTarget::TextObject(TextObjectTypeExt::BigWord, false);
                    if let Some(completed_state) = self.operator_manager.set_target(target) {
                        return self.execute_operator(completed_state);
                    }
                },
                KeyCode::Char('s') => {
                    // Sentence text object
                    let target = OperatorTarget::TextObject(TextObjectTypeExt::Sentence, false);
                    if let Some(completed_state) = self.operator_manager.set_target(target) {
                        return self.execute_operator(completed_state);
                    }
                },
                KeyCode::Char('p') => {
                    // Paragraph text object
                    let target = OperatorTarget::TextObject(TextObjectTypeExt::Paragraph, false);
                    if let Some(completed_state) = self.operator_manager.set_target(target) {
                        return self.execute_operator(completed_state);
                    }
                },
                // Handle inner/around text objects with 'i' and 'a' prefixes
                KeyCode::Char('i') => {
                    // Inner text object - wait for the next key
                    return self.handle_inner_text_object(key);
                },
                KeyCode::Char('a') => {
                    // Around text object - wait for the next key
                    return self.handle_around_text_object(key);
                },
                
                // Handle motion keys
                KeyCode::Char('j') | KeyCode::Down => {
                    // Down motion
                    let target = OperatorTarget::Motion(crate::cursor::Direction::Down);
                    if let Some(completed_state) = self.operator_manager.set_target(target) {
                        return self.execute_operator(completed_state);
                    }
                },
                KeyCode::Char('k') | KeyCode::Up => {
                    // Up motion
                    let target = OperatorTarget::Motion(crate::cursor::Direction::Up);
                    if let Some(completed_state) = self.operator_manager.set_target(target) {
                        return self.execute_operator(completed_state);
                    }
                },
                KeyCode::Char('h') | KeyCode::Left => {
                    // Left motion
                    let target = OperatorTarget::Motion(crate::cursor::Direction::Left);
                    if let Some(completed_state) = self.operator_manager.set_target(target) {
                        return self.execute_operator(completed_state);
                    }
                },
                KeyCode::Char('l') | KeyCode::Right => {
                    // Right motion
                    let target = OperatorTarget::Motion(crate::cursor::Direction::Right);
                    if let Some(completed_state) = self.operator_manager.set_target(target) {
                        return self.execute_operator(completed_state);
                    }
                },
                
                // Handle visual mode operators
                KeyCode::Char('v') => {
                    // Enter visual mode and remember the operator
                    self.mode_manager.enter_visual_mode();
                    // Start selection at current cursor position
                    let cursor_pos = self.cursor_manager.position();
                    self.selection_manager.start_selection(SelectionType::Character, cursor_pos);
                    return Ok(());
                },
                
                KeyCode::Char('V') => {
                    // Enter visual line mode and remember the operator
                    self.mode_manager.enter_visual_line_mode();
                    // Start selection at current cursor position
                    let cursor_pos = self.cursor_manager.position();
                    self.selection_manager.start_selection(SelectionType::Line, cursor_pos);
                    return Ok(());
                },
                
                // Handle line-wise operators (dd, yy, etc.)
                KeyCode::Char(c) => {
                    if let Some(op) = Operator::from_char(c) {
                        if op == state.operator {
                            // Line-wise operation (e.g., dd, yy, cc)
                            let cursor_pos = self.cursor_manager.position();
                            let target = OperatorTarget::LineRange(cursor_pos.line, cursor_pos.line);
                            if let Some(completed_state) = self.operator_manager.set_target(target) {
                                return self.execute_operator(completed_state);
                            }
                        }
                    }
                },
                
                _ => {
                    // Unhandled key, cancel the operator
                    self.operator_manager.cancel();
                    self.mode_manager.enter_normal_mode();
                }
            }
        } else {
            // No operator state, return to normal mode
            self.mode_manager.enter_normal_mode();
        }
        
        Ok(())
    }
    
    /// Execute an operator with its target
    fn execute_operator(&mut self, state: OperatorState) -> EditorResult<()> {
        // Return to normal mode
        self.mode_manager.enter_normal_mode();
        
        // Extract the operator and target before the match to avoid borrowing issues
        let operator = state.operator;
        let target = state.target.clone(); // Clone here to avoid borrowing issues
        
        match (operator, target) {
            (Operator::Delete, Some(OperatorTarget::TextObject(object_type, include_delimiters))) => {
                // Delete text object
                // Convert from text_object::TextObjectType to editor::TextObjectType
                let editor_object_type = match object_type {
                    TextObjectTypeExt::Word => TextObjectType::Word,
                    TextObjectTypeExt::BigWord => TextObjectType::BigWord,
                    TextObjectTypeExt::Sentence => TextObjectType::Sentence,
                    TextObjectTypeExt::Paragraph => TextObjectType::Paragraph,
                    TextObjectTypeExt::SingleQuoteBlock => TextObjectType::SingleQuoteBlock,
                    TextObjectTypeExt::DoubleQuoteBlock => TextObjectType::DoubleQuoteBlock,
                    TextObjectTypeExt::ParenBlock => TextObjectType::ParenBlock,
                    TextObjectTypeExt::BraceBlock => TextObjectType::BraceBlock,
                    TextObjectTypeExt::BracketBlock => TextObjectType::BracketBlock,
                    TextObjectTypeExt::AngleBlock => TextObjectType::AngleBlock,
                    TextObjectTypeExt::TagBlock => TextObjectType::TagBlock,
                    TextObjectTypeExt::BacktickBlock => TextObjectType::BacktickBlock,
                };
                
                self.delete_text_object(editor_object_type, include_delimiters)?;
            },
            (Operator::Change, Some(OperatorTarget::TextObject(object_type, include_delimiters))) => {
                // Change text object
                // Convert from text_object::TextObjectType to editor::TextObjectType
                let editor_object_type = match object_type {
                    TextObjectTypeExt::Word => TextObjectType::Word,
                    TextObjectTypeExt::BigWord => TextObjectType::BigWord,
                    TextObjectTypeExt::Sentence => TextObjectType::Sentence,
                    TextObjectTypeExt::Paragraph => TextObjectType::Paragraph,
                    TextObjectTypeExt::SingleQuoteBlock => TextObjectType::SingleQuoteBlock,
                    TextObjectTypeExt::DoubleQuoteBlock => TextObjectType::DoubleQuoteBlock,
                    TextObjectTypeExt::ParenBlock => TextObjectType::ParenBlock,
                    TextObjectTypeExt::BraceBlock => TextObjectType::BraceBlock,
                    TextObjectTypeExt::BracketBlock => TextObjectType::BracketBlock,
                    TextObjectTypeExt::AngleBlock => TextObjectType::AngleBlock,
                    TextObjectTypeExt::TagBlock => TextObjectType::TagBlock,
                    TextObjectTypeExt::BacktickBlock => TextObjectType::BacktickBlock,
                };
                
                self.change_text_object(editor_object_type, include_delimiters)?;
            },
            (Operator::Yank, Some(OperatorTarget::TextObject(object_type, include_delimiters))) => {
                // Yank text object
                // Convert from text_object::TextObjectType to editor::TextObjectType
                let editor_object_type = match object_type {
                    TextObjectTypeExt::Word => TextObjectType::Word,
                    TextObjectTypeExt::BigWord => TextObjectType::BigWord,
                    TextObjectTypeExt::Sentence => TextObjectType::Sentence,
                    TextObjectTypeExt::Paragraph => TextObjectType::Paragraph,
                    TextObjectTypeExt::SingleQuoteBlock => TextObjectType::SingleQuoteBlock,
                    TextObjectTypeExt::DoubleQuoteBlock => TextObjectType::DoubleQuoteBlock,
                    TextObjectTypeExt::ParenBlock => TextObjectType::ParenBlock,
                    TextObjectTypeExt::BraceBlock => TextObjectType::BraceBlock,
                    TextObjectTypeExt::BracketBlock => TextObjectType::BracketBlock,
                    TextObjectTypeExt::AngleBlock => TextObjectType::AngleBlock,
                    TextObjectTypeExt::TagBlock => TextObjectType::TagBlock,
                    TextObjectTypeExt::BacktickBlock => TextObjectType::BacktickBlock,
                };
                
                self.yank_text_object(editor_object_type, include_delimiters)?;
            },
            (Operator::Delete, Some(OperatorTarget::Motion(direction))) => {
                // Delete to motion
                self.delete_to_motion(direction)?;
            },
            (Operator::Change, Some(OperatorTarget::Motion(direction))) => {
                // Change to motion
                self.change_to_motion(direction)?;
            },
            (Operator::Yank, Some(OperatorTarget::Motion(direction))) => {
                // Yank to motion
                self.yank_to_motion(direction)?;
            },
            (Operator::Delete, Some(OperatorTarget::LineRange(start, end))) => {
                // Delete lines
                self.delete_lines(start, end)?;
            },
            (Operator::Change, Some(OperatorTarget::LineRange(start, end))) => {
                // Change lines
                self.change_lines(start, end)?;
            },
            (Operator::Yank, Some(OperatorTarget::LineRange(start, end))) => {
                // Yank lines
                self.yank_lines(start, end)?;
            },
            _ => {
                // Unsupported operator/target combination
                return Err(EditorError::Other(format!("Unsupported operator/target combination: {:?}", operator)));
            }
        }
        
        Ok(())
    }
    
    /// Execute a key command
    fn execute_key_command(&mut self, command: KeyCommand) -> EditorResult<()> {
        match command {
            KeyCommand::BuiltIn(cmd) => {
                match cmd.as_str() {
                    "quit" => self.quit(),
                    "enter_insert_mode" => self.mode_manager.enter_insert_mode(),
                    "enter_normal_mode" => self.mode_manager.enter_normal_mode(),
                    "enter_command_mode" => self.mode_manager.enter_command_mode(),
                    "undo" => { self.undo()?; },
                    "redo" => { self.redo()?; },
                    "paste" => { self.paste()?; },
                    "paste_before" => {
                        // Move cursor left, paste, then move cursor right
                        if let Some(buffer_id) = self.current_buffer_id() {
                            let buffer = self.buffer_manager.get_buffer(buffer_id)?;
                            self.cursor_manager.move_cursor(Direction::Left, buffer)?;
                            
                            // Clone the buffer to avoid borrowing issues
                            let buffer_id_copy = buffer_id;
                            self.paste()?;
                            
                            // Get the buffer again after paste
                            let buffer = self.buffer_manager.get_buffer(buffer_id_copy)?;
                            self.cursor_manager.move_cursor(Direction::Right, buffer)?;
                        }
                    },
                    "next_tab" => {
                        self.terminal.next_tab()?;
                    },
                    "prev_tab" => {
                        self.terminal.prev_tab()?;
                    },
                    "start_recording" => {
                        // This would normally get a register from the user
                        // For now, just use register 'a'
                        self.start_macro_recording('a');
                    },
                    "stop_recording" => {
                        self.stop_macro_recording();
                    },
                    "play_macro" => {
                        // This would normally get a register from the user
                        // For now, just use register 'a'
                        self.play_macro('a');
                    },
                    _ => return Err(EditorError::Other(format!("Unknown built-in command: {}", cmd))),
                }
            },
            KeyCommand::Custom(cmd) => {
                return Err(EditorError::Other(format!("Unknown custom command: {}", cmd)));
            },
            KeyCommand::Keys(sequence) => {
                // Execute each key in the sequence
                for key in sequence.keys() {
                    self.process_key(*key)?;
                }
            },
        }
        
        Ok(())
    }
    
    /// Legacy key processing method (for backward compatibility)
    fn process_key_legacy(&mut self, key: KeyEvent) -> EditorResult<()> {
        use crossterm::event::{KeyCode, KeyModifiers};
        use crate::mode::Mode;
        
        match (self.current_mode(), key.code) {
            // Basic cursor movement in normal mode
            (Mode::Normal, KeyCode::Up) | (Mode::Normal, KeyCode::Char('k')) => {
                if let Some(buffer_id) = self.current_buffer_id() {
                    let buffer = self.buffer_manager.get_buffer(buffer_id)?;
                    self.cursor_manager.move_cursor(Direction::Up, buffer)?;
                }
            },
            
            (Mode::Normal, KeyCode::Down) | (Mode::Normal, KeyCode::Char('j')) => {
                if let Some(buffer_id) = self.current_buffer_id() {
                    let buffer = self.buffer_manager.get_buffer(buffer_id)?;
                    self.cursor_manager.move_cursor(Direction::Down, buffer)?;
                }
            },
            
            (Mode::Normal, KeyCode::Left) | (Mode::Normal, KeyCode::Char('h')) => {
                if let Some(buffer_id) = self.current_buffer_id() {
                    let buffer = self.buffer_manager.get_buffer(buffer_id)?;
                    self.cursor_manager.move_cursor(Direction::Left, buffer)?;
                }
            },
            
            (Mode::Normal, KeyCode::Right) | (Mode::Normal, KeyCode::Char('l')) => {
                if let Some(buffer_id) = self.current_buffer_id() {
                    let buffer = self.buffer_manager.get_buffer(buffer_id)?;
                    self.cursor_manager.move_cursor(Direction::Right, buffer)?;
                }
            },
            
            // Word movement in normal mode
            (Mode::Normal, KeyCode::Char('w')) => {
                if let Some(buffer_id) = self.current_buffer_id() {
                    let buffer = self.buffer_manager.get_buffer(buffer_id)?;
                    self.cursor_manager.move_cursor(Direction::WordNext, buffer)?;
                }
            },
            
            (Mode::Normal, KeyCode::Char('b')) if !key.modifiers.contains(KeyModifiers::CONTROL) => {
                if let Some(buffer_id) = self.current_buffer_id() {
                    let buffer = self.buffer_manager.get_buffer(buffer_id)?;
                    self.cursor_manager.move_cursor(Direction::WordPrev, buffer)?;
                }
            },
            
            (Mode::Normal, KeyCode::Char('e')) => {
                if let Some(buffer_id) = self.current_buffer_id() {
                    let buffer = self.buffer_manager.get_buffer(buffer_id)?;
                    self.cursor_manager.move_cursor(Direction::WordEnd, buffer)?;
                }
            },
            
            // Line movement in normal mode
            (Mode::Normal, KeyCode::Home) | (Mode::Normal, KeyCode::Char('0')) => {
                if let Some(buffer_id) = self.current_buffer_id() {
                    let buffer = self.buffer_manager.get_buffer(buffer_id)?;
                    self.cursor_manager.move_cursor(Direction::LineStart, buffer)?;
                }
            },
            
            (Mode::Normal, KeyCode::End) | (Mode::Normal, KeyCode::Char('$')) => {
                if let Some(buffer_id) = self.current_buffer_id() {
                    let buffer = self.buffer_manager.get_buffer(buffer_id)?;
                    self.cursor_manager.move_cursor(Direction::LineEnd, buffer)?;
                }
            },
            
            // Buffer movement in normal mode
            (Mode::Normal, KeyCode::Char('g')) => {
                if let Some(buffer_id) = self.current_buffer_id() {
                    let buffer = self.buffer_manager.get_buffer(buffer_id)?;
                    self.cursor_manager.move_cursor(Direction::BufferStart, buffer)?;
                }
            },
            
            (Mode::Normal, KeyCode::Char('G')) => {
                if let Some(buffer_id) = self.current_buffer_id() {
                    let buffer = self.buffer_manager.get_buffer(buffer_id)?;
                    self.cursor_manager.move_cursor(Direction::BufferEnd, buffer)?;
                }
            },
            
            // Paragraph movement in normal mode
            (Mode::Normal, KeyCode::Char('{')) => {
                if let Some(buffer_id) = self.current_buffer_id() {
                    let buffer = self.buffer_manager.get_buffer(buffer_id)?;
                    self.cursor_manager.move_cursor(Direction::ParagraphStart, buffer)?;
                }
            },
            
            (Mode::Normal, KeyCode::Char('}')) => {
                if let Some(buffer_id) = self.current_buffer_id() {
                    let buffer = self.buffer_manager.get_buffer(buffer_id)?;
                    self.cursor_manager.move_cursor(Direction::ParagraphEnd, buffer)?;
                }
            },
            
            // Start search with '/' in normal mode
            (Mode::Normal, KeyCode::Char('/')) => {
                // TODO: Show search prompt in UI
                self.mode_manager.enter_command_mode();
            },
            
            // Find next with 'n' in normal mode
            (Mode::Normal, KeyCode::Char('n')) => {
                // TODO: Implement find next based on last search
                // For now, just a placeholder
            },
            
            // Find previous with 'N' in normal mode
            (Mode::Normal, KeyCode::Char('N')) => {
                // TODO: Implement find previous based on last search
                // For now, just a placeholder
            },
            
            // Enter visual mode in normal mode
            (Mode::Normal, KeyCode::Char('v')) => {
                self.mode_manager.enter_visual_mode();
                // Start selection at current cursor position
                let cursor_pos = self.cursor_manager.position();
                self.selection_manager.start_selection(SelectionType::Character, cursor_pos);
            },
            
            // Enter visual line mode in normal mode
            (Mode::Normal, KeyCode::Char('V')) => {
                self.mode_manager.enter_visual_line_mode();
                // Start selection at current cursor position
                let cursor_pos = self.cursor_manager.position();
                self.selection_manager.start_selection(SelectionType::Line, cursor_pos);
            },
            
            // Enter visual block mode in normal mode
            (Mode::Normal, KeyCode::Char('b')) if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.mode_manager.enter_visual_block_mode();
                // Start selection at current cursor position
                let cursor_pos = self.cursor_manager.position();
                self.selection_manager.start_selection(SelectionType::Block, cursor_pos);
            },
            
            // Exit visual mode with Escape
            (mode, KeyCode::Esc) if mode.is_visual() => {
                // End selection
                self.selection_manager.end_selection();
                self.mode_manager.enter_normal_mode();
            },
            
            // Update selection when moving in visual mode
            (mode, _) if mode.is_visual() => {
                // First, process the key normally to move the cursor
                let result = match (mode, key.code) {
                    (_, KeyCode::Up) | (_, KeyCode::Char('k')) => {
                        if let Some(buffer_id) = self.current_buffer_id() {
                            let buffer = self.buffer_manager.get_buffer(buffer_id)?;
                            self.cursor_manager.move_cursor(Direction::Up, buffer)?;
                        }
                        Ok(())
                    },
                    (_, KeyCode::Down) | (_, KeyCode::Char('j')) => {
                        if let Some(buffer_id) = self.current_buffer_id() {
                            let buffer = self.buffer_manager.get_buffer(buffer_id)?;
                            self.cursor_manager.move_cursor(Direction::Down, buffer)?;
                        }
                        Ok(())
                    },
                    (_, KeyCode::Left) | (_, KeyCode::Char('h')) => {
                        if let Some(buffer_id) = self.current_buffer_id() {
                            let buffer = self.buffer_manager.get_buffer(buffer_id)?;
                            self.cursor_manager.move_cursor(Direction::Left, buffer)?;
                        }
                        Ok(())
                    },
                    (_, KeyCode::Right) | (_, KeyCode::Char('l')) => {
                        if let Some(buffer_id) = self.current_buffer_id() {
                            let buffer = self.buffer_manager.get_buffer(buffer_id)?;
                            self.cursor_manager.move_cursor(Direction::Right, buffer)?;
                        }
                        Ok(())
                    },
                    (_, KeyCode::Char('w')) => {
                        if let Some(buffer_id) = self.current_buffer_id() {
                            let buffer = self.buffer_manager.get_buffer(buffer_id)?;
                            self.cursor_manager.move_cursor(Direction::WordNext, buffer)?;
                        }
                        Ok(())
                    },
                    (_, KeyCode::Char('b')) => {
                        if let Some(buffer_id) = self.current_buffer_id() {
                            let buffer = self.buffer_manager.get_buffer(buffer_id)?;
                            self.cursor_manager.move_cursor(Direction::WordPrev, buffer)?;
                        }
                        Ok(())
                    },
                    (_, KeyCode::Char('e')) => {
                        if let Some(buffer_id) = self.current_buffer_id() {
                            let buffer = self.buffer_manager.get_buffer(buffer_id)?;
                            self.cursor_manager.move_cursor(Direction::WordEnd, buffer)?;
                        }
                        Ok(())
                    },
                    (_, KeyCode::Home) | (_, KeyCode::Char('0')) => {
                        if let Some(buffer_id) = self.current_buffer_id() {
                            let buffer = self.buffer_manager.get_buffer(buffer_id)?;
                            self.cursor_manager.move_cursor(Direction::LineStart, buffer)?;
                        }
                        Ok(())
                    },
                    (_, KeyCode::End) | (_, KeyCode::Char('$')) => {
                        if let Some(buffer_id) = self.current_buffer_id() {
                            let buffer = self.buffer_manager.get_buffer(buffer_id)?;
                            self.cursor_manager.move_cursor(Direction::LineEnd, buffer)?;
                        }
                        Ok(())
                    },
                    (_, KeyCode::Char('g')) => {
                        if let Some(buffer_id) = self.current_buffer_id() {
                            let buffer = self.buffer_manager.get_buffer(buffer_id)?;
                            self.cursor_manager.move_cursor(Direction::BufferStart, buffer)?;
                        }
                        Ok(())
                    },
                    (_, KeyCode::Char('G')) => {
                        if let Some(buffer_id) = self.current_buffer_id() {
                            let buffer = self.buffer_manager.get_buffer(buffer_id)?;
                            self.cursor_manager.move_cursor(Direction::BufferEnd, buffer)?;
                        }
                        Ok(())
                    },
                    _ => Ok(()),
                };
                
                // Then update the selection with the new cursor position
                if result.is_ok() {
                    let cursor_pos = self.cursor_manager.position();
                    self.selection_manager.update_selection(cursor_pos);
                }
                
                // Return the result of the key processing
                return result;
            },
            
            // Delete word with 'dw' in normal mode
            (Mode::Normal, KeyCode::Char('d')) => {
                // TODO: Implement operator-pending mode
                // For now, just delete the word at the cursor
                self.delete_text_object(TextObjectType::Word, false)?;
            },
            
            // Delete inner word with 'diw' in normal mode
            (Mode::Normal, KeyCode::Char('i')) if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.delete_text_object(TextObjectType::Word, false)?;
            },
            
            // Delete around word with 'daw' in normal mode
            (Mode::Normal, KeyCode::Char('a')) if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.delete_text_object(TextObjectType::Word, true)?;
            },
            
            // Change word with 'cw' in normal mode
            (Mode::Normal, KeyCode::Char('c')) => {
                // TODO: Implement operator-pending mode
                // For now, just change the word at the cursor
                self.change_text_object(TextObjectType::Word, false)?;
            },
            
            // Yank word with 'yw' in normal mode
            (Mode::Normal, KeyCode::Char('y')) => {
                // TODO: Implement operator-pending mode
                // For now, just yank the word at the cursor
                self.yank_text_object(TextObjectType::Word, false)?;
            },
            
            // Paste after cursor with 'p' in normal mode
            (Mode::Normal, KeyCode::Char('p')) => {
                self.paste()?;
            },
            
            // Open line below with 'o' in normal mode
            (Mode::Normal, KeyCode::Char('o')) => {
                if let Some(buffer_id) = self.current_buffer_id() {
                    let buffer = self.buffer_manager.get_buffer_mut(buffer_id)?;
                    
                    // Get cursor position
                    let cursor_pos = self.cursor_manager.position();
                    
                    // Move to the end of the current line
                    self.cursor_manager.move_cursor(Direction::LineEnd, buffer)?;
                    
                    // Get the updated cursor position
                    let end_pos = self.cursor_manager.position();
                    
                    // Convert cursor position to character index
                    let cursor_position = buffer.position_to_char_idx(end_pos.line, end_pos.column)?;
                    
                    // Insert a newline at the cursor position
                    buffer.insert(cursor_position, "\n")?;
                    
                    // Update cursor position to the beginning of the new line
                    let new_position = buffer.char_idx_to_position(cursor_position + 1)?;
                    self.cursor_manager.set_position(new_position);
                    
                    // Enter insert mode
                    self.mode_manager.enter_insert_mode();
                }
            },
            
            // Open line above with 'O' in normal mode
            (Mode::Normal, KeyCode::Char('O')) => {
                if let Some(buffer_id) = self.current_buffer_id() {
                    let buffer = self.buffer_manager.get_buffer_mut(buffer_id)?;
                    
                    // Get cursor position
                    let cursor_pos = self.cursor_manager.position();
                    
                    // Move to the beginning of the current line
                    self.cursor_manager.move_cursor(Direction::LineStart, buffer)?;
                    
                    // Get the updated cursor position
                    let start_pos = self.cursor_manager.position();
                    
                    // Convert cursor position to character index
                    let cursor_position = buffer.position_to_char_idx(start_pos.line, start_pos.column)?;
                    
                    // Insert a newline at the cursor position
                    buffer.insert(cursor_position, "\n")?;
                    
                    // Update cursor position to the beginning of the original line
                    let new_position = buffer.char_idx_to_position(cursor_position)?;
                    self.cursor_manager.set_position(new_position);
                    
                    // Enter insert mode
                    self.mode_manager.enter_insert_mode();
                }
            },
            
            // Paste from register with '"ap' in normal mode
            (Mode::Normal, KeyCode::Char('"')) => {
                // TODO: Implement a proper way to get the next key press
                // For now, just paste from the unnamed register
                self.paste()?;
            },
            
            // Set mark with 'm' followed by a character in normal mode
            (Mode::Normal, KeyCode::Char('m')) => {
                // TODO: Implement a proper way to get the next key press
                // For now, just set a mark at the current position with a fixed name
                self.set_mark('a')?;
            },
            
            // Jump to mark with '`' followed by a character in normal mode
            (Mode::Normal, KeyCode::Char('`')) => {
                // TODO: Implement a proper way to get the next key press
                // For now, just jump to a mark with a fixed name
                self.jump_to_mark('a')?;
            },
            
            // Process command when Enter is pressed in command mode
            (Mode::Command, KeyCode::Enter) => {
                // Get the command text from the command buffer
                let command_text = self.command_buffer.clone();
                
                // Clear the command buffer
                self.command_buffer.clear();
                
                // Return to normal mode
                self.mode_manager.enter_normal_mode();
                
                // Process the command if it's not empty
                if !command_text.is_empty() {
                    self.process_command(&command_text)?;
                }
            },
            
            // Handle backspace in command mode
            (Mode::Command, KeyCode::Backspace) => {
                // Remove the last character from the command buffer
                self.command_buffer.pop();
            },
            
            // Handle escape in command mode
            (Mode::Command, KeyCode::Esc) => {
                // Clear the command buffer and return to normal mode
                self.command_buffer.clear();
                self.mode_manager.enter_normal_mode();
            },
            
            // Handle other keys in command mode
            (Mode::Command, KeyCode::Char(c)) => {
                // Add the character to the command buffer
                self.command_buffer.push(c);
            },
            
            // Handle key presses in insert mode
            (Mode::Insert, KeyCode::Char(c)) => {
                if let Some(buffer_id) = self.current_buffer_id() {
                    let buffer = self.buffer_manager.get_buffer_mut(buffer_id)?;
                    
                    // Get cursor position
                    let cursor_pos = self.cursor_manager.position();
                    
                    // Convert cursor position to character index
                    let cursor_position = buffer.position_to_char_idx(cursor_pos.line, cursor_pos.column)?;
                    
                    // Insert the character at the cursor position
                    buffer.insert(cursor_position, &c.to_string())?;
                    
                    // Update cursor position
                    let new_position = buffer.char_idx_to_position(cursor_position + 1)?;
                    self.cursor_manager.set_position(new_position);
                }
            },
            
            // Handle backspace in insert mode
            (Mode::Insert, KeyCode::Backspace) => {
                if let Some(buffer_id) = self.current_buffer_id() {
                    let buffer = self.buffer_manager.get_buffer_mut(buffer_id)?;
                    
                    // Get cursor position
                    let cursor_pos = self.cursor_manager.position();
                    
                    // Convert cursor position to character index
                    let cursor_position = buffer.position_to_char_idx(cursor_pos.line, cursor_pos.column)?;
                    
                    // Only delete if we're not at the beginning of the buffer
                    if cursor_position > 0 {
                        // Delete the character before the cursor
                        buffer.delete(cursor_position - 1, cursor_position)?;
                        
                        // Update cursor position
                        let new_position = buffer.char_idx_to_position(cursor_position - 1)?;
                        self.cursor_manager.set_position(new_position);
                    }
                }
            },
            
            // Handle enter in insert mode
            (Mode::Insert, KeyCode::Enter) => {
                if let Some(buffer_id) = self.current_buffer_id() {
                    let buffer = self.buffer_manager.get_buffer_mut(buffer_id)?;
                    
                    // Get cursor position
                    let cursor_pos = self.cursor_manager.position();
                    
                    // Convert cursor position to character index
                    let cursor_position = buffer.position_to_char_idx(cursor_pos.line, cursor_pos.column)?;
                    
                    // Insert a newline at the cursor position
                    buffer.insert(cursor_position, "\n")?;
                    
                    // Update cursor position
                    let new_position = buffer.char_idx_to_position(cursor_position + 1)?;
                    self.cursor_manager.set_position(new_position);
                }
            },
            
            // TODO: Handle other keys based on mode
            _ => {
                // Unhandled key
            }
        }
        
        Ok(())
    }

    /// Run the main editor loop
    pub fn run(&mut self) -> EditorResult<()> {
        // Initialize terminal
        self.terminal.init()?;
        self.running = true;
        
        // Reset the quit flag
        crate::command::reset_quit_flag();
        
        // Set the global editor reference for command handlers
        crate::command::set_editor(self);
        
        // Main event loop
        while self.running {
            // Render the current state
            self.render()?;
            
            // Handle input
            if let Some(key) = self.terminal.poll_key(100)? {
                self.process_key(key)?;
            }
            
            // Check if a command has requested to quit
            if crate::command::should_quit() {
                self.running = false;
            }
        }
        
        // Clean up terminal
        self.terminal.cleanup()?;
        
        Ok(())
    }
    
    /// Undo the last change in the current buffer
    pub fn undo(&mut self) -> EditorResult<bool> {
        if let Some(buffer_id) = self.current_buffer_id() {
            let buffer = self.buffer_manager.get_buffer_mut(buffer_id)?;
            match buffer.undo() {
                Ok(result) => Ok(result),
                Err(err) => Err(EditorError::Buffer(err.into())),
            }
        } else {
            Ok(false)
        }
    }
    
    /// Redo the last undone change in the current buffer
    pub fn redo(&mut self) -> EditorResult<bool> {
        if let Some(buffer_id) = self.current_buffer_id() {
            let buffer = self.buffer_manager.get_buffer_mut(buffer_id)?;
            match buffer.redo() {
                Ok(result) => Ok(result),
                Err(err) => Err(EditorError::Buffer(err.into())),
            }
        } else {
            Ok(false)
        }
    }
    
    /// Process a command
    pub fn process_command(&mut self, command_str: &str) -> EditorResult<()> {
        // Parse the command using the command parser
        match self.command_parser.parse_ex(command_str) {
            Ok(ex_cmd) => {
                // Execute the command using the ex command registry
                match self.ex_command_registry.execute(&ex_cmd) {
                    Ok(_) => Ok(()),
                    Err(err) => Err(EditorError::Other(format!("Command error: {}", err))),
                }
            },
            Err(err) => Err(EditorError::Other(format!("Invalid command: {}", err))),
        }
    }
    
    /// Quit the editor
    pub fn quit(&mut self) {
        self.running = false;
    }
    
    /// Search for a pattern in the current buffer
    pub fn search(&mut self, pattern: &str, case_sensitive: bool) -> EditorResult<Vec<(usize, usize, String)>> {
        if let Some(buffer_id) = self.current_buffer_id() {
            let buffer = self.buffer_manager.get_buffer(buffer_id)?;
            match buffer.search(pattern, case_sensitive) {
                Ok(results) => {
                    // TODO: Highlight search results in UI
                    // TODO: Move cursor to first match
                    Ok(results)
                },
                Err(err) => Err(EditorError::Buffer(err.into())),
            }
        } else {
            Ok(Vec::new())
        }
    }
    
    /// Search and replace text in the current buffer
    pub fn search_and_replace(&mut self, pattern: &str, replacement: &str, case_sensitive: bool, confirm: bool) -> EditorResult<usize> {
        if let Some(buffer_id) = self.current_buffer_id() {
            let buffer = self.buffer_manager.get_buffer_mut(buffer_id)?;
            
            if confirm {
                // TODO: Implement interactive confirmation for each replacement
                // For now, just do the replacement without confirmation
                match buffer.search_and_replace(pattern, replacement, case_sensitive) {
                    Ok(count) => Ok(count),
                    Err(err) => Err(EditorError::Buffer(err.into())),
                }
            } else {
                match buffer.search_and_replace(pattern, replacement, case_sensitive) {
                    Ok(count) => Ok(count),
                    Err(err) => Err(EditorError::Buffer(err.into())),
                }
            }
        } else {
            Ok(0)
        }
    }
    
    /// Find the next occurrence of a pattern
    pub fn find_next(&mut self, pattern: &str, case_sensitive: bool) -> EditorResult<Option<(usize, usize, String)>> {
        if let Some(buffer_id) = self.current_buffer_id() {
            let buffer = self.buffer_manager.get_buffer(buffer_id)?;
            
            // Get cursor position
            let cursor_pos = self.cursor_manager.position();
            
            match buffer.find_next(pattern, cursor_pos.line, cursor_pos.column, case_sensitive) {
                Ok(result) => {
                    // Move cursor to the match position if found
                    if let Some((line, column, _)) = result {
                        let new_pos = crate::cursor::CursorPosition::new(line, column);
                        self.cursor_manager.set_position(new_pos);
                    }
                    Ok(result)
                },
                Err(err) => Err(EditorError::Buffer(err.into())),
            }
        } else {
            Ok(None)
        }
    }
    
    /// Find the previous occurrence of a pattern
    pub fn find_prev(&mut self, pattern: &str, case_sensitive: bool) -> EditorResult<Option<(usize, usize, String)>> {
        if let Some(buffer_id) = self.current_buffer_id() {
            let buffer = self.buffer_manager.get_buffer(buffer_id)?;
            
            // Get cursor position
            let cursor_pos = self.cursor_manager.position();
            
            match buffer.find_prev(pattern, cursor_pos.line, cursor_pos.column, case_sensitive) {
                Ok(result) => {
                    // Move cursor to the match position if found
                    if let Some((line, column, _)) = result {
                        let new_pos = crate::cursor::CursorPosition::new(line, column);
                        self.cursor_manager.set_position(new_pos);
                    }
                    Ok(result)
                },
                Err(err) => Err(EditorError::Buffer(err.into())),
            }
        } else {
            Ok(None)
        }
    }
    
    /// Find a text object at the cursor position
    pub fn find_text_object(&self, object_type: TextObjectType, include_delimiters: bool) -> EditorResult<Option<TextObject>> {
        if let Some(buffer_id) = self.current_buffer_id() {
            let buffer = self.buffer_manager.get_buffer(buffer_id)?;
            
            // Get cursor position
            let cursor_pos = self.cursor_manager.position();
            
            // Convert cursor position to character index
            let cursor_position = buffer.position_to_char_idx(cursor_pos.line, cursor_pos.column)?;
            
            let text_object = self.find_text_object_internal(buffer, cursor_position, object_type, include_delimiters);
            Ok(text_object)
        } else {
            Ok(None)
        }
    }
    
    /// Delete a text object at the cursor position
    pub fn delete_text_object(&mut self, object_type: TextObjectType, include_delimiters: bool) -> EditorResult<bool> {
        if let Some(buffer_id) = self.current_buffer_id() {
            let buffer = self.buffer_manager.get_buffer(buffer_id)?;
            
            // Get cursor position
            let cursor_pos = self.cursor_manager.position();
            
            // Convert cursor position to character index
            let cursor_position = buffer.position_to_char_idx(cursor_pos.line, cursor_pos.column)?;
            
            if let Some(text_object) = self.find_text_object_internal(buffer, cursor_position, object_type, include_delimiters) {
                // Get the text from the buffer before deleting it
                let content = buffer.content();
                if text_object.start < content.len() && text_object.end <= content.len() {
                    let text = content[text_object.start..text_object.end].to_string();
                    
                    // Store the text in the unnamed register
                    let register_content = RegisterContent::character_wise(&text);
                    self.register_manager.set_register(RegisterType::Unnamed, register_content);
                }
                
                let buffer = self.buffer_manager.get_buffer_mut(buffer_id)?;
                // Delete the text object
                match buffer.delete(text_object.start, text_object.end) {
                    Ok(_) => Ok(true),
                    Err(err) => Err(EditorError::Buffer(err.into())),
                }
            } else {
                Ok(false)
            }
        } else {
            Ok(false)
        }
    }
    
    /// Change a text object at the cursor position
    pub fn change_text_object(&mut self, object_type: TextObjectType, include_delimiters: bool) -> EditorResult<bool> {
        if let Some(buffer_id) = self.current_buffer_id() {
            let buffer = self.buffer_manager.get_buffer(buffer_id)?;
            
            // Get cursor position
            let cursor_pos = self.cursor_manager.position();
            
            // Convert cursor position to character index
            let cursor_position = buffer.position_to_char_idx(cursor_pos.line, cursor_pos.column)?;
            
            if let Some(text_object) = self.find_text_object_internal(buffer, cursor_position, object_type, include_delimiters) {
                // Get the text from the buffer before deleting it
                let content = buffer.content();
                if text_object.start < content.len() && text_object.end <= content.len() {
                    let text = content[text_object.start..text_object.end].to_string();
                    
                    // Store the text in the unnamed register
                    let register_content = RegisterContent::character_wise(&text);
                    self.register_manager.set_register(RegisterType::Unnamed, register_content);
                }
                
                let buffer = self.buffer_manager.get_buffer_mut(buffer_id)?;
                // Delete the text object
                match buffer.delete(text_object.start, text_object.end) {
                    Ok(_) => {
                        // Enter insert mode
                        self.mode_manager.enter_insert_mode();
                        Ok(true)
                    },
                    Err(err) => Err(EditorError::Buffer(err.into())),
                }
            } else {
                Ok(false)
            }
        } else {
            Ok(false)
        }
    }
    
    /// Yank (copy) a text object at the cursor position
    pub fn yank_text_object(&mut self, object_type: TextObjectType, include_delimiters: bool) -> EditorResult<Option<String>> {
        if let Some(buffer_id) = self.current_buffer_id() {
            let buffer = self.buffer_manager.get_buffer(buffer_id)?;
            
            // Get cursor position
            let cursor_pos = self.cursor_manager.position();
            
            // Convert cursor position to character index
            let cursor_position = buffer.position_to_char_idx(cursor_pos.line, cursor_pos.column)?;
            
            match self.find_text_object_internal(buffer, cursor_position, object_type, include_delimiters) {
                Some(text_object) => {
                    // Get the text from the buffer
                    let content = buffer.content();
                    if text_object.start < content.len() && text_object.end <= content.len() {
                        let text = content[text_object.start..text_object.end].to_string();
                        
                        // Store the text in the unnamed register
                        let register_content = RegisterContent::character_wise(&text);
                        self.register_manager.set_register(RegisterType::Unnamed, register_content);
                        
                        Ok(Some(text))
                    } else {
                        Ok(None)
                    }
                },
                None => Ok(None),
            }
        } else {
            Ok(None)
        }
    }
    
    /// Set a mark at the current cursor position
    pub fn set_mark(&mut self, name: char) -> EditorResult<()> {
        if let Some(buffer_id) = self.current_buffer_id() {
            let buffer = self.buffer_manager.get_buffer_mut(buffer_id)?;
            
            // Get cursor position
            let cursor_pos = self.cursor_manager.position();
            
            buffer.set_mark(name, cursor_pos.line, cursor_pos.column)?;
            Ok(())
        } else {
            Err(EditorError::Other("No buffer selected".to_string()))
        }
    }
    
    /// Get a mark by name
    pub fn get_mark(&self, name: char) -> EditorResult<Option<(usize, usize)>> {
        if let Some(buffer_id) = self.current_buffer_id() {
            let buffer = self.buffer_manager.get_buffer(buffer_id)?;
            
            if let Some(mark) = buffer.get_mark(name) {
                Ok(Some((mark.line, mark.column)))
            } else {
                Ok(None)
            }
        } else {
            Err(EditorError::Other("No buffer selected".to_string()))
        }
    }
    
    /// Jump to a mark
    pub fn jump_to_mark(&mut self, name: char) -> EditorResult<bool> {
        if let Some(buffer_id) = self.current_buffer_id() {
            let buffer = self.buffer_manager.get_buffer(buffer_id)?;
            
            match buffer.jump_to_mark(name)? {
                Some((line, column)) => {
                    // Update cursor position
                    let cursor_pos = crate::cursor::CursorPosition::new(line, column);
                    self.cursor_manager.set_position(cursor_pos);
                    Ok(true)
                },
                None => Ok(false),
            }
        } else {
            Err(EditorError::Other("No buffer selected".to_string()))
        }
    }
    
    /// Delete a mark
    pub fn delete_mark(&mut self, name: char) -> EditorResult<bool> {
        if let Some(buffer_id) = self.current_buffer_id() {
            let buffer = self.buffer_manager.get_buffer_mut(buffer_id)?;
            
            Ok(buffer.remove_mark(name).is_some())
        } else {
            Err(EditorError::Other("No buffer selected".to_string()))
        }
    }
    
    /// Internal implementation of find_text_object
    fn find_text_object_internal(&self, buffer: &crate::buffer::Buffer, position: usize, object_type: TextObjectType, include_delimiters: bool) -> Option<TextObject> {
        // Use the text_object module's find_text_object function
        // Convert from editor::TextObjectType to text_object::TextObjectType
        let text_object_type = match object_type {
            TextObjectType::Word => TextObjectTypeExt::Word,
            TextObjectType::BigWord => TextObjectTypeExt::BigWord,
            TextObjectType::Sentence => TextObjectTypeExt::Sentence,
            TextObjectType::Paragraph => TextObjectTypeExt::Paragraph,
            TextObjectType::SingleQuoteBlock => TextObjectTypeExt::SingleQuoteBlock,
            TextObjectType::DoubleQuoteBlock => TextObjectTypeExt::DoubleQuoteBlock,
            TextObjectType::ParenBlock => TextObjectTypeExt::ParenBlock,
            TextObjectType::BraceBlock => TextObjectTypeExt::BraceBlock,
            TextObjectType::BracketBlock => TextObjectTypeExt::BracketBlock,
            TextObjectType::AngleBlock => TextObjectTypeExt::AngleBlock,
            TextObjectType::TagBlock => TextObjectTypeExt::TagBlock,
            TextObjectType::BacktickBlock => TextObjectTypeExt::BacktickBlock,
            _ => return None, // Handle other cases
        };
        
        let result = crate::text_object::find_text_object(buffer, position, text_object_type, include_delimiters);
        
        match result {
            Ok(Some(text_object_result)) => {
                // Convert from text_object::TextObject to editor::TextObject
                Some(TextObject {
                    object_type,
                    start: text_object_result.start,
                    end: text_object_result.end,
                    include_delimiters,
                })
            },
            _ => None,
        }
    }

    // We don't need the binary feature version of find_text_object_internal anymore
    
    /// Check if a character is a word character (alphanumeric or underscore)
    fn is_word_char(&self, ch: char) -> bool {
        ch.is_alphanumeric() || ch == '_'
    }
    
    /// Add a key mapping
    pub fn add_key_mapping(&mut self, mapping: KeyMapping) {
        self.key_handler.key_map_mut().add_mapping(mapping);
    }
    
    /// Remove a key mapping
    pub fn remove_key_mapping(&mut self, mode: crate::mode::Mode, sequence: &KeySequence) -> bool {
        self.key_handler.key_map_mut().remove_mapping(mode, sequence)
    }
    
    /// Clear all key mappings
    pub fn clear_key_mappings(&mut self) {
        self.key_handler.key_map_mut().clear();
    }
    
    /// Clear key mappings for a specific mode
    pub fn clear_mode_key_mappings(&mut self, mode: crate::mode::Mode) {
        self.key_handler.key_map_mut().clear_mode(mode);
    }
    
    /// Set the timeout for key sequences
    pub fn set_key_timeout(&mut self, timeout: u64) {
        self.key_handler.set_timeout(timeout);
    }
    
    /// Apply configuration settings to the editor
    pub fn apply_config(&mut self) {
        // Apply key mappings from the configuration
        let key_mappings = self.config_manager.get_key_mappings();
        for mapping in key_mappings {
            self.add_key_mapping(mapping);
        }
        
        // TODO: Apply other configuration settings
        // For now, just set some basic settings
        
        // Set key timeout
        // Default to 1000ms (1 second)
        self.set_key_timeout(1000);
    }
    
    /// Get a reference to the configuration manager
    pub fn config_manager(&self) -> &ConfigManager {
        &self.config_manager
    }
    
    /// Get a mutable reference to the configuration manager
    pub fn config_manager_mut(&mut self) -> &mut ConfigManager {
        &mut self.config_manager
    }
    
    /// Reload the configuration
    pub fn reload_config(&mut self) -> Result<(), crate::config::ConfigError> {
        self.config_manager.load()?;
        self.apply_config();
        Ok(())
    }
    
    /// Save the configuration
    pub fn save_config(&self) -> Result<(), crate::config::ConfigError> {
        self.config_manager.save()
    }
    
    /// Load the noxvim plugin
    fn load_noxvim_plugin(&mut self) {
        // Set the terminal UI reference for the plugin manager
        self.plugin_manager.set_terminal_ui(Arc::new(Mutex::new(self.terminal.clone())));
        
        // Set the plugin context references
        if let Ok(mut context) = self.plugin_manager.context().lock() {
            context.set_buffer_manager(Arc::new(Mutex::new(self.buffer_manager.clone())));
            context.set_mode_manager(Arc::new(Mutex::new(self.mode_manager.clone())));
            // We don't set the command registry since it's not cloneable
        }
        
        // Load the noxvim plugin
        let plugin_path = std::path::Path::new("plugins/noxvim.wasm");
        if plugin_path.exists() {
            match self.plugin_manager.load_plugin(plugin_path, "noxvim") {
                Ok(_) => {
                    println!("Loaded noxvim plugin");
                }
                Err(err) => {
                    eprintln!("Failed to load noxvim plugin: {}", err);
                }
            }
        } else {
            eprintln!("noxvim plugin not found at {}", plugin_path.display());
        }
    }
    
    /// Get the content of a register
    pub fn get_register(&self, register_type: RegisterType) -> Option<&RegisterContent> {
        self.register_manager.get_register(register_type)
    }
    
    /// Get the content of a register by character
    pub fn get_register_by_char(&self, c: char) -> Option<&RegisterContent> {
        self.register_manager.get_register_by_char(c)
    }
    
    /// Set the content of a register
    pub fn set_register(&mut self, register_type: RegisterType, content: RegisterContent) {
        self.register_manager.set_register(register_type, content);
    }
    
    /// Set the content of a register by character
    pub fn set_register_by_char(&mut self, c: char, content: RegisterContent) -> bool {
        self.register_manager.set_register_by_char(c, content)
    }
    
    /// Get the default register
    pub fn default_register(&self) -> RegisterType {
        self.register_manager.default_register()
    }
    
    /// Set the default register
    pub fn set_default_register(&mut self, register_type: RegisterType) {
        self.register_manager.set_default_register(register_type);
    }
    
    /// Get the content of the default register
    pub fn get_default_register_content(&self) -> Option<&RegisterContent> {
        self.register_manager.get_default_register_content()
    }
    
    /// Paste text from a register at the cursor position
    pub fn paste_from_register(&mut self, register_type: RegisterType) -> EditorResult<bool> {
        if let Some(buffer_id) = self.current_buffer_id() {
            let buffer = self.buffer_manager.get_buffer_mut(buffer_id)?;
            
            // Get cursor position
            let cursor_pos = self.cursor_manager.position();
            
            // Convert cursor position to character index
            let cursor_position = buffer.position_to_char_idx(cursor_pos.line, cursor_pos.column)?;
            
            // Get the register content
            if let Some(content) = self.register_manager.get_register(register_type) {
                let text = content.as_string();
                
                // Insert the text at the cursor position
                match buffer.insert(cursor_position, &text) {
                    Ok(_) => {
                        // Update cursor position
                        let new_position = buffer.char_idx_to_position(cursor_position + text.len())?;
                        self.cursor_manager.set_position(new_position);
                        Ok(true)
                    },
                    Err(err) => Err(EditorError::Buffer(err.into())),
                }
            } else {
                // No content in the register
                Ok(false)
            }
        } else {
            Ok(false)
        }
    }
    
    /// Paste text from the unnamed register at the cursor position
    pub fn paste(&mut self) -> EditorResult<bool> {
        self.paste_from_register(RegisterType::Unnamed)
    }
    
    /// Paste text from a register specified by character at the cursor position
    pub fn paste_from_register_char(&mut self, c: char) -> EditorResult<bool> {
        if let Some(register_type) = RegisterType::from_char(c) {
            self.paste_from_register(register_type)
        } else {
            Ok(false)
        }
    }
    
    /// Handle inner text object (i + key)
    fn handle_inner_text_object(&mut self, _key: KeyEvent) -> EditorResult<()> {
        use crossterm::event::{KeyCode, KeyModifiers};
        
        // Get the next key
        if let Some(key) = self.terminal.poll_key(1000)? {
            match key.code {
                KeyCode::Char('w') => {
                    // Inner word
                    let target = OperatorTarget::TextObject(TextObjectTypeExt::Word, false);
                    if let Some(completed_state) = self.operator_manager.set_target(target) {
                        return self.execute_operator(completed_state);
                    }
                },
                KeyCode::Char('W') => {
                    // Inner WORD
                    let target = OperatorTarget::TextObject(TextObjectTypeExt::BigWord, false);
                    if let Some(completed_state) = self.operator_manager.set_target(target) {
                        return self.execute_operator(completed_state);
                    }
                },
                KeyCode::Char('s') => {
                    // Inner sentence
                    let target = OperatorTarget::TextObject(TextObjectTypeExt::Sentence, false);
                    if let Some(completed_state) = self.operator_manager.set_target(target) {
                        return self.execute_operator(completed_state);
                    }
                },
                KeyCode::Char('p') => {
                    // Inner paragraph
                    let target = OperatorTarget::TextObject(TextObjectTypeExt::Paragraph, false);
                    if let Some(completed_state) = self.operator_manager.set_target(target) {
                        return self.execute_operator(completed_state);
                    }
                },
                KeyCode::Char('(') | KeyCode::Char(')') => {
                    // Inner parenthesis block
                    let target = OperatorTarget::TextObject(TextObjectTypeExt::ParenBlock, false);
                    if let Some(completed_state) = self.operator_manager.set_target(target) {
                        return self.execute_operator(completed_state);
                    }
                },
                KeyCode::Char('{') | KeyCode::Char('}') => {
                    // Inner brace block
                    let target = OperatorTarget::TextObject(TextObjectTypeExt::BraceBlock, false);
                    if let Some(completed_state) = self.operator_manager.set_target(target) {
                        return self.execute_operator(completed_state);
                    }
                },
                KeyCode::Char('[') | KeyCode::Char(']') => {
                    // Inner bracket block
                    let target = OperatorTarget::TextObject(TextObjectTypeExt::BracketBlock, false);
                    if let Some(completed_state) = self.operator_manager.set_target(target) {
                        return self.execute_operator(completed_state);
                    }
                },
                KeyCode::Char('<') | KeyCode::Char('>') => {
                    // Inner angle block
                    let target = OperatorTarget::TextObject(TextObjectTypeExt::AngleBlock, false);
                    if let Some(completed_state) = self.operator_manager.set_target(target) {
                        return self.execute_operator(completed_state);
                    }
                },
                KeyCode::Char('\'') => {
                    // Inner single quote block
                    let target = OperatorTarget::TextObject(TextObjectTypeExt::SingleQuoteBlock, false);
                    if let Some(completed_state) = self.operator_manager.set_target(target) {
                        return self.execute_operator(completed_state);
                    }
                },
                KeyCode::Char('"') => {
                    // Inner double quote block
                    let target = OperatorTarget::TextObject(TextObjectTypeExt::DoubleQuoteBlock, false);
                    if let Some(completed_state) = self.operator_manager.set_target(target) {
                        return self.execute_operator(completed_state);
                    }
                },
                KeyCode::Char('`') => {
                    // Inner backtick block
                    let target = OperatorTarget::TextObject(TextObjectTypeExt::BacktickBlock, false);
                    if let Some(completed_state) = self.operator_manager.set_target(target) {
                        return self.execute_operator(completed_state);
                    }
                },
                KeyCode::Char('t') => {
                    // Inner tag block
                    let target = OperatorTarget::TextObject(TextObjectTypeExt::TagBlock, false);
                    if let Some(completed_state) = self.operator_manager.set_target(target) {
                        return self.execute_operator(completed_state);
                    }
                },
                KeyCode::Esc => {
                    // Cancel the operator
                    self.operator_manager.cancel();
                    self.mode_manager.enter_normal_mode();
                },
                _ => {
                    // Unhandled key, cancel the operator
                    self.operator_manager.cancel();
                    self.mode_manager.enter_normal_mode();
                }
            }
        } else {
            // Timeout, cancel the operator
            self.operator_manager.cancel();
            self.mode_manager.enter_normal_mode();
        }
        
        Ok(())
    }
    
    /// Handle around text object (a + key)
    fn handle_around_text_object(&mut self, _key: KeyEvent) -> EditorResult<()> {
        use crossterm::event::{KeyCode, KeyModifiers};
        
        // Get the next key
        if let Some(key) = self.terminal.poll_key(1000)? {
            match key.code {
                KeyCode::Char('w') => {
                    // Around word
                    let target = OperatorTarget::TextObject(TextObjectTypeExt::Word, true);
                    if let Some(completed_state) = self.operator_manager.set_target(target) {
                        return self.execute_operator(completed_state);
                    }
                },
                KeyCode::Char('W') => {
                    // Around WORD
                    let target = OperatorTarget::TextObject(TextObjectTypeExt::BigWord, true);
                    if let Some(completed_state) = self.operator_manager.set_target(target) {
                        return self.execute_operator(completed_state);
                    }
                },
                KeyCode::Char('s') => {
                    // Around sentence
                    let target = OperatorTarget::TextObject(TextObjectTypeExt::Sentence, true);
                    if let Some(completed_state) = self.operator_manager.set_target(target) {
                        return self.execute_operator(completed_state);
                    }
                },
                KeyCode::Char('p') => {
                    // Around paragraph
                    let target = OperatorTarget::TextObject(TextObjectTypeExt::Paragraph, true);
                    if let Some(completed_state) = self.operator_manager.set_target(target) {
                        return self.execute_operator(completed_state);
                    }
                },
                KeyCode::Char('(') | KeyCode::Char(')') => {
                    // Around parenthesis block
                    let target = OperatorTarget::TextObject(TextObjectTypeExt::ParenBlock, true);
                    if let Some(completed_state) = self.operator_manager.set_target(target) {
                        return self.execute_operator(completed_state);
                    }
                },
                KeyCode::Char('{') | KeyCode::Char('}') => {
                    // Around brace block
                    let target = OperatorTarget::TextObject(TextObjectTypeExt::BraceBlock, true);
                    if let Some(completed_state) = self.operator_manager.set_target(target) {
                        return self.execute_operator(completed_state);
                    }
                },
                KeyCode::Char('[') | KeyCode::Char(']') => {
                    // Around bracket block
                    let target = OperatorTarget::TextObject(TextObjectTypeExt::BracketBlock, true);
                    if let Some(completed_state) = self.operator_manager.set_target(target) {
                        return self.execute_operator(completed_state);
                    }
                },
                KeyCode::Char('<') | KeyCode::Char('>') => {
                    // Around angle block
                    let target = OperatorTarget::TextObject(TextObjectTypeExt::AngleBlock, true);
                    if let Some(completed_state) = self.operator_manager.set_target(target) {
                        return self.execute_operator(completed_state);
                    }
                },
                KeyCode::Char('\'') => {
                    // Around single quote block
                    let target = OperatorTarget::TextObject(TextObjectTypeExt::SingleQuoteBlock, true);
                    if let Some(completed_state) = self.operator_manager.set_target(target) {
                        return self.execute_operator(completed_state);
                    }
                },
                KeyCode::Char('"') => {
                    // Around double quote block
                    let target = OperatorTarget::TextObject(TextObjectTypeExt::DoubleQuoteBlock, true);
                    if let Some(completed_state) = self.operator_manager.set_target(target) {
                        return self.execute_operator(completed_state);
                    }
                },
                KeyCode::Char('`') => {
                    // Around backtick block
                    let target = OperatorTarget::TextObject(TextObjectTypeExt::BacktickBlock, true);
                    if let Some(completed_state) = self.operator_manager.set_target(target) {
                        return self.execute_operator(completed_state);
                    }
                },
                KeyCode::Char('t') => {
                    // Around tag block
                    let target = OperatorTarget::TextObject(TextObjectTypeExt::TagBlock, true);
                    if let Some(completed_state) = self.operator_manager.set_target(target) {
                        return self.execute_operator(completed_state);
                    }
                },
                KeyCode::Esc => {
                    // Cancel the operator
                    self.operator_manager.cancel();
                    self.mode_manager.enter_normal_mode();
                },
                _ => {
                    // Unhandled key, cancel the operator
                    self.operator_manager.cancel();
                    self.mode_manager.enter_normal_mode();
                }
            }
        } else {
            // Timeout, cancel the operator
            self.operator_manager.cancel();
            self.mode_manager.enter_normal_mode();
        }
        
        Ok(())
    }
    
    /// Delete text from the cursor position to the result of a motion
    pub fn delete_to_motion(&mut self, direction: Direction) -> EditorResult<bool> {
        if let Some(buffer_id) = self.current_buffer_id() {
            let buffer = self.buffer_manager.get_buffer(buffer_id)?;
            
            // Get cursor position
            let start_pos = self.cursor_manager.position();
            
            // Move cursor according to the motion
            self.cursor_manager.move_cursor(direction, buffer)?;
            
            // Get new cursor position
            let end_pos = self.cursor_manager.position();
            
            // Convert positions to character indices
            let start_idx = buffer.position_to_char_idx(start_pos.line, start_pos.column)?;
            let end_idx = buffer.position_to_char_idx(end_pos.line, end_pos.column)?;
            
            // Determine the range to delete
            let (start, end) = if start_idx <= end_idx {
                (start_idx, end_idx)
            } else {
                (end_idx, start_idx)
            };
            
            // Get the text from the buffer before deleting it
            let content = buffer.content();
            if start < content.len() && end <= content.len() {
                let text = content[start..end].to_string();
                
                // Store the text in the unnamed register
                let register_content = RegisterContent::character_wise(&text);
                self.register_manager.set_register(RegisterType::Unnamed, register_content);
            }
            
            let buffer = self.buffer_manager.get_buffer_mut(buffer_id)?;
            // Delete the text
            match buffer.delete(start, end) {
                Ok(_) => Ok(true),
                Err(err) => Err(EditorError::Buffer(err.into())),
            }
        } else {
            Ok(false)
        }
    }
    
    /// Change text from the cursor position to the result of a motion
    pub fn change_to_motion(&mut self, direction: Direction) -> EditorResult<bool> {
        if let Some(buffer_id) = self.current_buffer_id() {
            let buffer = self.buffer_manager.get_buffer(buffer_id)?;
            
            // Get cursor position
            let start_pos = self.cursor_manager.position();
            
            // Move cursor according to the motion
            self.cursor_manager.move_cursor(direction, buffer)?;
            
            // Get new cursor position
            let end_pos = self.cursor_manager.position();
            
            // Convert positions to character indices
            let start_idx = buffer.position_to_char_idx(start_pos.line, start_pos.column)?;
            let end_idx = buffer.position_to_char_idx(end_pos.line, end_pos.column)?;
            
            // Determine the range to delete
            let (start, end) = if start_idx <= end_idx {
                (start_idx, end_idx)
            } else {
                (end_idx, start_idx)
            };
            
            // Get the text from the buffer before deleting it
            let content = buffer.content();
            if start < content.len() && end <= content.len() {
                let text = content[start..end].to_string();
                
                // Store the text in the unnamed register
                let register_content = RegisterContent::character_wise(&text);
                self.register_manager.set_register(RegisterType::Unnamed, register_content);
            }
            
            let buffer = self.buffer_manager.get_buffer_mut(buffer_id)?;
            // Delete the text
            match buffer.delete(start, end) {
                Ok(_) => {
                    // Enter insert mode
                    self.mode_manager.enter_insert_mode();
                    Ok(true)
                },
                Err(err) => Err(EditorError::Buffer(err.into())),
            }
        } else {
            Ok(false)
        }
    }
    
    /// Yank text from the cursor position to the result of a motion
    pub fn yank_to_motion(&mut self, direction: Direction) -> EditorResult<Option<String>> {
        if let Some(buffer_id) = self.current_buffer_id() {
            let buffer = self.buffer_manager.get_buffer(buffer_id)?;
            
            // Get cursor position
            let start_pos = self.cursor_manager.position();
            
            // Move cursor according to the motion
            self.cursor_manager.move_cursor(direction, buffer)?;
            
            // Get new cursor position
            let end_pos = self.cursor_manager.position();
            
            // Convert positions to character indices
            let start_idx = buffer.position_to_char_idx(start_pos.line, start_pos.column)?;
            let end_idx = buffer.position_to_char_idx(end_pos.line, end_pos.column)?;
            
            // Determine the range to yank
            let (start, end) = if start_idx <= end_idx {
                (start_idx, end_idx)
            } else {
                (end_idx, start_idx)
            };
            
            // Get the text from the buffer
            let content = buffer.content();
            if start < content.len() && end <= content.len() {
                let text = content[start..end].to_string();
                
                // Store the text in the unnamed register
                let register_content = RegisterContent::character_wise(&text);
                self.register_manager.set_register(RegisterType::Unnamed, register_content);
                
                // Return cursor to original position
                self.cursor_manager.set_position(start_pos);
                
                Ok(Some(text))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }
    
    /// Delete lines from start to end (inclusive)
    pub fn delete_lines(&mut self, start: usize, end: usize) -> EditorResult<bool> {
        if let Some(buffer_id) = self.current_buffer_id() {
            let buffer = self.buffer_manager.get_buffer(buffer_id)?;
            
            // Get the start and end positions
            let start_idx = buffer.position_to_char_idx(start, 0)?;
            let end_idx = if end + 1 < buffer.line_count() {
                buffer.position_to_char_idx(end + 1, 0)?
            } else {
                buffer.content().len()
            };
            
            // Get the text from the buffer before deleting it
            let content = buffer.content();
            if start_idx < content.len() && end_idx <= content.len() {
                let text = content[start_idx..end_idx].to_string();
                
                // Store the text in the unnamed register
                let lines: Vec<&str> = text.lines().collect();
                let register_content = RegisterContent::line_wise(&lines);
                self.register_manager.set_register(RegisterType::Unnamed, register_content);
            }
            
            let buffer = self.buffer_manager.get_buffer_mut(buffer_id)?;
            // Delete the lines
            match buffer.delete(start_idx, end_idx) {
                Ok(_) => {
                    // Move cursor to the start of the next line
                    let new_pos = crate::cursor::CursorPosition::new(start, 0);
                    self.cursor_manager.set_position(new_pos);
                    Ok(true)
                },
                Err(err) => Err(EditorError::Buffer(err.into())),
            }
        } else {
            Ok(false)
        }
    }
    
    /// Change lines from start to end (inclusive)
    pub fn change_lines(&mut self, start: usize, end: usize) -> EditorResult<bool> {
        if let Some(buffer_id) = self.current_buffer_id() {
            let buffer = self.buffer_manager.get_buffer(buffer_id)?;
            
            // Get the start and end positions
            let start_idx = buffer.position_to_char_idx(start, 0)?;
            let end_idx = if end + 1 < buffer.line_count() {
                buffer.position_to_char_idx(end + 1, 0)?
            } else {
                buffer.content().len()
            };
            
            // Get the text from the buffer before deleting it
            let content = buffer.content();
            if start_idx < content.len() && end_idx <= content.len() {
                let text = content[start_idx..end_idx].to_string();
                
                // Store the text in the unnamed register
                let lines: Vec<&str> = text.lines().collect();
                let register_content = RegisterContent::line_wise(&lines);
                self.register_manager.set_register(RegisterType::Unnamed, register_content);
            }
            
            let buffer = self.buffer_manager.get_buffer_mut(buffer_id)?;
            // Delete the lines
            match buffer.delete(start_idx, end_idx) {
                Ok(_) => {
                    // Move cursor to the start of the line
                    let new_pos = crate::cursor::CursorPosition::new(start, 0);
                    self.cursor_manager.set_position(new_pos);
                    
                    // Enter insert mode
                    self.mode_manager.enter_insert_mode();
                    Ok(true)
                },
                Err(err) => Err(EditorError::Buffer(err.into())),
            }
        } else {
            Ok(false)
        }
    }
    
    /// Yank lines from start to end (inclusive)
    pub fn yank_lines(&mut self, start: usize, end: usize) -> EditorResult<Option<String>> {
        if let Some(buffer_id) = self.current_buffer_id() {
            let buffer = self.buffer_manager.get_buffer(buffer_id)?;
            
            // Get the start and end positions
            let start_idx = buffer.position_to_char_idx(start, 0)?;
            let end_idx = if end + 1 < buffer.line_count() {
                buffer.position_to_char_idx(end + 1, 0)?
            } else {
                buffer.content().len()
            };
            
            // Get the text from the buffer
            let content = buffer.content();
            if start_idx < content.len() && end_idx <= content.len() {
                let text = content[start_idx..end_idx].to_string();
                
                // Store the text in the unnamed register
                let lines: Vec<&str> = text.lines().collect();
                let register_content = RegisterContent::line_wise(&lines);
                self.register_manager.set_register(RegisterType::Unnamed, register_content);
                
                Ok(Some(text))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }
    
    /// Start recording a macro to a register
    pub fn start_macro_recording(&mut self, register: char) -> bool {
        self.macro_recorder.start_recording(register)
    }
    
    /// Stop recording a macro
    pub fn stop_macro_recording(&mut self) -> bool {
        self.macro_recorder.stop_recording()
    }
    
    /// Get the current macro recording state
    pub fn macro_recording_state(&self) -> MacroRecorderState {
        self.macro_recorder.state()
    }
    
    /// Play back a macro from a register
    pub fn play_macro(&mut self, register: char) -> bool {
        self.macro_player.start_playback(register)
    }
    
    /// Check if a macro is currently being played
    pub fn is_playing_macro(&self) -> bool {
        self.macro_player.is_playing()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_editor_creation() {
        let editor = Editor::new();
        assert!(editor.is_ok());
    }
}