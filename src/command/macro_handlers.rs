//! Macro command handlers
//!
//! This module implements handlers for macro commands.

use crate::command::{ExCommand, ExCommandError, ExCommandResult, ExCommandRegistry};
use std::sync::Mutex;
use std::sync::Arc;
use std::sync::Mutex;
use crate::editor::Editor;
use std::sync::Mutex;
use std::sync::Arc;
use std::sync::Mutex;
use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::{Arc, Mutex};
use std::sync::Mutex;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::SystemTime;
use std::sync::Mutex;
use std::sync::Arc;
use std::sync::Mutex;

/// Macro recording state
#[derive(Debug, Clone)]
pub struct MacroRecording {
    /// Register name
    pub register: char,
    /// Recorded keystrokes
    pub keystrokes: Vec<String>,
    /// Start time
    pub start_time: SystemTime,
}

/// Macro manager
#[derive(Debug)]
pub struct MacroManager {
    /// Macros by register name
    pub macros: HashMap<char, Vec<String>>,
    /// Current recording
    pub recording: Option<MacroRecording>,
    /// Currently executing macro
    pub executing: Option<char>,
    /// Execution depth (to prevent infinite recursion)
    pub execution_depth: usize,
    /// Maximum execution depth
    pub max_execution_depth: usize,
}

impl MacroManager {
    /// Create a new macro manager
    pub fn new() -> Self {
        Self {
            macros: HashMap::new(),
            recording: None,
            executing: None,
            execution_depth: 0,
            max_execution_depth: 100,
        }
    }

    /// Start recording a macro
    pub fn start_recording(&mut self, register: char) -> ExCommandResult<()> {
        // Check if we're already recording
        if self.recording.is_some() {
            return Err(ExCommandError::Other("Already recording a macro".to_string()));
        }

        // Start recording
        self.recording = Some(MacroRecording {
            register,
            keystrokes: Vec::new(),
            start_time: SystemTime::now(),
        });

        Ok(())
    }

    /// Stop recording a macro
    pub fn stop_recording(&mut self) -> ExCommandResult<()> {
        // Check if we're recording
        let recording = match self.recording.take() {
            Some(recording) => recording,
            None => return Err(ExCommandError::Other("Not recording a macro".to_string())),
        };

        // Store the macro
        self.macros.insert(recording.register, recording.keystrokes);

        Ok(())
    }

    /// Record a keystroke
    pub fn record_keystroke(&mut self, keystroke: &str) -> ExCommandResult<()> {
        // Check if we're recording
        if let Some(recording) = &mut self.recording {
            // Record the keystroke
            recording.keystrokes.push(keystroke.to_string());
        }

        Ok(())
    }

    /// Play a macro
    pub fn play_macro(&mut self, register: char, editor: &mut Editor) -> ExCommandResult<()> {
        // Check if we're already executing a macro
        if self.executing.is_some() {
            // Increment the execution depth
            self.execution_depth += 1;

            // Check if we've exceeded the maximum execution depth
            if self.execution_depth > self.max_execution_depth {
                self.executing = None;
                self.execution_depth = 0;
                return Err(ExCommandError::Other("Maximum macro execution depth exceeded".to_string()));
            }
        } else {
            // Start executing
            self.executing = Some(register);
            self.execution_depth = 1;
        }

        // Get the macro
        let keystrokes = match self.macros.get(&register) {
            Some(keystrokes) => keystrokes.clone(),
            None => return Err(ExCommandError::Other(format!("No macro recorded in register {}", register))),
        };

        // Execute the keystrokes
        for keystroke in keystrokes {
            // Execute the keystroke
            if let Err(err) = editor.execute_keystroke(&keystroke) {
                // Stop executing
                self.executing = None;
                self.execution_depth = 0;

                return Err(ExCommandError::Other(format!("Failed to execute keystroke: {}", err)));
            }
        }

        // Decrement the execution depth
        if self.execution_depth > 1 {
            self.execution_depth -= 1;
        } else {
            // Stop executing
            self.executing = None;
            self.execution_depth = 0;
        }

        Ok(())
    }

    /// Get the macro in a register
    pub fn get_macro(&self, register: char) -> Option<&Vec<String>> {
        self.macros.get(&register)
    }

    /// Set a macro in a register
    pub fn set_macro(&mut self, register: char, keystrokes: Vec<String>) {
        self.macros.insert(register, keystrokes);
    }

    /// Clear a macro in a register
    pub fn clear_macro(&mut self, register: char) -> bool {
        self.macros.remove(&register).is_some()
    }

    /// Clear all macros
    pub fn clear_all_macros(&mut self) {
        self.macros.clear();
    }

    /// Get the current recording register
    pub fn get_recording_register(&self) -> Option<char> {
        self.recording.as_ref().map(|r| r.register)
    }

