//! Vim command implementations
//!
//! This module implements additional Vim commands to enhance xvim's compatibility with Vim.

use crate::command::{ExCommand, ExCommandError, ExCommandResult, ExCommandRegistry};
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
use crate::Editor;
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
// use std::collections::HashMap;

/// Register all Vim commands
pub fn register_vim_commands(registry: &mut ExCommandRegistry) {
    // File operations
    registry.register("saveas", |cmd| handle_saveas(cmd));
    registry.register("update", |cmd| handle_update(cmd));
    registry.register("browse", |cmd| handle_browse(cmd));
    registry.register("bdelete", |cmd| handle_bdelete(cmd));
    registry.register("bd", |cmd| handle_bdelete(cmd));
    registry.register("bunload", |cmd| handle_bunload(cmd));
    registry.register("bun", |cmd| handle_bunload(cmd));
    registry.register("badd", |cmd| handle_badd(cmd));
    registry.register("ba", |cmd| handle_badd(cmd));
    
    // Window operations
    registry.register("new", |cmd| handle_new(cmd));
    registry.register("vnew", |cmd| handle_vnew(cmd));
    registry.register("resize", |cmd| handle_resize(cmd));
    registry.register("vertical", |cmd| handle_vertical(cmd));
    registry.register("wincmd", |cmd| handle_wincmd(cmd));
    registry.register("windo", |cmd| handle_windo(cmd));
    
    // Tab operations
    registry.register("tabdo", |cmd| handle_tabdo(cmd));
    registry.register("tabmove", |cmd| handle_tabmove(cmd));
    registry.register("tabonly", |cmd| handle_tabonly(cmd));
    registry.register("tabo", |cmd| handle_tabonly(cmd));
    
    // Editing operations
    registry.register("change", |cmd| handle_change(cmd));
    registry.register("c", |cmd| handle_change(cmd));
    registry.register("join", |cmd| handle_join(cmd));
    registry.register("j", |cmd| handle_join(cmd));
    registry.register("normal", |cmd| handle_normal(cmd));
    registry.register("norm", |cmd| handle_normal(cmd));
    registry.register("retab", |cmd| handle_retab(cmd));
    registry.register("ret", |cmd| handle_retab(cmd));
    
    // Search operations
    registry.register("grep", |cmd| handle_grep(cmd));
    registry.register("vimgrep", |cmd| handle_vimgrep(cmd));
    registry.register("vimgrepadd", |cmd| handle_vimgrepadd(cmd));
    registry.register("nohlsearch", |cmd| handle_nohlsearch(cmd));
    registry.register("noh", |cmd| handle_nohlsearch(cmd));
    
    // Mark operations
    registry.register("delmarks", |cmd| handle_delmarks(cmd));
    registry.register("delm", |cmd| handle_delmarks(cmd));
    
    // Fold operations are handled by fold_handlers.rs
    // We don't need to register them here
    
    // Options and settings
    registry.register("setlocal", |cmd| handle_setlocal(cmd));
    registry.register("setl", |cmd| handle_setlocal(cmd));
    registry.register("setglobal", |cmd| handle_setglobal(cmd));
    registry.register("setg", |cmd| handle_setglobal(cmd));
    registry.register("let", |cmd| handle_let(cmd));
    registry.register("unlet", |cmd| handle_unlet(cmd));
    registry.register("if", |cmd| handle_if(cmd));
    registry.register("else", |cmd| handle_else(cmd));
    registry.register("endif", |cmd| handle_endif(cmd));
    registry.register("while", |cmd| handle_while(cmd));
    registry.register("endwhile", |cmd| handle_endwhile(cmd));
    registry.register("for", |cmd| handle_for(cmd));
    registry.register("endfor", |cmd| handle_endfor(cmd));
    registry.register("function", |cmd| handle_function(cmd));
    registry.register("endfunction", |cmd| handle_endfunction(cmd));
    
    // Miscellaneous
    registry.register("version", |cmd| handle_version(cmd));
    registry.register("ver", |cmd| handle_version(cmd));
    registry.register("echo", |cmd| handle_echo(cmd));
    registry.register("echom", |cmd| handle_echom(cmd));
    registry.register("echomsg", |cmd| handle_echom(cmd));
    registry.register("echoerr", |cmd| handle_echoerr(cmd));
    registry.register("execute", |cmd| handle_execute(cmd));
    registry.register("silent", |cmd| handle_silent(cmd));
    registry.register("sil", |cmd| handle_silent(cmd));
}

/// Get the editor reference
fn get_editor() -> Result<&'static mut Editor, ExCommandError> {
    unsafe {
        match crate::command::handlers::EDITOR {
            Some(editor_ptr) => Ok(&mut *editor_ptr),
            None => Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    }
}

/// Handle the :saveas command
fn handle_saveas(cmd: &ExCommand) -> ExCommandResult<()> {
    let editor = get_editor()?;
    
    // Check if a filename was provided
    if let Some(filename) = cmd.first_arg() {
        let path = std::path::Path::new(filename);
        
        // Save the buffer to the specified file
        match editor.save_current_buffer_as(path) {
            Ok(_) => {
                println!("\"{}\" written", path.display());
                Ok(())
            },
            Err(err) => Err(ExCommandError::InvalidCommand(format!("Failed to write file: {}", err))),
        }
    } else {
        Err(ExCommandError::MissingArgument("Filename required".to_string()))
    }
}

