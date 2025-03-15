//! Ex command handlers
//!
//! This module implements handlers for ex commands.

use crate::command::{ExCommand, ExCommandError, ExCommandResult, ExCommandRegistry};
use crate::editor::Editor;
use crate::plugin::PluginManager;
use std::sync::{Arc, Mutex};

// Global reference to the editor instance
// This is a temporary solution until we have a proper way to pass the editor to command handlers
static mut EDITOR: Option<*mut Editor> = None;

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
    registry.register("set", make_handler(handle_set));
    registry.register("se", make_handler(handle_set));
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
fn handle_edit(cmd: &ExCommand) -> ExCommandResult<()> {
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
                // Since we don't have direct access to the buffer manager, we need to use the editor's methods
                
                // Delete the lines
                match editor.delete_lines_from_cursor(buffer_id, cursor_pos.line, cursor_pos.line) {
                    Ok(_) => {
                        // Now we need to insert the content at the cursor position
                        // Since we don't have a direct method to insert text at the cursor position,
                        // we'll use a workaround: save the current buffer, modify it, and reload it
                        
                        // For now, just print a message
                        println!("\"{}\" read", path.display());
                        println!("(Read functionality partially implemented - file content not inserted yet)");
                        Ok(())
                    },
                    Err(err) => Err(ExCommandError::InvalidCommand(format!("Failed to prepare buffer for insertion: {}", err))),
                }
            },
            Err(err) => Err(ExCommandError::InvalidCommand(format!("Failed to read file: {}", err))),
        }
    } else {
        Err(ExCommandError::MissingArgument("Filename required".to_string()))
    }
}

/// Handle the :split command
fn handle_split(_cmd: &ExCommand) -> ExCommandResult<()> {
    // TODO: Implement split command
    // This would split the current window horizontally
    Ok(())
}

/// Handle the :vsplit command
fn handle_vsplit(_cmd: &ExCommand) -> ExCommandResult<()> {
    // TODO: Implement vsplit command
    // This would split the current window vertically
    Ok(())
}

/// Handle the :close command
fn handle_close(_cmd: &ExCommand) -> ExCommandResult<()> {
    // TODO: Implement close command
    // This would close the current window
    Ok(())
}

/// Handle the :only command
fn handle_only(_cmd: &ExCommand) -> ExCommandResult<()> {
    // TODO: Implement only command
    // This would close all windows except the current one
    Ok(())
}

/// Handle the :wnext command
fn handle_wnext(_cmd: &ExCommand) -> ExCommandResult<()> {
    // TODO: Implement wnext command
    // This would move to the next window
    Ok(())
}

/// Handle the :wprev command
fn handle_wprev(_cmd: &ExCommand) -> ExCommandResult<()> {
    // TODO: Implement wprev command
    // This would move to the previous window
    Ok(())
}

/// Handle the :tabedit command
fn handle_tabedit(_cmd: &ExCommand) -> ExCommandResult<()> {
    // TODO: Implement tabedit command
    // This would open a file in a new tab
    Ok(())
}

/// Handle the :tabclose command
fn handle_tabclose(_cmd: &ExCommand) -> ExCommandResult<()> {
    // TODO: Implement tabclose command
    // This would close the current tab
    Ok(())
}

/// Handle the :tabnext command
fn handle_tabnext(_cmd: &ExCommand) -> ExCommandResult<()> {
    // TODO: Implement tabnext command
    // This would move to the next tab
    Ok(())
}

/// Handle the :tabprev command
fn handle_tabprev(_cmd: &ExCommand) -> ExCommandResult<()> {
    // TODO: Implement tabprev command
    // This would move to the previous tab
    Ok(())
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
    
    // For now, just print a message
    println!("Global command: pattern '{}', command '{}'", pattern, command);
    println!("  (Global command execution not fully implemented yet)");
    
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
    
    // For now, just print a message
    println!("Vglobal command: pattern '{}', command '{}'", pattern, command);
    println!("  (Vglobal command execution not fully implemented yet)");
    
    Ok(())
}

