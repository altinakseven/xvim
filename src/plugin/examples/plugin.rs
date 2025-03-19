// Example plugin that demonstrates how to use the plugin system

use crate::plugin::{Plugin, PluginContext, PluginResult};
use crate::mode::normal_commands::NormalCommandHandler;

/// A simple plugin that demonstrates how to use the plugin system
pub struct ExamplePlugin {
    /// Plugin name
    name: String,
    /// Plugin version
    version: String,
    /// Plugin description
    description: String,
}

impl ExamplePlugin {
    /// Create a new ExamplePlugin
    pub fn new() -> Self {
        Self {
            name: "example_plugin".to_string(),
            version: "0.1.0".to_string(),
            description: "A simple example plugin".to_string(),
        }
    }
}

impl Plugin for ExamplePlugin {
    /// Initialize the plugin
    fn init(&mut self, context: &mut PluginContext) -> PluginResult<()> {
        // Get the mode manager from the context
        if let Some(mode_manager) = context.mode_manager() {
            // Lock the mode manager
            let mut mode_manager = mode_manager.lock().map_err(|_| "Failed to lock mode manager")?;
            
            // Get the normal command handler from the mode manager
            let normal_handler = mode_manager.get_normal_handler_mut();
            
            // Register a custom command to uppercase the current word
            normal_handler.register_command('U', |editor| {
                // Get the current buffer
                if let Some(buffer_id) = editor.current_buffer_id() {
                    let buffer = editor.get_buffer_manager().get_buffer(buffer_id)?;
                    
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

// Create a new plugin instance
pub fn create_plugin() -> Box<dyn Plugin> {
    Box::new(ExamplePlugin::new())
}