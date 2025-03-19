//! Configuration command handlers
//!
//! This module implements handlers for configuration commands.

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
use crate::config::{Config, ConfigManager, ConfigResult};
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

/// Global configuration manager
static mut CONFIG_MANAGER: Option<Arc<Mutex<ConfigManager>>> = None;

/// Initialize the configuration manager
pub fn init_config_manager() {
    unsafe {
        if CONFIG_MANAGER.is_none() {
            CONFIG_MANAGER = Some(Arc::new(Mutex::new(ConfigManager::new())));
            
            // Load the configuration
            if let Some(manager) = &CONFIG_MANAGER {
                let mut manager = manager.lock().unwrap();
                let _ = manager.load(); // Ignore errors, use defaults if loading fails
            }
        }
    }
}

/// Get the configuration manager
pub fn get_config_manager() -> Option<Arc<Mutex<ConfigManager>>> {
    unsafe {
        CONFIG_MANAGER.clone()
    }
}

/// Register configuration command handlers
pub fn register_config_handlers(registry: &mut ExCommandRegistry) {
    // Initialize the configuration manager
    init_config_manager();
    
    // Register configuration commands
    registry.register("config", handle_config);
    registry.register("set", handle_set);
    registry.register("setlocal", handle_set_local);
    registry.register("setglobal", handle_set_global);
    registry.register("colorscheme", handle_colorscheme);
}

/// Handle the :config command
fn handle_config(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the configuration manager
    let config_manager = match get_config_manager() {
        Some(manager) => manager,
        None => {
            init_config_manager();
            match get_config_manager() {
                Some(manager) => manager,
                None => return Err(ExCommandError::Other("Failed to initialize configuration manager".to_string())),
            }
        }
    };
    
    // Parse the command arguments
    let args = cmd.args_str();
    let parts: Vec<&str> = args.split_whitespace().collect();
    
    if parts.is_empty() {
        // Display the current configuration
        display_config(&config_manager)?;
    } else {
        match parts[0] {
            "set" => {
                // Set a configuration option
                if parts.len() < 3 {
                    return Err(ExCommandError::MissingArgument("Usage: config set <section>.<option> <value>".to_string()));
                }
                
                let option_path = parts[1];
                let value = parts[2..].join(" ");
                
                set_config_option(&config_manager, option_path, &value)?;
                println!("Set {} = {}", option_path, value);
            },
            "get" => {
                // Get a configuration option
                if parts.len() < 2 {
                    return Err(ExCommandError::MissingArgument("Usage: config get <section>.<option>".to_string()));
                }
                
                let option_path = parts[1];
                let value = get_config_option(&config_manager, option_path)?;
                println!("{} = {}", option_path, value);
            },
            "edit" => {
                // Edit the configuration file
                let editor = unsafe {
                    match crate::command::handlers::EDITOR {
                        Some(editor_ptr) => &mut *editor_ptr,
                        None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
                    }
                };
                
                let config_path = {
                    let manager = config_manager.lock().unwrap();
                    manager.config_path().to_path_buf()
                };
                
                // Open the configuration file in the editor
                crate::command::handle_edit(&ExCommand::new(
                    "edit",
                    crate::command::Range::default(),
                    crate::command::CommandFlags::default(),
                    vec![config_path.to_string_lossy().to_string()],
                    format!(":edit {}", config_path.to_string_lossy())
                ))?;
            },
            "reload" => {
                // Reload the configuration from disk
                let mut manager = config_manager.lock().unwrap();
                manager.load()?;
                println!("Configuration reloaded from {}", manager.config_path().display());
            },
            "save" => {
                // Save the configuration to disk
                let manager = config_manager.lock().unwrap();
                manager.save()?;
                println!("Configuration saved to {}", manager.config_path().display());
            },
            _ => {
                return Err(ExCommandError::InvalidArgument(format!("Unknown config subcommand: {}", parts[0])));
            }
        }
    }
    
    Ok(())
}

