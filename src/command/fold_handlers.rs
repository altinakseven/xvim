//! Fold command handlers
//!
//! This module implements handlers for fold commands.

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

/// Fold method
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoldMethod {
    /// Manual folding
    Manual,
    /// Indent-based folding
    Indent,
    /// Expression-based folding
    Expr,
    /// Syntax-based folding
    Syntax,
    /// Marker-based folding
    Marker,
    /// Diff-based folding
    Diff,
}

impl Default for FoldMethod {
    fn default() -> Self {
        FoldMethod::Manual
    }
}

/// Fold state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoldState {
    /// Fold is open
    Open,
    /// Fold is closed
    Closed,
}

/// Fold
#[derive(Debug, Clone)]
pub struct Fold {
    /// Start line
    pub start: usize,
    /// End line
    pub end: usize,
    /// Fold level
    pub level: usize,
    /// Fold state
    pub state: FoldState,
    /// Fold text (displayed when fold is closed)
    pub text: Option<String>,
}

impl Fold {
    /// Create a new fold
    pub fn new(start: usize, end: usize, level: usize) -> Self {
        Self {
            start,
            end,
            level,
            state: FoldState::Closed,
            text: None,
        }
    }

    /// Check if a line is within the fold
    pub fn contains(&self, line: usize) -> bool {
        line >= self.start && line <= self.end
    }

    /// Get the number of lines in the fold
    pub fn line_count(&self) -> usize {
        self.end - self.start + 1
    }
}

/// Fold manager
#[derive(Debug)]
pub struct FoldManager {
    /// Folds for each buffer
    pub folds: HashMap<usize, Vec<Fold>>,
    /// Fold method for each buffer
    pub methods: HashMap<usize, FoldMethod>,
    /// Fold column (used for displaying fold markers)
    pub fold_column: usize,
    /// Fold minimum width
    pub fold_min_width: usize,
    /// Fold text function
    pub fold_text: Option<String>,
}

impl FoldManager {
    /// Create a new fold manager
    pub fn new() -> Self {
        Self {
            folds: HashMap::new(),
            methods: HashMap::new(),
            fold_column: 0,
            fold_min_width: 20,
            fold_text: None,
        }
    }

    /// Get the folds for a buffer
    pub fn get_folds(&self, buffer_id: usize) -> Option<&Vec<Fold>> {
        self.folds.get(&buffer_id)
    }

    /// Get a mutable reference to the folds for a buffer
    pub fn get_folds_mut(&mut self, buffer_id: usize) -> Option<&mut Vec<Fold>> {
        self.folds.get_mut(&buffer_id)
    }

    /// Get the fold method for a buffer
    pub fn get_method(&self, buffer_id: usize) -> FoldMethod {
        *self.methods.get(&buffer_id).unwrap_or(&FoldMethod::Manual)
    }

    /// Set the fold method for a buffer
    pub fn set_method(&mut self, buffer_id: usize, method: FoldMethod) {
        self.methods.insert(buffer_id, method);
    }

    /// Create a fold
    pub fn create_fold(&mut self, buffer_id: usize, start: usize, end: usize, level: usize) -> Option<usize> {
        // Get or create the folds for the buffer
        let folds = self.folds.entry(buffer_id).or_insert_with(Vec::new);
        
        // Check if the fold overlaps with existing folds
        for (i, fold) in folds.iter().enumerate() {
            if (start >= fold.start && start <= fold.end) || (end >= fold.start && end <= fold.end) {
                // Fold overlaps with an existing fold
                return None;
            }
        }
        
        // Create the fold
        let fold = Fold::new(start, end, level);
        folds.push(fold);
        
        // Return the index of the new fold
        Some(folds.len() - 1)
    }

    /// Delete a fold
    pub fn delete_fold(&mut self, buffer_id: usize, fold_idx: usize) -> bool {
        if let Some(folds) = self.folds.get_mut(&buffer_id) {
            if fold_idx < folds.len() {
                folds.remove(fold_idx);
                return true;
            }
        }
        
        false
    }

    /// Delete all folds in a buffer
    pub fn delete_all_folds(&mut self, buffer_id: usize) -> bool {
        if let Some(folds) = self.folds.get_mut(&buffer_id) {
            folds.clear();
            return true;
        }
        
        false
    }