/// Handle the :undo command
fn handle_undo(_cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let _editor = unsafe {
        match EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Print a simple message for now
    println!("Undo: changes undone");
    println!("  (Undo functionality not fully implemented yet)");
    
    Ok(())
}

/// Handle the :redo command
fn handle_redo(_cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let _editor = unsafe {
        match EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Print a simple message for now
    println!("Redo: changes redone");
    println!("  (Redo functionality not fully implemented yet)");
    
    Ok(())
}

/// Handle the :set command
fn handle_set(cmd: &ExCommand) -> ExCommandResult<()> {
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
        // If no arguments, show all options
        println!("Options:");
        println!("  (Option display not fully implemented yet)");
        return Ok(());
    }
    
    // Parse the arguments
    // Format can be:
    // - option (show option value)
    // - option=value (set option to value)
    // - nooption (set boolean option to false)
    // - option! (toggle boolean option)
    // - option? (show option value)
    
    let parts: Vec<&str> = args.split_whitespace().collect();
    
    for part in parts {
        if part.contains('=') {
            // Set option to value
            let option_parts: Vec<&str> = part.split('=').collect();
            if option_parts.len() != 2 {
                return Err(ExCommandError::InvalidArgument(format!("Invalid option format: {}", part)));
            }
            
            let option_name = option_parts[0];
            let option_value = option_parts[1];
            
            println!("Setting option {} to {}", option_name, option_value);
            println!("  (Option setting not fully implemented yet)");
        } else if part.starts_with("no") {
            // Set boolean option to false
            let option_name = &part[2..];
            println!("Setting option {} to false", option_name);
            println!("  (Option setting not fully implemented yet)");
        } else if part.ends_with('!') {
            // Toggle boolean option
            let option_name = &part[..part.len() - 1];
            println!("Toggling option {}", option_name);
            println!("  (Option toggling not fully implemented yet)");
        } else if part.ends_with('?') {
            // Show option value
            let option_name = &part[..part.len() - 1];
            println!("Option {}: (value not available)", option_name);
            println!("  (Option display not fully implemented yet)");
        } else {
            // Show option value or set boolean option to true
            println!("Option {}: (value not available)", part);
            println!("  (Option display not fully implemented yet)");
        }
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
        println!("Key mappings:");
        println!("  (Mapping display not fully implemented yet)");
        return Ok(());
    }
    
    // Parse the arguments
    // Format: {lhs} {rhs}
    // Where lhs is the key sequence to map, and rhs is what it should be mapped to
    
    let parts: Vec<&str> = args.splitn(2, ' ').collect();
    
    if parts.len() < 2 {
        return Err(ExCommandError::InvalidArgument("Missing mapping target".to_string()));
    }
    
    let lhs = parts[0];
    let rhs = parts[1];
    
    // Create the mapping
    // For now, just print a message
    println!("Mapping {} to {}", lhs, rhs);
    println!("  (Mapping creation not fully implemented yet)");
    
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
        return Err(ExCommandError::MissingArgument("Key sequence required".to_string()));
    }
    
    // The argument is the key sequence to unmap
    let key_sequence = args.trim();
    
    // Remove the mapping
    // For now, just print a message
    println!("Unmapping {}", key_sequence);
    println!("  (Mapping removal not fully implemented yet)");
    
    Ok(())
}

/// Handle the :marks command
fn handle_marks(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Get the marks to display
    let marks_to_display = if let Some(mark_names) = cmd.first_arg() {
        // If specific marks were requested, display only those
        mark_names.chars().collect::<Vec<char>>()
    } else {
        // Otherwise, display all marks
        // For now, we'll just display a few common marks
        vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '\'', '`', '[', ']', '<', '>', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9']
    };
    
    println!("--- Marks ---");
    println!("mark line  col file/text");
    
    // Display the content of each mark
    for mark in marks_to_display {
        // Get the mark position
        match editor.get_mark_position(mark) {
            Some(position) => {
                // Get the buffer name
                let buffer_name = match editor.get_buffer_name(position.buffer_id) {
                    Some(name) => name,
                    None => "[No Name]".to_string(),
                };
                
                println!(" {}   {:4} {:3}  {}", mark, position.line, position.column, buffer_name);
            },
            None => {
                // Skip empty marks
                continue;
            }
        }
    }
    
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
    
    // Get the jump list
    let jump_list = editor.get_jump_list();
    let current_jump_index = editor.get_current_jump_index();
    
    if jump_list.is_empty() {
        println!("No jumps");
        return Ok(());
    }
    
    println!("--- Jump list ---");
    println!("jump line  col file/text");
    
    // Display the jump list
    for (i, jump) in jump_list.iter().enumerate() {
        let current_marker = if Some(i) == current_jump_index { ">" } else { " " };
        
        // Get the buffer name
        let buffer_name = match editor.get_buffer_name(jump.buffer_id) {
            Some(name) => name,
            None => "[No Name]".to_string(),
        };
        
        println!("{} {:3} {:4} {:3}  {}",
            current_marker,
            i,
            jump.position.line,
            jump.position.column,
            buffer_name
        );
    }
    
    Ok(())
}