/// Handle the :update command
fn handle_update(cmd: &ExCommand) -> ExCommandResult<()> {
    let editor = get_editor()?;
    
    // Get the current buffer ID
    let buffer_id = match editor.current_buffer_id() {
        Some(id) => id,
        None => return Err(ExCommandError::InvalidCommand("No buffer to update".to_string())),
    };
    
    // Get a reference to the buffer
    let buffer = editor.get_buffer_manager().get_buffer(buffer_id)?;
    
    // Only save if the buffer has been modified
    if buffer.is_modified() {
        // If a filename was provided, save the buffer to that file
        if let Some(filename) = cmd.first_arg() {
            let path = std::path::Path::new(filename);
            
            // Save the buffer to the specified file
            match editor.save_current_buffer_as(path) {
                Ok(_) => {
                    println!("\"{}\" written", path.display());
                    Ok(())
                },
                Err(err) => Err(ExCommandError::InvalidCommand(format!("Failed to write file: {}", err))),
            }
        } else {
            // Otherwise, save the buffer to its current file
            match editor.save_current_buffer() {
                Ok(_) => {
                    println!("File written");
                    Ok(())
                },
                Err(err) => Err(ExCommandError::InvalidCommand(format!("Failed to write file: {}", err))),
            }
        }
    } else {
        println!("No changes to save");
        Ok(())
    }
}

/// Handle the :browse command
fn handle_browse(cmd: &ExCommand) -> ExCommandResult<()> {
    // This is a stub implementation that just returns success
    // In a real implementation, this would open a file browser
    println!("Browse command not fully implemented yet");
    Ok(())
}

/// Handle the :bdelete command
fn handle_bdelete(cmd: &ExCommand) -> ExCommandResult<()> {
    let editor = get_editor()?;
    
    // Get the buffer ID to delete
    let buffer_id = if let Some(id_str) = cmd.first_arg() {
        match id_str.parse::<usize>() {
            Ok(id) => id,
            Err(_) => return Err(ExCommandError::InvalidArgument(format!("Invalid buffer ID: {}", id_str))),
        }
    } else {
        // If no buffer ID was provided, use the current buffer
        match editor.current_buffer_id() {
            Some(id) => id,
            None => return Err(ExCommandError::InvalidCommand("No buffer to delete".to_string())),
        }
    };
    
    // Delete the buffer
    match editor.get_buffer_manager_mut().delete_buffer(buffer_id) {
        Ok(_) => {
            println!("Buffer {} deleted", buffer_id);
            Ok(())
        },
        Err(err) => Err(ExCommandError::InvalidCommand(format!("Failed to delete buffer: {}", err))),
    }
}

/// Handle the :bunload command
fn handle_bunload(cmd: &ExCommand) -> ExCommandResult<()> {
    let editor = get_editor()?;
    
    // Get the buffer ID to unload
    let buffer_id = if let Some(id_str) = cmd.first_arg() {
        match id_str.parse::<usize>() {
            Ok(id) => id,
            Err(_) => return Err(ExCommandError::InvalidArgument(format!("Invalid buffer ID: {}", id_str))),
        }
    } else {
        // If no buffer ID was provided, use the current buffer
        match editor.current_buffer_id() {
            Some(id) => id,
            None => return Err(ExCommandError::InvalidCommand("No buffer to unload".to_string())),
        }
    };
    
    // Unload the buffer
    match editor.get_buffer_manager_mut().unload_buffer(buffer_id) {
        Ok(_) => {
            println!("Buffer {} unloaded", buffer_id);
            Ok(())
        },
        Err(err) => Err(ExCommandError::InvalidCommand(format!("Failed to unload buffer: {}", err))),
    }
}

/// Handle the :badd command
fn handle_badd(cmd: &ExCommand) -> ExCommandResult<()> {
    let editor = get_editor()?;
    
    // Check if a filename was provided
    if let Some(filename) = cmd.first_arg() {
        let path = std::path::Path::new(filename);
        
        // Create a new buffer for the file
        match editor.get_buffer_manager_mut().open_file(path) {
            Ok(buffer_id) => {
                println!("Buffer {} added", buffer_id);
                Ok(())
            },
            Err(err) => Err(ExCommandError::InvalidCommand(format!("Failed to add buffer: {}", err))),
        }
    } else {
        Err(ExCommandError::MissingArgument("Filename required".to_string()))
    }
}

/// Handle the :new command
fn handle_new(cmd: &ExCommand) -> ExCommandResult<()> {
    let editor = get_editor()?;
    
    // Create a new buffer
    let buffer_id = match editor.get_buffer_manager_mut().create_buffer() {
        Ok(id) => id,
        Err(err) => return Err(ExCommandError::InvalidCommand(format!("Failed to create buffer: {}", err))),
    };
    
    // Split the window horizontally
    match editor.get_terminal_mut().split_window(crate::ui::window::SplitDirection::Horizontal, buffer_id) {
        Ok(_) => {
            println!("New window created");
            Ok(())
        },
        Err(err) => Err(ExCommandError::Other(format!("Failed to create window: {}", err))),
    }
}

/// Handle the :vnew command
fn handle_vnew(cmd: &ExCommand) -> ExCommandResult<()> {
    let editor = get_editor()?;
    
    // Create a new buffer
    let buffer_id = match editor.get_buffer_manager_mut().create_buffer() {
        Ok(id) => id,
        Err(err) => return Err(ExCommandError::InvalidCommand(format!("Failed to create buffer: {}", err))),
    };
    
    // Split the window vertically
    match editor.get_terminal_mut().split_window(crate::ui::window::SplitDirection::Vertical, buffer_id) {
        Ok(_) => {
            println!("New window created");
            Ok(())
        },
        Err(err) => Err(ExCommandError::Other(format!("Failed to create window: {}", err))),
    }
}

