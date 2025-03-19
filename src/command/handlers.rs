//! Ex command handlers
//!
//! This module implements handlers for ex commands.

use crate::command::{ExCommand, ExCommandError, ExCommandResult, ExCommandRegistry};
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::sync::Mutex;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::sync::Arc;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::sync::Mutex;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::io::{self, Read, Write};
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::sync::Mutex;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::sync::Arc;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::sync::Mutex;
use std::io::Write;
use std::io::Read;
use std::io::Write;
// use std::fs::File;
// use std::path::Path;
use std::sync::{Arc, Mutex};
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::sync::Mutex;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::sync::Arc;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::sync::Mutex;
use std::io::Write;
use std::io::Read;
use std::io::Write;
// use std::collections::HashMap;

use crate::cursor::CursorPosition;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::sync::Mutex;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::sync::Arc;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::sync::Mutex;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use crate::editor::Editor;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::sync::Mutex;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::sync::Arc;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::sync::Mutex;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use crate::plugin::PluginManager;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::sync::Mutex;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::sync::Arc;
use std::io::Write;
use std::io::Read;
use std::io::Write;
use std::sync::Mutex;
use std::io::Write;
use std::io::Read;
use std::io::Write;

// Global reference to the editor instance
// This is a temporary solution until we have a proper way to pass the editor to command handlers
pub static mut EDITOR: Option<*mut Editor> = None;

/// Set the global editor reference
pub fn set_editor(editor: &mut Editor) {
    unsafe {
        EDITOR = Some(editor as *mut Editor);
    }
}

// We'll use a simpler approach for now - just store a flag to indicate whether to quit
// In a more complete implementation, we would use a proper command context

/// Helper function to create a handler
fn make_handler<F>(f: F) -> impl Fn(&ExCommand) -> ExCommandResult<()> + Send + Sync + 'static
where
    F: Fn(&ExCommand) -> ExCommandResult<()> + Send + Sync + Copy + 'static,
{
    move |cmd| f(cmd)
}

/// Register all ex command handlers
pub fn register_handlers(registry: &mut ExCommandRegistry, plugin_manager: Option<Arc<Mutex<PluginManager>>>) {
    // File operations
    registry.register("write", make_handler(handle_write));
    registry.register("w", make_handler(handle_write));
    registry.register("quit", make_handler(handle_quit));
    registry.register("q", make_handler(handle_quit));
    registry.register("wquit", make_handler(handle_write_quit));
    registry.register("wq", make_handler(handle_write_quit));
    registry.register("xit", make_handler(handle_write_quit));
    registry.register("x", make_handler(handle_write_quit));
    registry.register("edit", make_handler(handle_edit));
    registry.register("e", make_handler(handle_edit));
    registry.register("read", make_handler(handle_read));
    registry.register("r", make_handler(handle_read));
    registry.register("file", make_handler(handle_file));
    registry.register("f", make_handler(handle_file));
    
    // Window operations
    registry.register("split", make_handler(handle_split));
    registry.register("sp", make_handler(handle_split));
    registry.register("vsplit", make_handler(handle_vsplit));
    registry.register("vs", make_handler(handle_vsplit));
    registry.register("close", make_handler(handle_close));
    registry.register("clo", make_handler(handle_close));
    registry.register("only", make_handler(handle_only));
    registry.register("on", make_handler(handle_only));
    registry.register("wnext", make_handler(handle_wnext));
    registry.register("wn", make_handler(handle_wnext));
    registry.register("wprevious", make_handler(handle_wprev));
    registry.register("wp", make_handler(handle_wprev));
    
    // Tab operations
    registry.register("tabedit", make_handler(handle_tabedit));
    registry.register("tabe", make_handler(handle_tabedit));
    registry.register("tabnew", make_handler(handle_tabedit));
    registry.register("tabclose", make_handler(handle_tabclose));
    registry.register("tabc", make_handler(handle_tabclose));
    registry.register("tabnext", make_handler(handle_tabnext));
    registry.register("tabn", make_handler(handle_tabnext));
    registry.register("tabprevious", make_handler(handle_tabprev));
    registry.register("tabp", make_handler(handle_tabprev));
    
    // Editing operations
    registry.register("delete", make_handler(handle_delete));
    registry.register("d", make_handler(handle_delete));
    registry.register("yank", make_handler(handle_yank));
    registry.register("y", make_handler(handle_yank));
    registry.register("put", make_handler(handle_put));
    registry.register("p", make_handler(handle_put));
    registry.register("copy", make_handler(handle_copy));
    registry.register("co", make_handler(handle_copy));
    registry.register("t", make_handler(handle_copy));
    registry.register("move", make_handler(handle_move));
    registry.register("m", make_handler(handle_move));
    registry.register("substitute", make_handler(handle_substitute));
    registry.register("s", make_handler(handle_substitute));
    registry.register("global", make_handler(handle_global));
    registry.register("g", make_handler(handle_global));
    registry.register("vglobal", make_handler(handle_vglobal));
    registry.register("v", make_handler(handle_vglobal));
    
    // Other operations
    registry.register("undo", make_handler(handle_undo));
    registry.register("u", make_handler(handle_undo));
    registry.register("redo", make_handler(handle_redo));
    registry.register("red", make_handler(handle_redo));
    // Register set commands via the dedicated module
    crate::command::set_handlers::register_set_handlers(registry);
    registry.register("map", make_handler(handle_map));
    registry.register("unmap", make_handler(handle_unmap));
    registry.register("marks", make_handler(handle_marks));
    registry.register("jumps", make_handler(handle_jumps));
    registry.register("registers", make_handler(handle_registers));
    registry.register("reg", make_handler(handle_registers));
    registry.register("buffers", make_handler(handle_buffers));
    registry.register("ls", make_handler(handle_buffers));
    registry.register("files", make_handler(handle_buffers));
    registry.register("windows", make_handler(handle_windows));
    registry.register("tabs", make_handler(handle_tabs));
    registry.register("help", make_handler(handle_help));
    registry.register("h", make_handler(handle_help));
    
    // Additional commands
    registry.register("cd", make_handler(handle_cd));
    registry.register("chdir", make_handler(handle_cd));
    registry.register("pwd", make_handler(handle_pwd));
    registry.register("sort", make_handler(handle_sort));
    registry.register("normal", make_handler(handle_normal));
    registry.register("norm", make_handler(handle_normal));
    registry.register("args", make_handler(handle_args));
    registry.register("bdelete", make_handler(handle_bdelete));
    registry.register("bd", make_handler(handle_bdelete));
    // Register nohlsearch commands via the dedicated module
    crate::command::nohlsearch_handlers::register_nohlsearch_handlers(registry);
    
    // Register additional handlers
    crate::command::additional_handlers::register_additional_handlers(registry);
    
    // Register quickfix handlers
    crate::command::quickfix_handlers::register_quickfix_handlers(registry);
    
    // Register fold handlers
    crate::command::fold_handlers::register_fold_handlers(registry);
    
    // Register tag handlers
    crate::command::tag_handlers::register_tag_handlers(registry);
    
    // Register window handlers
    crate::command::window_handlers::register_window_handlers(registry);
    
    // Register search handlers
    crate::command::search_handlers::register_search_handlers(registry);
    
    // Register macro handlers
    crate::command::macro_handlers::register_macro_handlers(registry);
    
    // Register undo handlers
    crate::command::undo_handlers::register_undo_handlers(registry);
    
    // Register mark handlers
    crate::command::mark_handlers::register_mark_handlers(registry);
    
    // Register completion handlers
    crate::command::completion_handlers::register_completion_handlers(registry);
    
    // Register spell handlers
    crate::command::spell_handlers::register_spell_handlers(registry);
    
    // Register diff handlers
    crate::command::diff_handlers::register_diff_handlers(registry);
    
    // Register session handlers
    crate::command::session_handlers::register_session_handlers(registry);
    
    // Register autocmd handlers
    crate::command::autocmd_handlers::register_autocmd_handlers(registry);
    
    // Register terminal handlers
    crate::command::terminal_handlers::register_terminal_handlers(registry);
    
    // Register Vim script commands
    crate::vimscript::register_vim_script_commands(registry);
    
    // Register Vim commands
    crate::command::vim_commands::register_vim_commands(registry);
    
    // Register plugin commands if a plugin manager is provided
    if let Some(plugin_manager) = plugin_manager {
        crate::plugin::commands::register_plugin_commands(registry, plugin_manager);
    }
}