/// Handle the :set command
fn handle_set(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the configuration manager
    let config_manager = match get_config_manager() {
        Some(manager) => manager,
        None => {
            init_config_manager();
            match get_config_manager() {
                Some(manager) => manager,
                None => return Err(ExCommandError::Other("Failed to initialize configuration manager".to_string())),
            }
        }
    };
    
    // Parse the command arguments
    let args = cmd.args_str();
    
    if args.is_empty() {
        // Display all options that differ from their default values
        display_non_default_options(&config_manager)?;
        return Ok(());
    }
    
    // Handle multiple options separated by spaces
    let options: Vec<&str> = args.split_whitespace().collect();
    
    for option in options {
        if option.contains('=') {
            // Option with value: name=value
            let parts: Vec<&str> = option.splitn(2, '=').collect();
            let name = parts[0];
            let value = parts[1];
            
            set_option(&config_manager, name, value)?;
        } else if option.starts_with("no") {
            // Boolean option: noname
            let name = &option[2..];
            set_boolean_option(&config_manager, name, false)?;
        } else if option.ends_with('?') {
            // Query option: name?
            let name = &option[..option.len() - 1];
            display_option(&config_manager, name)?;
        } else if option.starts_with('+') {
            // Increment option: +name
            let name = &option[1..];
            increment_option(&config_manager, name)?;
        } else if option.starts_with('-') {
            // Decrement option: -name
            let name = &option[1..];
            decrement_option(&config_manager, name)?;
        } else if option.starts_with('^') {
            // Reset option to default: ^name
            let name = &option[1..];
            reset_option(&config_manager, name)?;
        } else {
            // Boolean option: name
            // Or just display the option if it's not boolean
            if is_boolean_option(option) {
                set_boolean_option(&config_manager, option, true)?;
            } else {
                display_option(&config_manager, option)?;
            }
        }
    }
    
    Ok(())
}

/// Handle the :setlocal command
fn handle_set_local(cmd: &ExCommand) -> ExCommandResult<()> {
    // For now, :setlocal is the same as :set
    // In the future, we'll implement buffer-local and window-local options
    handle_set(cmd)
}

/// Handle the :setglobal command
fn handle_set_global(cmd: &ExCommand) -> ExCommandResult<()> {
    // For now, :setglobal is the same as :set
    // In the future, we'll implement global-only option setting
    handle_set(cmd)
}

/// Handle the :colorscheme command
fn handle_colorscheme(cmd: &ExCommand) -> ExCommandResult<()> {
    // Get the configuration manager
    let config_manager = match get_config_manager() {
        Some(manager) => manager,
        None => {
            init_config_manager();
            match get_config_manager() {
                Some(manager) => manager,
                None => return Err(ExCommandError::Other("Failed to initialize configuration manager".to_string())),
            }
        }
    };
    
    // Parse the command arguments
    let args = cmd.args_str();
    
    if args.is_empty() {
        // Display the current colorscheme
        let manager = config_manager.lock().unwrap();
        let colorscheme = &manager.config().ui.color_scheme;
        println!("colorscheme={}", colorscheme);
        return Ok(());
    }
    
    // Set the colorscheme
    let colorscheme = args.trim();
    
    // TODO: Validate that the colorscheme exists
    
    // Update the configuration
    {
        let mut manager = config_manager.lock().unwrap();
        manager.config_mut().ui.color_scheme = colorscheme.to_string();
    }
    
    // Apply the colorscheme
    apply_colorscheme(colorscheme)?;
    
    println!("Colorscheme set to {}", colorscheme);
    
    Ok(())
}

