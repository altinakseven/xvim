//! UI Manager for the NoxVim plugin
//!
//! This module handles the UI components of the NoxVim plugin, including
//! the chat interface, output buffer, and input buffer.

use crate::xvim_plugin_api::*;

// Buffer IDs for the chat interface
static mut INPUT_BUFFER_ID: Option<usize> = None;
static mut OUTPUT_BUFFER_ID: Option<usize> = None;

/// Create the chat interface
///
/// This creates a split window with an input buffer at the bottom
/// and an output buffer at the top.
pub fn create_chat_interface() -> Result<(), String> {
    // Create the output buffer
    let output_buffer_id = create_buffer("NoxVim-Output", true, false)
        .map_err(|e| format!("Failed to create output buffer: {}", e))?;
    
    // Create the input buffer
    let input_buffer_id = create_buffer("NoxVim-Input", false, true)
        .map_err(|e| format!("Failed to create input buffer: {}", e))?;
    
    // Store the buffer IDs
    unsafe {
        OUTPUT_BUFFER_ID = Some(output_buffer_id);
        INPUT_BUFFER_ID = Some(input_buffer_id);
    }
    
    // Split the window horizontally
    split_window(SplitDirection::Horizontal)
        .map_err(|e| format!("Failed to split window: {}", e))?;
    
    // Set the output buffer in the top window
    set_window_buffer(0, output_buffer_id)
        .map_err(|e| format!("Failed to set output buffer: {}", e))?;
    
    // Set the input buffer in the bottom window
    set_window_buffer(1, input_buffer_id)
        .map_err(|e| format!("Failed to set input buffer: {}", e))?;
    
    // Set the window heights
    set_window_height(0, 15)
        .map_err(|e| format!("Failed to set output window height: {}", e))?;
    set_window_height(1, 5)
        .map_err(|e| format!("Failed to set input window height: {}", e))?;
    
    // Set up keybindings for the input buffer
    set_buffer_keymap(input_buffer_id, "n", "<C-]>", "NoxExecutePrompt")
        .map_err(|e| format!("Failed to set input buffer keymap: {}", e))?;
    
    // Focus the input buffer
    set_current_window(1)
        .map_err(|e| format!("Failed to focus input window: {}", e))?;
    
    // Set the input buffer to insert mode
    execute_command("startinsert")
        .map_err(|e| format!("Failed to start insert mode: {}", e))?;
    
    Ok(())
}

/// Update the output buffer with new content
///
/// This replaces the content of the output buffer with the given text.
pub fn update_output_buffer(text: &str) -> Result<(), String> {
    let output_buffer_id = unsafe { OUTPUT_BUFFER_ID.ok_or("Output buffer not created")? };
    
    // Set the output buffer content
    set_buffer_content(output_buffer_id, text)
        .map_err(|e| format!("Failed to update output buffer: {}", e))?;
    
    Ok(())
}

/// Append text to the output buffer
///
/// This adds the given text to the end of the output buffer.
pub fn append_to_output_buffer(text: &str) -> Result<(), String> {
    let output_buffer_id = unsafe { OUTPUT_BUFFER_ID.ok_or("Output buffer not created")? };
    
    // Get the current content
    let current_content = get_buffer_content(output_buffer_id)
        .ok_or("Failed to get output buffer content")?;
    
    // Append the new text
    let new_content = format!("{}\n{}", current_content, text);
    
    // Set the output buffer content
    set_buffer_content(output_buffer_id, &new_content)
        .map_err(|e| format!("Failed to update output buffer: {}", e))?;
    
    Ok(())
}

/// Get the content of the input buffer
///
/// This returns the current content of the input buffer.
pub fn get_input_buffer_content() -> Result<String, String> {
    let input_buffer_id = unsafe { INPUT_BUFFER_ID.ok_or("Input buffer not created")? };
    
    // Get the input buffer content
    get_buffer_content(input_buffer_id)
        .ok_or("Failed to get input buffer content".to_string())
}