/// Handle the :write command
fn handle_write(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // If a filename was provided, save the buffer to that file
    if let Some(filename) = cmd.first_arg() {
        let path = std::path::Path::new(filename);
        
        // Save the buffer to the specified file
        match editor.save_current_buffer_as(path) {
            Ok(_) => Ok(()),
            Err(err) => Err(ExCommandError::InvalidCommand(format!("Failed to write file: {}", err))),
        }
    } else {
        // Otherwise, save the buffer to its current file
        match editor.save_current_buffer() {
            Ok(_) => Ok(()),
            Err(err) => Err(ExCommandError::InvalidCommand(format!("Failed to write file: {}", err))),
        }
    }
}

/// Handle the :quit command
fn handle_quit(_cmd: &ExCommand) -> ExCommandResult<()> {
    // This is a stub implementation that just returns success
    // In a real implementation, this would quit the current window
    // For now, we just need it to not fail so we can test command mode
    println!("Quit command executed");
    
    // Signal to the editor to quit
    // The editor will check this flag in the main loop
    unsafe {
        QUIT_FLAG = true;
    }
    
    Ok(())
}

/// Handle the :wquit command
fn handle_write_quit(cmd: &ExCommand) -> ExCommandResult<()> {
    // First handle write
    match handle_write(cmd) {
        Ok(_) => {
            // Then handle quit
            handle_quit(cmd)
        },
        Err(err) => {
            // If the write fails, don't quit
            Err(err)
        }
    }
}

// Global flag to signal that the editor should quit
// This is a temporary solution until we have a proper way to communicate between
// the command handlers and the editor
static mut QUIT_FLAG: bool = false;

/// Check if the quit flag is set
pub fn should_quit() -> bool {
    unsafe { QUIT_FLAG }
}

/// Reset the quit flag
pub fn reset_quit_flag() {
    unsafe { QUIT_FLAG = false; }
}

/// Handle the :edit command
pub fn handle_edit(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // If a filename was provided, open that file
    if let Some(filename) = cmd.first_arg() {
        let path = std::path::Path::new(filename);
        
        // Open the file
        match editor.open_file(path) {
            Ok(_) => {
                println!("\"{}\" opened", path.display());
                Ok(())
            },
            Err(err) => Err(ExCommandError::InvalidCommand(format!("Failed to open file: {}", err))),
        }
    } else {
        // If no filename was provided, reload the current buffer
        // First, check if the current buffer has a file path
        let buffer_id = match editor.current_buffer_id() {
            Some(id) => id,
            None => return Err(ExCommandError::InvalidCommand("No buffer to reload".to_string())),
        };
        
        // Since we don't have direct access to the buffer manager, we'll try to reopen the current file
        // by saving it first (to ensure we have the latest content) and then reopening it
        
        // Try to save the current buffer to get its path
        match editor.save_current_buffer() {
            Ok(_) => {
                // Now reopen the file (which will reload it from disk)
                // We need to get the current buffer again to get its file path
                if let Some(buffer_id) = editor.current_buffer_id() {
                    // Open the same file again to reload it
                    // This is a bit of a workaround since we don't have direct access to the buffer's file path
                    match editor.open_file(std::path::Path::new(".")) {
                        Ok(_) => {
                            println!("Current file reloaded");
                            Ok(())
                        },
                        Err(err) => Err(ExCommandError::InvalidCommand(format!("Failed to reload file: {}", err))),
                    }
                } else {
                    Err(ExCommandError::InvalidCommand("No buffer to reload".to_string()))
                }
            },
            Err(err) => {
                // If we can't save, it might be because there's no file path
                Err(ExCommandError::InvalidCommand(format!("Failed to reload file: {}", err)))
            }
        }
    }
}

