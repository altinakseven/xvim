//! Additional Ex command handlers
//!
//! This module implements additional handlers for ex commands.

use crate::command::{ExCommand, ExCommandError, ExCommandResult, ExCommandRegistry};
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
use std::fs::File;
use std::io::Read;
use std::sync::Mutex;
use std::io::Read;
use std::sync::Arc;
use std::io::Read;
use std::sync::Mutex;
use std::io::Read;
use std::io::{self, BufRead, Read};
use std::io::Read;
use std::sync::Mutex;
use std::io::Read;
use std::sync::Arc;
use std::io::Read;
use std::sync::Mutex;
use std::io::Read;
use std::path::Path;
use std::io::Read;
use std::sync::Mutex;
use std::io::Read;
use std::sync::Arc;
use std::io::Read;
use std::sync::Mutex;
use std::io::Read;
use std::sync::{Arc, Mutex};
use std::io::Read;
use std::sync::Mutex;
use std::io::Read;
use std::sync::Arc;
use std::io::Read;
use std::sync::Mutex;
use std::io::Read;

// Global variable storage
static mut VARIABLES: Option<HashMap<String, String>> = None;

// Global command history
static mut COMMAND_HISTORY: Option<Vec<String>> = None;

/// Initialize the global variables
pub fn init_variables() {
    unsafe {
        if VARIABLES.is_none() {
            VARIABLES = Some(HashMap::new());
        }
    }
}

/// Initialize the command history
pub fn init_command_history() {
    unsafe {
        if COMMAND_HISTORY.is_none() {
            COMMAND_HISTORY = Some(Vec::new());
        }
    }
}

/// Add a command to the history
pub fn add_to_history(cmd: &str) {
    unsafe {
        if COMMAND_HISTORY.is_none() {
            init_command_history();
        }
        
        if let Some(history) = &mut COMMAND_HISTORY {
            // Don't add empty commands or duplicates of the last command
            if !cmd.trim().is_empty() && history.last().map_or(true, |last| last != cmd) {
                history.push(cmd.to_string());
                
                // Limit history size to 100 entries
                if history.len() > 100 {
                    history.remove(0);
                }
            }
        }
    }
}

/// Register additional ex command handlers
pub fn register_additional_handlers(registry: &mut ExCommandRegistry) {
    // Initialize global variables
    init_variables();
    
    // Initialize command history
    init_command_history();
    
    // Register additional commands
    registry.register("delmarks", handle_delmarks);
    registry.register("history", handle_history);
    registry.register("source", handle_source);
    registry.register("let", handle_let);
    registry.register("unlet", handle_unlet);
    registry.register("echo", handle_echo);
    registry.register("scriptnames", handle_scriptnames);
    registry.register("abbreviate", handle_abbreviate);
    registry.register("ab", handle_abbreviate);
    registry.register("unabbreviate", handle_unabbreviate);
    registry.register("una", handle_unabbreviate);
    registry.register("digraphs", handle_digraphs);
    registry.register("changes", handle_changes);
}

/// Handle the :delmarks command
fn handle_delmarks(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match crate::command::handlers::EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Get the marks to delete
    let marks_to_delete = if let Some(mark_names) = cmd.first_arg() {
        // If specific marks were requested, delete only those
        mark_names.chars().collect::<Vec<char>>()
    } else {
        // If no marks were specified, show an error
        return Err(ExCommandError::MissingArgument("Mark names required".to_string()));
    };
    
    // Delete each mark
    let mut deleted_count = 0;
    for mark in marks_to_delete {
        if editor.delete_mark(mark)? {
            deleted_count += 1;
        }
    }
    
    if deleted_count > 0 {
        println!("{} mark{} deleted", deleted_count, if deleted_count == 1 { "" } else { "s" });
    } else {
        println!("No marks deleted");
    }
    
    Ok(())
}