/// Clear the input buffer
///
/// This removes all content from the input buffer.
pub fn clear_input_buffer() -> Result<(), String> {
    let input_buffer_id = unsafe { INPUT_BUFFER_ID.ok_or("Input buffer not created")? };
    
    // Clear the input buffer
    set_buffer_content(input_buffer_id, "")
        .map_err(|e| format!("Failed to clear input buffer: {}", e))
}

/// Execute the prompt in the input buffer
///
/// This gets the content of the input buffer, clears it, and processes the prompt.
pub fn execute_prompt() -> Result<(), String> {
    // Get the input buffer content
    let prompt = get_input_buffer_content()?;
    
    // Clear the input buffer
    clear_input_buffer()?;
    
    // Process the prompt
    process_prompt(&prompt)?;
    
    Ok(())
}

/// Process a prompt
///
/// This sends the prompt to the AI service and displays the response.
fn process_prompt(prompt: &str) -> Result<(), String> {
    // Display the prompt in the output buffer
    append_to_output_buffer(&format!("\n## User\n\n{}\n", prompt))?;
    
    // Create a task for processing the prompt
    let prompt_str = prompt.to_string();
    let task_id = create_task("Process Prompt", "Processing user prompt", move || {
        // Get project context
        let context = crate::context_provider::get_project_context()?;
        
        // Process the prompt using the AI service
        let response = crate::ai_service::process_prompt(&prompt_str, &context)?;
        
        // Return the response
        Ok(response.into_bytes())
    }, Some(Box::new(|result| {
        match result {
            TaskResult::Success(data) => {
                let response = String::from_utf8_lossy(&data).to_string();
                
                // Display the response in the output buffer
                if let Err(err) = append_to_output_buffer(&format!("\n## Assistant\n\n{}", response)) {
                    editor_message(&format!("Failed to display response: {}", err));
                }
            },
            TaskResult::Error(err) => {
                editor_message(&format!("Failed to process prompt: {}", err));
                
                // Display the error in the output buffer
                if let Err(e) = append_to_output_buffer(&format!("\n## Error\n\n{}", err)) {
                    editor_message(&format!("Failed to display error: {}", e));
                }
            },
            TaskResult::Cancelled => {
                editor_message("Prompt processing was cancelled");
                
                // Display cancellation in the output buffer
                if let Err(err) = append_to_output_buffer("\n## Cancelled\n\nThe operation was cancelled.") {
                    editor_message(&format!("Failed to display cancellation: {}", err));
                }
            }
        }
    })));
    
    editor_message(&format!("Processing prompt (Task ID: {})", task_id));
    
    Ok(())
}

// Mock functions for the xvim plugin API

/// Create a new buffer
fn create_buffer(name: &str, readonly: bool, modifiable: bool) -> Result<usize, String> {
    // This would be implemented by the xvim plugin API
    Ok(0)
}

/// Split the current window
fn split_window(direction: SplitDirection) -> Result<(), String> {
    // This would be implemented by the xvim plugin API
    Ok(())
}

/// Set the buffer for a window
fn set_window_buffer(window_id: usize, buffer_id: usize) -> Result<(), String> {
    // This would be implemented by the xvim plugin API
    Ok(())
}

/// Set the height of a window
fn set_window_height(window_id: usize, height: usize) -> Result<(), String> {
    // This would be implemented by the xvim plugin API
    Ok(())
}

/// Set the current window
fn set_current_window(window_id: usize) -> Result<(), String> {
    // This would be implemented by the xvim plugin API
    Ok(())
}

/// Set a keymap for a buffer
fn set_buffer_keymap(buffer_id: usize, mode: &str, keys: &str, command: &str) -> Result<(), String> {
    // This would be implemented by the xvim plugin API
    Ok(())
}

/// Execute a command
fn execute_command(command: &str) -> Result<(), String> {
    // This would be implemented by the xvim plugin API
    Ok(())
}

/// Split direction for window splitting
pub enum SplitDirection {
    Horizontal,
    Vertical,
}