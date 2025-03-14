//! Ex command handlers
//!
//! This module implements handlers for ex commands.

use crate::command::{ExCommand, ExCommandError, ExCommandResult, ExCommandRegistry};
use crate::editor::Editor;

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

/// Register all ex command handlers
pub fn register_handlers(registry: &mut ExCommandRegistry) {
    // File operations
    registry.register("write", handle_write);
    registry.register("w", handle_write);
    registry.register("quit", handle_quit);
    registry.register("q", handle_quit);
    registry.register("wquit", handle_write_quit);
    registry.register("wq", handle_write_quit);
    registry.register("xit", handle_write_quit);
    registry.register("x", handle_write_quit);
    registry.register("edit", handle_edit);
    registry.register("e", handle_edit);
    registry.register("read", handle_read);
    registry.register("r", handle_read);
    
    // Window operations
    registry.register("split", handle_split);
    registry.register("sp", handle_split);
    registry.register("vsplit", handle_vsplit);
    registry.register("vs", handle_vsplit);
    registry.register("close", handle_close);
    registry.register("clo", handle_close);
    registry.register("only", handle_only);
    registry.register("on", handle_only);
    registry.register("wnext", handle_wnext);
    registry.register("wn", handle_wnext);
    registry.register("wprevious", handle_wprev);
    registry.register("wp", handle_wprev);
    
    // Tab operations
    registry.register("tabedit", handle_tabedit);
    registry.register("tabe", handle_tabedit);
    registry.register("tabnew", handle_tabedit);
    registry.register("tabclose", handle_tabclose);
    registry.register("tabc", handle_tabclose);
    registry.register("tabnext", handle_tabnext);
    registry.register("tabn", handle_tabnext);
    registry.register("tabprevious", handle_tabprev);
    registry.register("tabp", handle_tabprev);
    
    // Editing operations
    registry.register("delete", handle_delete);
    registry.register("d", handle_delete);
    registry.register("yank", handle_yank);
    registry.register("y", handle_yank);
    registry.register("put", handle_put);
    registry.register("p", handle_put);
    registry.register("copy", handle_copy);
    registry.register("co", handle_copy);
    registry.register("t", handle_copy);
    registry.register("move", handle_move);
    registry.register("m", handle_move);
    registry.register("substitute", handle_substitute);
    registry.register("s", handle_substitute);
    registry.register("global", handle_global);
    registry.register("g", handle_global);
    registry.register("vglobal", handle_vglobal);
    registry.register("v", handle_vglobal);
    
    // Other operations
    registry.register("undo", handle_undo);
    registry.register("u", handle_undo);
    registry.register("redo", handle_redo);
    registry.register("red", handle_redo);
    registry.register("set", handle_set);
    registry.register("se", handle_set);
    registry.register("map", handle_map);
    registry.register("unmap", handle_unmap);
    registry.register("marks", handle_marks);
    registry.register("jumps", handle_jumps);
    registry.register("registers", handle_registers);
    registry.register("reg", handle_registers);
    registry.register("buffers", handle_buffers);
    registry.register("ls", handle_buffers);
    registry.register("files", handle_buffers);
    registry.register("windows", handle_windows);
    registry.register("tabs", handle_tabs);
    registry.register("help", handle_help);
    registry.register("h", handle_help);
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
fn handle_edit(_cmd: &ExCommand) -> ExCommandResult<()> {
    // TODO: Implement edit command
    // This would open a file for editing
    // If no filename is provided, it would reload the current file
    Ok(())
}

/// Handle the :read command
fn handle_read(_cmd: &ExCommand) -> ExCommandResult<()> {
    // TODO: Implement read command
    // This would read a file into the current buffer
    Ok(())
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
fn handle_yank(_cmd: &ExCommand) -> ExCommandResult<()> {
    // TODO: Implement yank command
    // This would yank lines from the buffer
    Ok(())
}

/// Handle the :put command
fn handle_put(_cmd: &ExCommand) -> ExCommandResult<()> {
    // TODO: Implement put command
    // This would put yanked text into the buffer
    Ok(())
}

/// Handle the :copy command
fn handle_copy(_cmd: &ExCommand) -> ExCommandResult<()> {
    // TODO: Implement copy command
    // This would copy lines from one location to another
    Ok(())
}

/// Handle the :move command
fn handle_move(_cmd: &ExCommand) -> ExCommandResult<()> {
    // TODO: Implement move command
    // This would move lines from one location to another
    Ok(())
}

/// Handle the :substitute command
fn handle_substitute(_cmd: &ExCommand) -> ExCommandResult<()> {
    // TODO: Implement substitute command
    // This would perform search and replace
    Ok(())
}

/// Handle the :global command
fn handle_global(_cmd: &ExCommand) -> ExCommandResult<()> {
    // TODO: Implement global command
    // This would execute a command on lines matching a pattern
    Ok(())
}

/// Handle the :vglobal command
fn handle_vglobal(_cmd: &ExCommand) -> ExCommandResult<()> {
    // TODO: Implement vglobal command
    // This would execute a command on lines not matching a pattern
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
fn handle_set(_cmd: &ExCommand) -> ExCommandResult<()> {
    // TODO: Implement set command
    // This would set options
    Ok(())
}

/// Handle the :map command
fn handle_map(_cmd: &ExCommand) -> ExCommandResult<()> {
    // TODO: Implement map command
    // This would create a key mapping
    Ok(())
}

/// Handle the :unmap command
fn handle_unmap(_cmd: &ExCommand) -> ExCommandResult<()> {
    // TODO: Implement unmap command
    // This would remove a key mapping
    Ok(())
}

/// Handle the :marks command
fn handle_marks(_cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let _editor = unsafe {
        match EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Print a simple message for now
    println!("Mark list:");
    println!("  (Mark list display not fully implemented yet)");
    
    Ok(())
}

/// Handle the :jumps command
fn handle_jumps(_cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let _editor = unsafe {
        match EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Print a simple message for now
    println!("Jump list:");
    println!("  (Jump list display not fully implemented yet)");
    
    Ok(())
}

/// Handle the :registers command
fn handle_registers(_cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let _editor = unsafe {
        match EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Print a simple message for now
    println!("Register list:");
    println!("  (Register list display not fully implemented yet)");
    
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
fn handle_buffers(_cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Get the current buffer ID
    let current_buffer_id = editor.current_buffer_id();
    
    // Print a simple message for now
    println!("Buffer list (current: {:?}):", current_buffer_id);
    println!("  (Buffer list display not fully implemented yet)");
    
    Ok(())
}

/// Handle the :windows command
fn handle_windows(_cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let _editor = unsafe {
        match EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Print a simple message for now
    println!("Window list:");
    println!("  (Window list display not fully implemented yet)");
    
    Ok(())
}

/// Handle the :tabs command
fn handle_tabs(_cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let _editor = unsafe {
        match EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Print a simple message for now
    println!("Tab list:");
    println!("  (Tab list display not fully implemented yet)");
    
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
    
    // Print a simple message for now
    println!("Help for topic: {}", topic);
    println!("  (Help system not fully implemented yet)");
    
    Ok(())
}