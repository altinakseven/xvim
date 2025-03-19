//! Quickfix and location list command handlers
//!
//! This module implements handlers for quickfix and location list commands.

use crate::command::{ExCommand, ExCommandError, ExCommandResult, ExCommandRegistry};
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::path::Path;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::sync::Mutex;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::path::Path;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::sync::Arc;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::path::Path;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::sync::Mutex;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::path::Path;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::collections::HashMap;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::path::Path;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::sync::Mutex;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::path::Path;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::sync::Arc;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::path::Path;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::sync::Mutex;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::path::Path;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::collections::{HashMap, VecDeque};
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::path::Path;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::sync::Mutex;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::path::Path;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::sync::Arc;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::path::Path;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::sync::Mutex;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::path::Path;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::collections::HashMap;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::path::Path;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::sync::Mutex;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::path::Path;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::sync::Arc;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::path::Path;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::sync::Mutex;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::path::Path;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::path::Path;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::sync::Mutex;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::path::Path;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::sync::Arc;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::path::Path;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::sync::Mutex;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::path::Path;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::collections::HashMap;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::path::Path;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::sync::Mutex;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::path::Path;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::sync::Arc;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::path::Path;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::sync::Mutex;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::path::Path;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::io::{self, BufRead, Read};
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::path::Path;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::sync::Mutex;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::path::Path;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::sync::Arc;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::path::Path;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::sync::Mutex;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::path::Path;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::collections::HashMap;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::path::Path;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::sync::Mutex;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::path::Path;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::sync::Arc;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::path::Path;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::sync::Mutex;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::path::Path;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::path::Path;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::sync::Mutex;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::path::Path;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::sync::Arc;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::path::Path;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::sync::Mutex;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::path::Path;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::collections::HashMap;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::path::Path;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::sync::Mutex;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::path::Path;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::sync::Arc;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::path::Path;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::sync::Mutex;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::path::Path;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::process::{Command, Stdio};
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::path::Path;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::sync::Mutex;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::path::Path;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::sync::Arc;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::path::Path;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::sync::Mutex;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::path::Path;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::collections::HashMap;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::path::Path;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::sync::Mutex;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::path::Path;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::sync::Arc;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::path::Path;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::sync::Mutex;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::path::Path;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::sync::{Arc, Mutex};
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::path::Path;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::sync::Mutex;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::path::Path;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::sync::Arc;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::path::Path;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::sync::Mutex;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::path::Path;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::collections::HashMap;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::path::Path;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::sync::Mutex;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::path::Path;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::sync::Arc;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::path::Path;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::sync::Mutex;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;
use std::path::Path;
use std::io::Read;
use std::path::PathBuf;
use std::io::Read;

/// Quickfix entry
#[derive(Debug, Clone)]
pub struct QuickfixEntry {
    /// File path
    pub file: PathBuf,
    /// Line number
    pub line: usize,
    /// Column number
    pub col: Option<usize>,
    /// Error message
    pub text: String,
    /// Error type (error, warning, info)
    pub error_type: QuickfixErrorType,
}

/// Quickfix error type
#[derive(Debug, Clone, PartialEq)]
pub enum QuickfixErrorType {
    /// Error
    Error,
    /// Warning
    Warning,
    /// Information
    Info,
    /// Note
    Note,
}

/// Quickfix list
#[derive(Debug, Clone)]
pub struct QuickfixList {
    /// Entries in the quickfix list
    pub entries: Vec<QuickfixEntry>,
    /// Current position in the quickfix list
    pub current: usize,
    /// Title of the quickfix list
    pub title: String,
}

impl QuickfixList {
    /// Create a new quickfix list
    pub fn new(title: &str) -> Self {
        Self {
            entries: Vec::new(),
            current: 0,
            title: title.to_string(),
        }
    }

    /// Add an entry to the quickfix list
    pub fn add_entry(&mut self, entry: QuickfixEntry) {
        self.entries.push(entry);
    }

    /// Get the current entry
    pub fn current_entry(&self) -> Option<&QuickfixEntry> {
        if self.entries.is_empty() {
            None
        } else {
            Some(&self.entries[self.current])
        }
    }

    /// Move to the next entry
    pub fn next(&mut self) -> bool {
        if self.entries.is_empty() {
            return false;
        }

        if self.current + 1 < self.entries.len() {
            self.current += 1;
            true
        } else {
            false
        }
    }

    /// Move to the previous entry
    pub fn prev(&mut self) -> bool {
        if self.entries.is_empty() {
            return false;
        }

        if self.current > 0 {
            self.current -= 1;
            true
        } else {
            false
        }
    }

    /// Check if the quickfix list is empty
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Get the number of entries in the quickfix list
    pub fn len(&self) -> usize {
        self.entries.len()
    }
}

/// Quickfix manager
#[derive(Debug)]
pub struct QuickfixManager {
    /// Quickfix lists
    pub lists: VecDeque<QuickfixList>,
    /// Current quickfix list index
    pub current: usize,
    /// Location lists (one per window)
    pub location_lists: HashMap<usize, VecDeque<QuickfixList>>,
    /// Current location list index (per window)
    pub current_location: HashMap<usize, usize>,
    /// Quickfix window ID
    pub quickfix_window: Option<usize>,
    /// Location list window ID (per window)
    pub location_windows: HashMap<usize, usize>,
}

impl QuickfixManager {
    /// Create a new quickfix manager
    pub fn new() -> Self {
        Self {
            lists: VecDeque::new(),
            current: 0,
            location_lists: HashMap::new(),
            current_location: HashMap::new(),
            quickfix_window: None,
            location_windows: HashMap::new(),
        }
    }

    /// Add a quickfix list
    pub fn add_list(&mut self, list: QuickfixList) {
        self.lists.push_back(list);
        self.current = self.lists.len() - 1;
    }

    /// Get the current quickfix list
    pub fn current_list(&self) -> Option<&QuickfixList> {
        if self.lists.is_empty() {
            None
        } else {
            Some(&self.lists[self.current])
        }
    }

    /// Get a mutable reference to the current quickfix list
    pub fn current_list_mut(&mut self) -> Option<&mut QuickfixList> {
        if self.lists.is_empty() {
            None
        } else {
            Some(&mut self.lists[self.current])
        }
    }

    /// Move to the next quickfix list
    pub fn next_list(&mut self) -> bool {
        if self.lists.is_empty() {
            return false;
        }

        if self.current + 1 < self.lists.len() {
            self.current += 1;
            true
        } else {
            false
        }
    }

    /// Move to the previous quickfix list
    pub fn prev_list(&mut self) -> bool {
        if self.lists.is_empty() {
            return false;
        }

        if self.current > 0 {
            self.current -= 1;
            true
        } else {
            false
        }
    }

    /// Add a location list for a window
    pub fn add_location_list(&mut self, window_id: usize, list: QuickfixList) {
        if !self.location_lists.contains_key(&window_id) {
            self.location_lists.insert(window_id, VecDeque::new());
            self.current_location.insert(window_id, 0);
        }

        if let Some(lists) = self.location_lists.get_mut(&window_id) {
            lists.push_back(list);
            self.current_location.insert(window_id, lists.len() - 1);
        }
    }