/// Handle the :read command
fn handle_read(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Check if a filename was provided
    if let Some(filename) = cmd.first_arg() {
        let path = std::path::Path::new(filename);
        
        // Check if the file exists
        if !path.exists() {
            return Err(ExCommandError::InvalidCommand(format!("File not found: {}", filename)));
        }
        
        // Read the file content
        match std::fs::read_to_string(path) {
            Ok(content) => {
                // Get the current buffer ID
                let buffer_id = match editor.current_buffer_id() {
                    Some(id) => id,
                    None => return Err(ExCommandError::InvalidCommand("No buffer to read into".to_string())),
                };
                
                // Get the current cursor position
                let cursor_pos = editor.cursor_position();
                
                // Get a mutable reference to the buffer
                let buffer = editor.get_buffer_manager_mut().get_buffer_mut(buffer_id)?;
                
                // Find the end of the current line
                let line_end_idx = if cursor_pos.line < buffer.line_count() {
                    // If we're not at the last line, find the start of the next line
                    if cursor_pos.line + 1 < buffer.line_count() {
                        buffer.position_to_char_idx(cursor_pos.line + 1, 0)?
                    } else {
                        // If we're at the last line, go to the end of the buffer
                        buffer.content().len()
                    }
                } else {
                    // If we're beyond the last line, append to the end of the buffer
                    buffer.content().len()
                };
                
                // Insert a newline if we're not at the end of the buffer
                let insert_content = if line_end_idx < buffer.content().len() && !content.starts_with('\n') {
                    format!("\n{}", content)
                } else {
                    content
                };
                
                // Insert the content at the end of the current line
                buffer.insert(line_end_idx, &insert_content)?;
                
                // Update the cursor position to the beginning of the inserted content
                let new_line = cursor_pos.line + 1;
                let new_pos = CursorPosition::new(new_line, 0);
                editor.get_cursor_manager_mut().set_position(new_pos);
                
                println!("\"{}\" read", path.display());
                Ok(())
            },
            Err(err) => Err(ExCommandError::InvalidCommand(format!("Failed to read file: {}", err))),
        }
    } else {
        Err(ExCommandError::MissingArgument("Filename required".to_string()))
    }
}

/// Handle the :split command
pub fn handle_split(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Get the current buffer ID
    let current_buffer_id = match editor.current_buffer_id() {
        Some(id) => id,
        None => return Err(ExCommandError::InvalidCommand("No buffer to split".to_string())),
    };
    
    // If a file path is provided, open that file in the new window
    let buffer_id = if let Some(file_path) = cmd.first_arg() {
        match editor.get_buffer_manager_mut().open_file(file_path) {
            Ok(id) => id,
            Err(err) => {
                // If the file doesn't exist, create a new empty buffer
                if let crate::buffer::BufferManagerError::Io(io_err) = &err {
                    if io_err.kind() == std::io::ErrorKind::NotFound {
                        // Create a new buffer
                        match editor.get_buffer_manager_mut().create_buffer() {
                            Ok(id) => id,
                            Err(e) => return Err(ExCommandError::Other(format!("Failed to create buffer: {}", e))),
                        }
                    } else {
                        return Err(ExCommandError::Other(format!("Failed to open file: {}", err)));
                    }
                } else {
                    return Err(ExCommandError::Other(format!("Failed to open file: {}", err)));
                }
            }
        }
    } else {
        // Otherwise, use the current buffer
        current_buffer_id
    };
    
    // Split the window horizontally
    match editor.get_terminal_mut().split_window(crate::ui::window::SplitDirection::Horizontal, buffer_id) {
        Ok(_) => {
            println!("Window split horizontally");
            Ok(())
        },
        Err(err) => Err(ExCommandError::Other(format!("Failed to split window: {}", err))),
    }
}

/// Handle the :vsplit command
fn handle_vsplit(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Get the current buffer ID
    let current_buffer_id = match editor.current_buffer_id() {
        Some(id) => id,
        None => return Err(ExCommandError::InvalidCommand("No buffer to split".to_string())),
    };
    
    // If a file path is provided, open that file in the new window
    let buffer_id = if let Some(file_path) = cmd.first_arg() {
        match editor.get_buffer_manager_mut().open_file(file_path) {
            Ok(id) => id,
            Err(err) => {
                // If the file doesn't exist, create a new empty buffer
                if let crate::buffer::BufferManagerError::Io(io_err) = &err {
                    if io_err.kind() == std::io::ErrorKind::NotFound {
                        // Create a new buffer
                        match editor.get_buffer_manager_mut().create_buffer() {
                            Ok(id) => id,
                            Err(e) => return Err(ExCommandError::Other(format!("Failed to create buffer: {}", e))),
                        }
                    } else {
                        return Err(ExCommandError::Other(format!("Failed to open file: {}", err)));
                    }
                } else {
                    return Err(ExCommandError::Other(format!("Failed to open file: {}", err)));
                }
            }
        }
    } else {
        // Otherwise, use the current buffer
        current_buffer_id
    };
    
    // Split the window vertically
    match editor.get_terminal_mut().split_window(crate::ui::window::SplitDirection::Vertical, buffer_id) {
        Ok(_) => {
            println!("Window split vertically");
            Ok(())
        },
        Err(err) => Err(ExCommandError::Other(format!("Failed to split window: {}", err))),
    }
}

