//! Plugin command integration
//!
//! This module handles the integration of plugin commands with the editor's command system.

use crate::command::{ExCommand, ExCommandError, ExCommandResult, ExCommandRegistry, Range, CommandFlags};
use crate::plugin::PluginManager;
use crate::plugin::ai;
use std::sync::{Arc, Mutex};

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
        
        // Create a simplified version of the chat interface
        // This avoids the complex lock management that was causing issues
        
        // First, get the plugin manager lock
        let mut plugin_manager_guard = match plugin_manager.lock() {
            Ok(guard) => guard,
            Err(_) => return Err(ExCommandError::InvalidCommand("Failed to lock plugin manager".to_string())),
        };
        
        // Get the context
        let context = plugin_manager_guard.context();
        
        // Drop the plugin manager lock to avoid deadlocks
        drop(plugin_manager_guard);
        
        // Get the context lock
        let context_guard = match context.lock() {
            Ok(guard) => guard,
            Err(_) => return Err(ExCommandError::InvalidCommand("Failed to lock plugin context".to_string())),
        };
        
        // Get the buffer manager
        let buffer_manager = match context_guard.buffer_manager() {
            Some(bm) => bm,
            None => return Err(ExCommandError::InvalidCommand("Buffer manager not available".to_string())),
        };
        
        // Get the command registry
        let command_registry = match context_guard.command_registry() {
            Some(cr) => cr,
            None => return Err(ExCommandError::InvalidCommand("Command registry not available".to_string())),
        };
        
        // Drop the context lock to avoid deadlocks
        drop(context_guard);
        
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
        
        // Create the chat interface buffers
        eprintln!("DEBUG: Creating chat interface buffers");
        match ai::create_chat_interface(&mut buffer_manager_guard, &mut plugin_manager_guard) {
            Ok((output_buffer_id, input_buffer_id)) => {
                eprintln!("DEBUG: Created chat interface with output buffer ID {} and input buffer ID {}", output_buffer_id, input_buffer_id);
                
                // Create an AI conversation manager if it doesn't exist
                if plugin_manager_guard.ai_conversation_manager().is_none() {
                    eprintln!("DEBUG: Creating AI conversation manager");
                    let mut ai_manager = ai::AiConversationManager::new();
                    
                    // Create a conversation with the output and input buffer IDs
                    let conversation_id = ai_manager.create_conversation(output_buffer_id, input_buffer_id);
                    eprintln!("DEBUG: Created conversation with ID {}", conversation_id);
                    
                    // Store the AI conversation manager in the plugin manager
                    plugin_manager_guard.set_ai_conversation_manager(ai_manager);
                } else {
                    eprintln!("DEBUG: AI conversation manager already exists");
                    
                    // Get the AI conversation manager
                    let ai_manager = plugin_manager_guard.ai_conversation_manager_mut().unwrap();
                    
                    // Create a conversation with the output and input buffer IDs
                    let conversation_id = ai_manager.create_conversation(output_buffer_id, input_buffer_id);
                    eprintln!("DEBUG: Created conversation with ID {}", conversation_id);
                }
                
                // Get the command registry lock
                let command_registry_guard = match command_registry.lock() {
                    Ok(guard) => guard,
                    Err(_) => return Err(ExCommandError::InvalidCommand("Failed to lock command registry".to_string())),
                };
                
                // Drop locks before executing commands to avoid deadlocks
                drop(buffer_manager_guard);
                drop(plugin_manager_guard);
                drop(command_registry_guard);
                
                // Now use Ex commands to create the split window layout
                // This approach is inspired by claude.vim which avoids UI-related deadlocks
                
                // First, make sure we're viewing the output buffer
                let output_buffer_name = format!("NoxVim-Output");
                let edit_cmd = ExCommand {
                    name: "edit".to_string(),
                    args: vec![output_buffer_name],
                    range: Range::new(None, None),
                    flags: CommandFlags::default(),
                    raw: format!("edit NoxVim-Output"),
                };
                
                // Execute the edit command
                if let Err(e) = crate::command::handlers::handle_edit(&edit_cmd) {
                    eprintln!("DEBUG: Failed to switch to output buffer: {}", e);
                    // Continue anyway, as we've already created the buffers
                }
                
                // Now split the window horizontally and open the input buffer in the bottom window
                let split_cmd = ExCommand {
                    name: "split".to_string(),
                    args: vec![],
                    range: Range::new(None, None),
                    flags: CommandFlags::default(),
                    raw: format!("split"),
                };
                
                // Execute the split command
                if let Err(e) = crate::command::handlers::handle_split(&split_cmd) {
                    eprintln!("DEBUG: Failed to split window: {}", e);
                    // Continue anyway, as we've already created the buffers
                }
                
                // Switch to the input buffer in the new window
                let input_buffer_name = format!("NoxVim-Input");
                let edit_cmd = ExCommand {
                    name: "edit".to_string(),
                    args: vec![input_buffer_name],
                    range: Range::new(None, None),
                    flags: CommandFlags::default(),
                    raw: format!("edit NoxVim-Input"),
                };
                
                // Execute the edit command
                if let Err(e) = crate::command::handlers::handle_edit(&edit_cmd) {
                    eprintln!("DEBUG: Failed to switch to input buffer: {}", e);
                    // Continue anyway, as we've already created the buffers
                }
                
                eprintln!("DEBUG: Chat interface created successfully");
                return Ok(());
            },
            Err(e) => {
                eprintln!("DEBUG: Failed to create chat interface: {}", e);
                return Err(ExCommandError::InvalidCommand(format!("Failed to create chat interface: {}", e)));
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