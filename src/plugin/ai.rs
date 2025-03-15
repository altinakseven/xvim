//! AI conversation functionality for the xvim editor
//!
//! This module implements the AI conversation functionality for the xvim editor,
//! allowing users to chat with an AI assistant directly within the editor.

use anyhow::{anyhow, Result};
use crate::buffer::BufferManager;
use crate::plugin::PluginManager;
use crate::plugin::ui::UiElementType;

/// AI conversation state
pub struct AiConversation {
    /// Output buffer ID
    output_buffer_id: usize,
    /// Input buffer ID
    input_buffer_id: usize,
    /// Conversation history
    history: Vec<Message>,
    /// Auto-approve mode
    auto_approve: bool,
}

/// Message in the conversation
#[derive(Debug, Clone)]
pub struct Message {
    /// Message role (user or assistant)
    role: MessageRole,
    /// Message content
    content: String,
}

/// Message role
#[derive(Debug, Clone, PartialEq)]
pub enum MessageRole {
    /// User message
    User,
    /// Assistant message
    Assistant,
    /// System message
    System,
}

impl AiConversation {
    /// Create a new AI conversation
    pub fn new(output_buffer_id: usize, input_buffer_id: usize) -> Self {
        // Create a new conversation with a system message
        let mut history = Vec::new();
        history.push(Message {
            role: MessageRole::System,
            content: "You are an AI assistant integrated into the xvim editor. You help users with coding tasks, answer questions, and provide assistance with programming and software development.".to_string(),
        });
        
        Self {
            output_buffer_id,
            input_buffer_id,
            history,
            auto_approve: true,
        }
    }
    
    /// Add a user message to the conversation
    pub fn add_user_message(&mut self, content: &str) {
        self.history.push(Message {
            role: MessageRole::User,
            content: content.to_string(),
        });
    }
    
    /// Add an assistant message to the conversation
    pub fn add_assistant_message(&mut self, content: &str) {
        self.history.push(Message {
            role: MessageRole::Assistant,
            content: content.to_string(),
        });
    }
    
    /// Get the conversation history
    pub fn history(&self) -> &[Message] {
        &self.history
    }
    
    /// Get the output buffer ID
    pub fn output_buffer_id(&self) -> usize {
        self.output_buffer_id
    }
    
    /// Get the input buffer ID
    pub fn input_buffer_id(&self) -> usize {
        self.input_buffer_id
    }
    
    /// Set auto-approve mode
    pub fn set_auto_approve(&mut self, auto_approve: bool) {
        self.auto_approve = auto_approve;
    }
    
    /// Get auto-approve mode
    pub fn auto_approve(&self) -> bool {
        self.auto_approve
    }
    
    /// Process a user message
    pub fn process_message(&mut self, buffer_manager: &mut BufferManager, content: &str) -> Result<()> {
        // Add the user message to the conversation
        self.add_user_message(content);
        
        // Display the user message in the output buffer
        self.append_to_output_buffer(buffer_manager, &format!("\n## User\n\n{}\n", content))?;
        
        // Generate a response
        let response = self.generate_response()?;
        
        // Add the assistant message to the conversation
        self.add_assistant_message(&response);
        
        // Display the assistant message in the output buffer
        self.append_to_output_buffer(buffer_manager, &format!("\n## Assistant\n\n{}\n", response))?;
        
        Ok(())
    }
    
    /// Generate a response to the user's message
    fn generate_response(&self) -> Result<String> {
        // In a real implementation, this would call an AI service
        // For now, just return a mock response
        Ok("I'm a simple AI assistant integrated into xvim. I can help you with coding tasks, answer questions, and provide assistance with programming and software development. Currently, I'm running in a simplified mode without external API access, but I can still provide basic assistance and information.".to_string())
    }
    
    /// Append text to the output buffer
    fn append_to_output_buffer(&self, buffer_manager: &mut BufferManager, text: &str) -> Result<()> {
        // Get the output buffer
        let buffer = buffer_manager.get_buffer_mut(self.output_buffer_id)
            .map_err(|e| anyhow!("Failed to get output buffer: {}", e))?;
        
        // Get the current content length
        let content_len = buffer.content().len();
        
        // Append the new text at the end of the buffer
        buffer.insert(content_len, text)
            .map_err(|e| anyhow!("Failed to append to output buffer: {}", e))?;
        
        Ok(())
    }
    