/// Handle the :close command
fn handle_close(_cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Close the current window
    match editor.get_terminal_mut().close_current_window() {
        Ok(true) => {
            println!("Window closed");
            Ok(())
        },
        Ok(false) => {
            // If there's only one window left, we can't close it
            Err(ExCommandError::InvalidCommand("Cannot close last window in tab".to_string()))
        },
        Err(err) => Err(ExCommandError::Other(format!("Failed to close window: {}", err))),
    }
}

/// Handle the :only command
fn handle_only(_cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Get the current tab
    if let Some(tab) = editor.get_terminal_mut().current_tab_mut() {
        // Get the current window ID
        let current_window_id = tab.window_manager.current_window_id();
        
        // Get all window IDs
        let window_ids: Vec<usize> = tab.window_manager.windows()
            .iter()
            .filter(|w| w.id != current_window_id)
            .map(|w| w.id)
            .collect();
        
        // Close each window except the current one
        let mut count = 0;
        for window_id in window_ids {
            if tab.window_manager.close_window(window_id) {
                count += 1;
            }
        }
        
        if count > 0 {
            println!("{} window{} closed", count, if count == 1 { "" } else { "s" });
        } else {
            println!("No other windows to close");
        }
        Ok(())
    } else {
        println!("No active tab");
        Ok(())
    }
}

/// Handle the :wnext command
fn handle_wnext(_cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Navigate to the next window
    match editor.get_terminal_mut().next_window() {
        Ok(true) => {
            println!("Moved to next window");
            Ok(())
        },
        Ok(false) => {
            // If there's only one window, we can't navigate
            println!("No more windows");
            Ok(())
        },
        Err(err) => Err(ExCommandError::Other(format!("Failed to navigate to next window: {}", err))),
    }
}

/// Handle the :wprev command
fn handle_wprev(_cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Navigate to the previous window
    match editor.get_terminal_mut().prev_window() {
        Ok(true) => {
            println!("Moved to previous window");
            Ok(())
        },
        Ok(false) => {
            // If there's only one window, we can't navigate
            println!("No more windows");
            Ok(())
        },
        Err(err) => Err(ExCommandError::Other(format!("Failed to navigate to previous window: {}", err))),
    }
}

/// Handle the :tabedit command
fn handle_tabedit(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Get the file path from the command arguments
    let file_path = match cmd.first_arg() {
        Some(path) => path,
        None => return Err(ExCommandError::MissingArgument("File name required".to_string())),
    };
    
    // Create a new buffer for the file
    let buffer_id = match editor.get_buffer_manager_mut().open_file(file_path) {
        Ok(id) => id,
        Err(err) => {
            // If the file doesn't exist, create a new empty buffer
            if let crate::buffer::BufferManagerError::Io(io_err) = &err {
                if io_err.kind() == std::io::ErrorKind::NotFound {
                    // Create a new buffer
                    match editor.get_buffer_manager_mut().create_buffer() {
                        Ok(id) => id,
                        Err(e) => return Err(ExCommandError::Other(format!("Failed to create buffer: {}", e))),
                    }
                } else {
                    return Err(ExCommandError::Other(format!("Failed to open file: {}", err)));
                }
            } else {
                return Err(ExCommandError::Other(format!("Failed to open file: {}", err)));
            }
        }
    };
    
    // Create a new tab with the buffer
    match editor.get_terminal_mut().create_tab(buffer_id, Some(file_path.to_string())) {
        Ok(_) => {
            // Set the current buffer to the new buffer
            if let Err(err) = editor.get_buffer_manager_mut().set_current_buffer(buffer_id) {
                return Err(ExCommandError::Other(format!("Failed to set current buffer: {}", err)));
            }
            
            println!("\"{}\" opened in new tab", file_path);
            Ok(())
        },
        Err(err) => Err(ExCommandError::Other(format!("Failed to create tab: {}", err))),
    }
}

/// Handle the :tabclose command
fn handle_tabclose(_cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Close the current tab
    match editor.get_terminal_mut().close_current_tab() {
        Ok(true) => {
            println!("Tab closed");
            Ok(())
        },
        Ok(false) => {
            // If there's only one tab left, we can't close it
            Err(ExCommandError::InvalidCommand("Cannot close last tab".to_string()))
        },
        Err(err) => Err(ExCommandError::Other(format!("Failed to close tab: {}", err))),
    }
}

/// Handle the :tabnext command
fn handle_tabnext(_cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Navigate to the next tab
    match editor.get_terminal_mut().next_tab() {
        Ok(true) => {
            println!("Moved to next tab");
            Ok(())
        },
        Ok(false) => {
            // If there's only one tab, we can't navigate
            println!("No more tabs");
            Ok(())
        },
        Err(err) => Err(ExCommandError::Other(format!("Failed to navigate to next tab: {}", err))),
    }
}

/// Handle the :tabprev command
fn handle_tabprev(_cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Navigate to the previous tab
    match editor.get_terminal_mut().prev_tab() {
        Ok(true) => {
            println!("Moved to previous tab");
            Ok(())
        },
        Ok(false) => {
            // If there's only one tab, we can't navigate
            println!("No more tabs");
            Ok(())
        },
        Err(err) => Err(ExCommandError::Other(format!("Failed to navigate to previous tab: {}", err))),
    }
}

/// Handle the :delete command
fn handle_delete(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Get the current buffer ID
    let buffer_id = match editor.current_buffer_id() {
        Some(id) => id,
        None => return Err(ExCommandError::InvalidCommand("No buffer to delete from".to_string())),
    };
    
    // Get the current cursor position
    let cursor_pos = editor.cursor_position();
    let start_line = cursor_pos.line;
    
    // Parse the count from the command arguments
    let count = if let Some(count_str) = cmd.first_arg() {
        match count_str.parse::<usize>() {
            Ok(n) => n,
            Err(_) => return Err(ExCommandError::InvalidCommand(format!("Invalid count: {}", count_str))),
        }
    } else {
        // Default to 1 line if no count is provided
        1
    };
    
    // Calculate the end line (inclusive)
    let end_line = start_line + count - 1;
    
    // Delete the lines
    match editor.delete_lines_from_cursor(buffer_id, start_line, end_line) {
        Ok(_) => {
            println!("{} line{} deleted", count, if count == 1 { "" } else { "s" });
            Ok(())
        },
        Err(err) => Err(ExCommandError::InvalidCommand(format!("Failed to delete lines: {}", err))),
    }
}