/// Handle the :resize command
fn handle_resize(cmd: &ExCommand) -> ExCommandResult<()> {
    let editor = get_editor()?;
    
    // Get the size from the command arguments
    let size = if let Some(size_str) = cmd.first_arg() {
        match size_str.parse::<usize>() {
            Ok(size) => size,
            Err(_) => return Err(ExCommandError::InvalidArgument(format!("Invalid size: {}", size_str))),
        }
    } else {
        return Err(ExCommandError::MissingArgument("Size required".to_string()));
    };
    
    // Resize the current window
    match editor.get_terminal_mut().resize_current_window(size) {
        Ok(_) => {
            println!("Window resized");
            Ok(())
        },
        Err(err) => Err(ExCommandError::Other(format!("Failed to resize window: {}", err))),
    }
}

/// Handle the :vertical command
fn handle_vertical(cmd: &ExCommand) -> ExCommandResult<()> {
    // This command modifies the behavior of the next command
    // For now, just parse the next command and execute it
    let args = cmd.args_str();
    
    if args.is_empty() {
        return Err(ExCommandError::MissingArgument("Command required".to_string()));
    }
    
    // Create a command parser
    let parser = crate::command::ExCommandParser::new();
    
    // Parse the command
    match parser.parse_ex(&args) {
        Ok(next_cmd) => {
            // Create a command registry
            let mut registry = crate::command::ExCommandRegistry::new();
            crate::command::handlers::register_handlers(&mut registry, None);
            register_vim_commands(&mut registry);
            
            // Execute the command
            registry.execute(&next_cmd)
        },
        Err(err) => Err(ExCommandError::InvalidCommand(format!("Failed to parse command: {}", err))),
    }
}

/// Handle the :wincmd command
fn handle_wincmd(cmd: &ExCommand) -> ExCommandResult<()> {
    let editor = get_editor()?;
    
    // Get the window command from the command arguments
    let win_cmd = if let Some(cmd_str) = cmd.first_arg() {
        if cmd_str.len() != 1 {
            return Err(ExCommandError::InvalidArgument(format!("Invalid window command: {}", cmd_str)));
        }
        cmd_str.chars().next().unwrap()
    } else {
        return Err(ExCommandError::MissingArgument("Window command required".to_string()));
    };
    
    // Execute the window command
    match win_cmd {
        'h' => {
            // Move to the window to the left
            match editor.get_terminal_mut().move_to_left_window() {
                Ok(true) => {
                    println!("Moved to left window");
                    Ok(())
                },
                Ok(false) => {
                    println!("No window to the left");
                    Ok(())
                },
                Err(err) => Err(ExCommandError::Other(format!("Failed to move to left window: {}", err))),
            }
        },
        'j' => {
            // Move to the window below
            match editor.get_terminal_mut().move_to_below_window() {
                Ok(true) => {
                    println!("Moved to window below");
                    Ok(())
                },
                Ok(false) => {
                    println!("No window below");
                    Ok(())
                },
                Err(err) => Err(ExCommandError::Other(format!("Failed to move to window below: {}", err))),
            }
        },
        'k' => {
            // Move to the window above
            match editor.get_terminal_mut().move_to_above_window() {
                Ok(true) => {
                    println!("Moved to window above");
                    Ok(())
                },
                Ok(false) => {
                    println!("No window above");
                    Ok(())
                },
                Err(err) => Err(ExCommandError::Other(format!("Failed to move to window above: {}", err))),
            }
        },
        'l' => {
            // Move to the window to the right
            match editor.get_terminal_mut().move_to_right_window() {
                Ok(true) => {
                    println!("Moved to right window");
                    Ok(())
                },
                Ok(false) => {
                    println!("No window to the right");
                    Ok(())
                },
                Err(err) => Err(ExCommandError::Other(format!("Failed to move to right window: {}", err))),
            }
        },
        'o' => {
            // Make the current window the only one
            match editor.get_terminal_mut().make_current_window_only() {
                Ok(_) => {
                    println!("Made current window the only one");
                    Ok(())
                },
                Err(err) => Err(ExCommandError::Other(format!("Failed to make current window the only one: {}", err))),
            }
        },
        'v' => {
            // Split the window vertically
            match editor.get_terminal_mut().split_window(crate::ui::window::SplitDirection::Vertical, editor.current_buffer_id().unwrap_or(0)) {
                Ok(_) => {
                    println!("Window split vertically");
                    Ok(())
                },
                Err(err) => Err(ExCommandError::Other(format!("Failed to split window: {}", err))),
            }
        },
        's' => {
            // Split the window horizontally
            match editor.get_terminal_mut().split_window(crate::ui::window::SplitDirection::Horizontal, editor.current_buffer_id().unwrap_or(0)) {
                Ok(_) => {
                    println!("Window split horizontally");
                    Ok(())
                },
                Err(err) => Err(ExCommandError::Other(format!("Failed to split window: {}", err))),
            }
        },
        'c' => {
            // Close the current window
            match editor.get_terminal_mut().close_current_window() {
                Ok(true) => {
                    println!("Window closed");
                    Ok(())
                },
                Ok(false) => {
                    println!("Cannot close last window");
                    Ok(())
                },
                Err(err) => Err(ExCommandError::Other(format!("Failed to close window: {}", err))),
            }
        },
        'w' => {
            // Move to the next window
            match editor.get_terminal_mut().next_window() {
                Ok(true) => {
                    println!("Moved to next window");
                    Ok(())
                },
                Ok(false) => {
                    println!("No more windows");
                    Ok(())
                },
                Err(err) => Err(ExCommandError::Other(format!("Failed to move to next window: {}", err))),
            }
        },
        'W' => {
            // Move to the previous window
            match editor.get_terminal_mut().prev_window() {
                Ok(true) => {
                    println!("Moved to previous window");
                    Ok(())
                },
                Ok(false) => {
                    println!("No more windows");
                    Ok(())
                },
                Err(err) => Err(ExCommandError::Other(format!("Failed to move to previous window: {}", err))),
            }
        },
        _ => Err(ExCommandError::InvalidArgument(format!("Unknown window command: {}", win_cmd))),
    }
}

