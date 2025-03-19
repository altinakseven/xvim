// Example plugin that extends the Normal mode command handler with custom commands

// // use crate::editor::Editor;
use crate::mode::normal_commands::NormalCommandHandler;
use crate::plugin::{Plugin, PluginContext, PluginResult};

/// A plugin that adds custom Normal mode commands
pub struct NormalModePlugin {
    /// Plugin name
    name: String,
    /// Plugin version
    version: String,
    /// Plugin description
    description: String,
}

impl NormalModePlugin {
    /// Create a new NormalModePlugin
    pub fn new() -> Self {
        Self {
            name: "normal_mode_plugin".to_string(),
            version: "0.1.0".to_string(),
            description: "Adds custom Normal mode commands".to_string(),
        }
    }
    
    /// Register custom commands with the Normal mode command handler
    fn register_commands(&self, handler: &mut NormalCommandHandler) -> PluginResult<()> {
        // Register a command to uppercase the current word
        handler.register_command('U', |editor| {
            // Get the current buffer
            if let Some(buffer_id) = editor.current_buffer_id() {
                let buffer = editor.get_buffer_manager().get_buffer(buffer_id)?;
                
                // Get the cursor position
                let cursor_pos = editor.cursor_position();
                
                // Find the word at the cursor position
                if let Some(text_object) = editor.find_text_object(crate::editor::TextObjectType::Word, false)? {
                    // Get the word text
                    let content = buffer.content();
                    if text_object.start < content.len() && text_object.end <= content.len() {
                        let word = content[text_object.start..text_object.end].to_string();
                        
                        // Convert the word to uppercase
                        let uppercase_word = word.to_uppercase();
                        
                        // Replace the word with the uppercase version
                        let buffer = editor.get_buffer_manager_mut().get_buffer_mut(buffer_id)?;
                        buffer.delete(text_object.start, text_object.end)?;
                        buffer.insert(text_object.start, &uppercase_word)?;
                    }
                }
            }
            
            Ok(())
        });
        
        // Register a command to lowercase the current word
        handler.register_command('L', |editor| {
            // Get the current buffer
            if let Some(buffer_id) = editor.current_buffer_id() {
                let buffer = editor.get_buffer_manager().get_buffer(buffer_id)?;
                
                // Get the cursor position
                let cursor_pos = editor.cursor_position();
                
                // Find the word at the cursor position
                if let Some(text_object) = editor.find_text_object(crate::editor::TextObjectType::Word, false)? {
                    // Get the word text
                    let content = buffer.content();
                    if text_object.start < content.len() && text_object.end <= content.len() {
                        let word = content[text_object.start..text_object.end].to_string();
                        
                        // Convert the word to lowercase
                        let lowercase_word = word.to_lowercase();
                        
                        // Replace the word with the lowercase version
                        let buffer = editor.get_buffer_manager_mut().get_buffer_mut(buffer_id)?;
                        buffer.delete(text_object.start, text_object.end)?;
                        buffer.insert(text_object.start, &lowercase_word)?;
                    }
                }
            }
            
            Ok(())
        });
        
        // Register a command to reverse the current word
        handler.register_command('R', |editor| {
            // Get the current buffer
            if let Some(buffer_id) = editor.current_buffer_id() {
                let buffer = editor.get_buffer_manager().get_buffer(buffer_id)?;
                
                // Get the cursor position
                let cursor_pos = editor.cursor_position();
                
                // Find the word at the cursor position
                if let Some(text_object) = editor.find_text_object(crate::editor::TextObjectType::Word, false)? {
                    // Get the word text
                    let content = buffer.content();
                    if text_object.start < content.len() && text_object.end <= content.len() {
                        let word = content[text_object.start..text_object.end].to_string();
                        
                        // Reverse the word
                        let reversed_word: String = word.chars().rev().collect();
                        
                        // Replace the word with the reversed version
                        let buffer = editor.get_buffer_manager_mut().get_buffer_mut(buffer_id)?;
                        buffer.delete(text_object.start, text_object.end)?;
                        buffer.insert(text_object.start, &reversed_word)?;
                    }
                }
            }
            
            Ok(())
        });
        
        // Register a command to duplicate the current line
        handler.register_command('D', |editor| {
            // Get the current buffer
            if let Some(buffer_id) = editor.current_buffer_id() {
                let buffer = editor.get_buffer_manager().get_buffer(buffer_id)?;
                
                // Get the cursor position
                let cursor_pos = editor.cursor_position();
                
                // Get the current line
                let line = buffer.line(cursor_pos.line)?;
                
                // Get the end of the line
                let line_end = buffer.position_to_char_idx(cursor_pos.line, line.len())?;
                
                // Insert a newline and the line content
                let buffer = editor.get_buffer_manager_mut().get_buffer_mut(buffer_id)?;
                buffer.insert(line_end, &format!("\n{}", line))?;
            }
            
            Ok(())
        });
        
        Ok(())
    }
}

impl Plugin for NormalModePlugin {
    /// Initialize the plugin
    fn init(&mut self, context: &mut PluginContext) -> PluginResult<()> {
        // Get the mode manager from the context
        if let Some(mode_manager) = context.mode_manager() {
            // Lock the mode manager
            let mut mode_manager = mode_manager.lock().map_err(|_| "Failed to lock mode manager")?;
            
            // Get the normal command handler from the mode manager
            let normal_handler = mode_manager.get_normal_handler_mut();
            
            // Register custom commands
            self.register_commands(normal_handler)?;
        }
        
        Ok(())
    }
    
    /// Get the plugin name
    fn name(&self) -> &str {
        &self.name
    }
    
    /// Get the plugin version
    fn version(&self) -> &str {
        &self.version
    }
    
    /// Get the plugin description
    fn description(&self) -> &str {
        &self.description
    }
    
    /// Clean up the plugin
    fn cleanup(&mut self) -> PluginResult<()> {
        // No cleanup needed
        Ok(())
    }
}