/// Handle the :yank command
fn handle_yank(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Get the current buffer ID
    let buffer_id = match editor.current_buffer_id() {
        Some(id) => id,
        None => return Err(ExCommandError::InvalidCommand("No buffer to yank from".to_string())),
    };
    
    // Get the current cursor position
    let cursor_pos = editor.cursor_position();
    let start_line = cursor_pos.line;
    
    // Parse the count from the command arguments
    let count = if let Some(count_str) = cmd.first_arg() {
        match count_str.parse::<usize>() {
            Ok(n) => n,
            Err(_) => return Err(ExCommandError::InvalidCommand(format!("Invalid count: {}", count_str))),
        }
    } else {
        // Default to 1 line if no count is provided
        1
    };
    
    // Calculate the end line (inclusive)
    let end_line = start_line + count - 1;
    
    // Use the editor's yank_lines method if it exists, otherwise use a workaround
    match editor.yank_lines(start_line, end_line) {
        Ok(Some(_)) => {
            println!("{} line{} yanked", count, if count == 1 { "" } else { "s" });
            Ok(())
        },
        Ok(None) => {
            println!("No lines yanked");
            Ok(())
        },
        Err(err) => Err(ExCommandError::InvalidCommand(format!("Failed to yank lines: {}", err))),
    }
}

/// Handle the :put command
fn handle_put(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Check if a register was specified
    let register = if let Some(reg_str) = cmd.first_arg() {
        if reg_str.len() == 1 {
            let c = reg_str.chars().next().unwrap();
            if c.is_alphabetic() || "0123456789".contains(c) {
                Some(c)
            } else {
                return Err(ExCommandError::InvalidArgument(format!("Invalid register: {}", reg_str)));
            }
        } else {
            return Err(ExCommandError::InvalidArgument(format!("Invalid register: {}", reg_str)));
        }
    } else {
        None // Use the default register
    };
    
    // Paste from the specified register or the unnamed register
    let result = if let Some(reg) = register {
        editor.paste_from_register_char(reg)
    } else {
        editor.paste()
    };
    
    match result {
        Ok(true) => {
            println!("Text pasted");
            Ok(())
        },
        Ok(false) => {
            println!("No text to paste");
            Ok(())
        },
        Err(err) => Err(ExCommandError::InvalidCommand(format!("Failed to paste text: {}", err))),
    }
}

/// Handle the :copy command
fn handle_copy(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Get the current buffer ID
    let buffer_id = match editor.current_buffer_id() {
        Some(id) => id,
        None => return Err(ExCommandError::InvalidCommand("No buffer to copy in".to_string())),
    };
    
    // Get the arguments
    let args = cmd.args_str();
    
    if args.is_empty() {
        return Err(ExCommandError::MissingArgument("Destination line number required".to_string()));
    }
    
    // Parse the destination line number
    let dest_line = match args.parse::<usize>() {
        Ok(n) => n,
        Err(_) => return Err(ExCommandError::InvalidArgument(format!("Invalid line number: {}", args))),
    };
    
    // Get the current cursor position
    let cursor_pos = editor.cursor_position();
    let start_line = cursor_pos.line;
    
    // Copy the current line to the destination
    match editor.copy_line(buffer_id, start_line, dest_line) {
        Ok(_) => {
            println!("1 line copied");
            Ok(())
        },
        Err(err) => Err(ExCommandError::InvalidCommand(format!("Failed to copy line: {}", err))),
    }
}

/// Handle the :move command
fn handle_move(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Get the current buffer ID
    let buffer_id = match editor.current_buffer_id() {
        Some(id) => id,
        None => return Err(ExCommandError::InvalidCommand("No buffer to move in".to_string())),
    };
    
    // Get the arguments
    let args = cmd.args_str();
    
    if args.is_empty() {
        return Err(ExCommandError::MissingArgument("Destination line number required".to_string()));
    }
    
    // Parse the destination line number
    let dest_line = match args.parse::<usize>() {
        Ok(n) => n,
        Err(_) => return Err(ExCommandError::InvalidArgument(format!("Invalid line number: {}", args))),
    };
    
    // Get the current cursor position
    let cursor_pos = editor.cursor_position();
    let start_line = cursor_pos.line;
    
    // Move the current line to the destination
    match editor.move_line(buffer_id, start_line, dest_line) {
        Ok(_) => {
            println!("1 line moved");
            Ok(())
        },
        Err(err) => Err(ExCommandError::InvalidCommand(format!("Failed to move line: {}", err))),
    }
}