    /// Clear the input buffer
    fn clear_input_buffer(&self, buffer_manager: &mut BufferManager) -> Result<()> {
        // Get the input buffer
        let buffer = buffer_manager.get_buffer_mut(self.input_buffer_id)
            .map_err(|e| anyhow!("Failed to get input buffer: {}", e))?;
        
        // Get the content length
        let content_len = buffer.content().len();
        
        // Delete all content if there is any
        if content_len > 0 {
            buffer.delete(0, content_len)
                .map_err(|e| anyhow!("Failed to clear input buffer: {}", e))?;
        }
        
        Ok(())
    }
    
    /// Execute the prompt in the input buffer
    pub fn execute_prompt(&mut self, buffer_manager: &mut BufferManager) -> Result<()> {
        // Get the input buffer
        let buffer = buffer_manager.get_buffer(self.input_buffer_id)
            .map_err(|e| anyhow!("Failed to get input buffer: {}", e))?;
        
        // Get the input buffer content
        let content = buffer.content().to_string();
        
        // Clear the input buffer
        self.clear_input_buffer(buffer_manager)?;
        
        // Process the message
        self.process_message(buffer_manager, &content)?;
        
        Ok(())
    }
}

/// Global AI conversation manager
pub struct AiConversationManager {
    /// Active conversations
    conversations: Vec<AiConversation>,
}

impl AiConversationManager {
    /// Create a new AI conversation manager
    pub fn new() -> Self {
        Self {
            conversations: Vec::new(),
        }
    }
    
    /// Create a new conversation
    pub fn create_conversation(&mut self, output_buffer_id: usize, input_buffer_id: usize) -> usize {
        // Create a new conversation
        let conversation = AiConversation::new(output_buffer_id, input_buffer_id);
        
        // Add the conversation to the list
        self.conversations.push(conversation);
        
        // Return the conversation ID (index)
        self.conversations.len() - 1
    }
    
    /// Get a conversation by ID
    pub fn get_conversation(&self, id: usize) -> Option<&AiConversation> {
        self.conversations.get(id)
    }
    
    /// Get a mutable conversation by ID
    pub fn get_conversation_mut(&mut self, id: usize) -> Option<&mut AiConversation> {
        self.conversations.get_mut(id)
    }
    
    /// Get a conversation by buffer ID
    pub fn get_conversation_by_buffer(&self, buffer_id: usize) -> Option<&AiConversation> {
        self.conversations.iter().find(|c| c.output_buffer_id() == buffer_id || c.input_buffer_id() == buffer_id)
    }
    
    /// Get a mutable conversation by buffer ID
    pub fn get_conversation_by_buffer_mut(&mut self, buffer_id: usize) -> Option<&mut AiConversation> {
        self.conversations.iter_mut().find(|c| c.output_buffer_id() == buffer_id || c.input_buffer_id() == buffer_id)
    }
    
    /// Remove a conversation by ID
    pub fn remove_conversation(&mut self, id: usize) -> Result<()> {
        if id >= self.conversations.len() {
            return Err(anyhow!("Invalid conversation ID: {}", id));
        }
        
        self.conversations.remove(id);
        
        Ok(())
    }
    
    /// Remove a conversation by buffer ID
    pub fn remove_conversation_by_buffer(&mut self, buffer_id: usize) -> Result<()> {
        let index = self.conversations.iter().position(|c| c.output_buffer_id() == buffer_id || c.input_buffer_id() == buffer_id)
            .ok_or_else(|| anyhow!("No conversation found for buffer ID: {}", buffer_id))?;
        
        self.conversations.remove(index);
        
        Ok(())
    }
    
    /// Execute the prompt in the input buffer
    pub fn execute_prompt(&mut self, buffer_manager: &mut BufferManager, buffer_id: usize) -> Result<()> {
        // Get the conversation
        let conversation = self.get_conversation_by_buffer_mut(buffer_id)
            .ok_or_else(|| anyhow!("No conversation found for buffer ID: {}", buffer_id))?;
        
        // Execute the prompt
        conversation.execute_prompt(buffer_manager)?;
        
        Ok(())
    }
}