    /// Get the current executing register
    pub fn get_executing_register(&self) -> Option<char> {
        self.executing
    }

    /// Check if we're recording a macro
    pub fn is_recording(&self) -> bool {
        self.recording.is_some()
    }

    /// Check if we're executing a macro
    pub fn is_executing(&self) -> bool {
        self.executing.is_some()
    }
}

// Global macro manager
static mut MACRO_MANAGER: Option<MacroManager> = None;

/// Initialize the macro manager
pub fn init_macro_manager() {
    unsafe {
        if MACRO_MANAGER.is_none() {
            MACRO_MANAGER = Some(MacroManager::new());
        }
    }
}

/// Register macro command handlers
pub fn register_macro_handlers(registry: &mut ExCommandRegistry) {
    // Initialize the macro manager
    init_macro_manager();
    
    // Register macro commands
    registry.register("record", handle_record);
    registry.register("stoprecord", handle_stoprecord);
    registry.register("playback", handle_playback);
    registry.register("let", handle_let);
    registry.register("registers", handle_registers);
    registry.register("display", handle_display);
    registry.register("normal", handle_normal);
}

/// Handle the :record command
fn handle_record(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the macro manager
    let macro_manager = unsafe {
        match &mut MACRO_MANAGER {
            Some(manager) => manager,
            None => {
                init_macro_manager();
                match &mut MACRO_MANAGER {
                    Some(manager) => manager,
                    None => return Err(ExCommandError::Other("Failed to initialize macro manager".to_string())),
                }
            }
        }
    };
    
    // Get the register from the command arguments
    let register = match cmd.first_arg() {
        Some(reg) => {
            if reg.len() != 1 {
                return Err(ExCommandError::InvalidArgument(format!("Invalid register name: {}", reg)));
            }
            
            reg.chars().next().unwrap()
        },
        None => return Err(ExCommandError::MissingArgument("Register name required".to_string())),
    };
    
    // Start recording
    macro_manager.start_recording(register)?;
    
    println!("Recording into register {}", register);
    Ok(())
}

/// Handle the :stoprecord command
fn handle_stoprecord(_cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the macro manager
    let macro_manager = unsafe {
        match &mut MACRO_MANAGER {
            Some(manager) => manager,
            None => {
                init_macro_manager();
                match &mut MACRO_MANAGER {
                    Some(manager) => manager,
                    None => return Err(ExCommandError::Other("Failed to initialize macro manager".to_string())),
                }
            }
        }
    };
    
    // Get the current recording register
    let register = match macro_manager.get_recording_register() {
        Some(reg) => reg,
        None => return Err(ExCommandError::Other("Not recording a macro".to_string())),
    };
    
    // Stop recording
    macro_manager.stop_recording()?;
    
    println!("Finished recording into register {}", register);
    Ok(())
}

/// Handle the :playback command
fn handle_playback(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match crate::command::handlers::EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Get the macro manager
    let macro_manager = unsafe {
        match &mut MACRO_MANAGER {
            Some(manager) => manager,
            None => {
                init_macro_manager();
                match &mut MACRO_MANAGER {
                    Some(manager) => manager,
                    None => return Err(ExCommandError::Other("Failed to initialize macro manager".to_string())),
                }
            }
        }
    };
    
    // Get the register from the command arguments
    let register = match cmd.first_arg() {
        Some(reg) => {
            if reg.len() != 1 {
                return Err(ExCommandError::InvalidArgument(format!("Invalid register name: {}", reg)));
            }
            
            reg.chars().next().unwrap()
        },
        None => return Err(ExCommandError::MissingArgument("Register name required".to_string())),
    };
    
    // Get the count from the command arguments
    let count = if let Some(count_str) = cmd.args.get(1) {
        match count_str.parse::<usize>() {
            Ok(count) => count,
            Err(_) => return Err(ExCommandError::InvalidArgument(format!("Invalid count: {}", count_str))),
        }
    } else {
        1
    };
    
    // Play the macro
    for _ in 0..count {
        macro_manager.play_macro(register, editor)?;
    }
    
    println!("Played back register {} {} time(s)", register, count);
    Ok(())
}