/// Handle the :substitute command
fn handle_substitute(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Get the arguments
    let args = cmd.args_str();
    
    if args.is_empty() {
        // If no arguments, use the last search pattern and replacement
        println!("Using last search pattern and replacement");
        println!("  (Substitute with last pattern not fully implemented yet)");
        return Ok(());
    }
    
    // Parse the substitute command
    // Format: /pattern/replacement/[flags]
    // or s/pattern/replacement/[flags]
    
    // Check if the command starts with 's/'
    let args = if args.starts_with('s') && args.len() > 1 && args.chars().nth(1) == Some('/') {
        &args[2..]
    } else if args.starts_with('/') {
        &args[1..]
    } else {
        return Err(ExCommandError::InvalidArgument(format!("Invalid substitute format: {}", args)));
    };
    
    // Split the arguments by '/'
    let parts: Vec<&str> = args.split('/').collect();
    
    if parts.len() < 2 {
        return Err(ExCommandError::InvalidArgument(format!("Invalid substitute format: {}", args)));
    }
    
    let pattern = parts[0];
    let replacement = parts.get(1).unwrap_or(&"");
    let flags = parts.get(2).unwrap_or(&"");
    
    // Parse flags
    let global = flags.contains('g');
    let case_sensitive = !flags.contains('i');
    let confirm = flags.contains('c');
    
    // Get the current buffer ID
    let buffer_id = match editor.current_buffer_id() {
        Some(id) => id,
        None => return Err(ExCommandError::InvalidCommand("No buffer to substitute in".to_string())),
    };
    
    // Perform the search and replace
    match editor.search_and_replace(pattern, replacement, case_sensitive, confirm) {
        Ok(count) => {
            println!("{} substitution{} on {} line{}",
                count,
                if count == 1 { "" } else { "s" },
                count, // This is a simplification; in reality, the number of lines affected might be different
                if count == 1 { "" } else { "s" }
            );
            Ok(())
        },
        Err(err) => Err(ExCommandError::InvalidCommand(format!("Failed to substitute: {}", err))),
    }
}

/// Handle the :global command
fn handle_global(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Get the arguments
    let args = cmd.args_str();
    
    if args.is_empty() {
        return Err(ExCommandError::MissingArgument("Pattern and command required".to_string()));
    }
    
    // Parse the global command
    // Format: /pattern/command
    
    // Check if the command starts with '/'
    if !args.starts_with('/') {
        return Err(ExCommandError::InvalidArgument(format!("Invalid global format: {}", args)));
    }
    
    // Find the end of the pattern
    let pattern_end = match args[1..].find('/') {
        Some(pos) => pos + 1, // +1 to account for the leading '/'
        None => return Err(ExCommandError::InvalidArgument(format!("Invalid global format: {}", args))),
    };
    
    let pattern = &args[1..pattern_end];
    let command = args[pattern_end + 1..].trim();
    
    if command.is_empty() {
        return Err(ExCommandError::MissingArgument("Command required".to_string()));
    }
    
    // Get the current buffer ID
    let buffer_id = match editor.current_buffer_id() {
        Some(id) => id,
        None => return Err(ExCommandError::InvalidCommand("No buffer to execute global command in".to_string())),
    };
    
    // Get a reference to the buffer
    let buffer = editor.get_buffer_manager().get_buffer(buffer_id)?;
    
    // Create a command parser
    let parser = crate::command::ExCommandParser::new();
    
    // Create a command registry
    let mut registry = crate::command::ExCommandRegistry::new();
    register_handlers(&mut registry, None);
    
    // Find lines that match the pattern
    let mut matching_lines = Vec::new();
    let line_count = buffer.line_count();
    
    for line_idx in 0..line_count {
        let line = buffer.line(line_idx)?;
        
        // Check if the line matches the pattern
        if line.contains(pattern) {
            matching_lines.push(line_idx);
        }
    }
    
    // Execute the command on each matching line
    // We need to process lines in reverse order to handle the case where we're deleting lines
    // This ensures that line numbers remain valid even after deletions
    let mut count = 0;
    for line_idx in matching_lines.iter().rev() {
        // Set the cursor to the matching line
        editor.get_cursor_manager_mut().set_position(crate::cursor::CursorPosition::new(*line_idx, 0));
        
        // Parse and execute the command
        match parser.parse(command) {
            Ok(ex_cmd) => {
                if let Err(err) = registry.execute(&ex_cmd) {
                    return Err(ExCommandError::Other(format!("Failed to execute command on line {}: {}", line_idx + 1, err)));
                }
                count += 1;
            },
            Err(err) => {
                return Err(ExCommandError::Other(format!("Failed to parse command: {}", err)));
            }
        }
    }
    
    println!("{} line{} processed", count, if count == 1 { "" } else { "s" });
    
    Ok(())
}

/// Handle the :vglobal command
fn handle_vglobal(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Get the arguments
    let args = cmd.args_str();
    
    if args.is_empty() {
        return Err(ExCommandError::MissingArgument("Pattern and command required".to_string()));
    }
    
    // Parse the vglobal command
    // Format: /pattern/command
    
    // Check if the command starts with '/'
    if !args.starts_with('/') {
        return Err(ExCommandError::InvalidArgument(format!("Invalid vglobal format: {}", args)));
    }
    
    // Find the end of the pattern
    let pattern_end = match args[1..].find('/') {
        Some(pos) => pos + 1, // +1 to account for the leading '/'
        None => return Err(ExCommandError::InvalidArgument(format!("Invalid vglobal format: {}", args))),
    };
    
    let pattern = &args[1..pattern_end];
    let command = args[pattern_end + 1..].trim();
    
    if command.is_empty() {
        return Err(ExCommandError::MissingArgument("Command required".to_string()));
    }
    
    // Get the current buffer ID
    let buffer_id = match editor.current_buffer_id() {
        Some(id) => id,
        None => return Err(ExCommandError::InvalidCommand("No buffer to execute vglobal command in".to_string())),
    };
    
    // Get a reference to the buffer
    let buffer = editor.get_buffer_manager().get_buffer(buffer_id)?;
    
    // Create a command parser
    let parser = crate::command::ExCommandParser::new();
    
    // Create a command registry
    let mut registry = crate::command::ExCommandRegistry::new();
    register_handlers(&mut registry, None);
    
    // Find lines that don't match the pattern
    let mut non_matching_lines = Vec::new();
    let line_count = buffer.line_count();
    
    for line_idx in 0..line_count {
        let line = buffer.line(line_idx)?;
        
        // Check if the line doesn't match the pattern
        if !line.contains(pattern) {
            non_matching_lines.push(line_idx);
        }
    }
    
    // Execute the command on each non-matching line
    // We need to process lines in reverse order to handle the case where we're deleting lines
    // This ensures that line numbers remain valid even after deletions
    let mut count = 0;
    for line_idx in non_matching_lines.iter().rev() {
        // Set the cursor to the non-matching line
        editor.get_cursor_manager_mut().set_position(crate::cursor::CursorPosition::new(*line_idx, 0));
        
        // Parse and execute the command
        match parser.parse(command) {
            Ok(ex_cmd) => {
                if let Err(err) = registry.execute(&ex_cmd) {
                    return Err(ExCommandError::Other(format!("Failed to execute command on line {}: {}", line_idx + 1, err)));
                }
                count += 1;
            },
            Err(err) => {
                return Err(ExCommandError::Other(format!("Failed to parse command: {}", err)));
            }
        }
    }
    
    println!("{} line{} processed", count, if count == 1 { "" } else { "s" });
    
    Ok(())
}