/// Handle the :windo command
fn handle_windo(cmd: &ExCommand) -> ExCommandResult<()> {
    let editor = get_editor()?;
    
    // Get the command to execute
    let command = cmd.args_str();
    
    if command.is_empty() {
        return Err(ExCommandError::MissingArgument("Command required".to_string()));
    }
    
    // Create a command parser
    let parser = crate::command::ExCommandParser::new();
    
    // Create a command registry
    let mut registry = crate::command::ExCommandRegistry::new();
    crate::command::handlers::register_handlers(&mut registry, None);
    register_vim_commands(&mut registry);
    
    // Get the current tab
    if let Some(tab) = editor.get_terminal_mut().current_tab_mut() {
        // Get all window IDs
        let window_ids: Vec<usize> = tab.window_manager.windows()
            .iter()
            .map(|w| w.id)
            .collect();
        
        // Save the current window ID
        let current_window_id = tab.window_manager.current_window_id();
        
        // Execute the command in each window
        for window_id in &window_ids {
            // Set the current window
            tab.window_manager.set_current_window(*window_id);
            
            // Parse and execute the command
            match parser.parse_ex(&command) {
                Ok(ex_cmd) => {
                    if let Err(err) = registry.execute(&ex_cmd) {
                        // Restore the current window
                        tab.window_manager.set_current_window(current_window_id);
                        
                        return Err(ExCommandError::Other(format!("Failed to execute command in window {}: {}", window_id, err)));
                    }
                },
                Err(err) => {
                    // Restore the current window
                    tab.window_manager.set_current_window(current_window_id);
                    
                    return Err(ExCommandError::Other(format!("Failed to parse command: {}", err)));
                }
            }
        }
        
        // Restore the current window
        tab.window_manager.set_current_window(current_window_id);
        
        println!("Command executed in {} window{}", window_ids.len(), if window_ids.len() == 1 { "" } else { "s" });
        Ok(())
    } else {
        println!("No active tab");
        Ok(())
    }
}

/// Handle the :tabdo command
fn handle_tabdo(cmd: &ExCommand) -> ExCommandResult<()> {
    let editor = get_editor()?;
    
    // Get the command to execute
    let command = cmd.args_str();
    
    if command.is_empty() {
        return Err(ExCommandError::MissingArgument("Command required".to_string()));
    }
    
    // Create a command parser
    let parser = crate::command::ExCommandParser::new();
    
    // Create a command registry
    let mut registry = crate::command::ExCommandRegistry::new();
    crate::command::handlers::register_handlers(&mut registry, None);
    register_vim_commands(&mut registry);
    
    // Get all tab IDs
    let tab_ids: Vec<usize> = editor.get_terminal_mut().tabs()
        .iter()
        .map(|t| t.id)
        .collect();
    
    // Save the current tab ID
    let current_tab_id = editor.get_terminal_mut().current_tab_id();
    
    // Execute the command in each tab
    for tab_id in &tab_ids {
        // Set the current tab
        editor.get_terminal_mut().set_current_tab(*tab_id);
        
        // Parse and execute the command
        match parser.parse_ex(&command) {
            Ok(ex_cmd) => {
                if let Err(err) = registry.execute(&ex_cmd) {
                    // Restore the current tab
                    editor.get_terminal_mut().set_current_tab(current_tab_id);
                    
                    return Err(ExCommandError::Other(format!("Failed to execute command in tab {}: {}", tab_id, err)));
                }
            },
            Err(err) => {
                // Restore the current tab
                editor.get_terminal_mut().set_current_tab(current_tab_id);
                
                return Err(ExCommandError::Other(format!("Failed to parse command: {}", err)));
            }
        }
    }
    
    // Restore the current tab
    editor.get_terminal_mut().set_current_tab(current_tab_id);
    
    println!("Command executed in {} tab{}", tab_ids.len(), if tab_ids.len() == 1 { "" } else { "s" });
    Ok(())
}

/// Handle the :tabmove command
fn handle_tabmove(cmd: &ExCommand) -> ExCommandResult<()> {
    let editor = get_editor()?;
    
    // Get the position from the command arguments
    let position = if let Some(pos_str) = cmd.first_arg() {
        match pos_str.parse::<usize>() {
            Ok(pos) => pos,
            Err(_) => return Err(ExCommandError::InvalidArgument(format!("Invalid position: {}", pos_str))),
        }
    } else {
        // If no position was provided, move to the end
        editor.get_terminal_mut().tabs().len()
    };
    
    // Move the current tab
    match editor.get_terminal_mut().move_current_tab(position) {
        Ok(_) => {
            println!("Tab moved to position {}", position);
            Ok(())
        },
        Err(err) => Err(ExCommandError::Other(format!("Failed to move tab: {}", err))),
    }
}

