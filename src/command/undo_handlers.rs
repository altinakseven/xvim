//! Undo/redo command handlers
//!
//! This module implements handlers for undo/redo commands.

use crate::command::{ExCommand, ExCommandError, ExCommandResult, ExCommandRegistry};
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::collections::HashMap;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;
use std::io::Write;

/// Undo manager
#[derive(Debug)]
pub struct UndoManager {
    /// Undo tree for each buffer
    pub undo_trees: HashMap<usize, UndoTree>,
    /// Auto-save settings
    pub auto_save: bool,
    /// Auto-save interval (in seconds)
    pub auto_save_interval: u64,
    /// Last auto-save time
    pub last_auto_save: std::time::SystemTime,
    /// Persistent undo
    pub persistent_undo: bool,
    /// Persistent undo directory
    pub persistent_undo_dir: Option<std::path::PathBuf>,
    /// Maximum number of changes to remember
    pub max_changes: usize,
}

/// Undo tree
#[derive(Debug)]
pub struct UndoTree {
    /// Undo branches
    pub branches: Vec<UndoBranch>,
    /// Current branch index
    pub current_branch: usize,
    /// Current position in the current branch
    pub current_position: usize,
    /// Last save position
    pub last_save_position: Option<(usize, usize)>,
}

/// Undo branch
#[derive(Debug)]
pub struct UndoBranch {
    /// Changes in this branch
    pub changes: Vec<UndoChange>,
    /// Parent branch index
    pub parent: Option<usize>,
    /// Parent position
    pub parent_position: Option<usize>,
    /// Branch name
    pub name: Option<String>,
}

/// Undo change
#[derive(Debug, Clone)]
pub struct UndoChange {
    /// Change type
    pub change_type: UndoChangeType,
    /// Change position
    pub position: crate::cursor::CursorPosition,
    /// Change text
    pub text: String,
    /// Timestamp
    pub timestamp: std::time::SystemTime,
    /// Change description
    pub description: Option<String>,
}

/// Undo change type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UndoChangeType {
    /// Insert text
    Insert,
    /// Delete text
    Delete,
    /// Replace text
    Replace,
    /// Begin group
    BeginGroup,
    /// End group
    EndGroup,
}

impl UndoManager {
    /// Create a new undo manager
    pub fn new() -> Self {
        Self {
            undo_trees: HashMap::new(),
            auto_save: false,
            auto_save_interval: 300, // 5 minutes
            last_auto_save: std::time::SystemTime::now(),
            persistent_undo: false,
            persistent_undo_dir: None,
            max_changes: 1000,
        }
    }

    /// Get the undo tree for a buffer
    pub fn get_undo_tree(&self, buffer_id: usize) -> Option<&UndoTree> {
        self.undo_trees.get(&buffer_id)
    }

    /// Get a mutable reference to the undo tree for a buffer
    pub fn get_undo_tree_mut(&mut self, buffer_id: usize) -> Option<&mut UndoTree> {
        self.undo_trees.get_mut(&buffer_id)
    }

    /// Create an undo tree for a buffer
    pub fn create_undo_tree(&mut self, buffer_id: usize) -> &mut UndoTree {
        self.undo_trees.entry(buffer_id).or_insert_with(|| UndoTree {
            branches: vec![UndoBranch {
                changes: Vec::new(),
                parent: None,
                parent_position: None,
                name: None,
            }],
            current_branch: 0,
            current_position: 0,
            last_save_position: None,
        })
    }

    /// Add a change to the undo tree
    pub fn add_change(&mut self, buffer_id: usize, change: UndoChange) -> ExCommandResult<()> {
        // Get the undo tree for the buffer
        let undo_tree = self.create_undo_tree(buffer_id);
        
        // Get the current branch
        let branch = &mut undo_tree.branches[undo_tree.current_branch];
        
        // If we're not at the end of the branch, create a new branch
        if undo_tree.current_position < branch.changes.len() {
            // Create a new branch
            let new_branch = UndoBranch {
                changes: branch.changes[..undo_tree.current_position].to_vec(),
                parent: Some(undo_tree.current_branch),
                parent_position: Some(undo_tree.current_position),
                name: None,
            };
            
            // Add the new branch
            undo_tree.branches.push(new_branch);
            
            // Update the current branch
            undo_tree.current_branch = undo_tree.branches.len() - 1;
            
            // Get the new current branch
            let branch = &mut undo_tree.branches[undo_tree.current_branch];
            
            // Add the change
            branch.changes.push(change);
            
            // Update the current position
            undo_tree.current_position = branch.changes.len();
        } else {
            // Add the change to the current branch
            branch.changes.push(change);
            
            // Update the current position
            undo_tree.current_position = branch.changes.len();
        }
        
        // Check if we need to auto-save
        if self.auto_save {
            let now = std::time::SystemTime::now();
            
            if now.duration_since(self.last_auto_save).unwrap_or_default().as_secs() >= self.auto_save_interval {
                // Auto-save
                self.last_auto_save = now;
                
                // TODO: Implement auto-save
            }
        }
        
        // Check if we need to limit the number of changes
        if self.max_changes > 0 {
            // Get the total number of changes across all branches
            let total_changes: usize = undo_tree.branches.iter().map(|b| b.changes.len()).sum();
            
            if total_changes > self.max_changes {
                // TODO: Implement change limiting
            }
        }
        
        Ok(())
    }