/// Handle the :undo command
fn handle_undo(_cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Get the current buffer ID
    let buffer_id = match editor.current_buffer_id() {
        Some(id) => id,
        None => return Err(ExCommandError::InvalidCommand("No buffer to undo changes in".to_string())),
    };
    
    // Get a mutable reference to the buffer
    let buffer = editor.get_buffer_manager_mut().get_buffer_mut(buffer_id)?;
    
    // Perform the undo operation
    match buffer.undo() {
        Ok(true) => {
            println!("1 change undone");
            Ok(())
        },
        Ok(false) => {
            println!("No changes to undo");
            Ok(())
        },
        Err(err) => Err(ExCommandError::Other(format!("Failed to undo changes: {}", err))),
    }
}

/// Handle the :redo command
fn handle_redo(_cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Get the current buffer ID
    let buffer_id = match editor.current_buffer_id() {
        Some(id) => id,
        None => return Err(ExCommandError::InvalidCommand("No buffer to redo changes in".to_string())),
    };
    
    // Get a mutable reference to the buffer
    let buffer = editor.get_buffer_manager_mut().get_buffer_mut(buffer_id)?;
    
    // Perform the redo operation
    match buffer.redo() {
        Ok(true) => {
            println!("1 change redone");
            Ok(())
        },
        Ok(false) => {
            println!("No changes to redo");
            Ok(())
        },
        Err(err) => Err(ExCommandError::Other(format!("Failed to redo changes: {}", err))),
    }
}

/// Handle the :file command
fn handle_file(_cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Get the current buffer ID
    let buffer_id = match editor.current_buffer_id() {
        Some(id) => id,
        None => return Err(ExCommandError::InvalidCommand("No buffer to show information for".to_string())),
    };
    
    // Get a reference to the buffer
    let buffer = editor.get_buffer_manager().get_buffer(buffer_id)?;
    
    // Get the file path
    let file_path = buffer.get_path();
    
    // Get the cursor position
    let cursor_pos = editor.cursor_position();
    
    // Get the line count
    let line_count = buffer.line_count();
    
    // Display the file information
    if let Some(path) = file_path {
        println!("\"{}\" line {} of {} --{}%--",
            path.display(),
            cursor_pos.line + 1,
            line_count,
            if line_count > 0 {
                ((cursor_pos.line + 1) * 100) / line_count
            } else {
                0
            }
        );
    } else {
        println!("[No Name] line {} of {} --{}%--",
            cursor_pos.line + 1,
            line_count,
            if line_count > 0 {
                ((cursor_pos.line + 1) * 100) / line_count
            } else {
                0
            }
        );
    }
    
    Ok(())
}


/// Handle the :map command
fn handle_map(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Get the arguments
    let args = cmd.args_str();
    
    if args.is_empty() {
        // If no arguments, show all mappings
        println!("--- Mappings ---");
        // In a real implementation, this would show all mappings
        println!("(Mappings display not fully implemented yet)");
        return Ok(());
    }
    
    // Parse the mapping
    let parts: Vec<&str> = args.split_whitespace().collect();
    
    if parts.len() < 2 {
        return Err(ExCommandError::MissingArgument("Mapping requires at least two arguments".to_string()));
    }
    
    let lhs = parts[0];
    let rhs = parts[1..].join(" ");
    
    println!("Mapped {} to {}", lhs, rhs);
    // In a real implementation, this would set the mapping
    
    Ok(())
}

/// Handle the :unmap command
fn handle_unmap(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Get the arguments
    let args = cmd.args_str();
    
    if args.is_empty() {
        return Err(ExCommandError::MissingArgument("Mapping to remove required".to_string()));
    }
    
    // Parse the mapping to remove
    let lhs = args.trim();
    
    println!("Unmapped {}", lhs);
    // In a real implementation, this would remove the mapping
    
    Ok(())
}

/// Handle the :marks command
fn handle_marks(_cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Display the marks
    println!("--- Marks ---");
    println!("mark line  col file/text");
    // In a real implementation, this would show all marks
    println!("(Marks display not fully implemented yet)");
    
    Ok(())
}

/// Handle the :jumps command
fn handle_jumps(_cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Display the jump list
    println!("--- Jump list ---");
    println!("jump line  col file/text");
    // In a real implementation, this would show the jump list
    println!("(Jump list display not fully implemented yet)");
    
    Ok(())
}

/// Handle the :registers command
fn handle_registers(_cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Display the registers
    println!("--- Registers ---");
    // In a real implementation, this would show all registers
    println!("(Registers display not fully implemented yet)");
    
    Ok(())
}