    /// Open a fold
    pub fn open_fold(&mut self, buffer_id: usize, fold_idx: usize) -> bool {
        if let Some(folds) = self.folds.get_mut(&buffer_id) {
            if fold_idx < folds.len() {
                folds[fold_idx].state = FoldState::Open;
                return true;
            }
        }
        
        false
    }

    /// Close a fold
    pub fn close_fold(&mut self, buffer_id: usize, fold_idx: usize) -> bool {
        if let Some(folds) = self.folds.get_mut(&buffer_id) {
            if fold_idx < folds.len() {
                folds[fold_idx].state = FoldState::Closed;
                return true;
            }
        }
        
        false
    }

    /// Toggle a fold
    pub fn toggle_fold(&mut self, buffer_id: usize, fold_idx: usize) -> bool {
        if let Some(folds) = self.folds.get_mut(&buffer_id) {
            if fold_idx < folds.len() {
                folds[fold_idx].state = match folds[fold_idx].state {
                    FoldState::Open => FoldState::Closed,
                    FoldState::Closed => FoldState::Open,
                };
                return true;
            }
        }
        
        false
    }

    /// Open all folds in a buffer
    pub fn open_all_folds(&mut self, buffer_id: usize) -> bool {
        if let Some(folds) = self.folds.get_mut(&buffer_id) {
            for fold in folds.iter_mut() {
                fold.state = FoldState::Open;
            }
            return true;
        }
        
        false
    }

    /// Close all folds in a buffer
    pub fn close_all_folds(&mut self, buffer_id: usize) -> bool {
        if let Some(folds) = self.folds.get_mut(&buffer_id) {
            for fold in folds.iter_mut() {
                fold.state = FoldState::Closed;
            }
            return true;
        }
        
        false
    }

    /// Find the fold containing a line
    pub fn find_fold(&self, buffer_id: usize, line: usize) -> Option<usize> {
        if let Some(folds) = self.folds.get(&buffer_id) {
            for (i, fold) in folds.iter().enumerate() {
                if fold.contains(line) {
                    return Some(i);
                }
            }
        }
        
        None
    }

    /// Check if a line is visible (not hidden by a fold)
    pub fn is_line_visible(&self, buffer_id: usize, line: usize) -> bool {
        if let Some(folds) = self.folds.get(&buffer_id) {
            for fold in folds.iter() {
                if fold.state == FoldState::Closed && fold.contains(line) && line != fold.start {
                    return false;
                }
            }
        }
        
        true
    }

    /// Get the next visible line
    pub fn next_visible_line(&self, buffer_id: usize, line: usize, line_count: usize) -> usize {
        let mut next_line = line + 1;
        
        while next_line < line_count && !self.is_line_visible(buffer_id, next_line) {
            next_line += 1;
        }
        
        next_line
    }

    /// Get the previous visible line
    pub fn prev_visible_line(&self, buffer_id: usize, line: usize) -> usize {
        if line == 0 {
            return 0;
        }
        
        let mut prev_line = line - 1;
        
        while prev_line > 0 && !self.is_line_visible(buffer_id, prev_line) {
            prev_line -= 1;
        }
        
        prev_line
    }

    /// Create folds based on the fold method
    pub fn create_folds_for_buffer(&mut self, buffer_id: usize, editor: &Editor) -> ExCommandResult<()> {
        // Get the fold method for the buffer
        let method = self.get_method(buffer_id);
        
        // Clear existing folds
        self.delete_all_folds(buffer_id);
        
        // Create folds based on the method
        match method {
            FoldMethod::Manual => {
                // Manual folding doesn't create folds automatically
                Ok(())
            },
            FoldMethod::Indent => {
                // Create folds based on indentation
                self.create_indent_folds(buffer_id, editor)
            },
            FoldMethod::Expr => {
                // Expression-based folding not implemented yet
                Err(ExCommandError::Other("Expression-based folding not implemented yet".to_string()))
            },
            FoldMethod::Syntax => {
                // Syntax-based folding not implemented yet
                Err(ExCommandError::Other("Syntax-based folding not implemented yet".to_string()))
            },
            FoldMethod::Marker => {
                // Create folds based on markers
                self.create_marker_folds(buffer_id, editor)
            },
            FoldMethod::Diff => {
                // Diff-based folding not implemented yet
                Err(ExCommandError::Other("Diff-based folding not implemented yet".to_string()))
            },
        }
    }