/// Create a chat interface
///
/// This creates a split window with an input buffer at the bottom
/// and an output buffer at the top.
pub fn create_chat_interface(buffer_manager: &mut BufferManager, plugin_manager: &mut PluginManager) -> Result<(usize, usize)> {
    eprintln!("DEBUG: Starting create_chat_interface");
    
    // Create the output buffer
    eprintln!("DEBUG: Creating output buffer");
    let output_buffer_id = match buffer_manager.create_buffer() {
        Ok(id) => {
            eprintln!("DEBUG: Created output buffer with ID {}", id);
            id
        },
        Err(e) => {
            eprintln!("DEBUG: Failed to create output buffer: {}", e);
            return Err(anyhow!("Failed to create output buffer: {}", e));
        },
    };
    
    // Create the input buffer
    eprintln!("DEBUG: Creating input buffer");
    let input_buffer_id = match buffer_manager.create_buffer() {
        Ok(id) => {
            eprintln!("DEBUG: Created input buffer with ID {}", id);
            id
        },
        Err(e) => {
            eprintln!("DEBUG: Failed to create input buffer: {}", e);
            return Err(anyhow!("Failed to create input buffer: {}", e));
        },
    };
    
    // Set the output buffer content
    eprintln!("DEBUG: Getting output buffer for content insertion");
    let output_buffer = match buffer_manager.get_buffer_mut(output_buffer_id) {
        Ok(buffer) => {
            eprintln!("DEBUG: Got output buffer");
            buffer
        },
        Err(e) => {
            eprintln!("DEBUG: Failed to get output buffer: {}", e);
            return Err(anyhow!("Failed to get output buffer: {}", e));
        },
    };
    
    // Set the buffer name for the output buffer
    output_buffer.set_name("NoxVim-Output".to_string());
    
    // Insert the welcome message
    eprintln!("DEBUG: Inserting welcome message");
    let welcome_message = "# Welcome to NoxVim!\n\nI'm your AI assistant. How can I help you today?\n\n- Type your request below and press Ctrl-] to send it\n- Use specific commands like `:NoxGenerate` for targeted tasks\n- Toggle auto-approve mode with `:NoxToggleAutoApprove`";
    
    match output_buffer.insert(0, welcome_message) {
        Ok(_) => {
            eprintln!("DEBUG: Inserted welcome message");
        },
        Err(e) => {
            eprintln!("DEBUG: Failed to set output buffer content: {}", e);
            return Err(anyhow!("Failed to set output buffer content: {}", e));
        },
    };
    
    // Set the buffer name for the input buffer
    let input_buffer = match buffer_manager.get_buffer_mut(input_buffer_id) {
        Ok(buffer) => {
            eprintln!("DEBUG: Got input buffer");
            buffer
        },
        Err(e) => {
            eprintln!("DEBUG: Failed to get input buffer: {}", e);
            return Err(anyhow!("Failed to get input buffer: {}", e));
        },
    };
    
    input_buffer.set_name("NoxVim-Input".to_string());
    
    // Set the current buffer to the output buffer
    eprintln!("DEBUG: Setting current buffer to output buffer");
    if let Err(e) = buffer_manager.set_current_buffer(output_buffer_id) {
        eprintln!("DEBUG: Failed to set current buffer: {}", e);
        return Err(anyhow!("Failed to set current buffer: {}", e));
    }
    
    // Get the UI manager from the plugin manager
    eprintln!("DEBUG: Getting UI manager from plugin manager");
    let ui_manager = match plugin_manager.ui_manager() {
        Some(ui) => ui,
        None => {
            eprintln!("DEBUG: UI manager not available");
            return Err(anyhow!("UI manager not available"));
        }
    };
    
    // Get the terminal UI from the UI manager
    eprintln!("DEBUG: Getting terminal UI from UI manager");
    let terminal_arc = match ui_manager.terminal() {
        Some(terminal) => terminal,
        None => return Err(anyhow!("Terminal UI not available")),
    };
    
    // Lock the mutex to get a mutable reference to the TerminalUi
    eprintln!("DEBUG: Locking terminal UI mutex");
    let mut terminal = match terminal_arc.lock() {
        Ok(guard) => guard,
        Err(_) => return Err(anyhow!("Failed to lock terminal UI mutex")),
    };
    
    // Get the output buffer ID (current buffer before split)
    let current_buffer_id = match terminal.current_window() {
        Some(window) => window.buffer_id,
        None => return Err(anyhow!("No current window")),
    };
    
    // Make sure we're using the correct output buffer ID
    if current_buffer_id != output_buffer_id {
        eprintln!("DEBUG: Warning: Current buffer ID ({}) doesn't match output buffer ID ({})",
                 current_buffer_id, output_buffer_id);
    }
    
    // Split the window horizontally to create the input area at the bottom
    eprintln!("DEBUG: Splitting window horizontally");
    let split_result = terminal.split_window(crate::ui::window::SplitDirection::Horizontal, input_buffer_id)?;
    
    // Return based on the split result
    match split_result {
        Some(window_id) => {
            eprintln!("DEBUG: Split window successfully, new window ID: {}", window_id);
            
            // Navigate to the input window
            let next_result = terminal.next_window()?;
            
            if next_result {
                eprintln!("DEBUG: Navigated to input window");
            } else {
                eprintln!("DEBUG: Failed to navigate to input window");
            }
            
            // Return to the output window
            let prev_result = terminal.prev_window()?;
            
            if prev_result {
                eprintln!("DEBUG: Returned to output window");
            } else {
                eprintln!("DEBUG: Failed to return to output window");
            }
            
            eprintln!("DEBUG: Chat interface created successfully");
            
            // Return the buffer IDs for the output and input buffers
            Ok((output_buffer_id, input_buffer_id))
        },
        None => {
            eprintln!("DEBUG: Failed to split window, no window ID returned");
            Err(anyhow!("Failed to split window"))
        }
    }
}

