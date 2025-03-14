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
    register_plugin("noxvim", "0.1.0", "AI Agent Plugin for xvim", "xvim Team");
    
    // Register commands
    register_command("NoxChat", nox_chat_command);
    register_command("NoxGenerate", nox_generate_command);
    register_command("NoxRefactor", nox_refactor_command);
    register_command("NoxExplain", nox_explain_command);
    register_command("NoxFix", nox_fix_command);
    register_command("NoxTest", nox_test_command);
    register_command("NoxToggleAutoApprove", nox_toggle_auto_approve_command);
    
    // Register event handlers
    register_event_handler("buffer_created", buffer_created_handler);
    register_event_handler("buffer_changed", buffer_changed_handler);
    register_event_handler("cursor_moved", cursor_moved_handler);
    register_event_handler("mode_changed", mode_changed_handler);
    
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
    let buffer_id = get_current_buffer_id().ok_or("No active buffer")?;
    let cursor_pos = get_cursor_position().ok_or("Could not get cursor position")?;
    
    // Create a task for code generation
    let task_id = create_task("Code Generation", &format!("Generating code based on: {}", description), move || {
        // Get project context
        let context = context_provider::get_project_context()?;
        
        // Generate code using the AI service
        let generated_code = ai_service::generate_code(&description, &context)?;
        
        // Return the generated code
        Ok(generated_code.into_bytes())
    }, Some(Box::new(move |result| {
        match result {
            TaskResult::Success(data) => {
                let code = String::from_utf8_lossy(&data).to_string();
                
                // Check if auto-approve is enabled
                let auto_approve = unsafe { AUTO_APPROVE };
                
                if auto_approve {
                    // Insert the code at the cursor position
                    if let Err(err) = insert_at_cursor(buffer_id, &code) {
                        editor_message(&format!("Failed to insert code: {}", err));
                    } else {
                        editor_message("Code generated and inserted successfully");
                    }
                } else {
                    // Show the generated code in the output buffer
                    if let Err(err) = ui_manager::update_output_buffer(&format!("# Generated Code\n\n```\n{}\n```\n\nUse `:NoxApply` to apply this code", code)) {
                        editor_message(&format!("Failed to display generated code: {}", err));
                    }
                }
            },
            TaskResult::Error(err) => {
                editor_message(&format!("Code generation failed: {}", err));
            },
            TaskResult::Cancelled => {
                editor_message("Code generation was cancelled");
            }
        }
    })));
    
    editor_message(&format!("Started code generation task (ID: {})", task_id));
    
    Ok(())
}

/// Refactor the current selection or file
fn nox_refactor_command(_args: &[&str]) -> Result<(), String> {
    // Get the current buffer and selection
    let buffer_id = get_current_buffer_id().ok_or("No active buffer")?;
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
    let task_id = create_task("Code Refactoring", "Refactoring selected code", move || {
        // Get project context
        let context = context_provider::get_project_context()?;
        
        // Refactor code using the AI service
        let refactored_code = ai_service::refactor_code(&selected_text, &context)?;
        
        // Return the refactored code
        Ok(refactored_code.into_bytes())
    }, Some(Box::new(move |result| {
        match result {
            TaskResult::Success(data) => {
                let code = String::from_utf8_lossy(&data).to_string();
                
                // Check if auto-approve is enabled
                let auto_approve = unsafe { AUTO_APPROVE };
                
                if auto_approve {
                    // Replace the selected text with the refactored code
                    if let Err(err) = replace_text(buffer_id, selection.0, selection.1, &code) {
                        editor_message(&format!("Failed to apply refactored code: {}", err));
                    } else {
                        editor_message("Code refactored successfully");
                    }
                } else {
                    // Show the refactored code in the output buffer
                    if let Err(err) = ui_manager::update_output_buffer(&format!("# Refactored Code\n\n```\n{}\n```\n\nUse `:NoxApply` to apply this code", code)) {
                        editor_message(&format!("Failed to display refactored code: {}", err));
                    }
                }
            },
            TaskResult::Error(err) => {
                editor_message(&format!("Refactoring failed: {}", err));
            },
            TaskResult::Cancelled => {
                editor_message("Refactoring was cancelled");
            }
        }
    })));
    
    editor_message(&format!("Started refactoring task (ID: {})", task_id));
    
    Ok(())
}