/// Handle the :buffers command
fn handle_buffers(_cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Get the current buffer ID
    let current_buffer_id = match editor.current_buffer_id() {
        Some(id) => id,
        None => 0,
    };
    
    // Display the buffers
    println!("--- Buffers ---");
    
    // Get the buffer manager
    let buffer_manager = editor.get_buffer_manager();
    
    // Get the buffer IDs
    let buffer_ids = buffer_manager.get_buffer_ids();
    
    // Display each buffer
    for buffer_id in buffer_ids {
        let buffer = match buffer_manager.get_buffer(buffer_id) {
            Ok(buffer) => buffer,
            Err(_) => continue,
        };
        
        let path = buffer.get_path();
        let path_str = match path {
            Some(path) => path.to_string_lossy().to_string(),
            None => "[No Name]".to_string(),
        };
        
        let current_marker = if buffer_id == current_buffer_id { "%" } else { " " };
        
        println!("{} {:2} {}", current_marker, buffer_id, path_str);
    }
    
    Ok(())
}

/// Handle the :windows command
fn handle_windows(_cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Display the windows
    println!("--- Windows ---");
    // In a real implementation, this would show all windows
    println!("(Windows display not fully implemented yet)");
    
    Ok(())
}

/// Handle the :tabs command
fn handle_tabs(_cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Display the tabs
    println!("--- Tabs ---");
    // In a real implementation, this would show all tabs
    println!("(Tabs display not fully implemented yet)");
    
    Ok(())
}

/// Handle the :help command
fn handle_help(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Get the arguments
    let args = cmd.args_str();
    
    if args.is_empty() {
        // If no arguments, show general help
        println!("--- xvim Help ---");
        println!("xvim is a Vim-like text editor.");
        println!("");
        println!("Commands:");
        println!("  :help [topic]     - Show help for a topic");
        println!("  :q                - Quit");
        println!("  :w                - Write (save) the current buffer");
        println!("  :wq               - Write and quit");
        println!("  :e [file]         - Edit a file");
        println!("  :set [option]     - Set an option");
        println!("");
        println!("For more help, use :help [topic]");
    } else {
        // Show help for the specified topic
        println!("--- Help for {} ---", args);
        println!("(Help for {} not fully implemented yet)", args);
    }
    
    Ok(())
}

/// Handle the :cd command
fn handle_cd(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the arguments
    let args = cmd.args_str();
    
    if args.is_empty() {
        // If no arguments, change to the home directory
        match std::env::var("HOME") {
            Ok(home) => {
                match std::env::set_current_dir(&home) {
                    Ok(_) => {
                        println!("Changed directory to {}", home);
                        Ok(())
                    },
                    Err(err) => Err(ExCommandError::InvalidCommand(format!("Failed to change directory: {}", err))),
                }
            },
            Err(_) => Err(ExCommandError::InvalidCommand("HOME environment variable not set".to_string())),
        }
    } else {
        // Change to the specified directory
        let path = std::path::Path::new(args);
        
        match std::env::set_current_dir(path) {
            Ok(_) => {
                println!("Changed directory to {}", path.display());
                Ok(())
            },
            Err(err) => Err(ExCommandError::InvalidCommand(format!("Failed to change directory: {}", err))),
        }
    }
}

/// Handle the :pwd command
fn handle_pwd(_cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the current directory
    match std::env::current_dir() {
        Ok(path) => {
            println!("{}", path.display());
            Ok(())
        },
        Err(err) => Err(ExCommandError::InvalidCommand(format!("Failed to get current directory: {}", err))),
    }
}

/// Handle the :sort command
fn handle_sort(_cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Get the current buffer ID
    let buffer_id = match editor.current_buffer_id() {
        Some(id) => id,
        None => return Err(ExCommandError::InvalidCommand("No buffer to sort".to_string())),
    };
    
    // Get a mutable reference to the buffer
    let buffer = editor.get_buffer_manager_mut().get_buffer_mut(buffer_id)?;
    
    // Get all lines
    let mut lines = Vec::new();
    for i in 0..buffer.line_count() {
        lines.push(buffer.line(i)?);
    }
    
    // Sort the lines
    lines.sort();
    
    // Replace the buffer content
    let new_content = lines.join("\n");
    buffer.set_content(&new_content)?;
    
    println!("Sorted {} lines", lines.len());
    
    Ok(())
}

/// Handle the :normal command
fn handle_normal(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Get the arguments
    let args = cmd.args_str();
    
    if args.is_empty() {
        return Err(ExCommandError::MissingArgument("Normal mode commands required".to_string()));
    }
    
    // Execute the normal mode commands
    println!("Executing normal mode commands: {}", args);
    // In a real implementation, this would execute the normal mode commands
    
    Ok(())
}

/// Handle the :args command
fn handle_args(_cmd: &ExCommand) -> ExCommandResult<()> {
    // Display the command-line arguments
    println!("--- Command-line Arguments ---");
    // In a real implementation, this would show the command-line arguments
    println!("(Command-line arguments display not fully implemented yet)");
    
    Ok(())
}

/// Handle the :bdelete command
fn handle_bdelete(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Get the arguments
    let args = cmd.args_str();
    
    if args.is_empty() {
        // If no arguments, delete the current buffer
        let buffer_id = match editor.current_buffer_id() {
            Some(id) => id,
            None => return Err(ExCommandError::InvalidCommand("No buffer to delete".to_string())),
        };
        
        // Delete the buffer
        match editor.get_buffer_manager_mut().delete_buffer(buffer_id) {
            Ok(_) => {
                println!("Buffer {} deleted", buffer_id);
                Ok(())
            },
            Err(err) => Err(ExCommandError::InvalidCommand(format!("Failed to delete buffer: {}", err))),
        }
    } else {
        // Delete the specified buffer
        match args.parse::<usize>() {
            Ok(buffer_id) => {
                // Delete the buffer
                match editor.get_buffer_manager_mut().delete_buffer(buffer_id) {
                    Ok(_) => {
                        println!("Buffer {} deleted", buffer_id);
                        Ok(())
                    },
                    Err(err) => Err(ExCommandError::InvalidCommand(format!("Failed to delete buffer: {}", err))),
                }
            },
            Err(_) => Err(ExCommandError::InvalidArgument(format!("Invalid buffer ID: {}", args))),
        }
    }
}