    /// Get the current location list for a window
    pub fn current_location_list(&self, window_id: usize) -> Option<&QuickfixList> {
        if let Some(lists) = self.location_lists.get(&window_id) {
            if lists.is_empty() {
                None
            } else if let Some(current) = self.current_location.get(&window_id) {
                Some(&lists[*current])
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Get a mutable reference to the current location list for a window
    pub fn current_location_list_mut(&mut self, window_id: usize) -> Option<&mut QuickfixList> {
        if let Some(lists) = self.location_lists.get_mut(&window_id) {
            if lists.is_empty() {
                None
            } else if let Some(current) = self.current_location.get(&window_id) {
                Some(&mut lists[*current])
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Move to the next location list for a window
    pub fn next_location_list(&mut self, window_id: usize) -> bool {
        if let Some(lists) = self.location_lists.get(&window_id) {
            if lists.is_empty() {
                return false;
            }

            if let Some(current) = self.current_location.get(&window_id) {
                if *current + 1 < lists.len() {
                    self.current_location.insert(window_id, *current + 1);
                    return true;
                }
            }
        }

        false
    }

    /// Move to the previous location list for a window
    pub fn prev_location_list(&mut self, window_id: usize) -> bool {
        if let Some(lists) = self.location_lists.get(&window_id) {
            if lists.is_empty() {
                return false;
            }

            if let Some(current) = self.current_location.get(&window_id) {
                if *current > 0 {
                    self.current_location.insert(window_id, *current - 1);
                    return true;
                }
            }
        }

        false
    }
}

// Global quickfix manager
static mut QUICKFIX_MANAGER: Option<QuickfixManager> = None;

/// Initialize the quickfix manager
pub fn init_quickfix_manager() {
    unsafe {
        if QUICKFIX_MANAGER.is_none() {
            QUICKFIX_MANAGER = Some(QuickfixManager::new());
        }
    }
}

/// Register quickfix command handlers
pub fn register_quickfix_handlers(registry: &mut ExCommandRegistry) {
    // Initialize the quickfix manager
    init_quickfix_manager();
    
    // Register quickfix commands
    registry.register("copen", handle_copen);
    registry.register("cclose", handle_cclose);
    registry.register("cnext", handle_cnext);
    registry.register("cprevious", handle_cprev);
    registry.register("cfirst", handle_cfirst);
    registry.register("clast", handle_clast);
    registry.register("clist", handle_clist);
    registry.register("cc", handle_cc);
    registry.register("make", handle_make);
    registry.register("grep", handle_grep);
    registry.register("vimgrep", handle_vimgrep);
    
    // Register location list commands
    registry.register("lopen", handle_lopen);
}

/// Handle the :lopen command
fn handle_lopen(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match crate::command::handlers::EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Get the height from the command arguments
    let height = if let Some(height_str) = cmd.first_arg() {
        match height_str.parse::<usize>() {
            Ok(h) => h,
            Err(_) => return Err(ExCommandError::InvalidArgument(format!("Invalid height: {}", height_str))),
        }
    } else {
        10 // Default height
    };
    
    // Get the quickfix manager
    let quickfix_manager = unsafe {
        match &mut QUICKFIX_MANAGER {
            Some(manager) => manager,
            None => {
                init_quickfix_manager();
                match &mut QUICKFIX_MANAGER {
                    Some(manager) => manager,
                    None => return Err(ExCommandError::Other("Failed to initialize quickfix manager".to_string())),
                }
            }
        }
    };
    
    // Get the current window ID
    let window_id = match editor.get_terminal().current_window_id() {
        Some(id) => id,
        None => return Err(ExCommandError::Other("No active window".to_string())),
    };
    
    // Check if there's a location list for the current window
    if quickfix_manager.current_location_list(window_id).is_none() {
        println!("No location list");
        return Ok(());
    }
    
    // Check if the location window is already open
    if let Some(location_window_id) = quickfix_manager.location_windows.get(&window_id) {
        // If the window is already open, just focus it
        if let Err(err) = editor.focus_window(*location_window_id) {
            return Err(ExCommandError::Other(format!("Failed to focus location window: {}", err)));
        }
        
        println!("Location window already open");
        return Ok(());
    }
    
    // Create a new buffer for the location list
    let buffer_id = match editor.get_buffer_manager_mut().create_buffer() {
        Ok(id) => id,
        Err(err) => return Err(ExCommandError::Other(format!("Failed to create buffer: {}", err))),
    };
    
    // Set the buffer name
    if let Err(err) = editor.get_buffer_manager_mut().set_buffer_name(buffer_id, "location-list") {
        return Err(ExCommandError::Other(format!("Failed to set buffer name: {}", err)));
    }
    
    // Set the buffer as read-only
    if let Err(err) = editor.get_buffer_manager_mut().set_buffer_readonly(buffer_id, true) {
        return Err(ExCommandError::Other(format!("Failed to set buffer as read-only: {}", err)));
    }
    
    // Fill the buffer with the location list
    if let Some(list) = quickfix_manager.current_location_list(window_id) {
        let mut content = String::new();
        
        // Add the title
        content.push_str(&format!("--- Location List: {} ---\n", list.title));
        
        // Add the entries
        for (i, entry) in list.entries.iter().enumerate() {
            let current_marker = if i == list.current { ">" } else { " " };
            let error_type = match entry.error_type {
                QuickfixErrorType::Error => "E",
                QuickfixErrorType::Warning => "W",
                QuickfixErrorType::Info => "I",
                QuickfixErrorType::Note => "N",
            };
            
            let col_str = if let Some(col) = entry.col {
                format!(":{}", col)
            } else {
                String::new()
            };
            
            content.push_str(&format!("{} {:3} {} {}:{}{}: {}\n",
                current_marker,
                i + 1,
                error_type,
                entry.file.display(),
                entry.line,
                col_str,
                entry.text
            ));
        }
        
        // Get a mutable reference to the buffer
        let buffer = match editor.get_buffer_manager_mut().get_buffer_mut(buffer_id) {
            Ok(buffer) => buffer,
            Err(err) => return Err(ExCommandError::Other(format!("Failed to get buffer: {}", err))),
        };
        
        // Insert the content
        if let Err(err) = buffer.insert(0, &content) {
            return Err(ExCommandError::Other(format!("Failed to insert content: {}", err)));
        }
    }
    
    // Split the window horizontally
    let location_window_id = match editor.get_terminal_mut().split_window_horizontal_with_height(buffer_id, height) {
        Ok(id) => id,
        Err(err) => return Err(ExCommandError::Other(format!("Failed to split window: {}", err))),
    };
    
    // Store the window ID in the quickfix manager
    quickfix_manager.location_windows.insert(window_id, location_window_id);
    
    println!("Location window opened");
    Ok(())
}

/// Handle the :copen command
fn handle_copen(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match crate::command::handlers::EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Get the height from the command arguments
    let height = if let Some(height_str) = cmd.first_arg() {
        match height_str.parse::<usize>() {
            Ok(h) => h,
            Err(_) => return Err(ExCommandError::InvalidArgument(format!("Invalid height: {}", height_str))),
        }
    } else {
        10 // Default height
    };
    
    // Get the quickfix manager
    let quickfix_manager = unsafe {
        match &mut QUICKFIX_MANAGER {
            Some(manager) => manager,
            None => {
                init_quickfix_manager();
                match &mut QUICKFIX_MANAGER {
                    Some(manager) => manager,
                    None => return Err(ExCommandError::Other("Failed to initialize quickfix manager".to_string())),
                }
            }
        }
    };
    
    // Check if there's a quickfix list
    if quickfix_manager.current_list().is_none() {
        println!("No quickfix list");
        return Ok(());
    }
    
    // Check if the quickfix window is already open
    if let Some(window_id) = quickfix_manager.quickfix_window {
        // If the window is already open, just focus it
        if let Err(err) = editor.focus_window(window_id) {
            return Err(ExCommandError::Other(format!("Failed to focus quickfix window: {}", err)));
        }
        
        println!("Quickfix window already open");
        return Ok(());
    }
    
    // Create a new buffer for the quickfix list
    let buffer_id = match editor.get_buffer_manager_mut().create_buffer() {
        Ok(id) => id,
        Err(err) => return Err(ExCommandError::Other(format!("Failed to create buffer: {}", err))),
    };
    
    // Set the buffer name
    if let Err(err) = editor.get_buffer_manager_mut().set_buffer_name(buffer_id, "quickfix") {
        return Err(ExCommandError::Other(format!("Failed to set buffer name: {}", err)));
    }
    
    // Set the buffer as read-only
    if let Err(err) = editor.get_buffer_manager_mut().set_buffer_readonly(buffer_id, true) {
        return Err(ExCommandError::Other(format!("Failed to set buffer as read-only: {}", err)));
    }
    
    // Fill the buffer with the quickfix list
    if let Some(list) = quickfix_manager.current_list() {
        let mut content = String::new();
        
        // Add the title
        content.push_str(&format!("--- Quickfix List: {} ---\n", list.title));
        
        // Add the entries
        for (i, entry) in list.entries.iter().enumerate() {
            let current_marker = if i == list.current { ">" } else { " " };
            let error_type = match entry.error_type {
                QuickfixErrorType::Error => "E",
                QuickfixErrorType::Warning => "W",
                QuickfixErrorType::Info => "I",
                QuickfixErrorType::Note => "N",
            };
            
            let col_str = if let Some(col) = entry.col {
                format!(":{}", col)
            } else {
                String::new()
            };
            
            content.push_str(&format!("{} {:3} {} {}:{}{}: {}\n",
                current_marker,
                i + 1,
                error_type,
                entry.file.display(),
                entry.line,
                col_str,
                entry.text
            ));
        }
        
        // Get a mutable reference to the buffer
        let buffer = match editor.get_buffer_manager_mut().get_buffer_mut(buffer_id) {
            Ok(buffer) => buffer,
            Err(err) => return Err(ExCommandError::Other(format!("Failed to get buffer: {}", err))),
        };
        
        // Insert the content
        if let Err(err) = buffer.insert(0, &content) {
            return Err(ExCommandError::Other(format!("Failed to insert content: {}", err)));
        }
    }
    
    // Split the window horizontally
    let window_id = match editor.get_terminal_mut().split_window_horizontal_with_height(buffer_id, height) {
        Ok(id) => id,
        Err(err) => return Err(ExCommandError::Other(format!("Failed to split window: {}", err))),
    };
    
    // Store the window ID in the quickfix manager
    quickfix_manager.quickfix_window = Some(window_id);
    
    println!("Quickfix window opened");
    Ok(())
}

/// Handle the :cclose command
fn handle_cclose(_cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match crate::command::handlers::EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Get the quickfix manager
    let quickfix_manager = unsafe {
        match &mut QUICKFIX_MANAGER {
            Some(manager) => manager,
            None => {
                init_quickfix_manager();
                match &mut QUICKFIX_MANAGER {
                    Some(manager) => manager,
                    None => return Err(ExCommandError::Other("Failed to initialize quickfix manager".to_string())),
                }
            }
        }
    };
    
    // Check if the quickfix window is open
    if let Some(window_id) = quickfix_manager.quickfix_window {
        // Close the window
        if let Err(err) = editor.get_terminal_mut().close_window(window_id) {
            return Err(ExCommandError::Other(format!("Failed to close quickfix window: {}", err)));
        }
        
        // Clear the window ID in the quickfix manager
        quickfix_manager.quickfix_window = None;
        
        println!("Quickfix window closed");
    } else {
        println!("No quickfix window to close");
    }
    
    Ok(())
}

/// Handle the :cnext command
fn handle_cnext(_cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match crate::command::handlers::EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Get the quickfix manager
    let quickfix_manager = unsafe {
        match &mut QUICKFIX_MANAGER {
            Some(manager) => manager,
            None => {
                init_quickfix_manager();
                match &mut QUICKFIX_MANAGER {
                    Some(manager) => manager,
                    None => return Err(ExCommandError::Other("Failed to initialize quickfix manager".to_string())),
                }
            }
        }
    };
    
    // Check if there's a quickfix list
    if quickfix_manager.current_list().is_none() {
        return Err(ExCommandError::Other("No quickfix list".to_string()));
    }
    
    // Move to the next entry
    if let Some(list) = quickfix_manager.current_list_mut() {
        if list.next() {
            // Get the current entry
            if let Some(entry) = list.current_entry() {
                // Open the file
                if let Err(err) = editor.open_file(&entry.file) {
                    return Err(ExCommandError::Other(format!("Failed to open file: {}", err)));
                }
                
                // Move the cursor to the line and column
                let line = entry.line.saturating_sub(1); // Convert to 0-based
                let col = entry.col.unwrap_or(0).saturating_sub(1); // Convert to 0-based
                
                if let Err(err) = editor.get_cursor_manager_mut().set_position(crate::cursor::CursorPosition::new(line, col)) {
                    return Err(ExCommandError::Other(format!("Failed to set cursor position: {}", err)));
                }
                
                println!("({}/{}) {}", list.current + 1, list.len(), entry.text);
            }
        } else {
            return Err(ExCommandError::Other("No more entries".to_string()));
        }
    }
    
    Ok(())
}

/// Handle the :cprev command
fn handle_cprev(_cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match crate::command::handlers::EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Get the quickfix manager
    let quickfix_manager = unsafe {
        match &mut QUICKFIX_MANAGER {
            Some(manager) => manager,
            None => {
                init_quickfix_manager();
                match &mut QUICKFIX_MANAGER {
                    Some(manager) => manager,
                    None => return Err(ExCommandError::Other("Failed to initialize quickfix manager".to_string())),
                }
            }
        }
    };
    
    // Check if there's a quickfix list
    if quickfix_manager.current_list().is_none() {
        return Err(ExCommandError::Other("No quickfix list".to_string()));
    }
    
    // Move to the previous entry
    if let Some(list) = quickfix_manager.current_list_mut() {
        if list.prev() {
            // Get the current entry
            if let Some(entry) = list.current_entry() {
                // Open the file
                if let Err(err) = editor.open_file(&entry.file) {
                    return Err(ExCommandError::Other(format!("Failed to open file: {}", err)));
                }
                
                // Move the cursor to the line and column
                let line = entry.line.saturating_sub(1); // Convert to 0-based
                let col = entry.col.unwrap_or(0).saturating_sub(1); // Convert to 0-based
                
                if let Err(err) = editor.get_cursor_manager_mut().set_position(crate::cursor::CursorPosition::new(line, col)) {
                    return Err(ExCommandError::Other(format!("Failed to set cursor position: {}", err)));
                }
                
                println!("({}/{}) {}", list.current + 1, list.len(), entry.text);
            }
        } else {
            return Err(ExCommandError::Other("No previous entries".to_string()));
        }
    }
    
    Ok(())
}

/// Handle the :cfirst command
fn handle_cfirst(_cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match crate::command::handlers::EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Get the quickfix manager
    let quickfix_manager = unsafe {
        match &mut QUICKFIX_MANAGER {
            Some(manager) => manager,
            None => {
                init_quickfix_manager();
                match &mut QUICKFIX_MANAGER {
                    Some(manager) => manager,
                    None => return Err(ExCommandError::Other("Failed to initialize quickfix manager".to_string())),
                }
            }
        }
    };
    
    // Check if there's a quickfix list
    if quickfix_manager.current_list().is_none() {
        return Err(ExCommandError::Other("No quickfix list".to_string()));
    }
    
    // Move to the first entry
    if let Some(list) = quickfix_manager.current_list_mut() {
        if list.is_empty() {
            return Err(ExCommandError::Other("Quickfix list is empty".to_string()));
        }
        
        // Set the current entry to the first one
        list.current = 0;
        
        // Get the current entry
        if let Some(entry) = list.current_entry() {
            // Open the file
            if let Err(err) = editor.open_file(&entry.file) {
                return Err(ExCommandError::Other(format!("Failed to open file: {}", err)));
            }
            
            // Move the cursor to the line and column
            let line = entry.line.saturating_sub(1); // Convert to 0-based
            let col = entry.col.unwrap_or(0).saturating_sub(1); // Convert to 0-based
            
            if let Err(err) = editor.get_cursor_manager_mut().set_position(crate::cursor::CursorPosition::new(line, col)) {
                return Err(ExCommandError::Other(format!("Failed to set cursor position: {}", err)));
            }
            
            println!("(1/{}) {}", list.len(), entry.text);
        }
    }
    
    Ok(())
}

/// Handle the :clast command
fn handle_clast(_cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match crate::command::handlers::EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Get the quickfix manager
    let quickfix_manager = unsafe {
        match &mut QUICKFIX_MANAGER {
            Some(manager) => manager,
            None => {
                init_quickfix_manager();
                match &mut QUICKFIX_MANAGER {
                    Some(manager) => manager,
                    None => return Err(ExCommandError::Other("Failed to initialize quickfix manager".to_string())),
                }
            }
        }
    };
    
    // Check if there's a quickfix list
    if quickfix_manager.current_list().is_none() {
        return Err(ExCommandError::Other("No quickfix list".to_string()));
    }
    
    // Move to the last entry
    if let Some(list) = quickfix_manager.current_list_mut() {
        if list.is_empty() {
            return Err(ExCommandError::Other("Quickfix list is empty".to_string()));
        }
        
        // Set the current entry to the last one
        list.current = list.len() - 1;
        
        // Get the current entry
        if let Some(entry) = list.current_entry() {
            // Open the file
            if let Err(err) = editor.open_file(&entry.file) {
                return Err(ExCommandError::Other(format!("Failed to open file: {}", err)));
            }
            
            // Move the cursor to the line and column
            let line = entry.line.saturating_sub(1); // Convert to 0-based
            let col = entry.col.unwrap_or(0).saturating_sub(1); // Convert to 0-based
            
            if let Err(err) = editor.get_cursor_manager_mut().set_position(crate::cursor::CursorPosition::new(line, col)) {
                return Err(ExCommandError::Other(format!("Failed to set cursor position: {}", err)));
            }
            
            println!("({}/{}) {}", list.len(), list.len(), entry.text);
        }
    }
    
    Ok(())
}

/// Handle the :clist command
fn handle_clist(_cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the quickfix manager
    let quickfix_manager = unsafe {
        match &QUICKFIX_MANAGER {
            Some(manager) => manager,
            None => {
                init_quickfix_manager();
                match &QUICKFIX_MANAGER {
                    Some(manager) => manager,
                    None => return Err(ExCommandError::Other("Failed to initialize quickfix manager".to_string())),
                }
            }
        }
    };
    
    // Check if there's a quickfix list
    if let Some(list) = quickfix_manager.current_list() {
        if list.is_empty() {
            println!("Quickfix list is empty");
            return Ok(());
        }
        
        // Print the title
        println!("--- Quickfix List: {} ---", list.title);
        
        // Print the entries
        for (i, entry) in list.entries.iter().enumerate() {
            let current_marker = if i == list.current { ">" } else { " " };
            let error_type = match entry.error_type {
                QuickfixErrorType::Error => "E",
                QuickfixErrorType::Warning => "W",
                QuickfixErrorType::Info => "I",
                QuickfixErrorType::Note => "N",
            };
            
            let col_str = if let Some(col) = entry.col {
                format!(":{}", col)
            } else {
                String::new()
            };
            
            println!("{} {:3} {} {}:{}{}: {}",
                current_marker,
                i + 1,
                error_type,
                entry.file.display(),
                entry.line,
                col_str,
                entry.text
            );
        }
    } else {
        println!("No quickfix list");
    }
    
    Ok(())
}

/// Handle the :cc command
fn handle_cc(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match crate::command::handlers::EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Get the quickfix manager
    let quickfix_manager = unsafe {
        match &mut QUICKFIX_MANAGER {
            Some(manager) => manager,
            None => {
                init_quickfix_manager();
                match &mut QUICKFIX_MANAGER {
                    Some(manager) => manager,
                    None => return Err(ExCommandError::Other("Failed to initialize quickfix manager".to_string())),
                }
            }
        }
    };
    
    // Check if there's a quickfix list
    if quickfix_manager.current_list().is_none() {
        return Err(ExCommandError::Other("No quickfix list".to_string()));
    }
    
    // Get the entry number from the command arguments
    let entry_num = if let Some(num_str) = cmd.first_arg() {
        match num_str.parse::<usize>() {
            Ok(n) => n,
            Err(_) => return Err(ExCommandError::InvalidArgument(format!("Invalid entry number: {}", num_str))),
        }
    } else {
        // If no entry number is provided, use the current entry
        if let Some(list) = quickfix_manager.current_list() {
            list.current + 1 // Convert to 1-based
        } else {
            return Err(ExCommandError::Other("No quickfix list".to_string()));
        }
    };
    
    // Move to the specified entry
    if let Some(list) = quickfix_manager.current_list_mut() {
        if entry_num == 0 || entry_num > list.len() {
            return Err(ExCommandError::InvalidArgument(format!("Invalid entry number: {}", entry_num)));
        }
        
        // Set the current entry
        list.current = entry_num - 1; // Convert to 0-based
        
        // Get the current entry
        if let Some(entry) = list.current_entry() {
            // Open the file
            if let Err(err) = editor.open_file(&entry.file) {
                return Err(ExCommandError::Other(format!("Failed to open file: {}", err)));
            }
            
            // Move the cursor to the line and column
            let line = entry.line.saturating_sub(1); // Convert to 0-based
            let col = entry.col.unwrap_or(0).saturating_sub(1); // Convert to 0-based
            
            if let Err(err) = editor.get_cursor_manager_mut().set_position(crate::cursor::CursorPosition::new(line, col)) {
                return Err(ExCommandError::Other(format!("Failed to set cursor position: {}", err)));
            }
            
            println!("({}/{}) {}", entry_num, list.len(), entry.text);
        }
    }
    
    Ok(())
}

/// Handle the :make command
fn handle_make(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match crate::command::handlers::EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Get the make command arguments
    let args = cmd.args_str();
    
    // Save all modified buffers
    if let Err(err) = editor.save_all_buffers() {
        return Err(ExCommandError::Other(format!("Failed to save buffers: {}", err)));
    }
    
    // Get the current directory
    let current_dir = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(err) => return Err(ExCommandError::Other(format!("Failed to get current directory: {}", err))),
    };
    
    // Run the make command
    let output = match Command::new("make")
        .args(args.split_whitespace())
        .current_dir(&current_dir)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output() {
        Ok(output) => output,
        Err(err) => return Err(ExCommandError::Other(format!("Failed to run make command: {}", err))),
    };
    
    // Create a new quickfix list
    let mut quickfix_list = QuickfixList::new(&format!("make {}", args));
    
    // Parse the output
    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Parse the error messages
    // This is a simple implementation that handles GCC/Clang error format
    // A more complete implementation would handle different compiler formats
    for line in stderr.lines().chain(stdout.lines()) {
        // Try to match GCC/Clang error format: file:line:col: error: message
        if let Some(captures) = regex::Regex::new(r"^([^:]+):(\d+):(\d+): (error|warning|note): (.+)$").unwrap().captures(line) {
            let file = PathBuf::from(captures.get(1).unwrap().as_str());
            let line = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();
            let col = captures.get(3).unwrap().as_str().parse::<usize>().unwrap();
            let error_type = match captures.get(4).unwrap().as_str() {
                "error" => QuickfixErrorType::Error,
                "warning" => QuickfixErrorType::Warning,
                "note" => QuickfixErrorType::Note,
                _ => QuickfixErrorType::Info,
            };
            let text = captures.get(5).unwrap().as_str().to_string();
            
            quickfix_list.add_entry(QuickfixEntry {
                file,
                line,
                col: Some(col),
                text,
                error_type,
            });
        }
        // Try to match GCC/Clang error format without column: file:line: error: message
        else if let Some(captures) = regex::Regex::new(r"^([^:]+):(\d+): (error|warning|note): (.+)$").unwrap().captures(line) {
            let file = PathBuf::from(captures.get(1).unwrap().as_str());
            let line = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();
            let error_type = match captures.get(3).unwrap().as_str() {
                "error" => QuickfixErrorType::Error,
                "warning" => QuickfixErrorType::Warning,
                "note" => QuickfixErrorType::Note,
                _ => QuickfixErrorType::Info,
            };
            let text = captures.get(4).unwrap().as_str().to_string();
            
            quickfix_list.add_entry(QuickfixEntry {
                file,
                line,
                col: None,
                text,
                error_type,
            });
        }
    }
    
    // Add the quickfix list to the manager
    let quickfix_manager = unsafe {
        match &mut QUICKFIX_MANAGER {
            Some(manager) => manager,
            None => {
                init_quickfix_manager();
                match &mut QUICKFIX_MANAGER {
                    Some(manager) => manager,
                    None => return Err(ExCommandError::Other("Failed to initialize quickfix manager".to_string())),
                }
            }
        }
    };
    
    quickfix_manager.add_list(quickfix_list);
    
    // Print the result
    if let Some(list) = quickfix_manager.current_list() {
        if list.is_empty() {
            println!("No errors");
        } else {
            let error_count = list.entries.iter().filter(|e| e.error_type == QuickfixErrorType::Error).count();
            let warning_count = list.entries.iter().filter(|e| e.error_type == QuickfixErrorType::Warning).count();
            
            println!("{} error{}, {} warning{}",
                error_count,
                if error_count == 1 { "" } else { "s" },
                warning_count,
                if warning_count == 1 { "" } else { "s" }
            );
            
            // If there are errors, jump to the first one
            if !list.is_empty() {
                // Get the first entry
                if let Some(entry) = list.current_entry() {
                    // Open the file
                    if let Err(err) = editor.open_file(&entry.file) {
                        return Err(ExCommandError::Other(format!("Failed to open file: {}", err)));
                    }
                    
                    // Move the cursor to the line and column
                    let line = entry.line.saturating_sub(1); // Convert to 0-based
                    let col = entry.col.unwrap_or(0).saturating_sub(1); // Convert to 0-based
                    
                    if let Err(err) = editor.get_cursor_manager_mut().set_position(crate::cursor::CursorPosition::new(line, col)) {
                        return Err(ExCommandError::Other(format!("Failed to set cursor position: {}", err)));
                    }
                }
            }
        }
    }
    
    Ok(())
}

/// Handle the :grep command
fn handle_grep(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match crate::command::handlers::EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Get the grep command arguments
    let args = cmd.args_str();
    
    if args.is_empty() {
        return Err(ExCommandError::MissingArgument("Pattern required".to_string()));
    }
    
    // Get the current directory
    let current_dir = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(err) => return Err(ExCommandError::Other(format!("Failed to get current directory: {}", err))),
    };
    
    // Run the grep command
    let output = match Command::new("grep")
        .arg("-n") // Line numbers
        .args(args.split_whitespace())
        .current_dir(&current_dir)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output() {
        Ok(output) => output,
        Err(err) => return Err(ExCommandError::Other(format!("Failed to run grep command: {}", err))),
    };
    
    // Create a new quickfix list
    let mut quickfix_list = QuickfixList::new(&format!("grep {}", args));
    
    // Parse the output
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Parse the grep output
    // Format: file:line:text
    for line in stdout.lines() {
        if let Some(captures) = regex::Regex::new(r"^([^:]+):(\d+):(.+)$").unwrap().captures(line) {
            let file = PathBuf::from(captures.get(1).unwrap().as_str());
            let line_num = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();
            let text = captures.get(3).unwrap().as_str().to_string();
            
            quickfix_list.add_entry(QuickfixEntry {
                file,
                line: line_num,
                col: None,
                text,
                error_type: QuickfixErrorType::Info,
            });
        }
    }
    
    // Add the quickfix list to the manager
    let quickfix_manager = unsafe {
        match &mut QUICKFIX_MANAGER {
            Some(manager) => manager,
            None => {
                init_quickfix_manager();
                match &mut QUICKFIX_MANAGER {
                    Some(manager) => manager,
                    None => return Err(ExCommandError::Other("Failed to initialize quickfix manager".to_string())),
                }
            }
        }
    };
    
    quickfix_manager.add_list(quickfix_list);
    
    // Print the result
    if let Some(list) = quickfix_manager.current_list() {
        if list.is_empty() {
            println!("No matches found");
        } else {
            println!("{} match{} found",
                list.len(),
                if list.len() == 1 { "" } else { "es" }
            );
            
            // If there are matches, jump to the first one
            if !list.is_empty() {
                // Get the first entry
                if let Some(entry) = list.current_entry() {
                    // Open the file
                    if let Err(err) = editor.open_file(&entry.file) {
                        return Err(ExCommandError::Other(format!("Failed to open file: {}", err)));
                    }
                    
                    // Move the cursor to the line and column
                    let line = entry.line.saturating_sub(1); // Convert to 0-based
                    let col = entry.col.unwrap_or(0).saturating_sub(1); // Convert to 0-based
                    
                    if let Err(err) = editor.get_cursor_manager_mut().set_position(crate::cursor::CursorPosition::new(line, col)) {
                        return Err(ExCommandError::Other(format!("Failed to set cursor position: {}", err)));
                    }
                }
            }
        }
    }
    
    Ok(())
}

/// Handle the :vimgrep command
fn handle_vimgrep(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match crate::command::handlers::EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Get the vimgrep command arguments
    let args = cmd.args_str();
    
    if args.is_empty() {
        return Err(ExCommandError::MissingArgument("Pattern required".to_string()));
    }
    
    // Parse the arguments
    // Format: /pattern/ file1 file2 ...
    let parts: Vec<&str> = args.splitn(2, ' ').collect();
    
    if parts.len() < 2 {
        return Err(ExCommandError::InvalidArgument("Invalid vimgrep format. Expected: /pattern/ file1 file2 ...".to_string()));
    }
    
    let pattern = parts[0];
    let files = parts[1];
    
    // Check if the pattern is enclosed in delimiters
    if !pattern.starts_with('/') || !pattern.ends_with('/') {
        return Err(ExCommandError::InvalidArgument("Pattern must be enclosed in / characters".to_string()));
    }
    
    // Extract the pattern without the delimiters
    let pattern = &pattern[1..pattern.len() - 1];
    
    // Create a regex from the pattern
    let regex = match regex::Regex::new(pattern) {
        Ok(re) => re,
        Err(err) => return Err(ExCommandError::InvalidArgument(format!("Invalid regex pattern: {}", err))),
    };
    
    // Create a new quickfix list
    let mut quickfix_list = QuickfixList::new(&format!("vimgrep {} {}", pattern, files));
    
    // Get the current directory
    let current_dir = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(err) => return Err(ExCommandError::Other(format!("Failed to get current directory: {}", err))),
    };
    
    // Expand the file patterns
    let mut file_paths = Vec::new();
    
    for file_pattern in files.split_whitespace() {
        // Use glob to expand the pattern
        match glob::glob(&format!("{}/{}", current_dir.display(), file_pattern)) {
            Ok(paths) => {
                for path in paths {
                    if let Ok(path) = path {
                        file_paths.push(path);
                    }
                }
            },
            Err(err) => return Err(ExCommandError::Other(format!("Failed to expand file pattern: {}", err))),
        }
    }
    
    // Search each file
    for file_path in file_paths {
        // Open the file
        let file = match File::open(&file_path) {
            Ok(file) => file,
            Err(err) => {
                println!("Warning: Failed to open file {}: {}", file_path.display(), err);
                continue;
            }
        };
        
        // Read the file line by line
        let reader = io::BufReader::new(file);
        
        for (line_num, line_result) in reader.lines().enumerate() {
            let line = match line_result {
                Ok(line) => line,
                Err(err) => {
                    println!("Warning: Failed to read line from {}: {}", file_path.display(), err);
                    continue;
                }
            };
            
            // Check if the line matches the pattern
            if regex.is_match(&line) {
                // Add an entry for each match
                for captures in regex.captures_iter(&line) {
                    let match_text = captures.get(0).unwrap().as_str();
                    let col = line.find(match_text).unwrap() + 1; // Convert to 1-based
                    
                    quickfix_list.add_entry(QuickfixEntry {
                        file: file_path.clone(),
                        line: line_num + 1, // Convert to 1-based
                        col: Some(col),
                        text: line.clone(),
                        error_type: QuickfixErrorType::Info,
                    });
                }
            }
        }
    }
    
    // Add the quickfix list to the manager
    let quickfix_manager = unsafe {
        match &mut QUICKFIX_MANAGER {
            Some(manager) => manager,
            None => {
                init_quickfix_manager();
                match &mut QUICKFIX_MANAGER {
                    Some(manager) => manager,
                    None => return Err(ExCommandError::Other("Failed to initialize quickfix manager".to_string())),
                }
            }
        }
    };
    
    quickfix_manager.add_list(quickfix_list);
    
    // Print the result
    if let Some(list) = quickfix_manager.current_list() {
        if list.is_empty() {
            println!("No matches found");
        } else {
            println!("{} match{} found",
                list.len(),
                if list.len() == 1 { "" } else { "es" }
            );
            
            // If there are matches, jump to the first one
            if !list.is_empty() {
                // Get the first entry
                if let Some(entry) = list.current_entry() {
                    // Open the file
                    if let Err(err) = editor.open_file(&entry.file) {
                        return Err(ExCommandError::Other(format!("Failed to open file: {}", err)));
                    }
                    
                    // Move the cursor to the line and column
                    let line = entry.line.saturating_sub(1); // Convert to 0-based
                    let col = entry.col.unwrap_or(0).saturating_sub(1); // Convert to 0-based
                    
                    if let Err(err) = editor.get_cursor_manager_mut().set_position(crate::cursor::CursorPosition::new(line, col)) {
                        return Err(ExCommandError::Other(format!("Failed to set cursor position: {}", err)));
                    }
                }
            }
        }
    }
    
    Ok(())
}

/// Handle the :lclose command
fn handle_lclose(_cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match crate::command::handlers::EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Get the quickfix manager
    let quickfix_manager = unsafe {
        match &mut QUICKFIX_MANAGER {
            Some(manager) => manager,
            None => {
                init_quickfix_manager();
                match &mut QUICKFIX_MANAGER {
                    Some(manager) => manager,
                    None => return Err(ExCommandError::Other("Failed to initialize quickfix manager".to_string())),
                }
            }
        }
    };
    
    // Get the current window ID
    let window_id = match editor.get_terminal().current_window_id() {
        Some(id) => id,
        None => return Err(ExCommandError::Other("No active window".to_string())),
    };
    
    // Check if the location window is open
    if let Some(location_window_id) = quickfix_manager.location_windows.get(&window_id) {
        // Close the window
        if let Err(err) = editor.get_terminal_mut().close_window(*location_window_id) {
            return Err(ExCommandError::Other(format!("Failed to close location window: {}", err)));
        }
        
        // Remove the window ID from the quickfix manager
        quickfix_manager.location_windows.remove(&window_id);
        
        println!("Location window closed");
    } else {
        println!("No location window to close");
    }
    
    Ok(())
}

/// Handle the :lnext command
fn handle_lnext(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match crate::command::handlers::EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Get the quickfix manager
    let quickfix_manager = unsafe {
        match &mut QUICKFIX_MANAGER {
            Some(manager) => manager,
            None => {
                init_quickfix_manager();
                match &mut QUICKFIX_MANAGER {
                    Some(manager) => manager,
                    None => return Err(ExCommandError::Other("Failed to initialize quickfix manager".to_string())),
                }
            }
        }
    };
    
    // Get the current window ID
    let window_id = match editor.get_terminal().current_window_id() {
        Some(id) => id,
        None => return Err(ExCommandError::Other("No active window".to_string())),
    };
    
    // Check if there's a location list for the current window
    if quickfix_manager.current_location_list(window_id).is_none() {
        return Err(ExCommandError::Other("No location list".to_string()));
    }
    
    // Get the entry number from the command arguments
    let entry_num = if let Some(num_str) = cmd.first_arg() {
        match num_str.parse::<usize>() {
            Ok(n) => n,
            Err(_) => return Err(ExCommandError::InvalidArgument(format!("Invalid entry number: {}", num_str))),
        }
    } else {
        // If no entry number is provided, use the current entry
        if let Some(list) = quickfix_manager.current_location_list(window_id) {
            list.current + 1 // Convert to 1-based
        } else {
            return Err(ExCommandError::Other("No location list".to_string()));
        }
    };
    
    // Move to the specified entry
    if let Some(list) = quickfix_manager.current_location_list_mut(window_id) {
        if entry_num == 0 || entry_num > list.len() {
            return Err(ExCommandError::InvalidArgument(format!("Invalid entry number: {}", entry_num)));
        }
        
        // Set the current entry
        list.current = entry_num - 1; // Convert to 0-based
        
        // Get the current entry
        if let Some(entry) = list.current_entry() {
            // Open the file
            if let Err(err) = editor.open_file(&entry.file) {
                return Err(ExCommandError::Other(format!("Failed to open file: {}", err)));
            }
            
            // Move the cursor to the line and column
            let line = entry.line.saturating_sub(1); // Convert to 0-based
            let col = entry.col.unwrap_or(0).saturating_sub(1); // Convert to 0-based
            
            if let Err(err) = editor.get_cursor_manager_mut().set_position(crate::cursor::CursorPosition::new(line, col)) {
                return Err(ExCommandError::Other(format!("Failed to set cursor position: {}", err)));
            }
            
            println!("({}/{}) {}", entry_num, list.len(), entry.text);
        }
    }
    
    Ok(())
}

/// Handle the :lmake command
fn handle_lmake(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match crate::command::handlers::EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Get the make command arguments
    let args = cmd.args_str();
    
    // Save all modified buffers
    if let Err(err) = editor.save_all_buffers() {
        return Err(ExCommandError::Other(format!("Failed to save buffers: {}", err)));
    }
    
    // Get the current directory
    let current_dir = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(err) => return Err(ExCommandError::Other(format!("Failed to get current directory: {}", err))),
    };
    
    // Run the make command
    let output = match Command::new("make")
        .args(args.split_whitespace())
        .current_dir(&current_dir)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output() {
        Ok(output) => output,
        Err(err) => return Err(ExCommandError::Other(format!("Failed to run make command: {}", err))),
    };
    
    // Create a new quickfix list
    let mut quickfix_list = QuickfixList::new(&format!("make {}", args));
    
    // Parse the output
    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Parse the error messages
    // This is a simple implementation that handles GCC/Clang error format
    // A more complete implementation would handle different compiler formats
    for line in stderr.lines().chain(stdout.lines()) {
        // Try to match GCC/Clang error format: file:line:col: error: message
        if let Some(captures) = regex::Regex::new(r"^([^:]+):(\d+):(\d+): (error|warning|note): (.+)$").unwrap().captures(line) {
            let file = PathBuf::from(captures.get(1).unwrap().as_str());
            let line = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();
            let col = captures.get(3).unwrap().as_str().parse::<usize>().unwrap();
            let error_type = match captures.get(4).unwrap().as_str() {
                "error" => QuickfixErrorType::Error,
                "warning" => QuickfixErrorType::Warning,
                "note" => QuickfixErrorType::Note,
                _ => QuickfixErrorType::Info,
            };
            let text = captures.get(5).unwrap().as_str().to_string();
            
            quickfix_list.add_entry(QuickfixEntry {
                file,
                line,
                col: Some(col),
                text,
                error_type,
            });
        }
        // Try to match GCC/Clang error format without column: file:line: error: message
        else if let Some(captures) = regex::Regex::new(r"^([^:]+):(\d+): (error|warning|note): (.+)$").unwrap().captures(line) {
            let file = PathBuf::from(captures.get(1).unwrap().as_str());
            let line = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();
            let error_type = match captures.get(3).unwrap().as_str() {
                "error" => QuickfixErrorType::Error,
                "warning" => QuickfixErrorType::Warning,
                "note" => QuickfixErrorType::Note,
                _ => QuickfixErrorType::Info,
            };
            let text = captures.get(4).unwrap().as_str().to_string();
            
            quickfix_list.add_entry(QuickfixEntry {
                file,
                line,
                col: None,
                text,
                error_type,
            });
        }
    }
    
    // Get the quickfix manager
    let quickfix_manager = unsafe {
        match &mut QUICKFIX_MANAGER {
            Some(manager) => manager,
            None => {
                init_quickfix_manager();
                match &mut QUICKFIX_MANAGER {
                    Some(manager) => manager,
                    None => return Err(ExCommandError::Other("Failed to initialize quickfix manager".to_string())),
                }
            }
        }
    };
    
    // Get the current window ID
    let window_id = match editor.get_terminal().current_window_id() {
        Some(id) => id,
        None => return Err(ExCommandError::Other("No active window".to_string())),
    };
    
    // Add the location list to the manager
    quickfix_manager.add_location_list(window_id, quickfix_list);
    
    // Print the result
    if let Some(list) = quickfix_manager.current_location_list(window_id) {
        if list.is_empty() {
            println!("No errors");
        } else {
            let error_count = list.entries.iter().filter(|e| e.error_type == QuickfixErrorType::Error).count();
            let warning_count = list.entries.iter().filter(|e| e.error_type == QuickfixErrorType::Warning).count();
            
            println!("{} error{}, {} warning{}",
                error_count,
                if error_count == 1 { "" } else { "s" },
                warning_count,
                if warning_count == 1 { "" } else { "s" }
            );
            
            // If there are errors, jump to the first one
            if !list.is_empty() {
                // Get the first entry
                if let Some(entry) = list.current_entry() {
                    // Open the file
                    if let Err(err) = editor.open_file(&entry.file) {
                        return Err(ExCommandError::Other(format!("Failed to open file: {}", err)));
                    }
                    
                    // Move the cursor to the line and column
                    let line = entry.line.saturating_sub(1); // Convert to 0-based
                    let col = entry.col.unwrap_or(0).saturating_sub(1); // Convert to 0-based
                    
                    if let Err(err) = editor.get_cursor_manager_mut().set_position(crate::cursor::CursorPosition::new(line, col)) {
                        return Err(ExCommandError::Other(format!("Failed to set cursor position: {}", err)));
                    }
                }
            }
        }
    }
    
    Ok(())
}

/// Handle the :lgrep command
fn handle_lgrep(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match crate::command::handlers::EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Get the grep command arguments
    let args = cmd.args_str();
    
    if args.is_empty() {
        return Err(ExCommandError::MissingArgument("Pattern required".to_string()));
    }
    
    // Get the current directory
    let current_dir = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(err) => return Err(ExCommandError::Other(format!("Failed to get current directory: {}", err))),
    };
    
    // Run the grep command
    let output = match Command::new("grep")
        .arg("-n") // Line numbers
        .args(args.split_whitespace())
        .current_dir(&current_dir)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output() {
        Ok(output) => output,
        Err(err) => return Err(ExCommandError::Other(format!("Failed to run grep command: {}", err))),
    };
    
    // Create a new quickfix list
    let mut quickfix_list = QuickfixList::new(&format!("grep {}", args));
    
    // Parse the output
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Parse the grep output
    // Format: file:line:text
    for line in stdout.lines() {
        if let Some(captures) = regex::Regex::new(r"^([^:]+):(\d+):(.+)$").unwrap().captures(line) {
            let file = PathBuf::from(captures.get(1).unwrap().as_str());
            let line_num = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();
            let text = captures.get(3).unwrap().as_str().to_string();
            
            quickfix_list.add_entry(QuickfixEntry {
                file,
                line: line_num,
                col: None,
                text,
                error_type: QuickfixErrorType::Info,
            });
        }
    }
    
    // Get the quickfix manager
    let quickfix_manager = unsafe {
        match &mut QUICKFIX_MANAGER {
            Some(manager) => manager,
            None => {
                init_quickfix_manager();
                match &mut QUICKFIX_MANAGER {
                    Some(manager) => manager,
                    None => return Err(ExCommandError::Other("Failed to initialize quickfix manager".to_string())),
                }
            }
        }
    };
    
    // Get the current window ID
    let window_id = match editor.get_terminal().current_window_id() {
        Some(id) => id,
        None => return Err(ExCommandError::Other("No active window".to_string())),
    };
    
    // Add the location list to the manager
    quickfix_manager.add_location_list(window_id, quickfix_list);
    
    // Print the result
    if let Some(list) = quickfix_manager.current_location_list(window_id) {
        if list.is_empty() {
            println!("No matches found");
        } else {
            println!("{} match{} found",
                list.len(),
                if list.len() == 1 { "" } else { "es" }
            );
            
            // If there are matches, jump to the first one
            if !list.is_empty() {
                // Get the first entry
                if let Some(entry) = list.current_entry() {
                    // Open the file
                    if let Err(err) = editor.open_file(&entry.file) {
                        return Err(ExCommandError::Other(format!("Failed to open file: {}", err)));
                    }
                    
                    // Move the cursor to the line and column
                    let line = entry.line.saturating_sub(1); // Convert to 0-based
                    let col = entry.col.unwrap_or(0).saturating_sub(1); // Convert to 0-based
                    
                    if let Err(err) = editor.get_cursor_manager_mut().set_position(crate::cursor::CursorPosition::new(line, col)) {
                        return Err(ExCommandError::Other(format!("Failed to set cursor position: {}", err)));
                    }
                }
            }
        }
    }
    
    Ok(())
}

/// Handle the :lvimgrep command
fn handle_lvimgrep(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match crate::command::handlers::EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Get the vimgrep command arguments
    let args = cmd.args_str();
    
    if args.is_empty() {
        return Err(ExCommandError::MissingArgument("Pattern required".to_string()));
    }
    
    // Parse the arguments
    // Format: /pattern/ file1 file2 ...
    let parts: Vec<&str> = args.splitn(2, ' ').collect();
    
    if parts.len() < 2 {
        return Err(ExCommandError::InvalidArgument("Invalid vimgrep format. Expected: /pattern/ file1 file2 ...".to_string()));
    }
    
    let pattern = parts[0];
    let files = parts[1];
    
    // Check if the pattern is enclosed in delimiters
    if !pattern.starts_with('/') || !pattern.ends_with('/') {
        return Err(ExCommandError::InvalidArgument("Pattern must be enclosed in / characters".to_string()));
    }
    
    // Extract the pattern without the delimiters
    let pattern = &pattern[1..pattern.len() - 1];
    
    // Create a regex from the pattern
    let regex = match regex::Regex::new(pattern) {
        Ok(re) => re,
        Err(err) => return Err(ExCommandError::InvalidArgument(format!("Invalid regex pattern: {}", err))),
    };
    
    // Create a new quickfix list
    let mut quickfix_list = QuickfixList::new(&format!("vimgrep {} {}", pattern, files));
    
    // Get the current directory
    let current_dir = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(err) => return Err(ExCommandError::Other(format!("Failed to get current directory: {}", err))),
    };
    