    /// Create folds based on indentation
    fn create_indent_folds(&mut self, buffer_id: usize, editor: &Editor) -> ExCommandResult<()> {
        // Get the buffer
        let buffer = match editor.get_buffer_manager().get_buffer(buffer_id) {
            Ok(buffer) => buffer,
            Err(err) => return Err(ExCommandError::Other(format!("Failed to get buffer: {}", err))),
        };
        
        // Get the line count
        let line_count = buffer.line_count();
        
        // Create a stack to track fold levels
        let mut stack = Vec::new();
        
        // Iterate through the lines
        let mut i = 0;
        while i < line_count {
            // Get the line
            let line = match buffer.line(i) {
                Ok(line) => line,
                Err(err) => return Err(ExCommandError::Other(format!("Failed to get line: {}", err))),
            };
            
            // Calculate the indentation level
            let indent = line.chars().take_while(|c| c.is_whitespace()).count();
            
            // Check if this line starts a new fold
            if !line.trim().is_empty() {
                // Check if we need to close any folds
                loop {
                    // Get the last element without borrowing the stack
                    let should_close = match stack.last() {
                        Some((_, level)) if *level >= indent => true,
                        Some(_) => false,
                        None => false,
                    };
                    
                    if should_close {
                        // Now we can safely pop
                        if let Some((start, level)) = stack.pop() {
                            // Create the fold
                            self.create_fold(buffer_id, start, i - 1, level);
                        }
                    } else {
                        break;
                    }
                }
                
                // Start a new fold
                stack.push((i, indent));
            }
            
            i += 1;
        }
        
        // Close any remaining folds
        while let Some((start, level)) = stack.pop() {
            // Create the fold
            self.create_fold(buffer_id, start, line_count - 1, level);
        }
        
        Ok(())
    }

    /// Create folds based on markers
    fn create_marker_folds(&mut self, buffer_id: usize, editor: &Editor) -> ExCommandResult<()> {
        // Get the buffer
        let buffer = match editor.get_buffer_manager().get_buffer(buffer_id) {
            Ok(buffer) => buffer,
            Err(err) => return Err(ExCommandError::Other(format!("Failed to get buffer: {}", err))),
        };
        
        // Get the line count
        let line_count = buffer.line_count();
        
        // Create a stack to track fold levels
        let mut stack = Vec::new();
        
        // Iterate through the lines
        let mut i = 0;
        while i < line_count {
            // Get the line
            let line = match buffer.line(i) {
                Ok(line) => line,
                Err(err) => return Err(ExCommandError::Other(format!("Failed to get line: {}", err))),
            };
            
            // Check if this line contains a fold marker
            if line.contains("{{{") {
                // Extract the fold level
                let level = if let Some(pos) = line.find("{{{") {
                    if pos + 3 < line.len() && line[pos + 3..].chars().next().unwrap_or(' ').is_digit(10) {
                        line[pos + 3..].chars().next().unwrap().to_digit(10).unwrap() as usize
                    } else {
                        1
                    }
                } else {
                    1
                };
                
                // Start a new fold
                stack.push((i, level));
            } else if line.contains("}}}") {
                // Check if we have a matching fold start
                if let Some((start, level)) = stack.pop() {
                    // Create the fold
                    self.create_fold(buffer_id, start, i, level);
                }
            }
            
            i += 1;
        }
        
        Ok(())
    }
}

// Global fold manager
static mut FOLD_MANAGER: Option<FoldManager> = None;

/// Initialize the fold manager
pub fn init_fold_manager() {
    unsafe {
        if FOLD_MANAGER.is_none() {
            FOLD_MANAGER = Some(FoldManager::new());
        }
    }
}

/// Register fold command handlers
pub fn register_fold_handlers(registry: &mut ExCommandRegistry) {
    // Initialize the fold manager
    init_fold_manager();
    
    // Register fold commands
    registry.register("fold", handle_fold);
    registry.register("foldopen", handle_foldopen);
    registry.register("foldclose", handle_foldclose);
    registry.register("foldtoggle", handle_foldtoggle);
    registry.register("foldmethod", handle_foldmethod);
    registry.register("foldcolumn", handle_foldcolumn);
    registry.register("foldminlines", handle_foldminlines);
    registry.register("foldtext", handle_foldtext);
    registry.register("foldlevel", handle_foldlevel);
}