    /// Undo a change
    pub fn undo(&mut self, buffer_id: usize) -> ExCommandResult<Option<UndoChange>> {
        // Get the undo tree for the buffer
        let undo_tree = match self.get_undo_tree_mut(buffer_id) {
            Some(tree) => tree,
            None => return Ok(None),
        };
        
        // Get the current branch
        let branch = &undo_tree.branches[undo_tree.current_branch];
        
        // Check if we can undo
        if undo_tree.current_position == 0 {
            // Check if we have a parent branch
            if let (Some(parent_branch), Some(parent_position)) = (branch.parent, branch.parent_position) {
                // Switch to the parent branch
                undo_tree.current_branch = parent_branch;
                undo_tree.current_position = parent_position;
                
                // Get the change we're undoing
                let branch = &undo_tree.branches[undo_tree.current_branch];
                
                if undo_tree.current_position > 0 {
                    let change = branch.changes[undo_tree.current_position - 1].clone();
                    
                    // Update the current position
                    undo_tree.current_position -= 1;
                    
                    return Ok(Some(change));
                } else {
                    return Ok(None);
                }
            } else {
                return Ok(None);
            }
        }
        
        // Get the change we're undoing
        let change = branch.changes[undo_tree.current_position - 1].clone();
        
        // Update the current position
        undo_tree.current_position -= 1;
        
        Ok(Some(change))
    }

    /// Redo a change
    pub fn redo(&mut self, buffer_id: usize) -> ExCommandResult<Option<UndoChange>> {
        // Get the undo tree for the buffer
        let undo_tree = match self.get_undo_tree_mut(buffer_id) {
            Some(tree) => tree,
            None => return Ok(None),
        };
        
        // Get the current branch
        let branch = &undo_tree.branches[undo_tree.current_branch];
        
        // Check if we can redo
        if undo_tree.current_position >= branch.changes.len() {
            // Check if there's a branch that has this branch as its parent
            let child_branch = undo_tree.branches.iter().position(|b| {
                b.parent == Some(undo_tree.current_branch) && b.parent_position == Some(undo_tree.current_position)
            });
            
            if let Some(child_branch) = child_branch {
                // Switch to the child branch
                undo_tree.current_branch = child_branch;
                undo_tree.current_position = 0;
                
                // Get the change we're redoing
                let branch = &undo_tree.branches[undo_tree.current_branch];
                
                if !branch.changes.is_empty() {
                    let change = branch.changes[undo_tree.current_position].clone();
                    
                    // Update the current position
                    undo_tree.current_position += 1;
                    
                    return Ok(Some(change));
                } else {
                    return Ok(None);
                }
            } else {
                return Ok(None);
            }
        }
        
        // Get the change we're redoing
        let change = branch.changes[undo_tree.current_position].clone();
        
        // Update the current position
        undo_tree.current_position += 1;
        
        Ok(Some(change))
    }

    /// Begin an undo group
    pub fn begin_group(&mut self, buffer_id: usize, description: Option<String>) -> ExCommandResult<()> {
        // Create a begin group change
        let change = UndoChange {
            change_type: UndoChangeType::BeginGroup,
            position: crate::cursor::CursorPosition::new(0, 0),
            text: String::new(),
            timestamp: std::time::SystemTime::now(),
            description,
        };
        
        // Add the change
        self.add_change(buffer_id, change)
    }

    /// End an undo group
    pub fn end_group(&mut self, buffer_id: usize) -> ExCommandResult<()> {
        // Create an end group change
        let change = UndoChange {
            change_type: UndoChangeType::EndGroup,
            position: crate::cursor::CursorPosition::new(0, 0),
            text: String::new(),
            timestamp: std::time::SystemTime::now(),
            description: None,
        };
        
        // Add the change
        self.add_change(buffer_id, change)
    }

    /// Mark the current position as the last save position
    pub fn mark_save_position(&mut self, buffer_id: usize) -> ExCommandResult<()> {
        // Get the undo tree for the buffer
        let undo_tree = match self.get_undo_tree_mut(buffer_id) {
            Some(tree) => tree,
            None => return Ok(()),
        };
        
        // Mark the current position as the last save position
        undo_tree.last_save_position = Some((undo_tree.current_branch, undo_tree.current_position));
        
        Ok(())
    }

    /// Check if the buffer has been modified since the last save
    pub fn is_modified(&self, buffer_id: usize) -> bool {
        // Get the undo tree for the buffer
        let undo_tree = match self.get_undo_tree(buffer_id) {
            Some(tree) => tree,
            None => return false,
        };
        
        // Check if the current position is the last save position
        if let Some((branch, position)) = undo_tree.last_save_position {
            branch != undo_tree.current_branch || position != undo_tree.current_position
        } else {
            // If there's no last save position, the buffer is modified if there are any changes
            undo_tree.branches[undo_tree.current_branch].changes.len() > 0
        }
    }