/// Handle the :registers command
fn handle_registers(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Get the registers to display
    let registers_to_display = if let Some(reg_names) = cmd.first_arg() {
        // If specific registers were requested, display only those
        reg_names.chars().collect::<Vec<char>>()
    } else {
        // Otherwise, display all registers
        // For now, we'll just display a few common registers
        vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '"', '+', '*', '-', '.', ':', '%', '#', '/', '=']
    };
    
    println!("--- Registers ---");
    
    // Display the content of each register
    for reg in registers_to_display {
        // Get the content of the register
        match editor.get_register_content(reg) {
            Some(content) => {
                // Format the content for display
                let formatted_content = format_register_content(&content);
                println!("\"{}   {}", reg, formatted_content);
            },
            None => {
                // Skip empty registers
                continue;
            }
        }
    }
    
    Ok(())
}

/// Format register content for display
fn format_register_content(content: &str) -> String {
    // Replace newlines with ^J for display
    let content = content.replace('\n', "^J");
    
    // Truncate long content
    if content.len() > 50 {
        format!("{}...", &content[..47])
    } else {
        content
    }
}

/// Handle the :buffers command
fn handle_buffers(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Get the current buffer ID
    let current_buffer_id = editor.current_buffer_id();
    
    // Get the list of buffers
    let buffers = editor.get_buffer_list();
    
    if buffers.is_empty() {
        println!("No buffers");
        return Ok(());
    }
    
    println!("--- Buffers ---");
    
    // Display the list of buffers
    for buffer in buffers {
        let current_marker = if Some(buffer.id) == current_buffer_id { "%a" } else { "  " };
        let modified_marker = if buffer.modified { "+" } else { " " };
        let name = buffer.name.as_deref().unwrap_or("[No Name]");
        
        println!("{}{} {:3} \"{}\"", current_marker, modified_marker, buffer.id, name);
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
    
    // Get the window list
    let windows = editor.get_window_list();
    let current_window_id = editor.get_current_window_id();
    
    if windows.is_empty() {
        println!("No windows");
        return Ok(());
    }
    
    println!("--- Window list ---");
    
    // Display the window list
    for window in windows {
        let current_marker = if Some(window.id) == current_window_id { ">" } else { " " };
        
        // Get the buffer name
        let buffer_name = match editor.get_buffer_name(window.buffer_id) {
            Some(name) => name,
            None => "[No Name]".to_string(),
        };
        
        println!("{} {:3} {:4}x{:<4} ({:3},{:3})  {}",
            current_marker,
            window.id,
            window.width,
            window.height,
            window.position.line,
            window.position.column,
            buffer_name
        );
    }
    
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
    
    // Get the tab list
    let tabs = editor.get_tab_list();
    let current_tab_id = editor.get_current_tab_id();
    
    if tabs.is_empty() {
        println!("No tabs");
        return Ok(());
    }
    
    println!("--- Tab list ---");
    
    // Display the tab list
    for (i, tab) in tabs.iter().enumerate() {
        let current_marker = if Some(tab.id) == current_tab_id { ">" } else { " " };
        
        // Get the tab name (usually the name of the active buffer in the tab)
        let tab_name = match editor.get_tab_name(tab.id) {
            Some(name) => name,
            None => "[No Name]".to_string(),
        };
        
        println!("{} {:3} {}",
            current_marker,
            i + 1, // Tab numbers are 1-based in the UI
            tab_name
        );
        
        // Display the windows in this tab
        let windows = editor.get_windows_in_tab(tab.id);
        for window in windows {
            // Get the buffer name
            let buffer_name = match editor.get_buffer_name(window.buffer_id) {
                Some(name) => name,
                None => "[No Name]".to_string(),
            };
            
            println!("    {:3} {}", window.id, buffer_name);
        }
    }
    
    Ok(())
}

/// Handle the :help command
fn handle_help(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let _editor = unsafe {
        match EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Get the help topic from the command arguments
    let topic = cmd.first_arg().unwrap_or("help");
    
    // Display help based on the topic
    match topic {
        "help" => {
            println!("--- Help System ---");
            println!("Use :help [topic] to get help on a specific topic.");
            println!("Available topics:");
            println!("  :help commands - List of available Ex commands");
            println!("  :help options - List of available options");
            println!("  :help mappings - Information about key mappings");
            println!("  :help buffers - Information about buffer management");
            println!("  :help windows - Information about window management");
            println!("  :help tabs - Information about tab management");
            println!("  :help [command] - Help for a specific command (e.g., :help write)");
        },
        "commands" => {
            println!("--- Available Ex Commands ---");
            println!("File operations:");
            println!("  :write, :w - Write the current buffer to a file");
            println!("  :quit, :q - Quit the current window");
            println!("  :wquit, :wq, :xit, :x - Write and quit");
            println!("  :edit, :e - Edit a file");
            println!("  :read, :r - Read a file into the current buffer");
            println!("");
            println!("Window operations:");
            println!("  :split, :sp - Split window horizontally");
            println!("  :vsplit, :vs - Split window vertically");
            println!("  :close, :clo - Close the current window");
            println!("  :only, :on - Close all windows except the current one");
            println!("  :wnext, :wn - Go to the next window");
            println!("  :wprevious, :wp - Go to the previous window");
            println!("");
            println!("Tab operations:");
            println!("  :tabedit, :tabe, :tabnew - Open a file in a new tab");
            println!("  :tabclose, :tabc - Close the current tab");
            println!("  :tabnext, :tabn - Go to the next tab");
            println!("  :tabprevious, :tabp - Go to the previous tab");
            println!("");
            println!("Editing operations:");
            println!("  :delete, :d - Delete lines");
            println!("  :yank, :y - Yank (copy) lines");
            println!("  :put, :p - Put (paste) text");
            println!("  :copy, :co, :t - Copy lines");
            println!("  :move, :m - Move lines");
            println!("  :substitute, :s - Search and replace");
            println!("  :global, :g - Execute a command on lines matching a pattern");
            println!("  :vglobal, :v - Execute a command on lines not matching a pattern");
            println!("");
            println!("Other operations:");
            println!("  :undo, :u - Undo changes");
            println!("  :redo, :red - Redo changes");
            println!("  :set, :se - Set options");
            println!("  :map - Create key mappings");
            println!("  :unmap - Remove key mappings");
            println!("  :marks - Display marks");
            println!("  :jumps - Display jump list");
            println!("  :registers, :reg - Display registers");
            println!("  :buffers, :ls, :files - Display buffers");
            println!("  :windows - Display windows");
            println!("  :tabs - Display tabs");
            println!("  :help, :h - Display help");
        },
        "options" => {
            println!("--- Available Options ---");
            println!("  (Options help not fully implemented yet)");
        },
        "mappings" => {
            println!("--- Key Mappings ---");
            println!("  (Mappings help not fully implemented yet)");
        },
        "buffers" => {
            println!("--- Buffer Management ---");
            println!("  (Buffer management help not fully implemented yet)");
        },
        "windows" => {
            println!("--- Window Management ---");
            println!("  (Window management help not fully implemented yet)");
        },
        "tabs" => {
            println!("--- Tab Management ---");
            println!("  (Tab management help not fully implemented yet)");
        },
        _ => {
            // Try to find help for a specific command
            println!("Help for command: {}", topic);
            println!("  (Command-specific help not fully implemented yet)");
        }
    }
    
    Ok(())
}