/// Handle the :fold command
fn handle_fold(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match crate::command::handlers::EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Get the current buffer ID
    let buffer_id = match editor.current_buffer_id() {
        Some(id) => id,
        None => return Err(ExCommandError::InvalidCommand("No buffer to fold".to_string())),
    };
    
    // Get the fold manager
    let fold_manager = unsafe {
        match &mut FOLD_MANAGER {
            Some(manager) => manager,
            None => {
                init_fold_manager();
                match &mut FOLD_MANAGER {
                    Some(manager) => manager,
                    None => return Err(ExCommandError::Other("Failed to initialize fold manager".to_string())),
                }
            }
        }
    };
    
    // Parse the range from the command
    let start_line = if let Some(start) = cmd.range.start {
        match start {
            crate::command::RangeSpec::CurrentLine => editor.cursor_position().line,
            crate::command::RangeSpec::LastLine => {
                let buffer = editor.get_buffer_manager().get_buffer(buffer_id)?;
                buffer.line_count() - 1
            },
            crate::command::RangeSpec::LineNumber(n) => n - 1, // Convert to 0-based
            crate::command::RangeSpec::Mark(mark) => {
                match editor.get_mark_position(mark) {
                    Some(pos) => pos.line,
                    None => return Err(ExCommandError::InvalidRange(format!("Mark '{}' not set", mark))),
                }
            },
            crate::command::RangeSpec::Search(pattern) => {
                // Search for the pattern
                match editor.search(pattern.as_str(), true, false) {
                    Ok(Some(pos)) => pos.line,
                    Ok(None) => return Err(ExCommandError::InvalidRange(format!("Pattern not found: {}", pattern))),
                    Err(err) => return Err(ExCommandError::InvalidRange(format!("Search error: {}", err))),
                }
            },
            crate::command::RangeSpec::Offset(offset) => {
                let current_line = editor.cursor_position().line;
                if offset >= 0 {
                    current_line + offset as usize
                } else {
                    current_line.saturating_sub((-offset) as usize)
                }
            },
        }
    } else {
        editor.cursor_position().line
    };
    
    let end_line = if let Some(end) = cmd.range.end {
        match end {
            crate::command::RangeSpec::CurrentLine => editor.cursor_position().line,
            crate::command::RangeSpec::LastLine => {
                let buffer = editor.get_buffer_manager().get_buffer(buffer_id)?;
                buffer.line_count() - 1
            },
            crate::command::RangeSpec::LineNumber(n) => n - 1, // Convert to 0-based
            crate::command::RangeSpec::Mark(mark) => {
                match editor.get_mark_position(mark) {
                    Some(pos) => pos.line,
                    None => return Err(ExCommandError::InvalidRange(format!("Mark '{}' not set", mark))),
                }
            },
            crate::command::RangeSpec::Search(pattern) => {
                // Search for the pattern
                match editor.search(pattern.as_str(), true, false) {
                    Ok(Some(pos)) => pos.line,
                    Ok(None) => return Err(ExCommandError::InvalidRange(format!("Pattern not found: {}", pattern))),
                    Err(err) => return Err(ExCommandError::InvalidRange(format!("Search error: {}", err))),
                }
            },
            crate::command::RangeSpec::Offset(offset) => {
                let current_line = editor.cursor_position().line;
                if offset >= 0 {
                    current_line + offset as usize
                } else {
                    current_line.saturating_sub((-offset) as usize)
                }
            },
        }
    } else {
        start_line
    };
    
    // Create the fold
    match fold_manager.create_fold(buffer_id, start_line, end_line, 1) {
        Some(_) => {
            println!("Fold created for lines {}-{}", start_line + 1, end_line + 1);
            Ok(())
        },
        None => Err(ExCommandError::Other("Failed to create fold".to_string())),
    }
}