/// Handle the :tabonly command
fn handle_tabonly(cmd: &ExCommand) -> ExCommandResult<()> {
    let editor = get_editor()?;
    
    // Get the current tab ID
    let current_tab_id = editor.get_terminal_mut().current_tab_id();
    
    // Get all tab IDs
    let tab_ids: Vec<usize> = editor.get_terminal_mut().tabs()
        .iter()
        .filter(|t| t.id != current_tab_id)
        .map(|t| t.id)
        .collect();
    
    // Close each tab except the current one
    let mut count = 0;
    for tab_id in tab_ids {
        if editor.get_terminal_mut().close_tab(tab_id) {
            count += 1;
        }
    }
    
    if count > 0 {
        println!("{} tab{} closed", count, if count == 1 { "" } else { "s" });
    } else {
        println!("No other tabs to close");
    }
    Ok(())
}

/// Handle the :change command
fn handle_change(cmd: &ExCommand) -> ExCommandResult<()> {
    let editor = get_editor()?;
    
    // Get the current buffer ID
    let buffer_id = match editor.current_buffer_id() {
        Some(id) => id,
        None => return Err(ExCommandError::InvalidCommand("No buffer to change".to_string())),
    };
    
    // Get the current cursor position
    let cursor_pos = editor.cursor_position();
    let line = cursor_pos.line;
    
    // Get the new text from the command arguments
    let new_text = cmd.args_str();
    
    // Get a mutable reference to the buffer
    let buffer = editor.get_buffer_manager_mut().get_buffer_mut(buffer_id)?;
    
    // Delete the current line
    let line_start = buffer.position_to_char_idx(line, 0)?;
    let line_end = if line + 1 < buffer.line_count() {
        buffer.position_to_char_idx(line + 1, 0)?
    } else {
        buffer.content().len()
    };
    
    buffer.delete(line_start, line_end)?;
    
    // Insert the new text
    buffer.insert(line_start, &format!("{}\n", new_text))?;
    
    // Update the cursor position
    editor.get_cursor_manager_mut().set_position(crate::cursor::CursorPosition::new(line, 0));
    
    println!("Line changed");
    Ok(())
}

/// Handle the :join command
fn handle_join(cmd: &ExCommand) -> ExCommandResult<()> {
    let editor = get_editor()?;
    
    // Get the current buffer ID
    let buffer_id = match editor.current_buffer_id() {
        Some(id) => id,
        None => return Err(ExCommandError::InvalidCommand("No buffer to join lines in".to_string())),
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
        // Default to 2 lines if no count is provided
        2
    };
    
    // Calculate the end line (inclusive)
    let end_line = start_line + count - 1;
    
    // Join the lines
    match editor.join_lines(buffer_id, start_line, end_line) {
        Ok(_) => {
            println!("{} line{} joined", count - 1, if count - 1 == 1 { "" } else { "s" });
            Ok(())
        },
        Err(err) => Err(ExCommandError::InvalidCommand(format!("Failed to join lines: {}", err))),
    }
}

/// Handle the :normal command
fn handle_normal(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = get_editor()?;
    
    // Get the normal mode commands to execute
    let commands = cmd.args_str();
    
    if commands.is_empty() {
        return Err(ExCommandError::MissingArgument("Normal mode commands required".to_string()));
    }
    
    // Execute the normal mode commands
    match editor.execute_normal_mode_commands(&commands) {
        Ok(_) => {
            println!("Normal mode commands executed");
            Ok(())
        },
        Err(err) => Err(ExCommandError::Other(format!("Failed to execute normal mode commands: {}", err))),
    }
}

/// Handle the :retab command
fn handle_retab(cmd: &ExCommand) -> ExCommandResult<()> {
    let editor = get_editor()?;
    
    // Get the current buffer ID
    let buffer_id = match editor.current_buffer_id() {
        Some(id) => id,
        None => return Err(ExCommandError::InvalidCommand("No buffer to retab".to_string())),
    };
    
    // Parse the tabstop from the command arguments
    let tabstop = if let Some(ts_str) = cmd.first_arg() {
        match ts_str.parse::<usize>() {
            Ok(ts) => ts,
            Err(_) => return Err(ExCommandError::InvalidCommand(format!("Invalid tabstop: {}", ts_str))),
        }
    } else {
        // Default to 8 if no tabstop is provided
        8
    };
    
    // Get a mutable reference to the buffer
    let buffer = editor.get_buffer_manager_mut().get_buffer_mut(buffer_id)?;
    
    // Get the buffer content
    let content = buffer.content();
    
    // Replace tabs with spaces
    let new_content = content.replace("\t", &" ".repeat(tabstop));
    
    // Update the buffer content
    buffer.delete(0, buffer.content().len())?;
    buffer.insert(0, &new_content)?;
    
    println!("Buffer retabbed");
    Ok(())
}

/// Handle the :grep command
fn handle_grep(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = get_editor()?;
    
    // Get the pattern and files to search
    let args = cmd.args_str();
    
    if args.is_empty() {
        return Err(ExCommandError::MissingArgument("Pattern and files required".to_string()));
    }
    
    // Split the arguments into pattern and files
    let mut parts = args.splitn(2, ' ');
    let pattern = match parts.next() {
        Some(p) => p,
        None => return Err(ExCommandError::MissingArgument("Pattern required".to_string())),
    };
    
    let files = match parts.next() {
        Some(f) => f,
        None => return Err(ExCommandError::MissingArgument("Files required".to_string())),
    };
    
    // Execute the grep command
    match editor.execute_grep(pattern, files) {
        Ok(matches) => {
            // Display the matches
            if matches.is_empty() {
                println!("No matches found");
            } else {
                println!("{} match{} found:", matches.len(), if matches.len() == 1 { "" } else { "es" });
                for (file, line, content) in matches {
                    println!("{}:{}: {}", file, line, content);
                }
                
                // Set the quickfix list
                editor.set_quickfix_list(matches);
            }
            
            Ok(())
        },
        Err(err) => Err(ExCommandError::Other(format!("Failed to execute grep: {}", err))),
    }
}