/// Explain the selected code
fn nox_explain_command(_args: &[&str]) -> Result<(), String> {
    // Get the current buffer and selection
    let buffer_id = get_current_buffer_id().ok_or("No active buffer")?;
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
    let task_id = create_task("Code Explanation", "Explaining selected code", move || {
        // Explain code using the AI service
        let explanation = ai_service::explain_code(&selected_text)?;
        
        // Return the explanation
        Ok(explanation.into_bytes())
    }, Some(Box::new(move |result| {
        match result {
            TaskResult::Success(data) => {
                let explanation = String::from_utf8_lossy(&data).to_string();
                
                // Show the explanation in the output buffer
                if let Err(err) = ui_manager::update_output_buffer(&format!("# Code Explanation\n\n{}", explanation)) {
                    editor_message(&format!("Failed to display explanation: {}", err));
                }
            },
            TaskResult::Error(err) => {
                editor_message(&format!("Explanation failed: {}", err));
            },
            TaskResult::Cancelled => {
                editor_message("Explanation was cancelled");
            }
        }
    })));
    
    editor_message(&format!("Started explanation task (ID: {})", task_id));
    
    Ok(())
}

/// Fix issues in the current file
fn nox_fix_command(_args: &[&str]) -> Result<(), String> {
    // Get the current buffer
    let buffer_id = get_current_buffer_id().ok_or("No active buffer")?;
    let buffer_content = get_buffer_content(buffer_id).unwrap_or_default();
    
    // Create a task for fixing issues
    let task_id = create_task("Code Fixing", "Fixing issues in the current file", move || {
        // Get project context
        let context = context_provider::get_project_context()?;
        
        // Fix code using the AI service
        let fixed_code = ai_service::fix_code(&buffer_content, &context)?;
        
        // Return the fixed code
        Ok(fixed_code.into_bytes())
    }, Some(Box::new(move |result| {
        match result {
            TaskResult::Success(data) => {
                let code = String::from_utf8_lossy(&data).to_string();
                
                // Check if auto-approve is enabled
                let auto_approve = unsafe { AUTO_APPROVE };
                
                if auto_approve {
                    // Replace the entire buffer with the fixed code
                    if let Err(err) = set_buffer_content(buffer_id, &code) {
                        editor_message(&format!("Failed to apply fixed code: {}", err));
                    } else {
                        editor_message("Code fixed successfully");
                    }
                } else {
                    // Show the fixed code in the output buffer
                    if let Err(err) = ui_manager::update_output_buffer(&format!("# Fixed Code\n\n```\n{}\n```\n\nUse `:NoxApply` to apply this code", code)) {
                        editor_message(&format!("Failed to display fixed code: {}", err));
                    }
                }
            },
            TaskResult::Error(err) => {
                editor_message(&format!("Fixing failed: {}", err));
            },
            TaskResult::Cancelled => {
                editor_message("Fixing was cancelled");
            }
        }
    })));
    
    editor_message(&format!("Started fixing task (ID: {})", task_id));
    
    Ok(())
}

/// Generate tests for the current file
fn nox_test_command(_args: &[&str]) -> Result<(), String> {
    // Get the current buffer
    let buffer_id = get_current_buffer_id().ok_or("No active buffer")?;
    let buffer_content = get_buffer_content(buffer_id).unwrap_or_default();
    
    // Create a task for test generation
    let task_id = create_task("Test Generation", "Generating tests for the current file", move || {
        // Get project context
        let context = context_provider::get_project_context()?;
        
        // Generate tests using the AI service
        let tests = ai_service::generate_tests(&buffer_content, &context)?;
        
        // Return the tests
        Ok(tests.into_bytes())
    }, Some(Box::new(move |result| {
        match result {
            TaskResult::Success(data) => {
                let tests = String::from_utf8_lossy(&data).to_string();
                
                // Show the tests in the output buffer
                if let Err(err) = ui_manager::update_output_buffer(&format!("# Generated Tests\n\n```\n{}\n```\n\nUse `:NoxApply` to create a test file", tests)) {
                    editor_message(&format!("Failed to display tests: {}", err));
                }
            },
            TaskResult::Error(err) => {
                editor_message(&format!("Test generation failed: {}", err));
            },
            TaskResult::Cancelled => {
                editor_message("Test generation was cancelled");
            }
        }
    })));
    
    editor_message(&format!("Started test generation task (ID: {})", task_id));
    
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
        context_provider::update_mode_context(mode);
    }
    
    Ok(())
}