/// Handle the :foldopen command
fn handle_foldopen(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match crate::command::handlers::EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Get the current buffer ID
    let buffer_id = match editor.current_buffer_id() {
        Some(id) => id,
        None => return Err(ExCommandError::InvalidCommand("No buffer to fold".to_string())),
    };
    
    // Get the fold manager
    let fold_manager = unsafe {
        match &mut FOLD_MANAGER {
            Some(manager) => manager,
            None => {
                init_fold_manager();
                match &mut FOLD_MANAGER {
                    Some(manager) => manager,
                    None => return Err(ExCommandError::Other("Failed to initialize fold manager".to_string())),
                }
            }
        }
    };
    
    // Check if we should open all folds
    if cmd.args_str() == "all" {
        // Open all folds
        if fold_manager.open_all_folds(buffer_id) {
            println!("All folds opened");
            return Ok(());
        } else {
            return Err(ExCommandError::Other("No folds to open".to_string()));
        }
    }
    
    // Get the current line
    let line = editor.cursor_position().line;
    
    // Find the fold containing the current line
    if let Some(fold_idx) = fold_manager.find_fold(buffer_id, line) {
        // Open the fold
        if fold_manager.open_fold(buffer_id, fold_idx) {
            println!("Fold opened");
            return Ok(());
        }
    }
    
    Err(ExCommandError::Other("No fold found at cursor position".to_string()))
}

/// Handle the :foldclose command
fn handle_foldclose(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match crate::command::handlers::EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Get the current buffer ID
    let buffer_id = match editor.current_buffer_id() {
        Some(id) => id,
        None => return Err(ExCommandError::InvalidCommand("No buffer to fold".to_string())),
    };
    
    // Get the fold manager
    let fold_manager = unsafe {
        match &mut FOLD_MANAGER {
            Some(manager) => manager,
            None => {
                init_fold_manager();
                match &mut FOLD_MANAGER {
                    Some(manager) => manager,
                    None => return Err(ExCommandError::Other("Failed to initialize fold manager".to_string())),
                }
            }
        }
    };
    
    // Check if we should close all folds
    if cmd.args_str() == "all" {
        // Close all folds
        if fold_manager.close_all_folds(buffer_id) {
            println!("All folds closed");
            return Ok(());
        } else {
            return Err(ExCommandError::Other("No folds to close".to_string()));
        }
    }
    
    // Get the current line
    let line = editor.cursor_position().line;
    
    // Find the fold containing the current line
    if let Some(fold_idx) = fold_manager.find_fold(buffer_id, line) {
        // Close the fold
        if fold_manager.close_fold(buffer_id, fold_idx) {
            println!("Fold closed");
            return Ok(());
        }
    }
    
    Err(ExCommandError::Other("No fold found at cursor position".to_string()))
}

/// Handle the :foldtoggle command
fn handle_foldtoggle(_cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match crate::command::handlers::EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Get the current buffer ID
    let buffer_id = match editor.current_buffer_id() {
        Some(id) => id,
        None => return Err(ExCommandError::InvalidCommand("No buffer to fold".to_string())),
    };
    
    // Get the fold manager
    let fold_manager = unsafe {
        match &mut FOLD_MANAGER {
            Some(manager) => manager,
            None => {
                init_fold_manager();
                match &mut FOLD_MANAGER {
                    Some(manager) => manager,
                    None => return Err(ExCommandError::Other("Failed to initialize fold manager".to_string())),
                }
            }
        }
    };
    
    // Get the current line
    let line = editor.cursor_position().line;
    
    // Find the fold containing the current line
    if let Some(fold_idx) = fold_manager.find_fold(buffer_id, line) {
        // Toggle the fold
        if fold_manager.toggle_fold(buffer_id, fold_idx) {
            println!("Fold toggled");
            return Ok(());
        }
    }
    
    Err(ExCommandError::Other("No fold found at cursor position".to_string()))
}