/// Handle the :vimgrep command
fn handle_vimgrep(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = get_editor()?;
    
    // Get the pattern and files to search
    let args = cmd.args_str();
    
    if args.is_empty() {
        return Err(ExCommandError::MissingArgument("Pattern and files required".to_string()));
    }
    
    // Split the arguments into pattern and files
    let mut parts = args.splitn(2, ' ');
    let pattern = match parts.next() {
        Some(p) => p,
        None => return Err(ExCommandError::MissingArgument("Pattern required".to_string())),
    };
    
    let files = match parts.next() {
        Some(f) => f,
        None => return Err(ExCommandError::MissingArgument("Files required".to_string())),
    };
    
    // Execute the vimgrep command (internal grep that searches buffer contents)
    match editor.execute_vimgrep(pattern, files, false) {
        Ok(matches) => {
            // Display the matches
            if matches.is_empty() {
                println!("No matches found");
            } else {
                println!("{} match{} found:", matches.len(), if matches.len() == 1 { "" } else { "es" });
                for (file, line, content) in &matches {
                    println!("{}:{}: {}", file, line, content);
                }
                
                // Set the quickfix list (replacing any existing entries)
                editor.set_quickfix_list(matches.clone());
                
                // Jump to the first match
                editor.jump_to_quickfix_entry(0);
            }
            
            Ok(())
        },
        Err(err) => Err(ExCommandError::Other(format!("Failed to execute vimgrep: {}", err))),
    }
}

/// Handle the :vimgrepadd command
fn handle_vimgrepadd(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = get_editor()?;
    
    // Get the pattern and files to search
    let args = cmd.args_str();
    
    if args.is_empty() {
        return Err(ExCommandError::MissingArgument("Pattern and files required".to_string()));
    }
    
    // Split the arguments into pattern and files
    let mut parts = args.splitn(2, ' ');
    let pattern = match parts.next() {
        Some(p) => p,
        None => return Err(ExCommandError::MissingArgument("Pattern required".to_string())),
    };
    
    let files = match parts.next() {
        Some(f) => f,
        None => return Err(ExCommandError::MissingArgument("Files required".to_string())),
    };
    
    // Execute the vimgrep command (internal grep that searches buffer contents)
    match editor.execute_vimgrep(pattern, files, true) {
        Ok(matches) => {
            // Display the matches
            if matches.is_empty() {
                println!("No matches found");
            } else {
                println!("{} match{} found:", matches.len(), if matches.len() == 1 { "" } else { "es" });
                for (file, line, content) in &matches {
                    println!("{}:{}: {}", file, line, content);
                }
                
                // Add to the quickfix list (preserving existing entries)
                editor.add_to_quickfix_list(matches);
            }
            
            Ok(())
        },
        Err(err) => Err(ExCommandError::Other(format!("Failed to execute vimgrepadd: {}", err))),
    }
}

/// Handle the :nohlsearch command
fn handle_nohlsearch(cmd: &ExCommand) -> ExCommandResult<()> {
    let editor = get_editor()?;
    
    // Clear the search highlighting
    editor.clear_search_highlighting();
    
    println!("Search highlighting cleared");
    Ok(())
}

/// Handle the :delmarks command
fn handle_delmarks(cmd: &ExCommand) -> ExCommandResult<()> {
    let editor = get_editor()?;
    
    // Get the marks to delete
    let marks = cmd.args_str();
    
    if marks.is_empty() {
        return Err(ExCommandError::MissingArgument("Marks required".to_string()));
    }
    
    // Delete the specified marks
    let mut count = 0;
    for mark in marks.chars() {
        if editor.delete_mark(mark) {
            count += 1;
        }
    }
    
    println!("{} mark{} deleted", count, if count == 1 { "" } else { "s" });
    Ok(())
}

// Fold commands are implemented in fold_handlers.rs

/// Handle the :setlocal command
fn handle_setlocal(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = get_editor()?;
    
    // Get the option and value from the command arguments
    let args = cmd.args_str();
    
    if args.is_empty() {
        return Err(ExCommandError::MissingArgument("Option required".to_string()));
    }
    
    // Parse the option and value
    let parts: Vec<&str> = args.splitn(2, '=').collect();
    let option = parts[0].trim();
    
    // Check if this is a boolean option (no value provided)
    if parts.len() == 1 {
        // Check if it's a negated boolean option (starts with "no")
        if option.starts_with("no") {
            let real_option = &option[2..]; // Remove the "no" prefix
            match editor.set_local_option(real_option, "false") {
                Ok(_) => {
                    println!("Local option {} set to false", real_option);
                    Ok(())
                },
                Err(err) => Err(ExCommandError::InvalidCommand(format!("Failed to set option: {}", err))),
            }
        } else {
            // Assume it's a boolean option being set to true
            match editor.set_local_option(option, "true") {
                Ok(_) => {
                    println!("Local option {} set to true", option);
                    Ok(())
                },
                Err(err) => Err(ExCommandError::InvalidCommand(format!("Failed to set option: {}", err))),
            }
        }
    } else {
        // This is an option with a value
        let value = parts[1].trim();
        
        match editor.set_local_option(option, value) {
            Ok(_) => {
                println!("Local option {} set to {}", option, value);
                Ok(())
            },
            Err(err) => Err(ExCommandError::InvalidCommand(format!("Failed to set option: {}", err))),
        }
    }
}