    /// Get the number of changes that can be undone
    pub fn undo_count(&self, buffer_id: usize) -> usize {
        // Get the undo tree for the buffer
        let undo_tree = match self.get_undo_tree(buffer_id) {
            Some(tree) => tree,
            None => return 0,
        };
        
        // Count the number of changes that can be undone
        let mut count = undo_tree.current_position;
        let mut branch_idx = undo_tree.current_branch;
        
        while count == 0 {
            // Get the branch
            let branch = &undo_tree.branches[branch_idx];
            
            // Check if we have a parent branch
            if let (Some(parent_branch), Some(parent_position)) = (branch.parent, branch.parent_position) {
                // Switch to the parent branch
                branch_idx = parent_branch;
                count = parent_position;
            } else {
                break;
            }
        }
        
        count
    }

    /// Get the number of changes that can be redone
    pub fn redo_count(&self, buffer_id: usize) -> usize {
        // Get the undo tree for the buffer
        let undo_tree = match self.get_undo_tree(buffer_id) {
            Some(tree) => tree,
            None => return 0,
        };
        
        // Count the number of changes that can be redone
        let mut count = undo_tree.branches[undo_tree.current_branch].changes.len() - undo_tree.current_position;
        
        // Check if there are branches that have this branch as their parent
        for branch in &undo_tree.branches {
            if branch.parent == Some(undo_tree.current_branch) && branch.parent_position == Some(undo_tree.current_position) {
                count += branch.changes.len();
            }
        }
        
        count
    }

    /// Save the undo history to a file
    pub fn save_history(&self, buffer_id: usize, path: &std::path::Path) -> ExCommandResult<()> {
        // Get the undo tree for the buffer
        let undo_tree = match self.get_undo_tree(buffer_id) {
            Some(tree) => tree,
            None => return Ok(()),
        };
        
        // Serialize the undo tree
        let serialized = match bincode::serialize(undo_tree) {
            Ok(data) => data,
            Err(err) => return Err(ExCommandError::Other(format!("Failed to serialize undo tree: {}", err))),
        };
        
        // Write the serialized data to the file
        match std::fs::write(path, serialized) {
            Ok(_) => Ok(()),
            Err(err) => Err(ExCommandError::Other(format!("Failed to write undo history: {}", err))),
        }
    }

    /// Load the undo history from a file
    pub fn load_history(&mut self, buffer_id: usize, path: &std::path::Path) -> ExCommandResult<()> {
        // Read the serialized data from the file
        let serialized = match std::fs::read(path) {
            Ok(data) => data,
            Err(err) => return Err(ExCommandError::Other(format!("Failed to read undo history: {}", err))),
        };
        
        // Deserialize the undo tree
        let undo_tree: UndoTree = match bincode::deserialize(&serialized) {
            Ok(tree) => tree,
            Err(err) => return Err(ExCommandError::Other(format!("Failed to deserialize undo tree: {}", err))),
        };
        
        // Set the undo tree for the buffer
        self.undo_trees.insert(buffer_id, undo_tree);
        
        Ok(())
    }

    /// Clear the undo history for a buffer
    pub fn clear_history(&mut self, buffer_id: usize) -> ExCommandResult<()> {
        // Remove the undo tree for the buffer
        self.undo_trees.remove(&buffer_id);
        
        Ok(())
    }

    /// Set the auto-save settings
    pub fn set_auto_save(&mut self, enabled: bool, interval: Option<u64>) -> ExCommandResult<()> {
        // Set the auto-save flag
        self.auto_save = enabled;
        
        // Set the auto-save interval if provided
        if let Some(interval) = interval {
            self.auto_save_interval = interval;
        }
        
        Ok(())
    }

    /// Set the persistent undo settings
    pub fn set_persistent_undo(&mut self, enabled: bool, dir: Option<std::path::PathBuf>) -> ExCommandResult<()> {
        // Set the persistent undo flag
        self.persistent_undo = enabled;
        
        // Set the persistent undo directory if provided
        if let Some(dir) = dir {
            self.persistent_undo_dir = Some(dir);
        }
        
        Ok(())
    }

    /// Set the maximum number of changes to remember
    pub fn set_max_changes(&mut self, max_changes: usize) -> ExCommandResult<()> {
        // Set the maximum number of changes
        self.max_changes = max_changes;
        
        Ok(())
    }
}

// Global undo manager
static mut UNDO_MANAGER: Option<UndoManager> = None;

/// Initialize the undo manager
pub fn init_undo_manager() {
    unsafe {
        if UNDO_MANAGER.is_none() {
            UNDO_MANAGER = Some(UndoManager::new());
        }
    }
}

