//! Plugin command integration
//!
//! This module handles the integration of plugin commands with the editor's command system.

use crate::command::{ExCommand, ExCommandError, ExCommandResult, ExCommandRegistry};
use crate::plugin::PluginManager;
use std::sync::{Arc, Mutex};

/// Initialize plugin commands
///
/// This function registers all plugin commands with the editor's command registry.
pub fn register_plugin_commands(registry: &mut ExCommandRegistry, plugin_manager: Arc<Mutex<PluginManager>>) {
    // Register the NoxChat command
    {
        let pm = plugin_manager.clone();
        registry.register("NoxChat", move |_cmd| {
            handle_plugin_command("NoxChat", &[], pm.clone())
        });
    }

    // Register the NoxGenerate command
    {
        let pm = plugin_manager.clone();
        registry.register("NoxGenerate", move |cmd| {
            handle_plugin_command("NoxGenerate", &cmd.args, pm.clone())
        });
    }

    // Register the NoxRefactor command
    {
        let pm = plugin_manager.clone();
        registry.register("NoxRefactor", move |cmd| {
            handle_plugin_command("NoxRefactor", &cmd.args, pm.clone())
        });
    }

    // Register the NoxExplain command
    {
        let pm = plugin_manager.clone();
        registry.register("NoxExplain", move |cmd| {
            handle_plugin_command("NoxExplain", &cmd.args, pm.clone())
        });
    }

    // Register the NoxFix command
    {
        let pm = plugin_manager.clone();
        registry.register("NoxFix", move |cmd| {
            handle_plugin_command("NoxFix", &cmd.args, pm.clone())
        });
    }

    // Register the NoxTest command
    {
        let pm = plugin_manager.clone();
        registry.register("NoxTest", move |cmd| {
            handle_plugin_command("NoxTest", &cmd.args, pm.clone())
        });
    }

    // Register the NoxToggleAutoApprove command
    {
        let pm = plugin_manager.clone();
        registry.register("NoxToggleAutoApprove", move |cmd| {
            handle_plugin_command("NoxToggleAutoApprove", &cmd.args, pm.clone())
        });
    }
}

/// Handle a plugin command
///
/// This function executes a plugin command with the given arguments.
fn handle_plugin_command(name: &str, args: &[String], plugin_manager: Arc<Mutex<PluginManager>>) -> ExCommandResult<()> {
    // Add debug output
    println!("DEBUG: handle_plugin_command called with name: {}, args: {:?}", name, args);
    
    // Convert the arguments to a slice of &str
    let args_str: Vec<&str> = args.iter().map(|s| s.as_str()).collect();

    // Execute the command
    let mut plugin_manager = plugin_manager.lock().unwrap();
    println!("DEBUG: Acquired lock on plugin_manager, executing command: {}", name);
    
    match plugin_manager.execute_command(name, &args_str) {
        Ok(_) => {
            println!("DEBUG: Command {} executed successfully", name);
            Ok(())
        },
        Err(err) => {
            println!("DEBUG: Command {} failed with error: {}", name, err);
            Err(ExCommandError::InvalidCommand(format!("Failed to execute plugin command: {}", err)))
        },
    }
}