/// Display the current configuration
fn display_config(config_manager: &Arc<Mutex<ConfigManager>>) -> ExCommandResult<()> {
    let manager = config_manager.lock().unwrap();
    let config = manager.config();
    
    println!("Configuration:");
    println!("  Path: {}", manager.config_path().display());
    println!();
    
    println!("General:");
    println!("  use_system_clipboard = {}", config.general.use_system_clipboard);
    println!("  show_line_numbers = {}", config.general.show_line_numbers);
    println!("  syntax_highlighting = {}", config.general.syntax_highlighting);
    println!("  auto_indent = {}", config.general.auto_indent);
    println!("  tab_width = {}", config.general.tab_width);
    println!("  expand_tabs = {}", config.general.expand_tabs);
    println!();
    
    println!("UI:");
    println!("  color_scheme = {}", config.ui.color_scheme);
    println!("  font_family = {}", config.ui.font_family);
    println!("  font_size = {}", config.ui.font_size);
    println!("  show_status_line = {}", config.ui.show_status_line);
    println!("  show_command_line = {}", config.ui.show_command_line);
    println!();
    
    println!("Key Mappings:");
    if config.keymaps.normal.is_empty() {
        println!("  No normal mode mappings");
    } else {
        println!("  Normal mode:");
        for (key, cmd) in &config.keymaps.normal {
            println!("    {} -> {}", key, cmd);
        }
    }
    
    if config.keymaps.insert.is_empty() {
        println!("  No insert mode mappings");
    } else {
        println!("  Insert mode:");
        for (key, cmd) in &config.keymaps.insert {
            println!("    {} -> {}", key, cmd);
        }
    }
    
    if config.keymaps.visual.is_empty() {
        println!("  No visual mode mappings");
    } else {
        println!("  Visual mode:");
        for (key, cmd) in &config.keymaps.visual {
            println!("    {} -> {}", key, cmd);
        }
    }
    
    if config.keymaps.command.is_empty() {
        println!("  No command mode mappings");
    } else {
        println!("  Command mode:");
        for (key, cmd) in &config.keymaps.command {
            println!("    {} -> {}", key, cmd);
        }
    }
    
    Ok(())
}

/// Set a configuration option
fn set_config_option(config_manager: &Arc<Mutex<ConfigManager>>, option_path: &str, value: &str) -> ExCommandResult<()> {
    let mut manager = config_manager.lock().unwrap();
    let config = manager.config_mut();
    
    let parts: Vec<&str> = option_path.split('.').collect();
    if parts.len() != 2 {
        return Err(ExCommandError::InvalidArgument(format!("Invalid option path: {}", option_path)));
    }
    
    let section = parts[0];
    let option = parts[1];
    
    match section {
        "general" => {
            match option {
                "use_system_clipboard" => {
                    config.general.use_system_clipboard = parse_bool(value)?;
                },
                "show_line_numbers" => {
                    config.general.show_line_numbers = parse_bool(value)?;
                },
                "syntax_highlighting" => {
                    config.general.syntax_highlighting = parse_bool(value)?;
                },
                "auto_indent" => {
                    config.general.auto_indent = parse_bool(value)?;
                },
                "tab_width" => {
                    config.general.tab_width = parse_u8(value)?;
                },
                "expand_tabs" => {
                    config.general.expand_tabs = parse_bool(value)?;
                },
                _ => {
                    return Err(ExCommandError::InvalidArgument(format!("Unknown option: general.{}", option)));
                }
            }
        },
        "ui" => {
            match option {
                "color_scheme" => {
                    let colorscheme = value.to_string();
                    config.ui.color_scheme = colorscheme.clone();
                    
                    // Store the colorscheme name for later use
                    let colorscheme_name = colorscheme;
                    
                    // Drop the lock before calling apply_colorscheme
                    drop(manager);
                    
                    // Apply the colorscheme
                    let _ = apply_colorscheme(&colorscheme_name);
                },
                "font_family" => {
                    config.ui.font_family = value.to_string();
                },
                "font_size" => {
                    config.ui.font_size = parse_u8(value)?;
                },
                "show_status_line" => {
                    config.ui.show_status_line = parse_bool(value)?;
                },
                "show_command_line" => {
                    config.ui.show_command_line = parse_bool(value)?;
                },
                _ => {
                    return Err(ExCommandError::InvalidArgument(format!("Unknown option: ui.{}", option)));
                }
            }
        },
        "keymaps" => {
            let mode_str = option;
            let parts: Vec<&str> = value.splitn(2, ' ').collect();
            if parts.len() != 2 {
                return Err(ExCommandError::InvalidArgument("Invalid keymap format. Use: keymaps.<mode> <key> <command>".to_string()));
            }
            
            let key = parts[0];
            let command = parts[1];
            
            match mode_str {
                "normal" => {
                    config.keymaps.normal.insert(key.to_string(), command.to_string());
                },
                "insert" => {
                    config.keymaps.insert.insert(key.to_string(), command.to_string());
                },
                "visual" => {
                    config.keymaps.visual.insert(key.to_string(), command.to_string());
                },
                "command" => {
                    config.keymaps.command.insert(key.to_string(), command.to_string());
                },
                _ => {
                    return Err(ExCommandError::InvalidArgument(format!("Unknown keymap mode: {}", mode_str)));
                }
            }
        },
        _ => {
            return Err(ExCommandError::InvalidArgument(format!("Unknown configuration section: {}", section)));
        }
    }
    
    Ok(())
}