/// Register undo/redo command handlers
pub fn register_undo_handlers(registry: &mut ExCommandRegistry) {
    // Initialize the undo manager
    init_undo_manager();
    
    // Register undo/redo commands
    registry.register("undo", handle_undo);
    registry.register("u", handle_undo);
    registry.register("redo", handle_redo);
    registry.register("red", handle_redo);
    registry.register("undolist", handle_undolist);
    registry.register("undol", handle_undolist);
    registry.register("earlier", handle_earlier);
    registry.register("later", handle_later);
    registry.register("undojoin", handle_undojoin);
    registry.register("undoj", handle_undojoin);
}

/// Handle the :undo command
fn handle_undo(_cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match crate::command::handlers::EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Get the undo manager
    let undo_manager = unsafe {
        match &mut UNDO_MANAGER {
            Some(manager) => manager,
            None => {
                init_undo_manager();
                match &mut UNDO_MANAGER {
                    Some(manager) => manager,
                    None => return Err(ExCommandError::Other("Failed to initialize undo manager".to_string())),
                }
            }
        }
    };
    
    // Get the current buffer ID
    let buffer_id = match editor.current_buffer_id() {
        Some(id) => id,
        None => return Err(ExCommandError::InvalidCommand("No buffer to undo changes in".to_string())),
    };
    
    // Undo a change
    match undo_manager.undo(buffer_id)? {
        Some(change) => {
            // Apply the undo change
            match change.change_type {
                UndoChangeType::Insert => {
                    // For an insert change, we need to delete the inserted text
                    let buffer = editor.get_buffer_manager_mut().get_buffer_mut(buffer_id)?;
                    
                    // Calculate the range to delete
                    let start_idx = buffer.position_to_char_idx(change.position.line, change.position.column)?;
                    let end_idx = start_idx + change.text.len();
                    
                    // Delete the text
                    buffer.delete(start_idx, end_idx)?;
                    
                    // Set the cursor position
                    editor.get_cursor_manager_mut().set_position(change.position);
                    
                    println!("Undo: deleted inserted text");
                },
                UndoChangeType::Delete => {
                    // For a delete change, we need to insert the deleted text
                    let buffer = editor.get_buffer_manager_mut().get_buffer_mut(buffer_id)?;
                    
                    // Calculate the position to insert at
                    let insert_idx = buffer.position_to_char_idx(change.position.line, change.position.column)?;
                    
                    // Insert the text
                    buffer.insert(insert_idx, &change.text)?;
                    
                    // Set the cursor position
                    editor.get_cursor_manager_mut().set_position(change.position);
                    
                    println!("Undo: inserted deleted text");
                },
                UndoChangeType::Replace => {
                    // For a replace change, we need to replace the new text with the old text
                    let buffer = editor.get_buffer_manager_mut().get_buffer_mut(buffer_id)?;
                    
                    // Calculate the range to replace
                    let start_idx = buffer.position_to_char_idx(change.position.line, change.position.column)?;
                    let end_idx = start_idx + change.text.len();
                    
                    // Delete the new text
                    buffer.delete(start_idx, end_idx)?;
                    
                    // Insert the old text
                    buffer.insert(start_idx, &change.text)?;
                    
                    // Set the cursor position
                    editor.get_cursor_manager_mut().set_position(change.position);
                    
                    println!("Undo: replaced text");
                },
                UndoChangeType::BeginGroup => {
                    // For a begin group change, we need to undo all changes in the group
                    // This is handled by the undo manager
                    println!("Undo: begin group");
                },
                UndoChangeType::EndGroup => {
                    // For an end group change, we need to undo all changes in the group
                    // This is handled by the undo manager
                    println!("Undo: end group");
                },
            }
            
            Ok(())
        },
        None => {
            println!("Already at oldest change");
            Ok(())
        }
    }
}