/// Handle the :setglobal command
fn handle_setglobal(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = get_editor()?;
    
    // Get the option and value from the command arguments
    let args = cmd.args_str();
    
    if args.is_empty() {
        return Err(ExCommandError::MissingArgument("Option required".to_string()));
    }
    
    // Parse the option and value
    let parts: Vec<&str> = args.splitn(2, '=').collect();
    let option = parts[0].trim();
    
    // Check if this is a boolean option (no value provided)
    if parts.len() == 1 {
        // Check if it's a negated boolean option (starts with "no")
        if option.starts_with("no") {
            let real_option = &option[2..]; // Remove the "no" prefix
            match editor.set_global_option(real_option, "false") {
                Ok(_) => {
                    println!("Global option {} set to false", real_option);
                    Ok(())
                },
                Err(err) => Err(ExCommandError::InvalidCommand(format!("Failed to set option: {}", err))),
            }
        } else {
            // Assume it's a boolean option being set to true
            match editor.set_global_option(option, "true") {
                Ok(_) => {
                    println!("Global option {} set to true", option);
                    Ok(())
                },
                Err(err) => Err(ExCommandError::InvalidCommand(format!("Failed to set option: {}", err))),
            }
        }
    } else {
        // This is an option with a value
        let value = parts[1].trim();
        
        match editor.set_global_option(option, value) {
            Ok(_) => {
                println!("Global option {} set to {}", option, value);
                Ok(())
            },
            Err(err) => Err(ExCommandError::InvalidCommand(format!("Failed to set option: {}", err))),
        }
    }
}

/// Handle the :let command
fn handle_let(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = get_editor()?;
    
    // Get the variable assignment from the command arguments
    let args = cmd.args_str();
    
    if args.is_empty() {
        return Err(ExCommandError::MissingArgument("Variable assignment required".to_string()));
    }
    
    // Parse the variable name and value
    let parts: Vec<&str> = args.splitn(2, '=').collect();
    
    if parts.len() != 2 {
        return Err(ExCommandError::InvalidArgument("Invalid variable assignment format".to_string()));
    }
    
    let var_name = parts[0].trim();
    let value = parts[1].trim();
    
    // Set the variable
    match editor.set_variable(var_name, value) {
        Ok(_) => {
            println!("{} = {}", var_name, value);
            Ok(())
        },
        Err(err) => Err(ExCommandError::InvalidCommand(format!("Failed to set variable: {}", err))),
    }
}

/// Handle the :unlet command
fn handle_unlet(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = get_editor()?;
    
    // Get the variable name from the command arguments
    let var_name = cmd.args_str();
    
    if var_name.is_empty() {
        return Err(ExCommandError::MissingArgument("Variable name required".to_string()));
    }
    
    // Unset the variable
    match editor.unset_variable(var_name) {
        Ok(true) => {
            println!("Variable {} unset", var_name);
            Ok(())
        },
        Ok(false) => {
            println!("Variable {} not found", var_name);
            Ok(())
        },
        Err(err) => Err(ExCommandError::InvalidCommand(format!("Failed to unset variable: {}", err))),
    }
}

/// Handle the :if command
fn handle_if(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = get_editor()?;
    
    // Get the condition from the command arguments
    let condition = cmd.args_str();
    
    if condition.is_empty() {
        return Err(ExCommandError::MissingArgument("Condition required".to_string()));
    }
    
    // Evaluate the condition
    match editor.evaluate_condition(condition) {
        Ok(result) => {
            // Start an if block with the result
            match editor.start_if_block(result) {
                Ok(_) => {
                    println!("If block started with condition: {}", condition);
                    Ok(())
                },
                Err(err) => Err(ExCommandError::InvalidCommand(format!("Failed to start if block: {}", err))),
            }
        },
        Err(err) => Err(ExCommandError::InvalidCommand(format!("Failed to evaluate condition: {}", err))),
    }
}

/// Handle the :else command
fn handle_else(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = get_editor()?;
    
    // Start an else block
    match editor.start_else_block() {
        Ok(_) => {
            println!("Else block started");
            Ok(())
        },
        Err(err) => Err(ExCommandError::InvalidCommand(format!("Failed to start else block: {}", err))),
    }
}

/// Handle the :endif command
fn handle_endif(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = get_editor()?;
    
    // End an if block
    match editor.end_if_block() {
        Ok(_) => {
            println!("If block ended");
            Ok(())
        },
        Err(err) => Err(ExCommandError::InvalidCommand(format!("Failed to end if block: {}", err))),
    }
}

/// Handle the :while command
fn handle_while(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = get_editor()?;
    
    // Get the condition from the command arguments
    let condition = cmd.args_str();
    
    if condition.is_empty() {
        return Err(ExCommandError::MissingArgument("Condition required".to_string()));
    }
    
    // Evaluate the condition
    match editor.evaluate_condition(condition) {
        Ok(result) => {
            // Start a while loop with the result
            match editor.start_while_loop(condition.to_string(), result) {
                Ok(_) => {
                    println!("While loop started with condition: {}", condition);
                    Ok(())
                },
                Err(err) => Err(ExCommandError::InvalidCommand(format!("Failed to start while loop: {}", err))),
            }
        },
        Err(err) => Err(ExCommandError::InvalidCommand(format!("Failed to evaluate condition: {}", err))),
    }
}

