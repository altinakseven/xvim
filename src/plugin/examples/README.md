# xvim Plugin Examples

This directory contains example plugins for xvim that demonstrate various aspects of the plugin system.

## Normal Mode Plugin

The `normal_mode_plugin.rs` file demonstrates how to create a plugin that extends the Normal mode command handler with custom commands.

### Features

The plugin adds the following commands to Normal mode:

- `U` - Uppercase the current word
- `L` - Lowercase the current word
- `R` - Reverse the current word
- `D` - Duplicate the current line

### Usage

To use this plugin, you need to:

1. Build the plugin:
   ```bash
   cargo build --release --package normal_mode_plugin
   ```

2. Copy the plugin to the plugins directory:
   ```bash
   cp target/release/libnormal_mode_plugin.so plugins/
   ```

3. Load the plugin in xvim:
   ```
   :plugin load normal_mode_plugin
   ```

4. Use the commands in Normal mode:
   - Place the cursor on a word and press `U` to uppercase it
   - Place the cursor on a word and press `L` to lowercase it
   - Place the cursor on a word and press `R` to reverse it
   - Press `D` to duplicate the current line

### Implementation Details

The plugin demonstrates several important concepts:

1. **Accessing the Normal Command Handler**:
   ```rust
   // Get the mode manager from the context
   if let Some(mode_manager) = context.mode_manager() {
       // Lock the mode manager
       let mut mode_manager = mode_manager.lock().map_err(|_| "Failed to lock mode manager")?;
       
       // Get the normal command handler from the mode manager
       let normal_handler = mode_manager.get_normal_handler_mut();
       
       // Register custom commands
       self.register_commands(normal_handler)?;
   }
   ```

2. **Registering Custom Commands**:
   ```rust
   // Register a command to uppercase the current word
   handler.register_command('U', |editor| {
       // Implementation
   });
   ```

3. **Working with Text Objects**:
   ```rust
   // Find the word at the cursor position
   if let Some(text_object) = editor.find_text_object(crate::editor::TextObjectType::Word, false)? {
       // Get the word text
       let content = buffer.content();
       if text_object.start < content.len() && text_object.end <= content.len() {
           let word = content[text_object.start..text_object.end].to_string();
           
           // Process the word
           // ...
       }
   }
   ```

4. **Modifying Buffer Content**:
   ```rust
   // Replace the word with the processed version
   let buffer = editor.get_buffer_manager_mut().get_buffer_mut(buffer_id)?;
   buffer.delete(text_object.start, text_object.end)?;
   buffer.insert(text_object.start, &processed_word)?;
   ```

This example demonstrates how plugins can extend the editor's functionality by adding custom commands to the Normal mode command handler. The same approach can be used to add commands to other modes or to implement more complex functionality.

## Creating Your Own Plugins

To create your own plugin that extends the Normal mode command handler:

1. Create a new Rust crate with the following dependencies:
   ```toml
   [dependencies]
   xvim = { path = "../path/to/xvim" }
   ```

2. Implement the `Plugin` trait:
   ```rust
   use xvim::plugin::{Plugin, PluginContext, PluginResult};
   
   pub struct MyPlugin {
       name: String,
       version: String,
       description: String,
   }
   
   impl Plugin for MyPlugin {
       fn init(&mut self, context: &mut PluginContext) -> PluginResult<()> {
           // Implementation
       }
       
       fn name(&self) -> &str {
           &self.name
       }
       
       fn version(&self) -> &str {
           &self.version
       }
       
       fn description(&self) -> &str {
           &self.description
       }
       
       fn cleanup(&mut self) -> PluginResult<()> {
           Ok(())
       }
   }
   ```

3. Register your custom commands in the `init` method:
   ```rust
   fn init(&mut self, context: &mut PluginContext) -> PluginResult<()> {
       // Get the mode manager from the context
       if let Some(mode_manager) = context.mode_manager() {
           // Lock the mode manager
           let mut mode_manager = mode_manager.lock().map_err(|_| "Failed to lock mode manager")?;
           
           // Get the normal command handler from the mode manager
           let normal_handler = mode_manager.get_normal_handler_mut();
           
           // Register custom commands
           normal_handler.register_command('X', |editor| {
               // Implementation
               Ok(())
           });
       }
       
       Ok(())
   }
   ```

4. Build and load your plugin as described above.

This approach allows you to extend the editor's functionality in a modular and maintainable way, without modifying the core codebase.