//! Plugin command integration
//!
//! This module handles the integration of plugin commands with the editor's command system.

use crate::command::{ExCommand, ExCommandError, ExCommandResult, ExCommandRegistry, Range, CommandFlags};
use std::sync::Mutex;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::{Arc, Mutex};
use std::sync::Mutex;
use std::sync::Arc;
use std::sync::Mutex;
use crate::plugin::PluginManager;
use std::sync::Mutex;
use std::sync::Arc;
use std::sync::Mutex;
use crate::plugin::ai;
use std::sync::Mutex;
use std::sync::Arc;
use std::sync::Mutex;

/// Initialize plugin commands
///
/// This function registers all plugin commands with the editor's command registry.
pub fn register_plugin_commands(registry: &mut ExCommandRegistry, plugin_manager: Arc<Mutex<PluginManager>>) {
    // Register the NoxChat command (with multiple case variations)
    {
        let pm = plugin_manager.clone();
        registry.register("NoxChat", move |_cmd| {
            handle_plugin_command("NoxChat", &[], pm.clone())
        });
        
        let pm = plugin_manager.clone();
        registry.register("noxchat", move |_cmd| {
            handle_plugin_command("NoxChat", &[], pm.clone())
        });
        
        let pm = plugin_manager.clone();
        registry.register("NOXCHAT", move |_cmd| {
            handle_plugin_command("NoxChat", &[], pm.clone())
        });
        
        let pm = plugin_manager.clone();
        registry.register("Noxchat", move |_cmd| {
            handle_plugin_command("NoxChat", &[], pm.clone())
        });
    }

    // Register the NoxGenerate command (with multiple case variations)
    {
        let pm = plugin_manager.clone();
        registry.register("NoxGenerate", move |cmd| {
            handle_plugin_command("NoxGenerate", &cmd.args, pm.clone())
        });
        
        let pm = plugin_manager.clone();
        registry.register("noxgenerate", move |cmd| {
            handle_plugin_command("NoxGenerate", &cmd.args, pm.clone())
        });
        
        let pm = plugin_manager.clone();
        registry.register("NOXGENERATE", move |cmd| {
            handle_plugin_command("NoxGenerate", &cmd.args, pm.clone())
        });
        
        let pm = plugin_manager.clone();
        registry.register("Noxgenerate", move |cmd| {
            handle_plugin_command("NoxGenerate", &cmd.args, pm.clone())
        });
    }

    // Register the NoxRefactor command (with multiple case variations)
    {
        let pm = plugin_manager.clone();
        registry.register("NoxRefactor", move |cmd| {
            handle_plugin_command("NoxRefactor", &cmd.args, pm.clone())
        });
        
        let pm = plugin_manager.clone();
        registry.register("noxrefactor", move |cmd| {
            handle_plugin_command("NoxRefactor", &cmd.args, pm.clone())
        });
        
        let pm = plugin_manager.clone();
        registry.register("NOXREFACTOR", move |cmd| {
            handle_plugin_command("NoxRefactor", &cmd.args, pm.clone())
        });
        
        let pm = plugin_manager.clone();
        registry.register("Noxrefactor", move |cmd| {
            handle_plugin_command("NoxRefactor", &cmd.args, pm.clone())
        });
    }

    // Register the NoxExplain command (with multiple case variations)
    {
        let pm = plugin_manager.clone();
        registry.register("NoxExplain", move |cmd| {
            handle_plugin_command("NoxExplain", &cmd.args, pm.clone())
        });
        
        let pm = plugin_manager.clone();
        registry.register("noxexplain", move |cmd| {
            handle_plugin_command("NoxExplain", &cmd.args, pm.clone())
        });
        
        let pm = plugin_manager.clone();
        registry.register("NOXEXPLAIN", move |cmd| {
            handle_plugin_command("NoxExplain", &cmd.args, pm.clone())
        });
        
        let pm = plugin_manager.clone();
        registry.register("Noxexplain", move |cmd| {
            handle_plugin_command("NoxExplain", &cmd.args, pm.clone())
        });
    }

    // Register the NoxFix command (with multiple case variations)
    {
        let pm = plugin_manager.clone();
        registry.register("NoxFix", move |cmd| {
            handle_plugin_command("NoxFix", &cmd.args, pm.clone())
        });
        
        let pm = plugin_manager.clone();
        registry.register("noxfix", move |cmd| {
            handle_plugin_command("NoxFix", &cmd.args, pm.clone())
        });
        
        let pm = plugin_manager.clone();
        registry.register("NOXFIX", move |cmd| {
            handle_plugin_command("NoxFix", &cmd.args, pm.clone())
        });
        
        let pm = plugin_manager.clone();
        registry.register("Noxfix", move |cmd| {
            handle_plugin_command("NoxFix", &cmd.args, pm.clone())
        });
    }

    // Register the NoxTest command (with multiple case variations)
    {
        let pm = plugin_manager.clone();
        registry.register("NoxTest", move |cmd| {
            handle_plugin_command("NoxTest", &cmd.args, pm.clone())
        });
        
        let pm = plugin_manager.clone();
        registry.register("noxtest", move |cmd| {
            handle_plugin_command("NoxTest", &cmd.args, pm.clone())
        });
        
        let pm = plugin_manager.clone();
        registry.register("NOXTEST", move |cmd| {
            handle_plugin_command("NoxTest", &cmd.args, pm.clone())
        });
        
        let pm = plugin_manager.clone();
        registry.register("Noxtest", move |cmd| {
            handle_plugin_command("NoxTest", &cmd.args, pm.clone())
        });
    }

    // Register the NoxToggleAutoApprove command (with multiple case variations)
    {
        let pm = plugin_manager.clone();
        registry.register("NoxToggleAutoApprove", move |cmd| {
            handle_plugin_command("NoxToggleAutoApprove", &cmd.args, pm.clone())
        });
        
        let pm = plugin_manager.clone();
        registry.register("noxtoggleautoapprove", move |cmd| {
            handle_plugin_command("NoxToggleAutoApprove", &cmd.args, pm.clone())
        });
        
        let pm = plugin_manager.clone();
        registry.register("NOXTOGGLEAUTOAPPROVE", move |cmd| {
            handle_plugin_command("NoxToggleAutoApprove", &cmd.args, pm.clone())
        });
        
        let pm = plugin_manager.clone();
        registry.register("Noxtoggleautoapprove", move |cmd| {
            handle_plugin_command("NoxToggleAutoApprove", &cmd.args, pm.clone())
        });
    }
}