/// Handle the :redo command
fn handle_redo(_cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match crate::command::handlers::EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Get the undo manager
    let undo_manager = unsafe {
        match &mut UNDO_MANAGER {
            Some(manager) => manager,
            None => {
                init_undo_manager();
                match &mut UNDO_MANAGER {
                    Some(manager) => manager,
                    None => return Err(ExCommandError::Other("Failed to initialize undo manager".to_string())),
                }
            }
        }
    };
    
    // Get the current buffer ID
    let buffer_id = match editor.current_buffer_id() {
        Some(id) => id,
        None => return Err(ExCommandError::InvalidCommand("No buffer to redo changes in".to_string())),
    };
    
    // Redo a change
    match undo_manager.redo(buffer_id)? {
        Some(change) => {
            // Apply the redo change
            match change.change_type {
                UndoChangeType::Insert => {
                    // For an insert change, we need to insert the text
                    let buffer = editor.get_buffer_manager_mut().get_buffer_mut(buffer_id)?;
                    
                    // Calculate the position to insert at
                    let insert_idx = buffer.position_to_char_idx(change.position.line, change.position.column)?;
                    
                    // Insert the text
                    buffer.insert(insert_idx, &change.text)?;
                    
                    // Set the cursor position
                    let new_pos = buffer.char_idx_to_position(insert_idx + change.text.len())?;
                    editor.get_cursor_manager_mut().set_position(new_pos);
                    
                    println!("Redo: inserted text");
                },
                UndoChangeType::Delete => {
                    // For a delete change, we need to delete the text
                    let buffer = editor.get_buffer_manager_mut().get_buffer_mut(buffer_id)?;
                    
                    // Calculate the range to delete
                    let start_idx = buffer.position_to_char_idx(change.position.line, change.position.column)?;
                    let end_idx = start_idx + change.text.len();
                    
                    // Delete the text
                    buffer.delete(start_idx, end_idx)?;
                    
                    // Set the cursor position
                    editor.get_cursor_manager_mut().set_position(change.position);
                    
                    println!("Redo: deleted text");
                },
                UndoChangeType::Replace => {
                    // For a replace change, we need to replace the old text with the new text
                    let buffer = editor.get_buffer_manager_mut().get_buffer_mut(buffer_id)?;
                    
                    // Calculate the range to replace
                    let start_idx = buffer.position_to_char_idx(change.position.line, change.position.column)?;
                    let end_idx = start_idx + change.text.len();
                    
                    // Delete the old text
                    buffer.delete(start_idx, end_idx)?;
                    
                    // Insert the new text
                    buffer.insert(start_idx, &change.text)?;
                    
                    // Set the cursor position
                    let new_pos = buffer.char_idx_to_position(start_idx + change.text.len())?;
                    editor.get_cursor_manager_mut().set_position(new_pos);
                    
                    println!("Redo: replaced text");
                },
                UndoChangeType::BeginGroup => {
                    // For a begin group change, we need to redo all changes in the group
                    // This is handled by the undo manager
                    println!("Redo: begin group");
                },
                UndoChangeType::EndGroup => {
                    // For an end group change, we need to redo all changes in the group
                    // This is handled by the undo manager
                    println!("Redo: end group");
                },
            }
            
            Ok(())
        },
        None => {
            println!("Already at newest change");
            Ok(())
        }
    }
}

/// Handle the :undolist command
fn handle_undolist(_cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match crate::command::handlers::EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Get the undo manager
    let undo_manager = unsafe {
        match &UNDO_MANAGER {
            Some(manager) => manager,
            None => {
                init_undo_manager();
                match &UNDO_MANAGER {
                    Some(manager) => manager,
                    None => return Err(ExCommandError::Other("Failed to initialize undo manager".to_string())),
                }
            }
        }
    };
    
    // Get the current buffer ID
    let buffer_id = match editor.current_buffer_id() {
        Some(id) => id,
        None => return Err(ExCommandError::InvalidCommand("No buffer to list undo history for".to_string())),
    };
    
    // Get the undo tree for the buffer
    let undo_tree = match undo_manager.get_undo_tree(buffer_id) {
        Some(tree) => tree,
        None => {
            println!("No undo history");
            return Ok(());
        }
    };
    
    // Display the undo history
    println!("Undo history:");
    
    // Display the current branch
    let branch = &undo_tree.branches[undo_tree.current_branch];
    
    println!("Current branch: {}", undo_tree.current_branch);
    println!("Current position: {}", undo_tree.current_position);
    
    // Display the changes in the current branch
    for (i, change) in branch.changes.iter().enumerate() {
        let current_marker = if i == undo_tree.current_position { ">" } else { " " };
        
        let change_type = match change.change_type {
            UndoChangeType::Insert => "insert",
            UndoChangeType::Delete => "delete",
            UndoChangeType::Replace => "replace",
            UndoChangeType::BeginGroup => "begin group",
            UndoChangeType::EndGroup => "end group",
        };
        
        let description = match &change.description {
            Some(desc) => desc,
            None => "",
        };
        
        println!("{} {:3} {:10} ({:3},{:3}) {} {}",
            current_marker,
            i,
            change_type,
            change.position.line,
            change.position.column,
            format_timestamp(change.timestamp),
            description
        );
    }
    
    // Display the number of changes that can be undone and redone
    println!("Changes that can be undone: {}", undo_manager.undo_count(buffer_id));
    println!("Changes that can be redone: {}", undo_manager.redo_count(buffer_id));
    
    Ok(())
}