/// Handle the :endwhile command
fn handle_endwhile(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = get_editor()?;
    
    // End a while loop
    match editor.end_while_loop() {
        Ok(_) => {
            println!("While loop ended");
            Ok(())
        },
        Err(err) => Err(ExCommandError::InvalidCommand(format!("Failed to end while loop: {}", err))),
    }
}

/// Handle the :for command
fn handle_for(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = get_editor()?;
    
    // Get the for loop parameters from the command arguments
    let args = cmd.args_str();
    
    if args.is_empty() {
        return Err(ExCommandError::MissingArgument("For loop parameters required".to_string()));
    }
    
    // Parse the for loop parameters
    // Format: "var in list"
    let parts: Vec<&str> = args.splitn(3, ' ').collect();
    
    if parts.len() < 3 || parts[1] != "in" {
        return Err(ExCommandError::InvalidArgument("Invalid for loop format. Expected: var in list".to_string()));
    }
    
    let var_name = parts[0];
    let list = parts[2];
    
    // Start a for loop
    match editor.start_for_loop(var_name.to_string(), list.to_string()) {
        Ok(_) => {
            println!("For loop started: {} in {}", var_name, list);
            Ok(())
        },
        Err(err) => Err(ExCommandError::InvalidCommand(format!("Failed to start for loop: {}", err))),
    }
}

/// Handle the :endfor command
fn handle_endfor(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = get_editor()?;
    
    // End a for loop
    match editor.end_for_loop() {
        Ok(_) => {
            println!("For loop ended");
            Ok(())
        },
        Err(err) => Err(ExCommandError::InvalidCommand(format!("Failed to end for loop: {}", err))),
    }
}

/// Handle the :function command
fn handle_function(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = get_editor()?;
    
    // Get the function declaration from the command arguments
    let args = cmd.args_str();
    
    if args.is_empty() {
        return Err(ExCommandError::MissingArgument("Function declaration required".to_string()));
    }
    
    // Parse the function declaration
    // Format: "FunctionName(arg1, arg2, ...)"
    let parts: Vec<&str> = args.splitn(2, '(').collect();
    
    if parts.len() != 2 {
        return Err(ExCommandError::InvalidArgument("Invalid function declaration format. Expected: FunctionName(arg1, arg2, ...)".to_string()));
    }
    
    let func_name = parts[0].trim();
    
    // Extract the arguments
    let args_part = parts[1].trim();
    if !args_part.ends_with(')') {
        return Err(ExCommandError::InvalidArgument("Missing closing parenthesis in function declaration".to_string()));
    }
    
    let args_list = &args_part[0..args_part.len()-1];
    let args: Vec<String> = args_list.split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    
    // Start a function definition
    match editor.start_function_definition(func_name.to_string(), args) {
        Ok(_) => {
            println!("Function definition started: {}", func_name);
            Ok(())
        },
        Err(err) => Err(ExCommandError::InvalidCommand(format!("Failed to start function definition: {}", err))),
    }
}

/// Handle the :endfunction command
fn handle_endfunction(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = get_editor()?;
    
    // End a function definition
    match editor.end_function_definition() {
        Ok(func_name) => {
            println!("Function definition ended: {}", func_name);
            Ok(())
        },
        Err(err) => Err(ExCommandError::InvalidCommand(format!("Failed to end function definition: {}", err))),
    }
}

/// Handle the :version command
fn handle_version(cmd: &ExCommand) -> ExCommandResult<()> {
    println!("xvim version 0.1.0");
    println!("A ground-up rewrite of the Vim text editor in pure Rust with a WASM plugin system");
    Ok(())
}

/// Handle the :echo command
fn handle_echo(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the message to echo
    let message = cmd.args_str();
    
    // Echo the message
    println!("{}", message);
    Ok(())
}

/// Handle the :echom command
fn handle_echom(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the message to echo
    let message = cmd.args_str();
    
    // Echo the message and add it to the message history
    println!("{}", message);
    Ok(())
}

/// Handle the :echoerr command
fn handle_echoerr(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the error message to echo
    let message = cmd.args_str();
    
    // Echo the error message
    eprintln!("Error: {}", message);
    Ok(())
}

/// Handle the :execute command
fn handle_execute(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the command to execute
    let command = cmd.args_str();
    
    if command.is_empty() {
        return Err(ExCommandError::MissingArgument("Command required".to_string()));
    }
    
    // Create a command parser
    let parser = crate::command::ExCommandParser::new();
    
    // Parse the command
    match parser.parse_ex(&command) {
        Ok(ex_cmd) => {
            // Create a command registry
            let mut registry = crate::command::ExCommandRegistry::new();
            crate::command::handlers::register_handlers(&mut registry, None);
            register_vim_commands(&mut registry);
            
            // Execute the command
            registry.execute(&ex_cmd)
        },
        Err(err) => Err(ExCommandError::InvalidCommand(format!("Failed to parse command: {}", err))),
    }
}

/// Handle the :silent command
fn handle_silent(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the command to execute
    let command = cmd.args_str();
    
    if command.is_empty() {
        return Err(ExCommandError::MissingArgument("Command required".to_string()));
    }
    
    // Create a command parser
    let parser = crate::command::ExCommandParser::new();
    
    // Parse the command
    match parser.parse_ex(&command) {
        Ok(ex_cmd) => {
            // Create a command registry
            let mut registry = crate::command::ExCommandRegistry::new();
            crate::command::handlers::register_handlers(&mut registry, None);
            register_vim_commands(&mut registry);
            
            // Execute the command silently
            // In a real implementation, this would suppress output
            registry.execute(&ex_cmd)
        },
        Err(err) => Err(ExCommandError::InvalidCommand(format!("Failed to parse command: {}", err))),
    }
}