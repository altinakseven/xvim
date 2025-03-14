//! AI Service for the NoxVim plugin
//!
//! This module handles communication with the AI backend, including
//! sending requests, receiving responses, and processing the results.

use crate::context_provider::ProjectContext;
use crate::config::NoxVimConfig;

/// Process a prompt
///
/// This sends the prompt to the AI service and returns the response.
pub fn process_prompt(prompt: &str, context: &ProjectContext) -> Result<String, String> {
    // Get the configuration
    let config = unsafe { crate::CONFIG.as_ref().ok_or("Configuration not initialized")? };
    
    // Create the request
    let request = create_request(prompt, context, config)?;
    
    // Send the request to the AI service
    let response = send_request(&request, config)
        .map_err(|e| format!("Failed to send request: {}", e))?;
    
    // Process the response
    let result = process_response(&response)?;
    
    Ok(result)
}

/// Generate code based on a description
///
/// This sends a code generation request to the AI service and returns the generated code.
pub fn generate_code(description: &str, context: &ProjectContext) -> Result<String, String> {
    // Get the configuration
    let config = unsafe { crate::CONFIG.as_ref().ok_or("Configuration not initialized")? };
    
    // Create the prompt
    let prompt = format!(
        "Generate code based on the following description:\n\n{}\n\nPlease provide only the code without any explanations or markdown formatting.",
        description
    );
    
    // Create the request
    let request = create_request(&prompt, context, config)?;
    
    // Send the request to the AI service
    let response = send_request(&request, config)
        .map_err(|e| format!("Failed to send request: {}", e))?;
    
    // Process the response
    let result = process_response(&response)?;
    
    // Extract the code from the response
    let code = extract_code(&result)?;
    
    Ok(code)
}

/// Refactor code
///
/// This sends a refactoring request to the AI service and returns the refactored code.
pub fn refactor_code(code: &str, context: &ProjectContext) -> Result<String, String> {
    // Get the configuration
    let config = unsafe { crate::CONFIG.as_ref().ok_or("Configuration not initialized")? };
    
    // Create the prompt
    let prompt = format!(
        "Refactor the following code to improve its quality, readability, and performance:\n\n```\n{}\n```\n\nPlease provide only the refactored code without any explanations or markdown formatting.",
        code
    );
    
    // Create the request
    let request = create_request(&prompt, context, config)?;
    
    // Send the request to the AI service
    let response = send_request(&request, config)
        .map_err(|e| format!("Failed to send request: {}", e))?;
    
    // Process the response
    let result = process_response(&response)?;
    
    // Extract the code from the response
    let refactored_code = extract_code(&result)?;
    
    Ok(refactored_code)
}

/// Explain code
///
/// This sends a code explanation request to the AI service and returns the explanation.
pub fn explain_code(code: &str) -> Result<String, String> {
    // Get the configuration
    let config = unsafe { crate::CONFIG.as_ref().ok_or("Configuration not initialized")? };
    
    // Create the prompt
    let prompt = format!(
        "Explain the following code in detail:\n\n```\n{}\n```\n\nPlease provide a clear and comprehensive explanation of what the code does, how it works, and any important patterns or concepts it uses.",
        code
    );
    
    // Create the request
    let request = create_request(&prompt, &ProjectContext::default(), config)?;
    
    // Send the request to the AI service
    let response = send_request(&request, config)
        .map_err(|e| format!("Failed to send request: {}", e))?;
    
    // Process the response
    let result = process_response(&response)?;
    
    Ok(result)
}

/// Fix code
///
/// This sends a code fixing request to the AI service and returns the fixed code.
pub fn fix_code(code: &str, context: &ProjectContext) -> Result<String, String> {
    // Get the configuration
    let config = unsafe { crate::CONFIG.as_ref().ok_or("Configuration not initialized")? };
    
    // Create the prompt
    let prompt = format!(
        "Fix any issues in the following code:\n\n```\n{}\n```\n\nPlease provide only the fixed code without any explanations or markdown formatting.",
        code
    );
    
    // Create the request
    let request = create_request(&prompt, context, config)?;
    
    // Send the request to the AI service
    let response = send_request(&request, config)
        .map_err(|e| format!("Failed to send request: {}", e))?;
    
    // Process the response
    let result = process_response(&response)?;
    
    // Extract the code from the response
    let fixed_code = extract_code(&result)?;
    
    Ok(fixed_code)
}

/// Generate tests
///
/// This sends a test generation request to the AI service and returns the generated tests.
pub fn generate_tests(code: &str, context: &ProjectContext) -> Result<String, String> {
    // Get the configuration
    let config = unsafe { crate::CONFIG.as_ref().ok_or("Configuration not initialized")? };
    
    // Create the prompt
    let prompt = format!(
        "Generate comprehensive tests for the following code:\n\n```\n{}\n```\n\nPlease provide only the test code without any explanations or markdown formatting.",
        code
    );
    
    // Create the request
    let request = create_request(&prompt, context, config)?;
    
    // Send the request to the AI service
    let response = send_request(&request, config)
        .map_err(|e| format!("Failed to send request: {}", e))?;
    
    // Process the response
    let result = process_response(&response)?;
    
    // Extract the code from the response
    let tests = extract_code(&result)?;
    
    Ok(tests)
}