/// Handle the :earlier command
fn handle_earlier(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match crate::command::handlers::EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Get the undo manager
    let undo_manager = unsafe {
        match &mut UNDO_MANAGER {
            Some(manager) => manager,
            None => {
                init_undo_manager();
                match &mut UNDO_MANAGER {
                    Some(manager) => manager,
                    None => return Err(ExCommandError::Other("Failed to initialize undo manager".to_string())),
                }
            }
        }
    };
    
    // Get the current buffer ID
    let buffer_id = match editor.current_buffer_id() {
        Some(id) => id,
        None => return Err(ExCommandError::InvalidCommand("No buffer to go to earlier state".to_string())),
    };
    
    // Parse the command arguments
    let args = cmd.args_str();
    
    if args.is_empty() {
        // If no arguments, go back one change
        handle_undo(cmd)
    } else {
        // Parse the argument
        let mut count = 0;
        let mut unit = "";
        
        // Check if the argument is a number
        if let Ok(n) = args.parse::<usize>() {
            // Go back n changes
            count = n;
            unit = "changes";
        } else {
            // Parse the argument as a time specification
            let mut chars = args.chars();
            let mut num_str = String::new();
            
            // Parse the number
            while let Some(c) = chars.next() {
                if c.is_digit(10) {
                    num_str.push(c);
                } else {
                    // Parse the unit
                    unit = &args[num_str.len()..];
                    break;
                }
            }
            
            // Parse the number
            count = match num_str.parse::<usize>() {
                Ok(n) => n,
                Err(_) => return Err(ExCommandError::InvalidArgument(format!("Invalid count: {}", num_str))),
            };
        }
        
        // Go back the specified number of units
        match unit {
            "changes" | "" => {
                // Go back count changes
                for _ in 0..count {
                    if undo_manager.undo(buffer_id)?.is_none() {
                        break;
                    }
                }
                
                println!("Went back {} changes", count);
                Ok(())
            },
            "s" | "sec" | "secs" | "second" | "seconds" => {
                // Go back count seconds
                let now = std::time::SystemTime::now();
                let target_time = now - std::time::Duration::from_secs(count as u64);
                
                // Get the undo tree for the buffer
                let undo_tree = match undo_manager.get_undo_tree(buffer_id) {
                    Some(tree) => tree,
                    None => {
                        println!("No undo history");
                        return Ok(());
                    }
                };
                
                // Get the current branch
                let branch = &undo_tree.branches[undo_tree.current_branch];
                
                // Find the first change that is older than the target time
                let mut i = undo_tree.current_position;
                
                while i > 0 {
                    i -= 1;
                    
                    let change = &branch.changes[i];
                    
                    if change.timestamp <= target_time {
                        // Found a change that is older than the target time
                        break;
                    }
                }
                
                // Go back to that change
                while undo_tree.current_position > i {
                    if undo_manager.undo(buffer_id)?.is_none() {
                        break;
                    }
                }
                
                println!("Went back {} seconds", count);
                Ok(())
            },
            "m" | "min" | "mins" | "minute" | "minutes" => {
                // Go back count minutes
                let now = std::time::SystemTime::now();
                let target_time = now - std::time::Duration::from_secs(count as u64 * 60);
                
                // Get the undo tree for the buffer
                let undo_tree = match undo_manager.get_undo_tree(buffer_id) {
                    Some(tree) => tree,
                    None => {
                        println!("No undo history");
                        return Ok(());
                    }
                };
                
                // Get the current branch
                let branch = &undo_tree.branches[undo_tree.current_branch];
                
                // Find the first change that is older than the target time
                let mut i = undo_tree.current_position;
                
                while i > 0 {
                    i -= 1;
                    
                    let change = &branch.changes[i];
                    
                    if change.timestamp <= target_time {
                        // Found a change that is older than the target time
                        break;
                    }
                }
                
                // Go back to that change
                while undo_tree.current_position > i {
                    if undo_manager.undo(buffer_id)?.is_none() {
                        break;
                    }
                }
                
                println!("Went back {} minutes", count);
                Ok(())
            },
            "h" | "hr" | "hrs" | "hour" | "hours" => {
                // Go back count hours
                let now = std::time::SystemTime::now();
                let target_time = now - std::time::Duration::from_secs(count as u64 * 60 * 60);
                
                // Get the undo tree for the buffer
                let undo_tree = match undo_manager.get_undo_tree(buffer_id) {
                    Some(tree) => tree,
                    None => {
                        println!("No undo history");
                        return Ok(());
                    }
                };
                
                // Get the current branch
                let branch = &undo_tree.branches[undo_tree.current_branch];
                
                // Find the first change that is older than the target time
                let mut i = undo_tree.current_position;
                
                while i > 0 {
                    i -= 1;
                    
                    let change = &branch.changes[i];
                    
                    if change.timestamp <= target_time {
                        // Found a change that is older than the target time
                        break;
                    }
                }
                
                // Go back to that change
                while undo_tree.current_position > i {
                    if undo_manager.undo(buffer_id)?.is_none() {
                        break;
                    }
                }
                
                println!("Went back {} hours", count);
                Ok(())
            },
            "d" | "day" | "days" => {
                // Go back count days
                let now = std::time::SystemTime::now();
                let target_time = now - std::time::Duration::from_secs(count as u64 * 60 * 60 * 24);
                
                // Get the undo tree for the buffer
                let undo_tree = match undo_manager.get_undo_tree(buffer_id) {
                    Some(tree) => tree,
                    None => {
                        println!("No undo history");
                        return Ok(());
                    }
                };
                
                // Get the current branch
                let branch = &undo_tree.branches[undo_tree.current_branch];
                
                // Find the first change that is older than the target time
                let mut i = undo_tree.current_position;
                
                while i > 0 {
                    i -= 1;
                    
                    let change = &branch.changes[i];
                    
                    if change.timestamp <= target_time {
                        // Found a change that is older than the target time
                        break;
                    }
                }
                
                // Go back to that change
                while undo_tree.current_position > i {
                    if undo_manager.undo(buffer_id)?.is_none() {
                        break;
                    }
                }
                
                println!("Went back {} days", count);
                Ok(())
            },
            "f" | "file" | "files" => {
                // Go back count file writes
                // Get the undo tree for the buffer
                let undo_tree = match undo_manager.get_undo_tree(buffer_id) {
                    Some(tree) => tree,
                    None => {
                        println!("No undo history");
                        return Ok(());
                    }
                };
                
                // Check if there's a last save position
                if let Some((branch, position)) = undo_tree.last_save_position {
                    // Go back to the last save position
                    if branch == undo_tree.current_branch {
                        // We're in the same branch
                        while undo_tree.current_position > position {
                            if undo_manager.undo(buffer_id)?.is_none() {
                                break;
                            }
                        }
                    } else {
                        // We need to go back to a different branch
                        // This is more complex and not implemented yet
                        return Err(ExCommandError::Other("Going back to a different branch is not implemented yet".to_string()));
                    }
                    
                    println!("Went back to last file write");
                    Ok(())
                } else {
                    println!("No file writes in undo history");
                    Ok(())
                }
            },
            _ => Err(ExCommandError::InvalidArgument(format!("Invalid unit: {}", unit))),
        }
    }
}

