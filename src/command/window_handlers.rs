//! Window and tab command handlers
//!
//! This module implements handlers for window and tab commands.

use crate::command::{ExCommand, ExCommandError, ExCommandResult, ExCommandRegistry};
use std::io::Read;
use std::sync::Mutex;
use std::io::Read;
use std::sync::Arc;
use std::io::Read;
use std::sync::Mutex;
use std::io::Read;
use crate::editor::Editor;
use std::io::Read;
use std::sync::Mutex;
use std::io::Read;
use std::sync::Arc;
use std::io::Read;
use std::sync::Mutex;
use std::io::Read;
use crate::cursor::CursorPosition;
use std::io::Read;
use std::sync::Mutex;
use std::io::Read;
use std::sync::Arc;
use std::io::Read;
use std::sync::Mutex;
use std::io::Read;
use std::collections::HashMap;
use std::io::Read;
use std::sync::Mutex;
use std::io::Read;
use std::sync::Arc;
use std::io::Read;
use std::sync::Mutex;
use std::io::Read;
// use std::fs::File;
use std::io::{self, Read};
use std::io::Read;
use std::sync::Mutex;
use std::io::Read;
use std::sync::Arc;
use std::io::Read;
use std::sync::Mutex;
use std::io::Read;
// use std::path::Path;
use std::sync::{Arc, Mutex};
use std::io::Read;
use std::sync::Mutex;
use std::io::Read;
use std::sync::Arc;
use std::io::Read;
use std::sync::Mutex;
use std::io::Read;

/// Window layout
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowLayout {
    /// Horizontal layout (windows stacked vertically)
    Horizontal,
    /// Vertical layout (windows side by side)
    Vertical,
    /// Grid layout (windows in a grid)
    Grid,
}

/// Window manager
#[derive(Debug)]
pub struct WindowManager {
    /// Current layout
    pub layout: WindowLayout,
    /// Minimum window width
    pub min_width: usize,
    /// Minimum window height
    pub min_height: usize,
    /// Equal size flag
    pub equal_size: bool,
    /// Window focus history
    pub focus_history: Vec<usize>,
}

impl WindowManager {
    /// Create a new window manager
    pub fn new() -> Self {
        Self {
            layout: WindowLayout::Horizontal,
            min_width: 10,
            min_height: 3,
            equal_size: true,
            focus_history: Vec::new(),
        }
    }

    /// Set the layout
    pub fn set_layout(&mut self, layout: WindowLayout) {
        self.layout = layout;
    }

    /// Set the minimum window width
    pub fn set_min_width(&mut self, width: usize) {
        self.min_width = width;
    }

    /// Set the minimum window height
    pub fn set_min_height(&mut self, height: usize) {
        self.min_height = height;
    }

    /// Set the equal size flag
    pub fn set_equal_size(&mut self, equal: bool) {
        self.equal_size = equal;
    }

    /// Add a window to the focus history
    pub fn add_to_focus_history(&mut self, window_id: usize) {
        // Remove the window from the history if it's already there
        self.focus_history.retain(|&id| id != window_id);
        
        // Add the window to the end of the history
        self.focus_history.push(window_id);
        
        // Limit the history size to 20
        if self.focus_history.len() > 20 {
            self.focus_history.remove(0);
        }
    }

    /// Get the previous window in the focus history
    pub fn prev_focus(&self, current_window_id: usize) -> Option<usize> {
        // Find the current window in the history
        let pos = self.focus_history.iter().position(|&id| id == current_window_id)?;
        
        // Get the previous window
        if pos > 0 {
            Some(self.focus_history[pos - 1])
        } else {
            None
        }
    }

    /// Get the next window in the focus history
    pub fn next_focus(&self, current_window_id: usize) -> Option<usize> {
        // Find the current window in the history
        let pos = self.focus_history.iter().position(|&id| id == current_window_id)?;
        
        // Get the next window
        if pos + 1 < self.focus_history.len() {
            Some(self.focus_history[pos + 1])
        } else {
            None
        }
    }
}

// Global window manager
static mut WINDOW_MANAGER: Option<WindowManager> = None;

/// Initialize the window manager
pub fn init_window_manager() {
    unsafe {
        if WINDOW_MANAGER.is_none() {
            WINDOW_MANAGER = Some(WindowManager::new());
        }
    }
}