/// Handle the :history command
fn handle_history(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the history type from the command arguments
    let history_type = cmd.first_arg().unwrap_or("cmd");
    
    // Get the command history
    let history = unsafe {
        match &COMMAND_HISTORY {
            Some(history) => history,
            None => {
                init_command_history();
                match &COMMAND_HISTORY {
                    Some(history) => history,
                    None => return Err(ExCommandError::Other("Failed to initialize command history".to_string())),
                }
            }
        }
    };
    
    match history_type {
        "cmd" | ":" => {
            // Display command history
            println!("--- Command History ---");
            for (i, cmd) in history.iter().enumerate() {
                println!("{:3} {}", i + 1, cmd);
            }
        },
        "search" | "/" | "?" => {
            // Display search history (not implemented yet)
            println!("--- Search History ---");
            println!("  (Search history not implemented yet)");
        },
        "expr" | "=" => {
            // Display expression history (not implemented yet)
            println!("--- Expression History ---");
            println!("  (Expression history not implemented yet)");
        },
        "input" | "@" => {
            // Display input history (not implemented yet)
            println!("--- Input History ---");
            println!("  (Input history not implemented yet)");
        },
        "debug" | ">" => {
            // Display debug history (not implemented yet)
            println!("--- Debug History ---");
            println!("  (Debug history not implemented yet)");
        },
        "all" => {
            // Display all history
            println!("--- Command History ---");
            for (i, cmd) in history.iter().enumerate() {
                println!("{:3} {}", i + 1, cmd);
            }
            println!("\n--- Search History ---");
            println!("  (Search history not implemented yet)");
            println!("\n--- Expression History ---");
            println!("  (Expression history not implemented yet)");
            println!("\n--- Input History ---");
            println!("  (Input history not implemented yet)");
            println!("\n--- Debug History ---");
            println!("  (Debug history not implemented yet)");
        },
        _ => {
            return Err(ExCommandError::InvalidArgument(format!("Invalid history type: {}", history_type)));
        }
    }
    
    Ok(())
}

/// Handle the :source command
fn handle_source(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match crate::command::handlers::EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Get the file path from the command arguments
    let file_path = match cmd.first_arg() {
        Some(path) => path,
        None => return Err(ExCommandError::MissingArgument("File name required".to_string())),
    };
    
    // Open the file
    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(err) => return Err(ExCommandError::Other(format!("Failed to open file: {}", err))),
    };
    
    // Create a command parser
    let parser = crate::command::CommandParser::new();
    
    // Create a command registry
    let mut registry = crate::command::ExCommandRegistry::new();
    crate::command::handlers::register_handlers(&mut registry, None);
    register_additional_handlers(&mut registry);
    
    // Read the file line by line and execute each line as a command
    let reader = io::BufReader::new(file);
    let mut line_number = 0;
    
    for line in reader.lines() {
        line_number += 1;
        
        match line {
            Ok(line) => {
                // Skip empty lines and comments
                let trimmed = line.trim();
                if trimmed.is_empty() || trimmed.starts_with('"') {
                    continue;
                }
                
                // Parse the line as an ex command
                match parser.parse_ex(trimmed) {
                    Ok(ex_cmd) => {
                        // Execute the command
                        if let Err(err) = registry.execute(&ex_cmd) {
                            return Err(ExCommandError::Other(format!("Error executing command on line {}: {}", line_number, err)));
                        }
                        
                        // Add the command to the history
                        add_to_history(trimmed);
                    },
                    Err(err) => {
                        return Err(ExCommandError::Other(format!("Error parsing command on line {}: {}", line_number, err)));
                    }
                }
            },
            Err(err) => {
                return Err(ExCommandError::Other(format!("Error reading line {}: {}", line_number, err)));
            }
        }
    }
    
    println!("Sourced {} lines from {}", line_number, file_path);
    Ok(())
}