/// Handle the :later command
fn handle_later(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match crate::command::handlers::EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Get the undo manager
    let undo_manager = unsafe {
        match &mut UNDO_MANAGER {
            Some(manager) => manager,
            None => {
                init_undo_manager();
                match &mut UNDO_MANAGER {
                    Some(manager) => manager,
                    None => return Err(ExCommandError::Other("Failed to initialize undo manager".to_string())),
                }
            }
        }
    };
    
    // Get the current buffer ID
    let buffer_id = match editor.current_buffer_id() {
        Some(id) => id,
        None => return Err(ExCommandError::InvalidCommand("No buffer to go to later state".to_string())),
    };
    
    // Parse the command arguments
    let args = cmd.args_str();
    
    if args.is_empty() {
        // If no arguments, go forward one change
        handle_redo(cmd)
    } else {
        // Parse the argument
        let mut count = 0;
        let mut unit = "";
        
        // Check if the argument is a number
        if let Ok(n) = args.parse::<usize>() {
            // Go forward n changes
            count = n;
            unit = "changes";
        } else {
            // Parse the argument as a time specification
            let mut chars = args.chars();
            let mut num_str = String::new();
            
            // Parse the number
            while let Some(c) = chars.next() {
                if c.is_digit(10) {
                    num_str.push(c);
                } else {
                    // Parse the unit
                    unit = &args[num_str.len()..];
                    break;
                }
            }
            
            // Parse the number
            count = match num_str.parse::<usize>() {
                Ok(n) => n,
                Err(_) => return Err(ExCommandError::InvalidArgument(format!("Invalid count: {}", num_str))),
            };
        }
        
        // Go forward the specified number of units
        match unit {
            "changes" | "" => {
                // Go forward count changes
                for _ in 0..count {
                    if undo_manager.redo(buffer_id)?.is_none() {
                        break;
                    }
                }
                
                println!("Went forward {} changes", count);
                Ok(())
            },
            "s" | "sec" | "secs" | "second" | "seconds" => {
                // Go forward count seconds
                let now = std::time::SystemTime::now();
                let target_time = now + std::time::Duration::from_secs(count as u64);
                
                // Get the undo tree for the buffer
                let undo_tree = match undo_manager.get_undo_tree(buffer_id) {
                    Some(tree) => tree,
                    None => {
                        println!("No undo history");
                        return Ok(());
                    }
                };
                
                // Get the current branch
                let branch = &undo_tree.branches[undo_tree.current_branch];
                
                // Find the first change that is newer than the target time
                let mut i = undo_tree.current_position;
                
                while i < branch.changes.len() {
                    let change = &branch.changes[i];
                    
                    if change.timestamp >= target_time {
                        // Found a change that is newer than the target time
                        break;
                    }
                    
                    i += 1;
                }
                
                // Go forward to that change
                while undo_tree.current_position < i {
                    if undo_manager.redo(buffer_id)?.is_none() {
                        break;
                    }
                }
                
                println!("Went forward {} seconds", count);
                Ok(())
            },
            "m" | "min" | "mins" | "minute" | "minutes" => {
                // Go forward count minutes
                let now = std::time::SystemTime::now();
                let target_time = now + std::time::Duration::from_secs(count as u64 * 60);
                
                // Get the undo tree for the buffer
                let undo_tree = match undo_manager.get_undo_tree(buffer_id) {
                    Some(tree) => tree,
                    None => {
                        println!("No undo history");
                        return Ok(());
                    }
                };
                
                // Get the current branch
                let branch = &undo_tree.branches[undo_tree.current_branch];
                
                // Find the first change that is newer than the target time
                let mut i = undo_tree.current_position;
                
                while i < branch.changes.len() {
                    let change = &branch.changes[i];
                    
                    if change.timestamp >= target_time {
                        // Found a change that is newer than the target time
                        break;
                    }
                    
                    i += 1;
                }
                
                // Go forward to that change
                while undo_tree.current_position < i {
                    if undo_manager.redo(buffer_id)?.is_none() {
                        break;
                    }
                }
                
                println!("Went forward {} minutes", count);
                Ok(())
            },
            "h" | "hr" | "hrs" | "hour" | "hours" => {
                // Go forward count hours
                let now = std::time::SystemTime::now();
                let target_time = now + std::time::Duration::from_secs(count as u64 * 60 * 60);
                
                // Get the undo tree for the buffer
                let undo_tree = match undo_manager.get_undo_tree(buffer_id) {
                    Some(tree) => tree,
                    None => {
                        println!("No undo history");
                        return Ok(());
                    }
                };
                
                // Get the current branch
                let branch = &undo_tree.branches[undo_tree.current_branch];
                
                // Find the first change that is newer than the target time
                let mut i = undo_tree.current_position;
                
                while i < branch.changes.len() {
                    let change = &branch.changes[i];
                    
                    if change.timestamp >= target_time {
                        // Found a change that is newer than the target time
                        break;
                    }
                    
                    i += 1;
                }
                
                // Go forward to that change
                while undo_tree.current_position < i {
                    if undo_manager.redo(buffer_id)?.is_none() {
                        break;
                    }
                }
                
                println!("Went forward {} hours", count);
                Ok(())
            },
            "d" | "day" | "days" => {
                // Go forward count days
                let now = std::time::SystemTime::now();
                let target_time = now + std::time::Duration::from_secs(count as u64 * 60 * 60 * 24);
                
                // Get the undo tree for the buffer
                let undo_tree = match undo_manager.get_undo_tree(buffer_id) {
                    Some(tree) => tree,
                    None => {
                        println!("No undo history");
                        return Ok(());
                    }
                };
                
                // Get the current branch
                let branch = &undo_tree.branches[undo_tree.current_branch];
                
                // Find the first change that is newer than the target time
                let mut i = undo_tree.current_position;
                
                while i < branch.changes.len() {
                    let change = &branch.changes[i];
                    
                    if change.timestamp >= target_time {
                        // Found a change that is newer than the target time
                        break;
                    }
                    
                    i += 1;
                }
                
                // Go forward to that change
                while undo_tree.current_position < i {
                    if undo_manager.redo(buffer_id)?.is_none() {
                        break;
                    }
                }
                
                println!("Went forward {} days", count);
                Ok(())
            },
            "f" | "file" | "files" => {
                // Go forward count file writes
                // Get the undo tree for the buffer
                let undo_tree = match undo_manager.get_undo_tree(buffer_id) {
                    Some(tree) => tree,
                    None => {
                        println!("No undo history");
                        return Ok(());
                    }
                };
                
                // Check if there's a last save position
                if let Some((branch, position)) = undo_tree.last_save_position {
                    // Go forward to the last save position
                    if branch == undo_tree.current_branch {
                        // We're in the same branch
                        while undo_tree.current_position < position {
                            if undo_manager.redo(buffer_id)?.is_none() {
                                break;
                            }
                        }
                    } else {
                        // We need to go forward to a different branch
                        // This is more complex and not implemented yet
                        return Err(ExCommandError::Other("Going forward to a different branch is not implemented yet".to_string()));
                    }
                    
                    println!("Went forward to last file write");
                    Ok(())
                } else {
                    println!("No file writes in undo history");
                    Ok(())
                }
            },
            _ => Err(ExCommandError::InvalidArgument(format!("Invalid unit: {}", unit))),
        }
    }
}