// The get_ai_manager function has been removed since we're handling the conversation directly

/// Handle the Ctrl-] key press in the input buffer
pub fn handle_ctrl_right_bracket(buffer_manager: &mut BufferManager, plugin_manager: &mut PluginManager, buffer_id: usize) -> Result<()> {
    eprintln!("DEBUG: Handling Ctrl-] key press for buffer {}", buffer_id);
    
    // Check if we have an AI conversation manager
    if let Some(ai_manager) = plugin_manager.ai_conversation_manager_mut() {
        // Try to execute the prompt using the AI conversation manager
        eprintln!("DEBUG: Using AI conversation manager to execute prompt");
        match ai_manager.execute_prompt(buffer_manager, buffer_id) {
            Ok(_) => {
                eprintln!("DEBUG: Successfully executed prompt using AI conversation manager");
                return Ok(());
            },
            Err(e) => {
                eprintln!("DEBUG: Failed to execute prompt using AI conversation manager: {}", e);
                // Fall back to the simple implementation
            }
        }
    } else {
        eprintln!("DEBUG: No AI conversation manager available, using simple implementation");
    }
    
    // Simple implementation (fallback)
    // Get the buffer content
    let buffer_result = buffer_manager.get_buffer(buffer_id);
    if buffer_result.is_err() {
        return Err(anyhow!("Failed to get buffer: {}", buffer_result.err().unwrap()));
    }
    
    let buffer = buffer_result.unwrap();
    
    // Get the buffer content
    let content = buffer.content().to_string();
    eprintln!("DEBUG: Got input buffer content: {}", content);
    
    // Create a simple response
    let response = format!("You said: {}\n\nI'm a simple AI assistant integrated into xvim. Currently, I'm running in a simplified mode without external API access, but I can still provide basic assistance and information.", content);
    
    // Clear the input buffer
    let buffer_mut_result = buffer_manager.get_buffer_mut(buffer_id);
    if buffer_mut_result.is_err() {
        return Err(anyhow!("Failed to get buffer for clearing: {}", buffer_mut_result.err().unwrap()));
    }
    
    let buffer = buffer_mut_result.unwrap();
    
    let content_len = buffer.content().len();
    if content_len > 0 {
        let delete_result = buffer.delete(0, content_len);
        if delete_result.is_err() {
            return Err(anyhow!("Failed to clear input buffer: {}", delete_result.err().unwrap()));
        }
    }
    
    // Find the output buffer (assuming it's buffer_id - 1)
    let output_buffer_id = if buffer_id > 0 { buffer_id - 1 } else { buffer_id };
    
    // Append the response to the output buffer
    let output_buffer_result = buffer_manager.get_buffer_mut(output_buffer_id);
    if output_buffer_result.is_err() {
        return Err(anyhow!("Failed to get output buffer: {}", output_buffer_result.err().unwrap()));
    }
    
    let output_buffer = output_buffer_result.unwrap();
    
    let output_content_len = output_buffer.content().len();
    let insert_result = output_buffer.insert(output_content_len, &format!("\n\n## User\n\n{}\n\n## Assistant\n\n{}", content, response));
    if insert_result.is_err() {
        return Err(anyhow!("Failed to append to output buffer: {}", insert_result.err().unwrap()));
    }
    
    eprintln!("DEBUG: Successfully processed input and generated response");
    Ok(())
}