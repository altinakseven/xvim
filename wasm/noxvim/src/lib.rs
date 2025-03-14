//! NoxVim: AI Agent Plugin for xvim
//!
//! This plugin provides AI-powered code assistance, automation, and chat capabilities
//! directly within the xvim editor, similar to Roo-Code in VSCode.

// Import the xvim plugin API
use xvim_plugin_api::*;

// Module declarations
mod ai_service;
mod ui_manager;
mod context_provider;
mod command_executor;
mod config;
mod utils;

use config::NoxVimConfig;

// Global state
static mut CONFIG: Option<NoxVimConfig> = None;
static mut AUTO_APPROVE: bool = true;

// Plugin entry point
#[no_mangle]
pub extern "C" fn init() -> i32 {
    // Initialize configuration
    unsafe {
        CONFIG = Some(NoxVimConfig::default());
    }
    
    // Register the plugin with the editor
    register_plugin(PluginInfo {
        name: "noxvim".to_string(),
        version: "0.1.0".to_string(),
        description: "AI Agent Plugin for xvim".to_string(),
        author: "xvim Team".to_string(),
    }).map_err(|e| {
        log_message(&format!("Failed to register plugin: {}", e));
    }).ok();
    
    // Register commands
    register_command("NoxChat", "Open the NoxVim chat interface", nox_chat_command).ok();
    register_command("NoxGenerate", "Generate code based on a description", nox_generate_command).ok();
    register_command("NoxRefactor", "Refactor the current selection or file", nox_refactor_command).ok();
    register_command("NoxExplain", "Explain the selected code", nox_explain_command).ok();
    register_command("NoxFix", "Fix issues in the current file", nox_fix_command).ok();
    register_command("NoxTest", "Generate tests for the current file", nox_test_command).ok();
    register_command("NoxToggleAutoApprove", "Toggle auto-approve mode", nox_toggle_auto_approve_command).ok();
    
    // Register event handlers
    register_event_handler("buffer_created", buffer_created_handler).ok();
    register_event_handler("buffer_changed", buffer_changed_handler).ok();
    register_event_handler("cursor_moved", cursor_moved_handler).ok();
    register_event_handler("mode_changed", mode_changed_handler).ok();
    
    // Log initialization
    log_message("NoxVim plugin initialized");
    
    // Return success
    0
}

// Command handlers

/// Open the NoxVim chat interface
fn nox_chat_command(_args: &[&str]) -> Result<(), String> {
    // Create the chat interface
    ui_manager::create_chat_interface()?;
    
    // Display welcome message
    ui_manager::update_output_buffer("# Welcome to NoxVim!\n\nI'm your AI assistant. How can I help you today?\n\n- Type your request below and press Ctrl-] to send it\n- Use specific commands like `:NoxGenerate` for targeted tasks\n- Toggle auto-approve mode with `:NoxToggleAutoApprove`")?;
    
    Ok(())
}

/// Generate code based on a description
fn nox_generate_command(args: &[&str]) -> Result<(), String> {
    if args.is_empty() {
        return Err("Please provide a description of the code to generate".to_string());
    }
    
    // Join all arguments into a single description
    let description = args.join(" ");
    
    // Get the current buffer and cursor position
    let buffer_id = get_current_buffer_id().map_err(|e| format!("No active buffer: {}", e))?;
    let cursor_pos = get_cursor_position().map_err(|e| format!("Could not get cursor position: {}", e))?;
    
    // Create a task for code generation
    let task_id = create_task("Code Generation", &format!("Generating code based on: {}", description), Some(Box::new(move |result| {
        // This is a simplified implementation since we can't run async tasks
        // In a real implementation, we would:
        // 1. Get project context
        // 2. Generate code using the AI service
        // 3. Return the generated code
        
        // For now, just show a message
        editor_message("Code generation task started (simplified implementation)");
    }))).map_err(|e| format!("Failed to create task: {}", e))?;
    
    editor_message(&format!("Started code generation task (ID: {:?})", task_id));
    
    Ok(())
}

/// Refactor the current selection or file
fn nox_refactor_command(_args: &[&str]) -> Result<(), String> {
    // Get the current buffer and selection
    let buffer_id = get_current_buffer_id().map_err(|e| format!("No active buffer: {}", e))?;
    let selection = get_selection().unwrap_or_else(|| {
        // If no selection, use the entire buffer
        let content = get_buffer_content(buffer_id).unwrap_or_default();
        (0, content.len())
    });
    
    // Get the selected text
    let buffer_content = get_buffer_content(buffer_id).unwrap_or_default();
    let selected_text = buffer_content.get(selection.0..selection.1)
        .ok_or("Invalid selection range")?
        .to_string();
    
    // Create a task for refactoring
    let task_id = create_task("Code Refactoring", "Refactoring selected code", Some(Box::new(move |result| {
        // This is a simplified implementation since we can't run async tasks
        // In a real implementation, we would:
        // 1. Get project context
        // 2. Refactor code using the AI service
        // 3. Return the refactored code
        
        // For now, just show a message
        editor_message("Code refactoring task started (simplified implementation)");
    }))).map_err(|e| format!("Failed to create task: {}", e))?;
    
    editor_message(&format!("Started refactoring task (ID: {:?})", task_id));
    
    Ok(())
}