/// Handle the :foldmethod command
fn handle_foldmethod(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match crate::command::handlers::EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Get the current buffer ID
    let buffer_id = match editor.current_buffer_id() {
        Some(id) => id,
        None => return Err(ExCommandError::InvalidCommand("No buffer to set fold method for".to_string())),
    };
    
    // Get the fold manager
    let fold_manager = unsafe {
        match &mut FOLD_MANAGER {
            Some(manager) => manager,
            None => {
                init_fold_manager();
                match &mut FOLD_MANAGER {
                    Some(manager) => manager,
                    None => return Err(ExCommandError::Other("Failed to initialize fold manager".to_string())),
                }
            }
        }
    };
    
    // Get the method from the command arguments
    let method_str = cmd.args_str();
    
    if method_str.is_empty() {
        // Display the current method
        let method = fold_manager.get_method(buffer_id);
        let method_str = match method {
            FoldMethod::Manual => "manual",
            FoldMethod::Indent => "indent",
            FoldMethod::Expr => "expr",
            FoldMethod::Syntax => "syntax",
            FoldMethod::Marker => "marker",
            FoldMethod::Diff => "diff",
        };
        
        println!("foldmethod={}", method_str);
        return Ok(());
    }
    
    // Parse the method
    let method = match method_str.as_str() {
        "manual" => FoldMethod::Manual,
        "indent" => FoldMethod::Indent,
        "expr" => FoldMethod::Expr,
        "syntax" => FoldMethod::Syntax,
        "marker" => FoldMethod::Marker,
        "diff" => FoldMethod::Diff,
        _ => return Err(ExCommandError::InvalidArgument(format!("Invalid fold method: {}", method_str))),
    };
    
    // Set the method
    fold_manager.set_method(buffer_id, method);
    
    // Create folds based on the method
    match fold_manager.create_folds_for_buffer(buffer_id, editor) {
        Ok(_) => {
            println!("foldmethod={}", method_str);
            Ok(())
        },
        Err(err) => Err(err),
    }
}

/// Handle the :foldcolumn command
fn handle_foldcolumn(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the fold manager
    let fold_manager = unsafe {
        match &mut FOLD_MANAGER {
            Some(manager) => manager,
            None => {
                init_fold_manager();
                match &mut FOLD_MANAGER {
                    Some(manager) => manager,
                    None => return Err(ExCommandError::Other("Failed to initialize fold manager".to_string())),
                }
            }
        }
    };
    
    // Get the width from the command arguments
    let width_str = cmd.args_str();
    
    if width_str.is_empty() {
        // Display the current width
        println!("foldcolumn={}", fold_manager.fold_column);
        return Ok(());
    }
    
    // Parse the width
    let width = match width_str.parse::<usize>() {
        Ok(w) => w,
        Err(_) => return Err(ExCommandError::InvalidArgument(format!("Invalid fold column width: {}", width_str))),
    };
    
    // Set the width
    fold_manager.fold_column = width;
    
    println!("foldcolumn={}", width);
    Ok(())
}

/// Handle the :foldminlines command
fn handle_foldminlines(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the fold manager
    let fold_manager = unsafe {
        match &mut FOLD_MANAGER {
            Some(manager) => manager,
            None => {
                init_fold_manager();
                match &mut FOLD_MANAGER {
                    Some(manager) => manager,
                    None => return Err(ExCommandError::Other("Failed to initialize fold manager".to_string())),
                }
            }
        }
    };
    
    // Get the min width from the command arguments
    let min_width_str = cmd.args_str();
    
    if min_width_str.is_empty() {
        // Display the current min width
        println!("foldminwidth={}", fold_manager.fold_min_width);
        return Ok(());
    }
    
    // Parse the min width
    let min_width = match min_width_str.parse::<usize>() {
        Ok(w) => w,
        Err(_) => return Err(ExCommandError::InvalidArgument(format!("Invalid fold min width: {}", min_width_str))),
    };
    
    // Set the min width
    fold_manager.fold_min_width = min_width;
    
    println!("foldminwidth={}", min_width);
    Ok(())
}

/// Handle the :foldtext command
fn handle_foldtext(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the fold manager
    let fold_manager = unsafe {
        match &mut FOLD_MANAGER {
            Some(manager) => manager,
            None => {
                init_fold_manager();
                match &mut FOLD_MANAGER {
                    Some(manager) => manager,
                    None => return Err(ExCommandError::Other("Failed to initialize fold manager".to_string())),
                }
            }
        }
    };
    
    // Get the text function from the command arguments
    let text_func = cmd.args_str();
    
    if text_func.is_empty() {
        // Display the current text function
        if let Some(func) = &fold_manager.fold_text {
            println!("foldtext={}", func);
        } else {
            println!("foldtext=");
        }
        return Ok(());
    }
    
    // Set the text function
    fold_manager.fold_text = Some(text_func.to_string());
    
    println!("foldtext={}", text_func);
    Ok(())
}

/// Handle the :foldlevel command
fn handle_foldlevel(_cmd: &ExCommand) -> ExCommandResult<()> {
    // This is a stub implementation
    println!("Fold level commands not fully implemented yet");
    Ok(())
}