/// Get a configuration option
fn get_config_option(config_manager: &Arc<Mutex<ConfigManager>>, option_path: &str) -> ExCommandResult<String> {
    let manager = config_manager.lock().unwrap();
    let config = manager.config();
    
    let parts: Vec<&str> = option_path.split('.').collect();
    if parts.len() != 2 {
        return Err(ExCommandError::InvalidArgument(format!("Invalid option path: {}", option_path)));
    }
    
    let section = parts[0];
    let option = parts[1];
    
    match section {
        "general" => {
            match option {
                "use_system_clipboard" => {
                    Ok(config.general.use_system_clipboard.to_string())
                },
                "show_line_numbers" => {
                    Ok(config.general.show_line_numbers.to_string())
                },
                "syntax_highlighting" => {
                    Ok(config.general.syntax_highlighting.to_string())
                },
                "auto_indent" => {
                    Ok(config.general.auto_indent.to_string())
                },
                "tab_width" => {
                    Ok(config.general.tab_width.to_string())
                },
                "expand_tabs" => {
                    Ok(config.general.expand_tabs.to_string())
                },
                _ => {
                    Err(ExCommandError::InvalidArgument(format!("Unknown option: general.{}", option)))
                }
            }
        },
        "ui" => {
            match option {
                "color_scheme" => {
                    Ok(config.ui.color_scheme.clone())
                },
                "font_family" => {
                    Ok(config.ui.font_family.clone())
                },
                "font_size" => {
                    Ok(config.ui.font_size.to_string())
                },
                "show_status_line" => {
                    Ok(config.ui.show_status_line.to_string())
                },
                "show_command_line" => {
                    Ok(config.ui.show_command_line.to_string())
                },
                _ => {
                    Err(ExCommandError::InvalidArgument(format!("Unknown option: ui.{}", option)))
                }
            }
        },
        _ => {
            Err(ExCommandError::InvalidArgument(format!("Unknown configuration section: {}", section)))
        }
    }
}

/// Parse a boolean value
fn parse_bool(value: &str) -> ExCommandResult<bool> {
    match value.to_lowercase().as_str() {
        "true" | "yes" | "1" | "on" => Ok(true),
        "false" | "no" | "0" | "off" => Ok(false),
        _ => Err(ExCommandError::InvalidArgument(format!("Invalid boolean value: {}", value))),
    }
}

/// Parse an unsigned 8-bit integer
fn parse_u8(value: &str) -> ExCommandResult<u8> {
    match value.parse::<u8>() {
        Ok(n) => Ok(n),
        Err(_) => Err(ExCommandError::InvalidArgument(format!("Invalid number: {}", value))),
    }
}