    // Expand the file patterns
    let mut file_paths = Vec::new();
    
    for file_pattern in files.split_whitespace() {
        // Use glob to expand the pattern
        match glob::glob(&format!("{}/{}", current_dir.display(), file_pattern)) {
            Ok(paths) => {
                for path in paths {
                    if let Ok(path) = path {
                        file_paths.push(path);
                    }
                }
            },
            Err(err) => return Err(ExCommandError::Other(format!("Failed to expand file pattern: {}", err))),
        }
    }
    
    // Search each file
    for file_path in file_paths {
        // Open the file
        let file = match File::open(&file_path) {
            Ok(file) => file,
            Err(err) => {
                println!("Warning: Failed to open file {}: {}", file_path.display(), err);
                continue;
            }
        };
        
        // Read the file line by line
        let reader = io::BufReader::new(file);
        
        for (line_num, line_result) in reader.lines().enumerate() {
            let line = match line_result {
                Ok(line) => line,
                Err(err) => {
                    println!("Warning: Failed to read line from {}: {}", file_path.display(), err);
                    continue;
                }
            };
            
            // Check if the line matches the pattern
            if regex.is_match(&line) {
                // Add an entry for each match
                for captures in regex.captures_iter(&line) {
                    let match_text = captures.get(0).unwrap().as_str();
                    let col = line.find(match_text).unwrap() + 1; // Convert to 1-based
                    
                    quickfix_list.add_entry(QuickfixEntry {
                        file: file_path.clone(),
                        line: line_num + 1, // Convert to 1-based
                        col: Some(col),
                        text: line.clone(),
                        error_type: QuickfixErrorType::Info,
                    });
                }
            }
        }
    }
    