/// Register window and tab command handlers
pub fn register_window_handlers(registry: &mut ExCommandRegistry) {
    // Initialize the window manager
    init_window_manager();
    
    // Register window commands
    registry.register("wincmd", handle_wincmd);
    registry.register("winsize", handle_winsize);
    registry.register("winchoose", handle_winchoose);
    registry.register("winpos", handle_winpos);
    registry.register("winlayout", handle_winlayout);
    registry.register("windo", handle_windo);
    registry.register("winfixwidth", handle_winfixwidth);
    registry.register("winfixheight", handle_winfixheight);
    registry.register("winminwidth", handle_winminwidth);
    registry.register("winminheight", handle_winminheight);
    registry.register("winsaveview", handle_winsaveview);
    registry.register("winrestview", handle_winrestview);
    
    // Register tab commands
    registry.register("tabdo", handle_tabdo);
    registry.register("tabmove", handle_tabmove);
    registry.register("tabonly", handle_tabonly);
    registry.register("tabnew", handle_tabnew);
    registry.register("tabfind", handle_tabfind);
    registry.register("tabrewind", handle_tabrewind);
    registry.register("tablast", handle_tablast);
    registry.register("tabfirst", handle_tabfirst);
}

/// Handle the :wincmd command
fn handle_wincmd(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match crate::command::handlers::EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Get the window command from the command arguments
    let win_cmd = match cmd.first_arg() {
        Some(cmd) => cmd,
        None => return Err(ExCommandError::MissingArgument("Window command required".to_string())),
    };
    
    // Execute the window command
    match win_cmd {
        "h" | "left" => {
            // Move to the window to the left
            match editor.get_terminal_mut().focus_left_window() {
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
        "j" | "down" => {
            // Move to the window below
            match editor.get_terminal_mut().focus_down_window() {
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
        "k" | "up" => {
            // Move to the window above
            match editor.get_terminal_mut().focus_up_window() {
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
        "l" | "right" => {
            // Move to the window to the right
            match editor.get_terminal_mut().focus_right_window() {
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
        "w" | "W" => {
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
        "W" => {
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
        "t" | "T" => {
            // Move to the top window
            match editor.get_terminal_mut().focus_top_window() {
                Ok(true) => {
                    println!("Moved to top window");
                    Ok(())
                },
                Ok(false) => {
                    println!("No top window");
                    Ok(())
                },
                Err(err) => Err(ExCommandError::Other(format!("Failed to move to top window: {}", err))),
            }
        },
        "b" | "B" => {
            // Move to the bottom window
            match editor.get_terminal_mut().focus_bottom_window() {
                Ok(true) => {
                    println!("Moved to bottom window");
                    Ok(())
                },
                Ok(false) => {
                    println!("No bottom window");
                    Ok(())
                },
                Err(err) => Err(ExCommandError::Other(format!("Failed to move to bottom window: {}", err))),
            }
        },
        "p" | "P" => {
            // Move to the previous window in the focus history
            let window_manager = unsafe {
                match &WINDOW_MANAGER {
                    Some(manager) => manager,
                    None => {
                        init_window_manager();
                        match &WINDOW_MANAGER {
                            Some(manager) => manager,
                            None => return Err(ExCommandError::Other("Failed to initialize window manager".to_string())),
                        }
                    }
                }
            };
            
            // Get the current window ID
            let current_window_id = match editor.get_current_window_id() {
                Some(id) => id,
                None => return Err(ExCommandError::Other("No current window".to_string())),
            };
            
            // Get the previous window in the focus history
            let prev_window_id = match window_manager.prev_focus(current_window_id) {
                Some(id) => id,
                None => return Err(ExCommandError::Other("No previous window in focus history".to_string())),
            };
            
            // Focus the previous window
            match editor.focus_window(prev_window_id) {
                Ok(_) => {
                    println!("Moved to previous window in focus history");
                    Ok(())
                },
                Err(err) => Err(ExCommandError::Other(format!("Failed to move to previous window: {}", err))),
            }
        },
        "n" | "N" => {
            // Move to the next window in the focus history
            let window_manager = unsafe {
                match &WINDOW_MANAGER {
                    Some(manager) => manager,
                    None => {
                        init_window_manager();
                        match &WINDOW_MANAGER {
                            Some(manager) => manager,
                            None => return Err(ExCommandError::Other("Failed to initialize window manager".to_string())),
                        }
                    }
                }
            };
            
            // Get the current window ID
            let current_window_id = match editor.get_current_window_id() {
                Some(id) => id,
                None => return Err(ExCommandError::Other("No current window".to_string())),
            };
            
            // Get the next window in the focus history
            let next_window_id = match window_manager.next_focus(current_window_id) {
                Some(id) => id,
                None => return Err(ExCommandError::Other("No next window in focus history".to_string())),
            };
            
            // Focus the next window
            match editor.focus_window(next_window_id) {
                Ok(_) => {
                    println!("Moved to next window in focus history");
                    Ok(())
                },
                Err(err) => Err(ExCommandError::Other(format!("Failed to move to next window: {}", err))),
            }
        },
        "=" => {
            // Make all windows equal size
            match editor.get_terminal_mut().equalize_windows() {
                Ok(_) => {
                    println!("Windows equalized");
                    Ok(())
                },
                Err(err) => Err(ExCommandError::Other(format!("Failed to equalize windows: {}", err))),
            }
        },
        "o" | "O" => {
            // Make the current window the only one
            match editor.get_terminal_mut().only_window() {
                Ok(_) => {
                    println!("Only window");
                    Ok(())
                },
                Err(err) => Err(ExCommandError::Other(format!("Failed to make only window: {}", err))),
            }
        },
        "s" | "S" => {
            // Split the window horizontally
            match editor.get_terminal_mut().split_window_horizontal() {
                Ok(_) => {
                    println!("Window split horizontally");
                    Ok(())
                },
                Err(err) => Err(ExCommandError::Other(format!("Failed to split window horizontally: {}", err))),
            }
        },
        "v" | "V" => {
            // Split the window vertically
            match editor.get_terminal_mut().split_window_vertical() {
                Ok(_) => {
                    println!("Window split vertically");
                    Ok(())
                },
                Err(err) => Err(ExCommandError::Other(format!("Failed to split window vertically: {}", err))),
            }
        },
        "c" | "C" => {
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
        },
        "+" => {
            // Increase the window height
            match editor.get_terminal_mut().increase_window_height() {
                Ok(_) => {
                    println!("Window height increased");
                    Ok(())
                },
                Err(err) => Err(ExCommandError::Other(format!("Failed to increase window height: {}", err))),
            }
        },
        "-" => {
            // Decrease the window height
            match editor.get_terminal_mut().decrease_window_height() {
                Ok(_) => {
                    println!("Window height decreased");
                    Ok(())
                },
                Err(err) => Err(ExCommandError::Other(format!("Failed to decrease window height: {}", err))),
            }
        },
        ">" => {
            // Increase the window width
            match editor.get_terminal_mut().increase_window_width() {
                Ok(_) => {
                    println!("Window width increased");
                    Ok(())
                },
                Err(err) => Err(ExCommandError::Other(format!("Failed to increase window width: {}", err))),
            }
        },
        "<" => {
            // Decrease the window width
            match editor.get_terminal_mut().decrease_window_width() {
                Ok(_) => {
                    println!("Window width decreased");
                    Ok(())
                },
                Err(err) => Err(ExCommandError::Other(format!("Failed to decrease window width: {}", err))),
            }
        },
        _ => Err(ExCommandError::InvalidArgument(format!("Invalid window command: {}", win_cmd))),
    }
}

/// Handle the :winsize command
fn handle_winsize(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match crate::command::handlers::EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Get the window size from the command arguments
    let args = cmd.args_str();
    
    if args.is_empty() {
        // If no arguments, display the current window size
        let window = match editor.get_current_window() {
            Some(window) => window,
            None => return Err(ExCommandError::Other("No current window".to_string())),
        };
        
        println!("Window size: {}x{}", window.width, window.height);
        return Ok(());
    }
    
    // Parse the window size
    let parts: Vec<&str> = args.split('x').collect();
    
    if parts.len() != 2 {
        return Err(ExCommandError::InvalidArgument(format!("Invalid window size format: {}", args)));
    }
    
    let width = match parts[0].parse::<usize>() {
        Ok(w) => w,
        Err(_) => return Err(ExCommandError::InvalidArgument(format!("Invalid width: {}", parts[0]))),
    };
    
    let height = match parts[1].parse::<usize>() {
        Ok(h) => h,
        Err(_) => return Err(ExCommandError::InvalidArgument(format!("Invalid height: {}", parts[1]))),
    };
    
    // Set the window size
    match editor.get_terminal_mut().set_window_size(width, height) {
        Ok(_) => {
            println!("Window size set to {}x{}", width, height);
            Ok(())
        },
        Err(err) => Err(ExCommandError::Other(format!("Failed to set window size: {}", err))),
    }
}

/// Handle the :winchoose command
fn handle_winchoose(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match crate::command::handlers::EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Get the window ID from the command arguments
    let window_id = match cmd.first_arg() {
        Some(id_str) => match id_str.parse::<usize>() {
            Ok(id) => id,
            Err(_) => return Err(ExCommandError::InvalidArgument(format!("Invalid window ID: {}", id_str))),
        },
        None => {
            // If no window ID is provided, display a list of windows
            let windows = editor.get_window_list();
            
            if windows.is_empty() {
                println!("No windows");
                return Ok(());
            }
            
            println!("--- Window list ---");
            
            for window in windows {
                let current_marker = if Some(window.id) == editor.get_current_window_id() { ">" } else { " " };
                
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
            
            // Prompt for selection
            print!("Enter window ID (CR to abort): ");
            std::io::stdout().flush().unwrap();
            
            // Read the selection
            let mut selection = String::new();
            std::io::stdin().read_line(&mut selection).unwrap();
            
            // Parse the selection
            let selection = selection.trim();
            
            if selection.is_empty() {
                return Ok(());
            }
            
            match selection.parse::<usize>() {
                Ok(id) => id,
                Err(_) => return Err(ExCommandError::InvalidArgument(format!("Invalid window ID: {}", selection))),
            }
        }
    };
    
    // Focus the window
    match editor.focus_window(window_id) {
        Ok(_) => {
            println!("Window {} focused", window_id);
            Ok(())
        },
        Err(err) => Err(ExCommandError::Other(format!("Failed to focus window: {}", err))),
    }
}

/// Handle the :winpos command
fn handle_winpos(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match crate::command::handlers::EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Get the window position from the command arguments
    let args = cmd.args_str();
    
    if args.is_empty() {
        // If no arguments, display the current window position
        let window = match editor.get_current_window() {
            Some(window) => window,
            None => return Err(ExCommandError::Other("No current window".to_string())),
        };
        
        println!("Window position: ({},{})", window.position.line, window.position.column);
        return Ok(());
    }
    
    // Parse the window position
    let parts: Vec<&str> = args.split(',').collect();
    
    if parts.len() != 2 {
        return Err(ExCommandError::InvalidArgument(format!("Invalid window position format: {}", args)));
    }
    
    let line = match parts[0].parse::<usize>() {
        Ok(l) => l,
        Err(_) => return Err(ExCommandError::InvalidArgument(format!("Invalid line: {}", parts[0]))),
    };
    
    let column = match parts[1].parse::<usize>() {
        Ok(c) => c,
        Err(_) => return Err(ExCommandError::InvalidArgument(format!("Invalid column: {}", parts[1]))),
    };
    
    // Set the window position
    match editor.get_terminal_mut().set_window_position(line, column) {
        Ok(_) => {
            println!("Window position set to ({},{})", line, column);
            Ok(())
        },
        Err(err) => Err(ExCommandError::Other(format!("Failed to set window position: {}", err))),
    }
}

/// Handle the :winlayout command
fn handle_winlayout(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the window manager
    let window_manager = unsafe {
        match &mut WINDOW_MANAGER {
            Some(manager) => manager,
            None => {
                init_window_manager();
                match &mut WINDOW_MANAGER {
                    Some(manager) => manager,
                    None => return Err(ExCommandError::Other("Failed to initialize window manager".to_string())),
                }
            }
        }
    };
    
    // Get the layout from the command arguments
    let layout_str = cmd.args_str();
    
    if layout_str.is_empty() {
        // If no arguments, display the current layout
        let layout_str = match window_manager.layout {
            WindowLayout::Horizontal => "horizontal",
            WindowLayout::Vertical => "vertical",
            WindowLayout::Grid => "grid",
        };
        
        println!("Window layout: {}", layout_str);
        return Ok(());
    }
    
    // Parse the layout
    let layout = match layout_str.as_str() {
        "horizontal" | "h" => WindowLayout::Horizontal,
        "vertical" | "v" => WindowLayout::Vertical,
        "grid" | "g" => WindowLayout::Grid,
        _ => return Err(ExCommandError::InvalidArgument(format!("Invalid layout: {}", layout_str))),
    };
    
    // Set the layout
    window_manager.set_layout(layout);
    
    println!("Window layout set to {}", layout_str);
    Ok(())
}

/// Handle the :windo command
fn handle_windo(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match crate::command::handlers::EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Get the command to execute from the command arguments
    let command = cmd.args_str();
    
    if command.is_empty() {
        return Err(ExCommandError::MissingArgument("Command required".to_string()));
    }
    
    // Get the list of windows
    let windows = editor.get_window_list();
    
    if windows.is_empty() {
        return Err(ExCommandError::Other("No windows".to_string()));
    }
    
    // Remember the current window ID
    let current_window_id = match editor.get_current_window_id() {
        Some(id) => id,
        None => return Err(ExCommandError::Other("No current window".to_string())),
    };
    
    // Create a command parser
    let parser = crate::command::ExCommandParser::new();
    
    // Create a command registry
    let mut registry = crate::command::ExCommandRegistry::new();
    crate::command::handlers::register_handlers(&mut registry, None);
    
    // Execute the command in each window
    for window in windows {
        // Focus the window
        if let Err(err) = editor.focus_window(window.id) {
            return Err(ExCommandError::Other(format!("Failed to focus window {}: {}", window.id, err)));
        }
        
        // Parse and execute the command
        match parser.parse(command.as_str()) {
            Ok(ex_cmd) => {
                if let Err(err) = registry.execute(&ex_cmd) {
                    return Err(ExCommandError::Other(format!("Failed to execute command in window {}: {}", window.id, err)));
                }
            },
            Err(err) => {
                return Err(ExCommandError::Other(format!("Failed to parse command: {}", err)));
            }
        }
    }
    
    // Restore the current window
    if let Err(err) = editor.focus_window(current_window_id) {
        return Err(ExCommandError::Other(format!("Failed to restore current window: {}", err)));
    }
    
    Ok(())
}

/// Handle the :winfixwidth command
fn handle_winfixwidth(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match crate::command::handlers::EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Get the current window
    let window_id = match editor.get_current_window_id() {
        Some(id) => id,
        None => return Err(ExCommandError::Other("No current window".to_string())),
    };
    
    // Get the fix width flag from the command arguments
    let fix_width = if cmd.args_str().is_empty() {
        // If no arguments, toggle the fix width flag
        !editor.get_terminal().is_window_width_fixed(window_id)
    } else {
        // Otherwise, parse the flag
        match cmd.args_str().as_str() {
            "on" | "true" | "1" => true,
            "off" | "false" | "0" => false,
            _ => return Err(ExCommandError::InvalidArgument(format!("Invalid fix width flag: {}", cmd.args_str()))),
        }
    };
    
    // Set the fix width flag
    match editor.get_terminal_mut().set_window_width_fixed(window_id, fix_width) {
        Ok(_) => {
            println!("Window width fixed: {}", fix_width);
            Ok(())
        },
        Err(err) => Err(ExCommandError::Other(format!("Failed to set window width fixed: {}", err))),
    }
}

/// Handle the :winfixheight command
fn handle_winfixheight(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match crate::command::handlers::EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Get the current window
    let window_id = match editor.get_current_window_id() {
        Some(id) => id,
        None => return Err(ExCommandError::Other("No current window".to_string())),
    };
    
    // Get the fix height flag from the command arguments
    let fix_height = if cmd.args_str().is_empty() {
        // If no arguments, toggle the fix height flag
        !editor.get_terminal().is_window_height_fixed(window_id)
    } else {
        // Otherwise, parse the flag
        match cmd.args_str().as_str() {
            "on" | "true" | "1" => true,
            "off" | "false" | "0" => false,
            _ => return Err(ExCommandError::InvalidArgument(format!("Invalid fix height flag: {}", cmd.args_str()))),
        }
    };
    
    // Set the fix height flag
    match editor.get_terminal_mut().set_window_height_fixed(window_id, fix_height) {
        Ok(_) => {
            println!("Window height fixed: {}", fix_height);
            Ok(())
        },
        Err(err) => Err(ExCommandError::Other(format!("Failed to set window height fixed: {}", err))),
    }
}

/// Handle the :winminwidth command
fn handle_winminwidth(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the window manager
    let window_manager = unsafe {
        match &mut WINDOW_MANAGER {
            Some(manager) => manager,
            None => {
                init_window_manager();
                match &mut WINDOW_MANAGER {
                    Some(manager) => manager,
                    None => return Err(ExCommandError::Other("Failed to initialize window manager".to_string())),
                }
            }
        }
    };
    
    // Get the minimum width from the command arguments
    let min_width_str = cmd.args_str();
    
    if min_width_str.is_empty() {
        // If no arguments, display the current minimum width
        println!("Window minimum width: {}", window_manager.min_width);
        return Ok(());
    }
    
    // Parse the minimum width
    let min_width = match min_width_str.parse::<usize>() {
        Ok(w) => w,
        Err(_) => return Err(ExCommandError::InvalidArgument(format!("Invalid minimum width: {}", min_width_str))),
    };
    
    // Set the minimum width
    window_manager.set_min_width(min_width);
    
    println!("Window minimum width set to {}", min_width);
    Ok(())
}

/// Handle the :winminheight command
fn handle_winminheight(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the window manager
    let window_manager = unsafe {
        match &mut WINDOW_MANAGER {
            Some(manager) => manager,
            None => {
                init_window_manager();
                match &mut WINDOW_MANAGER {
                    Some(manager) => manager,
                    None => return Err(ExCommandError::Other("Failed to initialize window manager".to_string())),
                }
            }
        }
    };
    
    // Get the minimum height from the command arguments
    let min_height_str = cmd.args_str();
    
    if min_height_str.is_empty() {
        // If no arguments, display the current minimum height
        println!("Window minimum height: {}", window_manager.min_height);
        return Ok(());
    }
    
    // Parse the minimum height
    let min_height = match min_height_str.parse::<usize>() {
        Ok(h) => h,
        Err(_) => return Err(ExCommandError::InvalidArgument(format!("Invalid minimum height: {}", min_height_str))),
    };
    
    // Set the minimum height
    window_manager.set_min_height(min_height);
    
    println!("Window minimum height set to {}", min_height);
    Ok(())
}

/// Window view
#[derive(Debug, Clone)]
pub struct WindowView {
    /// Cursor position
    pub cursor: CursorPosition,
    /// Top line
    pub top_line: usize,
    /// Left column
    pub left_col: usize,
    /// Cursor line
    pub cursor_line: usize,
    /// Cursor column
    pub cursor_col: usize,
    /// Cursor line offset
    pub cursor_line_offset: usize,
    /// Cursor column offset
    pub cursor_col_offset: usize,
}

// Global window views
static mut WINDOW_VIEWS: Option<HashMap<usize, WindowView>> = None;

/// Initialize the window views
fn init_window_views() {
    unsafe {
        if WINDOW_VIEWS.is_none() {
            WINDOW_VIEWS = Some(HashMap::new());
        }
    }
}

/// Handle the :winsaveview command
fn handle_winsaveview(_cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match crate::command::handlers::EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Get the current window
    let window_id = match editor.get_current_window_id() {
        Some(id) => id,
        None => return Err(ExCommandError::Other("No current window".to_string())),
    };
    
    // Get the current cursor position
    let cursor_pos = editor.cursor_position();
    
    // Get the current view
    let view = WindowView {
        cursor: cursor_pos.clone(),
        top_line: editor.get_terminal().get_window_top_line(window_id),
        left_col: editor.get_terminal().get_window_left_col(window_id),
        cursor_line: cursor_pos.line,
        cursor_col: cursor_pos.column,
        cursor_line_offset: 0, // Not implemented yet
        cursor_col_offset: 0, // Not implemented yet
    };
    
    // Save the view
    unsafe {
        if WINDOW_VIEWS.is_none() {
            init_window_views();
        }
        
        if let Some(views) = &mut WINDOW_VIEWS {
            views.insert(window_id, view);
        }
    }
    
    println!("Window view saved");
    Ok(())
}

/// Handle the :winrestview command
fn handle_winrestview(_cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match crate::command::handlers::EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Get the current window
    let window_id = match editor.get_current_window_id() {
        Some(id) => id,
        None => return Err(ExCommandError::Other("No current window".to_string())),
    };
    
    // Get the saved view
    let view = unsafe {
        if WINDOW_VIEWS.is_none() {
            init_window_views();
        }
        
        if let Some(views) = &WINDOW_VIEWS {
            match views.get(&window_id) {
                Some(view) => view.clone(),
                None => return Err(ExCommandError::Other(format!("No saved view for window {}", window_id))),
            }
        } else {
            return Err(ExCommandError::Other("Failed to initialize window views".to_string()));
        }
    };
    
    // Restore the view
    
    // Set the cursor position
    if let Err(err) = editor.get_cursor_manager_mut().set_position(view.cursor) {
        return Err(ExCommandError::Other(format!("Failed to set cursor position: {}", err)));
    }
    
    // Set the top line
    if let Err(err) = editor.get_terminal_mut().set_window_top_line(window_id, view.top_line) {
        return Err(ExCommandError::Other(format!("Failed to set window top line: {}", err)));
    }
    
    // Set the left column
    if let Err(err) = editor.get_terminal_mut().set_window_left_col(window_id, view.left_col) {
        return Err(ExCommandError::Other(format!("Failed to set window left column: {}", err)));
    }
    
    println!("Window view restored");
    Ok(())
}

/// Handle the :tabdo command
fn handle_tabdo(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match crate::command::handlers::EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Get the command to execute from the command arguments
    let command = cmd.args_str();
    
    if command.is_empty() {
        return Err(ExCommandError::MissingArgument("Command required".to_string()));
    }
    
    // Get the list of tabs
    let tabs = editor.get_tab_list();
    
    if tabs.is_empty() {
        return Err(ExCommandError::Other("No tabs".to_string()));
    }
    
    // Remember the current tab ID
    let current_tab_id = match editor.get_current_tab_id() {
        Some(id) => id,
        None => return Err(ExCommandError::Other("No current tab".to_string())),
    };
    
    // Create a command parser
    let parser = crate::command::ExCommandParser::new();
    
    // Create a command registry
    let mut registry = crate::command::ExCommandRegistry::new();
    crate::command::handlers::register_handlers(&mut registry, None);
    
    // Execute the command in each tab
    for tab in tabs {
        // Focus the tab
        if let Err(err) = editor.get_terminal_mut().focus_tab(tab.id) {
            return Err(ExCommandError::Other(format!("Failed to focus tab {}: {}", tab.id, err)));
        }
        
        // Parse and execute the command
        match parser.parse(command.as_str()) {
            Ok(ex_cmd) => {
                if let Err(err) = registry.execute(&ex_cmd) {
                    return Err(ExCommandError::Other(format!("Failed to execute command in tab {}: {}", tab.id, err)));
                }
            },
            Err(err) => {
                return Err(ExCommandError::Other(format!("Failed to parse command: {}", err)));
            }
        }
    }
    
    // Restore the current tab
    if let Err(err) = editor.get_terminal_mut().focus_tab(current_tab_id) {
        return Err(ExCommandError::Other(format!("Failed to restore current tab: {}", err)));
    }
    
    Ok(())
}

/// Handle the :tabmove command
fn handle_tabmove(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match crate::command::handlers::EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Get the tab position from the command arguments
    let pos_str = cmd.args_str();
    
    // Get the current tab ID
    let tab_id = match editor.get_current_tab_id() {
        Some(id) => id,
        None => return Err(ExCommandError::Other("No current tab".to_string())),
    };
    
    // Parse the position
    let pos = if pos_str.is_empty() {
        // If no position is provided, move the tab to the end
        editor.get_tab_list().len() - 1
    } else if pos_str == "0" || pos_str == "$" {
        // Move the tab to the beginning or end
        if pos_str == "0" {
            0
        } else {
            editor.get_tab_list().len() - 1
        }
    } else {
        // Parse the position as a number
        match pos_str.parse::<usize>() {
            Ok(p) => p,
            Err(_) => return Err(ExCommandError::InvalidArgument(format!("Invalid tab position: {}", pos_str))),
        }
    };
    
    // Move the tab
    match editor.get_terminal_mut().move_tab(tab_id, pos) {
        Ok(_) => {
            println!("Tab moved to position {}", pos);
            Ok(())
        },
        Err(err) => Err(ExCommandError::Other(format!("Failed to move tab: {}", err))),
    }
}

/// Handle the :tabonly command
fn handle_tabonly(_cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match crate::command::handlers::EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Get the current tab ID
    let tab_id = match editor.get_current_tab_id() {
        Some(id) => id,
        None => return Err(ExCommandError::Other("No current tab".to_string())),
    };
    
    // Close all other tabs
    match editor.get_terminal_mut().close_other_tabs(tab_id) {
        Ok(_) => {
            println!("All other tabs closed");
            Ok(())
        },
        Err(err) => Err(ExCommandError::Other(format!("Failed to close other tabs: {}", err))),
    }
}

/// Handle the :tabnew command
fn handle_tabnew(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match crate::command::handlers::EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Get the file path from the command arguments
    let file_path = cmd.first_arg();
    
    // Create a new buffer
    let buffer_id = if let Some(path) = file_path {
        // Try to open the file
        match editor.get_buffer_manager_mut().open_file(path) {
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
        // Create a new empty buffer
        match editor.get_buffer_manager_mut().create_buffer() {
            Ok(id) => id,
            Err(err) => return Err(ExCommandError::Other(format!("Failed to create buffer: {}", err))),
        }
    };
    
    // Create a new tab with the buffer
    match editor.get_terminal_mut().create_tab(buffer_id, file_path.map(|p| p.to_string())) {
        Ok(tab_id) => {
            // Set the current buffer to the new buffer
            if let Err(err) = editor.get_buffer_manager_mut().set_current_buffer(buffer_id) {
                return Err(ExCommandError::Other(format!("Failed to set current buffer: {}", err)));
            }
            
            println!("New tab created");
            Ok(())
        },
        Err(err) => Err(ExCommandError::Other(format!("Failed to create tab: {}", err))),
    }
}

/// Handle the :tabfind command
fn handle_tabfind(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match crate::command::handlers::EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Get the file pattern from the command arguments
    let file_pattern = match cmd.first_arg() {
        Some(pattern) => pattern,
        None => return Err(ExCommandError::MissingArgument("File pattern required".to_string())),
    };
    
    // Get the current directory
    let current_dir = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(err) => return Err(ExCommandError::Other(format!("Failed to get current directory: {}", err))),
    };
    
    // Find files matching the pattern
    let mut matching_files = Vec::new();
    
    // Use glob to find matching files
    match glob::glob(&format!("{}/{}", current_dir.display(), file_pattern)) {
        Ok(paths) => {
            for path in paths {
                if let Ok(path) = path {
                    matching_files.push(path);
                }
            }
        },
        Err(err) => return Err(ExCommandError::Other(format!("Failed to find files: {}", err))),
    }
    
    if matching_files.is_empty() {
        return Err(ExCommandError::Other(format!("No files matching pattern: {}", file_pattern)));
    }
    
    // If there's only one matching file, open it in a new tab
    if matching_files.len() == 1 {
        let file_path = matching_files[0].to_string_lossy().to_string();
        
        // Create a new buffer for the file
        let buffer_id = match editor.get_buffer_manager_mut().open_file(&file_path) {
            Ok(id) => id,
            Err(err) => return Err(ExCommandError::Other(format!("Failed to open file: {}", err))),
        };
        
        // Create a new tab with the buffer
        match editor.get_terminal_mut().create_tab(buffer_id, Some(file_path.clone())) {
            Ok(_) => {
                // Set the current buffer to the new buffer
                if let Err(err) = editor.get_buffer_manager_mut().set_current_buffer(buffer_id) {
                    return Err(ExCommandError::Other(format!("Failed to set current buffer: {}", err)));
                }
                
                println!("\"{}\" opened in new tab", file_path);
                return Ok(());
            },
            Err(err) => return Err(ExCommandError::Other(format!("Failed to create tab: {}", err))),
        }
    }
    
    // If there are multiple matching files, display a list and prompt for selection
    println!("--- Matching files ---");
    
    for (i, file) in matching_files.iter().enumerate() {
        println!("{:3} {}", i + 1, file.display());
    }
    
    // Prompt for selection
    print!("Enter file number (CR to abort): ");
    std::io::stdout().flush().unwrap();
    
    // Read the selection
    let mut selection = String::new();
    std::io::stdin().read_line(&mut selection).unwrap();
    
    // Parse the selection
    let selection = selection.trim();
    
    if selection.is_empty() {
        return Ok(());
    }
    
    let index = match selection.parse::<usize>() {
        Ok(n) => n - 1, // Convert to 0-based
        Err(_) => return Err(ExCommandError::InvalidArgument(format!("Invalid selection: {}", selection))),
    };
    
    // Check if the selection is valid
    if index >= matching_files.len() {
        return Err(ExCommandError::InvalidArgument(format!("Invalid selection: {}", selection)));
    }
    
    // Open the selected file in a new tab
    let file_path = matching_files[index].to_string_lossy().to_string();
    
    // Create a new buffer for the file
    let buffer_id = match editor.get_buffer_manager_mut().open_file(&file_path) {
        Ok(id) => id,
        Err(err) => return Err(ExCommandError::Other(format!("Failed to open file: {}", err))),
    };
    
    // Create a new tab with the buffer
    match editor.get_terminal_mut().create_tab(buffer_id, Some(file_path.clone())) {
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

/// Handle the :tabrewind command
fn handle_tabrewind(_cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match crate::command::handlers::EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Get the first tab
    let tabs = editor.get_tab_list();
    
    if tabs.is_empty() {
        return Err(ExCommandError::Other("No tabs".to_string()));
    }
    
    // Focus the first tab
    match editor.get_terminal_mut().focus_tab(tabs[0].id) {
        Ok(_) => {
            println!("Moved to first tab");
            Ok(())
        },
        Err(err) => Err(ExCommandError::Other(format!("Failed to move to first tab: {}", err))),
    }
}

/// Handle the :tablast command
fn handle_tablast(_cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match crate::command::handlers::EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Get the last tab
    let tabs = editor.get_tab_list();
    
    if tabs.is_empty() {
        return Err(ExCommandError::Other("No tabs".to_string()));
    }
    
    // Focus the last tab
    match editor.get_terminal_mut().focus_tab(tabs.last().unwrap().id) {
        Ok(_) => {
            println!("Moved to last tab");
            Ok(())
        },
        Err(err) => Err(ExCommandError::Other(format!("Failed to move to last tab: {}", err))),
    }
}

/// Handle the :tabfirst command
fn handle_tabfirst(cmd: &ExCommand) -> ExCommandResult<()> {
    // This is just an alias for :tabrewind
    handle_tabrewind(cmd)
}