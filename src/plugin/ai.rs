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
/// This creates a simple read-only information buffer instead of a chat interface
/// to avoid cursor positioning issues.
pub fn create_chat_interface(buffer_manager: &mut BufferManager, _plugin_manager: &mut PluginManager) -> Result<(usize, usize)> {
    eprintln!("DEBUG: Starting create_chat_interface");
    
    // Create a buffer
    eprintln!("DEBUG: Creating buffer");
    let buffer_id = buffer_manager.create_buffer()?;
    eprintln!("DEBUG: Created buffer with ID {}", buffer_id);
    
    // Set the buffer name
    let buffer = buffer_manager.get_buffer_mut(buffer_id)?;
    buffer.set_name("NoxVim-Info".to_string());
    
    // Insert an information message instead of a chat interface
    let info_message = "# NoxVim Information\n\n";
    let info_message = format!("{}## About NoxVim\n\n", info_message);
    let info_message = format!("{}NoxVim is a Vim-like text editor written in Rust with a WebAssembly (WASM) plugin system.\n\n", info_message);
    let info_message = format!("{}## Current Status\n\n", info_message);
    let info_message = format!("{}The AI chat functionality is currently under development and not fully functional.\n", info_message);
    let info_message = format!("{}There are issues with cursor positioning that need to be resolved.\n\n", info_message);
    let info_message = format!("{}## Available Commands\n\n", info_message);
    let info_message = format!("{}* `:NoxGenerate` - Generate code based on a prompt\n", info_message);
    let info_message = format!("{}* `:NoxRefactor` - Refactor code based on a prompt\n", info_message);
    let info_message = format!("{}* `:NoxExplain` - Explain code based on a prompt\n", info_message);
    let info_message = format!("{}* `:NoxFix` - Fix code based on a prompt\n", info_message);
    let info_message = format!("{}* `:NoxTest` - Generate tests based on a prompt\n", info_message);
    let info_message = format!("{}* `:NoxToggleAutoApprove` - Toggle auto-approve mode\n\n", info_message);
    
    buffer.insert(0, &info_message)?;
    
    // Set the buffer as read-only to prevent editing AFTER inserting content
    buffer.set_read_only(true);
    
    eprintln!("DEBUG: Info buffer created successfully");
    
    // Return the buffer ID twice (for compatibility with the old API)
    Ok((buffer_id, buffer_id))
}

// The get_ai_manager function has been removed since we're handling the conversation directly

/// Handle the Ctrl-] key press in the input buffer
///
/// This function is simplified to avoid cursor positioning issues.
/// Instead of trying to insert content into the buffer, it just returns
/// a message that the functionality is under development.
pub fn handle_ctrl_right_bracket(buffer_manager: &mut BufferManager, _plugin_manager: &mut PluginManager, buffer_id: usize) -> Result<()> {
    eprintln!("DEBUG: Handling Ctrl-] key press for buffer {}", buffer_id);
    
    // Get the buffer (read-only is fine since we're not modifying it)
    let buffer_result = buffer_manager.get_buffer(buffer_id);
    if buffer_result.is_err() {
        return Err(anyhow!("Failed to get buffer: {}", buffer_result.err().unwrap()));
    }
    
    let buffer = buffer_result.unwrap();
    // Check if the buffer has a name
    let buffer_name = buffer.name();
    // If the buffer is not the NoxVim-Input buffer, do nothing
    if buffer_name != "NoxVim-Input" {
        // Not the input buffer, do nothing
        eprintln!("DEBUG: Not the NoxVim-Input buffer, ignoring Ctrl-]");
        return Ok(());
    }
    
    // Instead of trying to modify the buffer, just log a message
    eprintln!("DEBUG: AI functionality is under development");
    
    // Return success without modifying any buffers
    Ok(())
}