/// Explain the selected code
fn nox_explain_command(_args: &[&str]) -> Result<(), String> {
    // Get the current buffer and selection
    let buffer_id = get_current_buffer_id().map_err(|e| format!("No active buffer: {}", e))?;
    let selection = get_selection().unwrap_or_else(|| {
        // If no selection, use the entire buffer
        let content = get_buffer_content(buffer_id).unwrap_or_default();
        (0, content.len())
    });
    
    // Get the selected text
    let buffer_content = get_buffer_content(buffer_id).unwrap_or_default();
    let selected_text = buffer_content.get(selection.0..selection.1)
        .ok_or("Invalid selection range")?
        .to_string();
    
    // Create a task for explanation
    let task_id = create_task("Code Explanation", "Explaining selected code", Some(Box::new(move |result| {
        // This is a simplified implementation since we can't run async tasks
        // In a real implementation, we would:
        // 1. Explain code using the AI service
        // 2. Return the explanation
        
        // For now, just show a message
        editor_message("Code explanation task started (simplified implementation)");
    }))).map_err(|e| format!("Failed to create task: {}", e))?;
    
    editor_message(&format!("Started explanation task (ID: {:?})", task_id));
    
    Ok(())
}

/// Fix issues in the current file
fn nox_fix_command(_args: &[&str]) -> Result<(), String> {
    // Get the current buffer
    let buffer_id = get_current_buffer_id().map_err(|e| format!("No active buffer: {}", e))?;
    let buffer_content = get_buffer_content(buffer_id).unwrap_or_default();
    
    // Create a task for fixing issues
    let task_id = create_task("Code Fixing", "Fixing issues in the current file", Some(Box::new(move |result| {
        // This is a simplified implementation since we can't run async tasks
        // In a real implementation, we would:
        // 1. Get project context
        // 2. Fix code using the AI service
        // 3. Return the fixed code
        
        // For now, just show a message
        editor_message("Code fixing task started (simplified implementation)");
    }))).map_err(|e| format!("Failed to create task: {}", e))?;
    
    editor_message(&format!("Started fixing task (ID: {:?})", task_id));
    
    Ok(())
}

/// Generate tests for the current file
fn nox_test_command(_args: &[&str]) -> Result<(), String> {
    // Get the current buffer
    let buffer_id = get_current_buffer_id().map_err(|e| format!("No active buffer: {}", e))?;
    let buffer_content = get_buffer_content(buffer_id).unwrap_or_default();
    
    // Create a task for test generation
    let task_id = create_task("Test Generation", "Generating tests for the current file", Some(Box::new(move |result| {
        // This is a simplified implementation since we can't run async tasks
        // In a real implementation, we would:
        // 1. Get project context
        // 2. Generate tests using the AI service
        // 3. Return the tests
        
        // For now, just show a message
        editor_message("Test generation task started (simplified implementation)");
    }))).map_err(|e| format!("Failed to create task: {}", e))?;
    
    editor_message(&format!("Started test generation task (ID: {:?})", task_id));
    
    Ok(())
}

/// Toggle auto-approve mode
fn nox_toggle_auto_approve_command(_args: &[&str]) -> Result<(), String> {
    unsafe {
        AUTO_APPROVE = !AUTO_APPROVE;
        editor_message(&format!("Auto-approve mode: {}", if AUTO_APPROVE { "ON" } else { "OFF" }));
    }
    
    Ok(())
}

// Event handlers

/// Handle buffer created events
fn buffer_created_handler(event: &Event) -> Result<(), String> {
    if let Event::BufferCreated(buffer_id) = event {
        // Log buffer creation
        log_message(&format!("Buffer {} was created", buffer_id));
        
        // Update context
        context_provider::update_buffer_context(*buffer_id);
    }
    
    Ok(())
}

/// Handle buffer changed events
fn buffer_changed_handler(event: &Event) -> Result<(), String> {
    if let Event::BufferChanged(buffer_id) = event {
        // Update context
        context_provider::update_buffer_context(*buffer_id);
    }
    
    Ok(())
}

/// Handle cursor moved events
fn cursor_moved_handler(event: &Event) -> Result<(), String> {
    if let Event::CursorMoved(buffer_id, line, column) = event {
        // Update context
        context_provider::update_cursor_context(*buffer_id, *line, *column);
    }
    
    Ok(())
}

/// Handle mode changed events
fn mode_changed_handler(event: &Event) -> Result<(), String> {
    if let Event::ModeChanged(mode) = event {
        // Update context
        context_provider::update_mode_context(&format!("{:?}", mode));
    }
    
    Ok(())
}

// Helper functions

/// Insert text at the cursor position
fn insert_at_cursor(buffer_id: usize, text: &str) -> Result<(), String> {
    let cursor_pos = get_cursor_position().map_err(|e| format!("Could not get cursor position: {}", e))?;
    let char_idx = position_to_char_idx(buffer_id, cursor_pos.line, cursor_pos.column)
        .map_err(|e| format!("Failed to convert position to character index: {}", e))?;
    
    insert_text(buffer_id, char_idx, text)
        .map_err(|e| format!("Failed to insert text: {}", e))
}

/// Replace text in a buffer
fn replace_text(buffer_id: usize, start: usize, end: usize, text: &str) -> Result<(), String> {
    // Delete the existing text
    delete_text(buffer_id, start, end)
        .map_err(|e| format!("Failed to delete text: {}", e))?;
    
    // Insert the new text
    insert_text(buffer_id, start, text)
        .map_err(|e| format!("Failed to insert text: {}", e))
}

// Import the xvim plugin API
use xvim_plugin_api::*;