/// Handle the :undojoin command
fn handle_undojoin(_cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match crate::command::handlers::EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Get the undo manager
    let undo_manager = unsafe {
        match &mut UNDO_MANAGER {
            Some(manager) => manager,
            None => {
                init_undo_manager();
                match &mut UNDO_MANAGER {
                    Some(manager) => manager,
                    None => return Err(ExCommandError::Other("Failed to initialize undo manager".to_string())),
                }
            }
        }
    };
    
    // Get the current buffer ID
    let buffer_id = match editor.current_buffer_id() {
        Some(id) => id,
        None => return Err(ExCommandError::InvalidCommand("No buffer to join undo for".to_string())),
    };
    
    // Begin an undo group
    undo_manager.begin_group(buffer_id, Some("undojoin".to_string()))?;
    
    println!("Next change will be joined with the previous one");
    Ok(())
}

/// Format a timestamp
fn format_timestamp(timestamp: std::time::SystemTime) -> String {
    // Get the current time
    let now = std::time::SystemTime::now();
    
    // Calculate the duration since the timestamp
    let duration = match now.duration_since(timestamp) {
        Ok(duration) => duration,
        Err(_) => return "future".to_string(),
    };
    
    // Format the duration
    if duration.as_secs() < 60 {
        format!("{}s ago", duration.as_secs())
    } else if duration.as_secs() < 60 * 60 {
        format!("{}m ago", duration.as_secs() / 60)
    } else if duration.as_secs() < 60 * 60 * 24 {
        format!("{}h ago", duration.as_secs() / (60 * 60))
    } else {
        format!("{}d ago", duration.as_secs() / (60 * 60 * 24))
    }
}