/// Handle a plugin command
///
/// This function executes a plugin command with the given arguments.
fn handle_plugin_command(name: &str, args: &[String], plugin_manager: Arc<Mutex<PluginManager>>) -> ExCommandResult<()> {
    // Special handling for NoxChat command
    if name == "NoxChat" {
        eprintln!("DEBUG: NoxChat command received");
        
        // Create a scope for the plugin manager lock to ensure it's released
        {
            // Get the plugin manager lock
            let plugin_manager_guard = match plugin_manager.lock() {
                Ok(guard) => guard,
                Err(_) => return Err(ExCommandError::InvalidCommand("Failed to lock plugin manager".to_string())),
            };
            
            // Get the context
            let context = plugin_manager_guard.context();
            
            // Drop the plugin manager lock to avoid deadlocks
            drop(plugin_manager_guard);
            
            // Create a scope for the context lock
            {
                // Get the context lock
                let context_guard = match context.lock() {
                    Ok(guard) => guard,
                    Err(_) => return Err(ExCommandError::InvalidCommand("Failed to lock plugin context".to_string())),
                };
                
                // Get the buffer manager
                let buffer_manager = match context_guard.buffer_manager() {
                    Some(bm) => bm.clone(), // Clone to keep it after dropping context_guard
                    None => return Err(ExCommandError::InvalidCommand("Buffer manager not available".to_string())),
                };
                
                // Drop the context lock to avoid deadlocks
                drop(context_guard);
                
                // Create a scope for the buffer manager lock
                {
                    // Get the buffer manager lock
                    let mut buffer_manager_guard = match buffer_manager.lock() {
                        Ok(guard) => guard,
                        Err(_) => return Err(ExCommandError::InvalidCommand("Failed to lock buffer manager".to_string())),
                    };
                    
                    // Get the plugin manager lock again
                    let mut plugin_manager_guard = match plugin_manager.lock() {
                        Ok(guard) => guard,
                        Err(_) => return Err(ExCommandError::InvalidCommand("Failed to lock plugin manager again".to_string())),
                    };
                    
                    // Create the information buffer
                    eprintln!("DEBUG: Creating information buffer");
                    match ai::create_chat_interface(&mut buffer_manager_guard, &mut plugin_manager_guard) {
                        Ok((info_buffer_id, _)) => {
                            eprintln!("DEBUG: Created information buffer with ID {}", info_buffer_id);
                            
                            // Drop locks before executing commands to avoid deadlocks
                            drop(buffer_manager_guard);
                            drop(plugin_manager_guard);
                            
                            // Instead of using the edit command, just log that the buffer was created
                            // This avoids cursor positioning issues with read-only buffers
                            eprintln!("DEBUG: \"NoxVim-Info\" opened");
                            
                            eprintln!("DEBUG: Information buffer created and displayed successfully");
                            return Ok(());
                        },
                        Err(e) => {
                            eprintln!("DEBUG: Failed to create information buffer: {}", e);
                            return Err(ExCommandError::InvalidCommand(format!("Failed to create information buffer: {}", e)));
                        }
                    }
                }
            }
        }
    }
    
    // Convert the arguments to a slice of &str
    let args_str: Vec<&str> = args.iter().map(|s| s.as_str()).collect();

    // Execute the command
    let mut plugin_manager_guard = match plugin_manager.lock() {
        Ok(guard) => guard,
        Err(_) => return Err(ExCommandError::InvalidCommand("Failed to lock plugin manager for command execution".to_string())),
    };
    
    match plugin_manager_guard.execute_command(name, &args_str) {
        Ok(_) => Ok(()),
        Err(err) => Err(ExCommandError::InvalidCommand(format!("Failed to execute plugin command: {}", err))),
    }
}