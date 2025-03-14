//! UI Manager for the NoxVim plugin
//!
//! This module handles the UI components of the NoxVim plugin, including
//! the chat interface, output buffer, and input buffer.

use xvim_plugin_api::*;

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
    
    // In a real implementation, we would:
    // 1. Split the window horizontally
    // 2. Set the output buffer in the top window
    // 3. Set the input buffer in the bottom window
    // 4. Set the window heights
    // 5. Set up keybindings for the input buffer
    // 6. Focus the input buffer
    
    // For now, just log that we're creating the chat interface
    log_message("Creating chat interface (simplified implementation)");
    
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
        .map_err(|e| format!("Failed to get output buffer content: {}", e))?;
    
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
        .map_err(|e| format!("Failed to get input buffer content: {}", e))
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
    let task_id = create_task("Process Prompt", "Processing user prompt", Some(Box::new(move |result| {
        // This is a simplified implementation since we can't run async tasks
        // In a real implementation, we would:
        // 1. Get project context
        // 2. Process the prompt using the AI service
        // 3. Return the response
        
        // For now, just show a message
        editor_message("Prompt processing task started (simplified implementation)");
        
        // Display a mock response in the output buffer
        if let Err(err) = append_to_output_buffer("\n## Assistant\n\nThis is a mock response from the AI assistant.") {
            editor_message(&format!("Failed to display response: {}", err));
        }
    }))).map_err(|e| format!("Failed to create task: {}", e))?;
    
    editor_message(&format!("Processing prompt (Task ID: {:?})", task_id));
    
    Ok(())
}

// These functions are now provided by the xvim_plugin_api crate