//! Nohlsearch command handlers
//!
//! This module implements handlers for nohlsearch commands.

use crate::command::{ExCommand, ExCommandError, ExCommandResult, ExCommandRegistry};
// use crate::editor::Editor;

/// Handle the :nohlsearch command
pub fn handle_nohlsearch(_cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match crate::command::handlers::EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Get the search manager
    let search_manager = unsafe {
        match &mut crate::command::search_handlers::SEARCH_MANAGER {
            Some(manager) => manager,
            None => {
                crate::command::search_handlers::init_search_manager();
                match &mut crate::command::search_handlers::SEARCH_MANAGER {
                    Some(manager) => manager,
                    None => return Err(ExCommandError::Other("Failed to initialize search manager".to_string())),
                }
            }
        }
    };
    
    // Clear the highlights
    search_manager.clear_highlights();
    
    println!("Search highlighting turned off");
    Ok(())
}

/// Register nohlsearch command handlers
pub fn register_nohlsearch_handlers(registry: &mut ExCommandRegistry) {
    // Register nohlsearch commands
    registry.register("nohlsearch", handle_nohlsearch);
    registry.register("noh", handle_nohlsearch);
}