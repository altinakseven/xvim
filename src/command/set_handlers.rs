//! Set command handlers
//!
//! This module implements handlers for set commands.

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

/// Set option
#[derive(Debug, Clone)]
pub enum SetOption {
    /// Boolean option
    Boolean(bool),
    /// String option
    String(String),
    /// Number option
    Number(i64),
}

/// Set manager
#[derive(Debug)]
pub struct SetManager {
    /// Options
    pub options: HashMap<String, SetOption>,
}

impl SetManager {
    /// Create a new set manager
    pub fn new() -> Self {
        let mut options = HashMap::new();
        
        // Initialize default options
        options.insert("number".to_string(), SetOption::Boolean(false));
        options.insert("relativenumber".to_string(), SetOption::Boolean(false));
        options.insert("tabstop".to_string(), SetOption::Number(8));
        options.insert("shiftwidth".to_string(), SetOption::Number(8));
        options.insert("expandtab".to_string(), SetOption::Boolean(false));
        options.insert("autoindent".to_string(), SetOption::Boolean(true));
        options.insert("smartindent".to_string(), SetOption::Boolean(true));
        options.insert("ignorecase".to_string(), SetOption::Boolean(false));
        options.insert("smartcase".to_string(), SetOption::Boolean(false));
        options.insert("hlsearch".to_string(), SetOption::Boolean(true));
        options.insert("incsearch".to_string(), SetOption::Boolean(true));
        options.insert("wrap".to_string(), SetOption::Boolean(true));
        options.insert("linebreak".to_string(), SetOption::Boolean(false));
        options.insert("list".to_string(), SetOption::Boolean(false));
        options.insert("listchars".to_string(), SetOption::String("tab:> ,trail:-,nbsp:+".to_string()));
        options.insert("cursorline".to_string(), SetOption::Boolean(false));
        options.insert("cursorcolumn".to_string(), SetOption::Boolean(false));
        options.insert("colorcolumn".to_string(), SetOption::String("".to_string()));
        options.insert("syntax".to_string(), SetOption::String("on".to_string()));
        options.insert("background".to_string(), SetOption::String("dark".to_string()));
        
        Self { options }
    }
    
    /// Get an option
    pub fn get(&self, name: &str) -> Option<&SetOption> {
        self.options.get(name)
    }
    
    /// Set an option
    pub fn set(&mut self, name: &str, value: SetOption) {
        self.options.insert(name.to_string(), value);
    }
    
    /// Toggle a boolean option
    pub fn toggle(&mut self, name: &str) -> Result<bool, String> {
        match self.options.get(name) {
            Some(SetOption::Boolean(value)) => {
                let new_value = !value;
                self.options.insert(name.to_string(), SetOption::Boolean(new_value));
                Ok(new_value)
            },
            Some(_) => Err(format!("Option {} is not a boolean option", name)),
            None => Err(format!("Unknown option: {}", name)),
        }
    }
    
    /// Reset all options to their default values
    pub fn reset(&mut self) {
        *self = Self::new();
    }
}

// Global set manager
static mut SET_MANAGER: Option<SetManager> = None;

/// Initialize the set manager
pub fn init_set_manager() {
    unsafe {
        if SET_MANAGER.is_none() {
            SET_MANAGER = Some(SetManager::new());
        }
    }
}

/// Register set command handlers
pub fn register_set_handlers(registry: &mut ExCommandRegistry) {
    // Initialize the set manager
    init_set_manager();
    
    // Register set commands
    registry.register("set", handle_set);
    registry.register("setlocal", handle_set_local);
    registry.register("setglobal", handle_set_global);
}