/// Display options that differ from their default values
fn display_non_default_options(config_manager: &Arc<Mutex<ConfigManager>>) -> ExCommandResult<()> {
    let manager = config_manager.lock().unwrap();
    let config = manager.config();
    let default_config = Config::default();
    
    // General options
    if config.general.use_system_clipboard != default_config.general.use_system_clipboard {
        println!("  use_system_clipboard={}", config.general.use_system_clipboard);
    }
    if config.general.show_line_numbers != default_config.general.show_line_numbers {
        println!("  show_line_numbers={}", config.general.show_line_numbers);
    }
    if config.general.syntax_highlighting != default_config.general.syntax_highlighting {
        println!("  syntax_highlighting={}", config.general.syntax_highlighting);
    }
    if config.general.auto_indent != default_config.general.auto_indent {
        println!("  auto_indent={}", config.general.auto_indent);
    }
    if config.general.tab_width != default_config.general.tab_width {
        println!("  tab_width={}", config.general.tab_width);
    }
    if config.general.expand_tabs != default_config.general.expand_tabs {
        println!("  expand_tabs={}", config.general.expand_tabs);
    }
    
    // UI options
    if config.ui.color_scheme != default_config.ui.color_scheme {
        println!("  color_scheme={}", config.ui.color_scheme);
    }
    if config.ui.font_family != default_config.ui.font_family {
        println!("  font_family={}", config.ui.font_family);
    }
    if config.ui.font_size != default_config.ui.font_size {
        println!("  font_size={}", config.ui.font_size);
    }
    if config.ui.show_status_line != default_config.ui.show_status_line {
        println!("  show_status_line={}", config.ui.show_status_line);
    }
    if config.ui.show_command_line != default_config.ui.show_command_line {
        println!("  show_command_line={}", config.ui.show_command_line);
    }
    
    Ok(())
}

/// Set an option using the Vim-style :set command
fn set_option(config_manager: &Arc<Mutex<ConfigManager>>, name: &str, value: &str) -> ExCommandResult<()> {
    let mut manager = config_manager.lock().unwrap();
    let config = manager.config_mut();
    
    match name {
        // General options
        "clipboard" => {
            config.general.use_system_clipboard = value == "unnamed";
        },
        "number" | "nu" => {
            config.general.show_line_numbers = parse_bool(value)?;
        },
        "syntax" => {
            config.general.syntax_highlighting = value == "on" || value == "enable";
        },
        "autoindent" | "ai" => {
            config.general.auto_indent = parse_bool(value)?;
        },
        "tabstop" | "ts" => {
            config.general.tab_width = parse_u8(value)?;
        },
        "expandtab" | "et" => {
            config.general.expand_tabs = parse_bool(value)?;
        },
        
        // UI options
        "background" | "bg" => {
            // TODO: Set the background color
        },
        "colorscheme" | "colo" => {
            let colorscheme = value.to_string();
            config.ui.color_scheme = colorscheme.clone();
            
            // Store the colorscheme name for later use
            let colorscheme_name = colorscheme;
            
            // Drop the lock before calling apply_colorscheme
            drop(manager);
            
            // Apply the colorscheme
            let _ = apply_colorscheme(&colorscheme_name);
        },
        "guifont" => {
            config.ui.font_family = value.to_string();
        },
        "guifontsize" => {
            config.ui.font_size = parse_u8(value)?;
        },
        "laststatus" => {
            config.ui.show_status_line = value != "0";
        },
        "cmdheight" => {
            config.ui.show_command_line = value != "0";
        },
        
        _ => {
            return Err(ExCommandError::InvalidArgument(format!("Unknown option: {}", name)));
        }
    }
    
    Ok(())
}