/// Create a request
///
/// This creates a request to send to the AI service.
fn create_request(prompt: &str, context: &ProjectContext, config: &NoxVimConfig) -> Result<String, String> {
    // Create the context string
    let context_str = format_context(context, config)?;
    
    // Create the full prompt with context
    let full_prompt = format!("{}\n\nContext:\n{}", prompt, context_str);
    
    // Create the request JSON
    let request = format!(
        r#"{{
            "model": "{}",
            "messages": [
                {{
                    "role": "system",
                    "content": "You are NoxVim, an AI assistant integrated into the xvim editor. You help users with coding tasks, answer questions, and provide assistance with programming and software development."
                }},
                {{
                    "role": "user",
                    "content": "{}"
                }}
            ],
            "temperature": 0.7,
            "max_tokens": 2000
        }}"#,
        config.model,
        escape_json(&full_prompt)
    );
    
    Ok(request)
}

/// Format the context
///
/// This formats the project context into a string for inclusion in the prompt.
fn format_context(context: &ProjectContext, config: &NoxVimConfig) -> Result<String, String> {
    // Format the current buffer
    let current_buffer = if let Some(buffer_id) = context.current_buffer_id {
        if let Some(buffer) = context.buffers.iter().find(|b| b.id == buffer_id) {
            format!(
                "Current buffer: {} ({})\nContent:\n```\n{}\n```",
                buffer.name,
                buffer.file_path.as_deref().unwrap_or("No file path"),
                buffer.content
            )
        } else {
            "No current buffer".to_string()
        }
    } else {
        "No current buffer".to_string()
    };
    
    // Format the cursor position
    let cursor_pos = if let Some((line, column)) = context.cursor_position {
        format!("Cursor position: line {}, column {}", line, column)
    } else {
        "Cursor position: unknown".to_string()
    };
    
    // Format the current mode
    let mode = if let Some(mode) = &context.current_mode {
        format!("Current mode: {}", mode)
    } else {
        "Current mode: unknown".to_string()
    };
    
    // Format the selection
    let selection = if let Some((start, end)) = context.selection {
        format!("Selection: characters {} to {}", start, end)
    } else {
        "No selection".to_string()
    };
    
    // Format the project structure
    let project_structure = format!(
        "Project root: {}\nFiles: {}",
        context.structure.root_dir,
        context.structure.files.len()
    );
    
    // Combine all context information
    let context_str = format!(
        "{}\n\n{}\n\n{}\n\n{}\n\n{}",
        current_buffer,
        cursor_pos,
        mode,
        selection,
        project_structure
    );
    
    // Truncate the context if it's too large
    if context_str.len() > config.max_context {
        Ok(format!(
            "{}... (truncated, {} characters total)",
            &context_str[..config.max_context],
            context_str.len()
        ))
    } else {
        Ok(context_str)
    }
}

/// Send a request to the AI service
///
/// This sends a request to the AI service and returns the response.
fn send_request(request: &str, config: &NoxVimConfig) -> Result<String, String> {
    // This would send the request to the AI service
    // For now, just return a mock response
    
    // Log the request
    crate::xvim_plugin_api::log_message(&format!("Sending request to AI service: {}", request));
    
    // Check if the API key is set
    if config.api_key.is_empty() {
        return Err("API key not set. Please set g:noxvim_api_key in your xvim configuration.".to_string());
    }
    
    // In a real implementation, this would send an HTTP request to the AI service
    // For now, just return a mock response
    Ok(r#"{
        "choices": [
            {
                "message": {
                    "content": "This is a mock response from the AI service. In a real implementation, this would be the actual response from the AI service."
                }
            }
        ]
    }"#.to_string())
}

/// Process a response from the AI service
///
/// This extracts the content from the response JSON.
fn process_response(response: &str) -> Result<String, String> {
    // This would parse the response JSON and extract the content
    // For now, just return a mock response
    
    // In a real implementation, this would parse the JSON and extract the content
    // For example:
    // let json: serde_json::Value = serde_json::from_str(response)
    //     .map_err(|e| format!("Failed to parse response JSON: {}", e))?;
    // let content = json["choices"][0]["message"]["content"]
    //     .as_str()
    //     .ok_or("Failed to extract content from response")?;
    // Ok(content.to_string())
    
    Ok("This is a mock response from the AI service. In a real implementation, this would be the actual response from the AI service.".to_string())
}

/// Extract code from a response
///
/// This extracts code blocks from a markdown-formatted response.
fn extract_code(response: &str) -> Result<String, String> {
    // This would extract code blocks from a markdown-formatted response
    // For now, just return the response as-is
    
    // In a real implementation, this would look for code blocks and extract them
    // For example:
    // let re = regex::Regex::new(r"```(?:\w+)?\n([\s\S]*?)\n```")
    //     .map_err(|e| format!("Failed to create regex: {}", e))?;
    // let mut code = String::new();
    // for cap in re.captures_iter(response) {
    //     if let Some(m) = cap.get(1) {
    //         code.push_str(m.as_str());
    //         code.push_str("\n\n");
    //     }
    // }
    // if code.is_empty() {
    //     Ok(response.to_string())
    // } else {
    //     Ok(code)
    // }
    
    Ok(response.to_string())
}

/// Escape a string for JSON
///
/// This escapes special characters in a string for inclusion in JSON.
fn escape_json(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}