/// Handle the :let command for registers
fn handle_let(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the macro manager
    let macro_manager = unsafe {
        match &mut MACRO_MANAGER {
            Some(manager) => manager,
            None => {
                init_macro_manager();
                match &mut MACRO_MANAGER {
                    Some(manager) => manager,
                    None => return Err(ExCommandError::Other("Failed to initialize macro manager".to_string())),
                }
            }
        }
    };
    
    // Parse the let command
    // Format: :let @{register} = "{keystrokes}"
    let args = cmd.args_str();
    
    if !args.starts_with('@') {
        // Not a register assignment, delegate to the variable handler
        return crate::command::additional_handlers::handle_let(cmd);
    }
    
    // Parse the register name
    if args.len() < 2 {
        return Err(ExCommandError::InvalidArgument("Invalid register name".to_string()));
    }
    
    let register = args.chars().nth(1).unwrap();
    
    // Find the equals sign
    let equals_pos = match args.find('=') {
        Some(pos) => pos,
        None => return Err(ExCommandError::InvalidArgument("Missing equals sign".to_string())),
    };
    
    // Get the keystrokes
    let keystrokes_str = args[equals_pos + 1..].trim();
    
    // Remove quotes if present
    let keystrokes_str = if keystrokes_str.starts_with('"') && keystrokes_str.ends_with('"') {
        &keystrokes_str[1..keystrokes_str.len() - 1]
    } else {
        keystrokes_str
    };
    
    // Parse the keystrokes
    let keystrokes: Vec<String> = keystrokes_str.split(' ').map(|s| s.to_string()).collect();
    
    // Set the macro
    macro_manager.set_macro(register, keystrokes);
    
    println!("Register @{} set", register);
    Ok(())
}

/// Handle the :registers command
fn handle_registers(_cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the macro manager
    let macro_manager = unsafe {
        match &MACRO_MANAGER {
            Some(manager) => manager,
            None => {
                init_macro_manager();
                match &MACRO_MANAGER {
                    Some(manager) => manager,
                    None => return Err(ExCommandError::Other("Failed to initialize macro manager".to_string())),
                }
            }
        }
    };
    
    // Display the registers
    println!("--- Registers ---");
    
    for (register, keystrokes) in &macro_manager.macros {
        println!("\"{}   {}", register, keystrokes.join(" "));
    }
    
    Ok(())
}

/// Handle the :display command
fn handle_display(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the macro manager
    let macro_manager = unsafe {
        match &MACRO_MANAGER {
            Some(manager) => manager,
            None => {
                init_macro_manager();
                match &MACRO_MANAGER {
                    Some(manager) => manager,
                    None => return Err(ExCommandError::Other("Failed to initialize macro manager".to_string())),
                }
            }
        }
    };
    
    // Get the register from the command arguments
    let register = match cmd.first_arg() {
        Some(reg) => {
            if reg.len() != 1 {
                return Err(ExCommandError::InvalidArgument(format!("Invalid register name: {}", reg)));
            }
            
            reg.chars().next().unwrap()
        },
        None => return Err(ExCommandError::MissingArgument("Register name required".to_string())),
    };
    
    // Get the macro
    let keystrokes = match macro_manager.get_macro(register) {
        Some(keystrokes) => keystrokes,
        None => return Err(ExCommandError::Other(format!("No macro recorded in register {}", register))),
    };
    
    // Display the macro
    println!("--- Register {} ---", register);
    println!("{}", keystrokes.join(" "));
    
    Ok(())
}

/// Handle the :normal command
fn handle_normal(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match crate::command::handlers::EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Get the keystrokes from the command arguments
    let keystrokes = cmd.args_str();
    
    if keystrokes.is_empty() {
        return Err(ExCommandError::MissingArgument("Keystrokes required".to_string()));
    }
    
    // Execute the keystrokes
    for keystroke in keystrokes.split_whitespace() {
        if let Err(err) = editor.execute_keystroke(keystroke) {
            return Err(ExCommandError::Other(format!("Failed to execute keystroke: {}", err)));
        }
    }
    
    Ok(())
}

/// Record a keystroke
pub fn record_keystroke(keystroke: &str) -> ExCommandResult<()> {
    // Get the macro manager
    let macro_manager = unsafe {
        match &mut MACRO_MANAGER {
            Some(manager) => manager,
            None => {
                init_macro_manager();
                match &mut MACRO_MANAGER {
                    Some(manager) => manager,
                    None => return Err(ExCommandError::Other("Failed to initialize macro manager".to_string())),
                }
            }
        }
    };
    
    // Record the keystroke
    macro_manager.record_keystroke(keystroke)
}

/// Check if we're recording a macro
pub fn is_recording() -> bool {
    unsafe {
        match &MACRO_MANAGER {
            Some(manager) => manager.is_recording(),
            None => false,
        }
    }
}

/// Check if we're executing a macro
pub fn is_executing() -> bool {
    unsafe {
        match &MACRO_MANAGER {
            Some(manager) => manager.is_executing(),
            None => false,
        }
    }
}

/// Get the current recording register
pub fn get_recording_register() -> Option<char> {
    unsafe {
        match &MACRO_MANAGER {
            Some(manager) => manager.get_recording_register(),
            None => None,
        }
    }
}

/// Get the current executing register
pub fn get_executing_register() -> Option<char> {
    unsafe {
        match &MACRO_MANAGER {
            Some(manager) => manager.get_executing_register(),
            None => None,
        }
    }
}