/// Set a boolean option
fn set_boolean_option(config_manager: &Arc<Mutex<ConfigManager>>, name: &str, value: bool) -> ExCommandResult<()> {
    let mut manager = config_manager.lock().unwrap();
    let config = manager.config_mut();
    
    match name {
        // General options
        "number" | "nu" => {
            config.general.show_line_numbers = value;
        },
        "autoindent" | "ai" => {
            config.general.auto_indent = value;
        },
        "expandtab" | "et" => {
            config.general.expand_tabs = value;
        },
        
        // UI options
        "laststatus" => {
            config.ui.show_status_line = value;
        },
        
        _ => {
            if !is_boolean_option(name) {
                return Err(ExCommandError::InvalidArgument(format!("Not a boolean option: {}", name)));
            }
        }
    }
    
    Ok(())
}

/// Check if an option is a boolean option
fn is_boolean_option(name: &str) -> bool {
    matches!(name, 
        "number" | "nu" | 
        "autoindent" | "ai" | 
        "expandtab" | "et" | 
        "laststatus"
    )
}

/// Display an option
fn display_option(config_manager: &Arc<Mutex<ConfigManager>>, name: &str) -> ExCommandResult<()> {
    let manager = config_manager.lock().unwrap();
    let config = manager.config();
    
    match name {
        // General options
        "clipboard" => {
            println!("  clipboard={}", if config.general.use_system_clipboard { "unnamed" } else { "" });
        },
        "number" | "nu" => {
            println!("  {}={}", name, config.general.show_line_numbers);
        },
        "syntax" => {
            println!("  syntax={}", if config.general.syntax_highlighting { "on" } else { "off" });
        },
        "autoindent" | "ai" => {
            println!("  {}={}", name, config.general.auto_indent);
        },
        "tabstop" | "ts" => {
            println!("  {}={}", name, config.general.tab_width);
        },
        "expandtab" | "et" => {
            println!("  {}={}", name, config.general.expand_tabs);
        },
        
        // UI options
        "background" | "bg" => {
            // TODO: Get the background color
            println!("  {}=dark", name);
        },
        "colorscheme" | "colo" => {
            println!("  {}={}", name, config.ui.color_scheme);
        },
        "guifont" => {
            println!("  guifont={}", config.ui.font_family);
        },
        "guifontsize" => {
            println!("  guifontsize={}", config.ui.font_size);
        },
        "laststatus" => {
            println!("  laststatus={}", if config.ui.show_status_line { "2" } else { "0" });
        },
        "cmdheight" => {
            println!("  cmdheight={}", if config.ui.show_command_line { "1" } else { "0" });
        },
        
        "all" => {
            // Display all options
            println!("--- General Options ---");
            println!("  clipboard={}", if config.general.use_system_clipboard { "unnamed" } else { "" });
            println!("  number={}", config.general.show_line_numbers);
            println!("  syntax={}", if config.general.syntax_highlighting { "on" } else { "off" });
            println!("  autoindent={}", config.general.auto_indent);
            println!("  tabstop={}", config.general.tab_width);
            println!("  expandtab={}", config.general.expand_tabs);
            
            println!("--- UI Options ---");
            println!("  background=dark");
            println!("  colorscheme={}", config.ui.color_scheme);
            println!("  guifont={}", config.ui.font_family);
            println!("  guifontsize={}", config.ui.font_size);
            println!("  laststatus={}", if config.ui.show_status_line { "2" } else { "0" });
            println!("  cmdheight={}", if config.ui.show_command_line { "1" } else { "0" });
        },
        
        _ => {
            return Err(ExCommandError::InvalidArgument(format!("Unknown option: {}", name)));
        }
    }
    
    Ok(())
}