/// Handle the :set command
pub fn handle_set(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the editor reference
    let editor = unsafe {
        match crate::command::handlers::EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Get the set manager
    let set_manager = unsafe {
        match &mut SET_MANAGER {
            Some(manager) => manager,
            None => {
                init_set_manager();
                match &mut SET_MANAGER {
                    Some(manager) => manager,
                    None => return Err(ExCommandError::Other("Failed to initialize set manager".to_string())),
                }
            }
        }
    };
    
    // Get the arguments
    let args = cmd.args_str();
    
    if args.is_empty() {
        // If no arguments, show all options
        println!("--- Options ---");
        
        // Sort the options by name
        let mut options: Vec<(&String, &SetOption)> = set_manager.options.iter().collect();
        options.sort_by(|a, b| a.0.cmp(b.0));
        
        // Display each option
        for (name, value) in options {
            match value {
                SetOption::Boolean(value) => {
                    if *value {
                        println!("  {}", name);
                    } else {
                        println!("  no{}", name);
                    }
                },
                SetOption::String(value) => {
                    println!("  {}={}", name, value);
                },
                SetOption::Number(value) => {
                    println!("  {}={}", name, value);
                },
            }
        }
        
        return Ok(());
    }
    
    // Parse the option
    let parts: Vec<&str> = args.split('=').collect();
    
    if parts.len() == 1 {
        // If only one part, it's a boolean option or a show option
        let option = parts[0].trim();
        
        if option.starts_with("no") {
            // Boolean option being turned off
            let option_name = &option[2..];
            
            // Check if the option exists
            match set_manager.get(option_name) {
                Some(SetOption::Boolean(_)) => {
                    // Set the option to false
                    set_manager.set(option_name, SetOption::Boolean(false));
                    println!("Option {} set to off", option_name);
                },
                Some(_) => {
                    return Err(ExCommandError::InvalidArgument(format!("Option {} is not a boolean option", option_name)));
                },
                None => {
                    return Err(ExCommandError::InvalidArgument(format!("Unknown option: {}", option_name)));
                },
            }
        } else if option.starts_with('?') {
            // Show the option value
            let option_name = &option[1..];
            
            // Check if the option exists
            match set_manager.get(option_name) {
                Some(SetOption::Boolean(value)) => {
                    println!("{}={}", option_name, if *value { "on" } else { "off" });
                },
                Some(SetOption::String(value)) => {
                    println!("{}={}", option_name, value);
                },
                Some(SetOption::Number(value)) => {
                    println!("{}={}", option_name, value);
                },
                None => {
                    return Err(ExCommandError::InvalidArgument(format!("Unknown option: {}", option_name)));
                },
            }
        } else if option.starts_with('&') {
            // Reset the option to its default value
            let option_name = &option[1..];
            
            // Check if the option exists
            match set_manager.get(option_name) {
                Some(_) => {
                    // Reset the option
                    let default_manager = SetManager::new();
                    let default_value = default_manager.get(option_name).unwrap();
                    set_manager.set(option_name, default_value.clone());
                    println!("Option {} reset to default", option_name);
                },
                None => {
                    return Err(ExCommandError::InvalidArgument(format!("Unknown option: {}", option_name)));
                },
            }
        } else if option == "all" {
            // Show all options
            println!("--- All Options ---");
            
            // Sort the options by name
            let mut options: Vec<(&String, &SetOption)> = set_manager.options.iter().collect();
            options.sort_by(|a, b| a.0.cmp(b.0));
            
            // Display each option
            for (name, value) in options {
                match value {
                    SetOption::Boolean(value) => {
                        if *value {
                            println!("  {}", name);
                        } else {
                            println!("  no{}", name);
                        }
                    },
                    SetOption::String(value) => {
                        println!("  {}={}", name, value);
                    },
                    SetOption::Number(value) => {
                        println!("  {}={}", name, value);
                    },
                }
            }
        } else {
            // Boolean option being turned on or show option value
            
            // Check if the option exists
            match set_manager.get(option) {
                Some(SetOption::Boolean(_)) => {
                    // Set the option to true
                    set_manager.set(option, SetOption::Boolean(true));
                    println!("Option {} set to on", option);
                },
                Some(SetOption::String(value)) => {
                    println!("{}={}", option, value);
                },
                Some(SetOption::Number(value)) => {
                    println!("{}={}", option, value);
                },
                None => {
                    return Err(ExCommandError::InvalidArgument(format!("Unknown option: {}", option)));
                },
            }
        }
    } else if parts.len() == 2 {
        // Setting an option value
        let option_name = parts[0].trim();
        let option_value = parts[1].trim();
        
        // Check if the option exists
        match set_manager.get(option_name) {
            Some(SetOption::Boolean(_)) => {
                // Parse the boolean value
                match option_value {
                    "on" | "true" | "yes" | "1" => {
                        set_manager.set(option_name, SetOption::Boolean(true));
                        println!("Option {} set to on", option_name);
                    },
                    "off" | "false" | "no" | "0" => {
                        set_manager.set(option_name, SetOption::Boolean(false));
                        println!("Option {} set to off", option_name);
                    },
                    _ => {
                        return Err(ExCommandError::InvalidArgument(format!("Invalid boolean value: {}", option_value)));
                    },
                }
            },
            Some(SetOption::String(_)) => {
                // Set the string value
                set_manager.set(option_name, SetOption::String(option_value.to_string()));
                println!("Option {} set to {}", option_name, option_value);
            },
            Some(SetOption::Number(_)) => {
                // Parse the number value
                match option_value.parse::<i64>() {
                    Ok(value) => {
                        set_manager.set(option_name, SetOption::Number(value));
                        println!("Option {} set to {}", option_name, value);
                    },
                    Err(_) => {
                        return Err(ExCommandError::InvalidArgument(format!("Invalid number value: {}", option_value)));
                    },
                }
            },
            None => {
                return Err(ExCommandError::InvalidArgument(format!("Unknown option: {}", option_name)));
            },
        }
    } else {
        return Err(ExCommandError::InvalidArgument(format!("Invalid set format: {}", args)));
    }
    
    Ok(())
}

/// Handle the :setlocal command
fn handle_set_local(cmd: &ExCommand) -> ExCommandResult<()> {
    // For now, just delegate to the global set command
    handle_set(cmd)
}

/// Handle the :setglobal command
fn handle_set_global(cmd: &ExCommand) -> ExCommandResult<()> {
    // For now, just delegate to the global set command
    handle_set(cmd)
}

/// Get a boolean option
pub fn get_bool_option(name: &str) -> bool {
    unsafe {
        match &SET_MANAGER {
            Some(manager) => {
                match manager.get(name) {
                    Some(SetOption::Boolean(value)) => *value,
                    _ => false,
                }
            },
            None => false,
        }
    }
}

/// Get a string option
pub fn get_string_option(name: &str) -> String {
    unsafe {
        match &SET_MANAGER {
            Some(manager) => {
                match manager.get(name) {
                    Some(SetOption::String(value)) => value.clone(),
                    _ => String::new(),
                }
            },
            None => String::new(),
        }
    }
}

/// Get a number option
pub fn get_number_option(name: &str) -> i64 {
    unsafe {
        match &SET_MANAGER {
            Some(manager) => {
                match manager.get(name) {
                    Some(SetOption::Number(value)) => *value,
                    _ => 0,
                }
            },
            None => 0,
        }
    }
}

/// Set a boolean option
pub fn set_bool_option(name: &str, value: bool) {
    unsafe {
        if let Some(manager) = &mut SET_MANAGER {
            manager.set(name, SetOption::Boolean(value));
        }
    }
}

/// Set a string option
pub fn set_string_option(name: &str, value: &str) {
    unsafe {
        if let Some(manager) = &mut SET_MANAGER {
            manager.set(name, SetOption::String(value.to_string()));
        }
    }
}

/// Set a number option
pub fn set_number_option(name: &str, value: i64) {
    unsafe {
        if let Some(manager) = &mut SET_MANAGER {
            manager.set(name, SetOption::Number(value));
        }
    }
}