    // Get the quickfix manager
    let quickfix_manager = unsafe {
        match &mut QUICKFIX_MANAGER {
            Some(manager) => manager,
            None => {
                init_quickfix_manager();
                match &mut QUICKFIX_MANAGER {
                    Some(manager) => manager,
                    None => return Err(ExCommandError::Other("Failed to initialize quickfix manager".to_string())),
                }
            }
        }
    };
    
    // Get the current window ID
    let window_id = match editor.get_terminal().current_window_id() {
        Some(id) => id,
        None => return Err(ExCommandError::Other("No active window".to_string())),
    };
    
    // Add the location list to the manager
    quickfix_manager.add_location_list(window_id, quickfix_list);
    
    // Print the result
    if let Some(list) = quickfix_manager.current_location_list(window_id) {
        if list.is_empty() {
            println!("No matches found");
        } else {
            println!("{} match{} found",
                list.len(),
                if list.len() == 1 { "" } else { "es" }
            );
            
            // If there are matches, jump to the first one
            if !list.is_empty() {
                // Get the first entry
                if let Some(entry) = list.current_entry() {
                    // Open the file
                    if let Err(err) = editor.open_file(&entry.file) {
                        return Err(ExCommandError::Other(format!("Failed to open file: {}", err)));
                    }
                    
                    // Move the cursor to the line and column
                    let line = entry.line.saturating_sub(1); // Convert to 0-based
                    let col = entry.col.unwrap_or(0).saturating_sub(1); // Convert to 0-based
                    
                    if let Err(err) = editor.get_cursor_manager_mut().set_position(crate::cursor::CursorPosition::new(line, col)) {
                        return Err(ExCommandError::Other(format!("Failed to set cursor position: {}", err)));
                    }
                }
            }
        }
    }
    
    Ok(())
}