// Helper functions

/// Insert text at the cursor position
fn insert_at_cursor(buffer_id: usize, text: &str) -> Result<(), String> {
    let cursor_pos = get_cursor_position().ok_or("Could not get cursor position")?;
    let char_idx = position_to_char_idx(buffer_id, cursor_pos.0, cursor_pos.1)
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

// Plugin API mock for the example
mod xvim_plugin_api {
    // Plugin registration
    pub fn register_plugin(name: &str, version: &str, description: &str, author: &str) {
        // This would be implemented by the xvim plugin API
    }
    
    // Command registration
    pub fn register_command(name: &str, handler: fn(&[&str]) -> Result<(), String>) {
        // This would be implemented by the xvim plugin API
    }
    
    // Event handler registration
    pub fn register_event_handler(event_type: &str, handler: fn(&Event) -> Result<(), String>) {
        // This would be implemented by the xvim plugin API
    }
    
    // Display a message in the editor
    pub fn editor_message(message: &str) {
        // This would be implemented by the xvim plugin API
    }
    
    // Log a message to the plugin log
    pub fn log_message(message: &str) {
        // This would be implemented by the xvim plugin API
    }
    
    // Get the current buffer ID
    pub fn get_current_buffer_id() -> Option<usize> {
        // This would be implemented by the xvim plugin API
        Some(0)
    }
    
    // Get the cursor position
    pub fn get_cursor_position() -> Option<(usize, usize)> {
        // This would be implemented by the xvim plugin API
        Some((0, 0))
    }
    
    // Get the current selection
    pub fn get_selection() -> Option<(usize, usize)> {
        // This would be implemented by the xvim plugin API
        None
    }
    
    // Get buffer content
    pub fn get_buffer_content(buffer_id: usize) -> Option<String> {
        // This would be implemented by the xvim plugin API
        Some(String::new())
    }
    
    // Set buffer content
    pub fn set_buffer_content(buffer_id: usize, content: &str) -> Result<(), String> {
        // This would be implemented by the xvim plugin API
        Ok(())
    }
    
    // Insert text at a position
    pub fn insert_text(buffer_id: usize, position: usize, text: &str) -> Result<(), String> {
        // This would be implemented by the xvim plugin API
        Ok(())
    }
    
    // Delete text
    pub fn delete_text(buffer_id: usize, start: usize, end: usize) -> Result<(), String> {
        // This would be implemented by the xvim plugin API
        Ok(())
    }
    
    // Convert position to character index
    pub fn position_to_char_idx(buffer_id: usize, line: usize, column: usize) -> Result<usize, String> {
        // This would be implemented by the xvim plugin API
        Ok(0)
    }
    
    // Create a task
    pub fn create_task<F, T>(name: &str, description: &str, f: F, callback: Option<Box<dyn FnOnce(TaskResult) + Send + 'static>>) -> String
    where
        F: FnOnce() -> Result<T, String> + Send + 'static,
        T: Into<Vec<u8>> + Send + 'static,
    {
        // This would be implemented by the xvim plugin API
        "task_1".to_string()
    }
    
    // Event types
    pub enum Event {
        BufferCreated(usize),
        BufferDeleted(usize),
        BufferChanged(usize),
        ModeChanged(String),
        CursorMoved(usize, usize, usize),
        CommandExecuted(String),
        Custom(String, Vec<u8>),
    }
    
    // Task result
    pub enum TaskResult {
        Success(Vec<u8>),
        Error(String),
        Cancelled,
    }
}

// Re-export the xvim plugin API
use xvim_plugin_api::*;