/// Increment a numeric option
fn increment_option(config_manager: &Arc<Mutex<ConfigManager>>, name: &str) -> ExCommandResult<()> {
    let mut manager = config_manager.lock().unwrap();
    let config = manager.config_mut();
    
    match name {
        "tabstop" | "ts" => {
            config.general.tab_width = config.general.tab_width.saturating_add(1);
        },
        "guifontsize" => {
            config.ui.font_size = config.ui.font_size.saturating_add(1);
        },
        _ => {
            return Err(ExCommandError::InvalidArgument(format!("Not a numeric option: {}", name)));
        }
    }
    
    Ok(())
}

/// Decrement a numeric option
fn decrement_option(config_manager: &Arc<Mutex<ConfigManager>>, name: &str) -> ExCommandResult<()> {
    let mut manager = config_manager.lock().unwrap();
    let config = manager.config_mut();
    
    match name {
        "tabstop" | "ts" => {
            config.general.tab_width = config.general.tab_width.saturating_sub(1);
        },
        "guifontsize" => {
            config.ui.font_size = config.ui.font_size.saturating_sub(1);
        },
        _ => {
            return Err(ExCommandError::InvalidArgument(format!("Not a numeric option: {}", name)));
        }
    }
    
    Ok(())
}

/// Reset an option to its default value
fn reset_option(config_manager: &Arc<Mutex<ConfigManager>>, name: &str) -> ExCommandResult<()> {
    let mut manager = config_manager.lock().unwrap();
    let config = manager.config_mut();
    let default_config = Config::default();
    
    match name {
        // General options
        "clipboard" => {
            config.general.use_system_clipboard = default_config.general.use_system_clipboard;
        },
        "number" | "nu" => {
            config.general.show_line_numbers = default_config.general.show_line_numbers;
        },
        "syntax" => {
            config.general.syntax_highlighting = default_config.general.syntax_highlighting;
        },
        "autoindent" | "ai" => {
            config.general.auto_indent = default_config.general.auto_indent;
        },
        "tabstop" | "ts" => {
            config.general.tab_width = default_config.general.tab_width;
        },
        "expandtab" | "et" => {
            config.general.expand_tabs = default_config.general.expand_tabs;
        },
        
        // UI options
        "colorscheme" | "colo" => {
            let colorscheme = default_config.ui.color_scheme.clone();
            config.ui.color_scheme = colorscheme.clone();
            
            // Store the colorscheme name for later use
            let colorscheme_name = colorscheme;
            
            // Drop the lock before calling apply_colorscheme
            drop(manager);
            
            // Apply the colorscheme
            let _ = apply_colorscheme(&colorscheme_name);
        },
        "guifont" => {
            config.ui.font_family = default_config.ui.font_family.clone();
        },
        "guifontsize" => {
            config.ui.font_size = default_config.ui.font_size;
        },
        "laststatus" => {
            config.ui.show_status_line = default_config.ui.show_status_line;
        },
        "cmdheight" => {
            config.ui.show_command_line = default_config.ui.show_command_line;
        },
        
        "all" => {
            // Reset all options
            *config = Config::default();
        },
        
        _ => {
            return Err(ExCommandError::InvalidArgument(format!("Unknown option: {}", name)));
        }
    }
    
    Ok(())
}

/// Apply a colorscheme
fn apply_colorscheme(colorscheme: &str) -> ExCommandResult<()> {
    // Get the editor instance
    let editor = unsafe {
        match crate::command::handlers::EDITOR {
            Some(editor_ptr) => &mut *editor_ptr,
            None => return Err(ExCommandError::InvalidCommand("Editor not initialized".to_string())),
        }
    };
    
    // Apply the colorscheme
    match crate::syntax::apply_color_scheme(editor, colorscheme) {
        Ok(true) => {
            println!("Applied colorscheme: {}", colorscheme);
            Ok(())
        },
        Ok(false) => {
            Err(ExCommandError::InvalidArgument(format!("Colorscheme not found: {}", colorscheme)))
        },
        Err(err) => {
            Err(ExCommandError::Other(format!("Failed to apply colorscheme: {}", err)))
        }
    }
}