/// Handle the :let command
pub fn handle_let(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the arguments
    let args = cmd.args_str();
    
    if args.is_empty() {
        // If no arguments, show all variables
        let variables = unsafe {
            match &VARIABLES {
                Some(vars) => vars,
                None => {
                    init_variables();
                    match &VARIABLES {
                        Some(vars) => vars,
                        None => return Err(ExCommandError::Other("Failed to initialize variables".to_string())),
                    }
                }
            }
        };
        
        if variables.is_empty() {
            println!("No variables defined");
        } else {
            println!("--- Variables ---");
            for (name, value) in variables {
                println!("{} = \"{}\"", name, value);
            }
        }
        
        return Ok(());
    }
    
    // Parse the variable assignment
    // Format: name = value
    let parts: Vec<&str> = args.splitn(2, '=').collect();
    
    if parts.len() != 2 {
        return Err(ExCommandError::InvalidArgument(format!("Invalid variable assignment: {}", args)));
    }
    
    let var_name = parts[0].trim();
    let var_value = parts[1].trim();
    
    // Remove quotes if present
    let var_value = if (var_value.starts_with('"') && var_value.ends_with('"')) || 
                      (var_value.starts_with('\'') && var_value.ends_with('\'')) {
        &var_value[1..var_value.len() - 1]
    } else {
        var_value
    };
    
    // Set the variable
    unsafe {
        if VARIABLES.is_none() {
            init_variables();
        }
        
        if let Some(vars) = &mut VARIABLES {
            vars.insert(var_name.to_string(), var_value.to_string());
        }
    }
    
    println!("{} = \"{}\"", var_name, var_value);
    Ok(())
}

/// Handle the :unlet command
fn handle_unlet(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the variable name from the command arguments
    let var_name = match cmd.first_arg() {
        Some(name) => name,
        None => return Err(ExCommandError::MissingArgument("Variable name required".to_string())),
    };
    
    // Delete the variable
    unsafe {
        if VARIABLES.is_none() {
            init_variables();
        }
        
        if let Some(vars) = &mut VARIABLES {
            if vars.remove(var_name).is_some() {
                println!("Variable {} deleted", var_name);
            } else {
                if cmd.flags.force {
                    // If force flag is set, don't show an error
                    println!("No such variable: {}", var_name);
                } else {
                    return Err(ExCommandError::Other(format!("No such variable: {}", var_name)));
                }
            }
        }
    }
    
    Ok(())
}

/// Handle the :echo command
fn handle_echo(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the arguments
    let args = cmd.args_str();
    
    if args.is_empty() {
        // If no arguments, just print a newline
        println!();
        return Ok(());
    }
    
    // Parse the expression
    // For now, just echo the literal text
    // In a real implementation, we would evaluate the expression
    
    // Remove quotes if present
    let text = if (args.starts_with('"') && args.ends_with('"')) || 
                 (args.starts_with('\'') && args.ends_with('\'')) {
        &args[1..args.len() - 1]
    } else {
        args
    };
    
    // Print the text
    println!("{}", text);
    
    Ok(())
}

/// Handle the :scriptnames command
fn handle_scriptnames(_cmd: &ExCommand) -> ExCommandResult<()> {
    // In a real implementation, this would show all sourced script files
    println!("--- Sourced Script Files ---");
    println!("  (Script names not implemented yet)");
    
    Ok(())
}

/// Handle the :abbreviate command
fn handle_abbreviate(cmd: &ExCommand) -> ExCommandResult<()> {
    // In a real implementation, this would define an abbreviation
    println!("--- Abbreviations ---");
    println!("  (Abbreviations not implemented yet)");
    
    Ok(())
}

/// Handle the :unabbreviate command
fn handle_unabbreviate(cmd: &ExCommand) -> ExCommandResult<()> {
    // In a real implementation, this would remove an abbreviation
    println!("--- Remove Abbreviation ---");
    println!("  (Abbreviation removal not implemented yet)");
    
    Ok(())
}

/// Handle the :digraphs command
fn handle_digraphs(_cmd: &ExCommand) -> ExCommandResult<()> {
    // In a real implementation, this would show all digraphs
    println!("--- Digraphs ---");
    println!("  (Digraphs not implemented yet)");
    
    Ok(())
}

/// Handle the :changes command
fn handle_changes(_cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match crate::command::handlers::EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // In a real implementation, this would show the change list
    println!("--- Change List ---");
    println!("  (Change list not implemented yet)");
    
    Ok(())
}