# xvim Plugin System

The xvim editor features a powerful plugin system based on WebAssembly (WASM) that allows extending the editor with plugins written in any language that can compile to WebAssembly.

## Architecture

The plugin system consists of several components:

1. **Plugin Manager**: Manages the lifecycle of plugins, including loading, unloading, and initialization.
2. **WASM Runtime**: Executes WebAssembly modules in a sandboxed environment.
3. **Plugin API**: Provides a set of functions that plugins can use to interact with the editor.
4. **Event System**: Allows plugins to receive notifications about editor events.

```
+----------------+     +----------------+     +----------------+
|                |     |                |     |                |
|  xvim Editor   |<--->|  Plugin API    |<--->|  Plugin (WASM) |
|                |     |                |     |                |
+----------------+     +----------------+     +----------------+
        ^                      ^                      ^
        |                      |                      |
        v                      v                      v
+----------------+     +----------------+     +----------------+
|                |     |                |     |                |
|  Event System  |<--->|  WASM Runtime  |<--->|  WASI Sandbox  |
|                |     |                |     |                |
+----------------+     +----------------+     +----------------+
```

## Plugin Lifecycle

1. **Loading**: The plugin is loaded from a WASM file into the WASM runtime.
2. **Initialization**: The plugin's `init` function is called to initialize the plugin.
3. **Execution**: The plugin registers commands and event handlers, and responds to events.
4. **Unloading**: The plugin is unloaded when the editor exits or when explicitly unloaded.

## Plugin API

The plugin API provides the following functionality:

### Registration

- `register_plugin(name, version, description, author)`: Register the plugin with the editor.
- `register_command(name, handler)`: Register a command with the editor.
- `register_event_handler(event_type, handler)`: Register an event handler.

### Editor Interaction

- `editor_message(message)`: Display a message in the editor.
- `log_message(message)`: Log a message to the plugin log.
- `get_current_buffer()`: Get the current buffer.
- `get_buffer_content(buffer_id)`: Get the content of a buffer.
- `set_buffer_content(buffer_id, content)`: Set the content of a buffer.
- `get_cursor_position()`: Get the current cursor position.
- `set_cursor_position(line, column)`: Set the cursor position.
- `get_current_mode()`: Get the current editor mode.

### Event Types

- `BufferCreated(buffer_id)`: A buffer was created.
- `BufferDeleted(buffer_id)`: A buffer was deleted.
- `BufferChanged(buffer_id)`: A buffer was changed.
- `ModeChanged(mode)`: The editor mode was changed.
- `CursorMoved(buffer_id, line, column)`: The cursor was moved.
- `CommandExecuted(command)`: A command was executed.
- `Custom(name, data)`: A custom event.

## Developing Plugins

Plugins for xvim can be written in any language that can compile to WebAssembly with WASI support. The most common languages are Rust, C/C++, and AssemblyScript.

### Plugin Structure

A plugin must export an `init` function that is called when the plugin is loaded:

```rust
#[no_mangle]
pub extern "C" fn init() -> i32 {
    // Initialize the plugin
    // Return 0 for success, non-zero for failure
    0
}
```

### Command Handlers

Command handlers are functions that are called when a command is executed:

```rust
fn my_command(args: &[&str]) -> Result<(), String> {
    // Handle the command
    Ok(())
}
```

### Event Handlers

Event handlers are functions that are called when an event occurs:

```rust
fn my_event_handler(event: &Event) -> Result<(), String> {
    // Handle the event
    Ok(())
}
```

## Security

The plugin system uses WASI (WebAssembly System Interface) to provide a sandboxed environment for plugins. This ensures that plugins cannot access the host system directly, but only through the provided API.

## Example Plugin

See the `wasm/hello_plugin` directory for a simple example plugin that demonstrates the basic functionality of the plugin API.

## Building Plugins

To build a plugin, you need to have Rust and the WebAssembly target installed:

```bash
# Install the WebAssembly target
rustup target add wasm32-wasi

# Build the plugin
cargo build --target wasm32-wasi --release
```

## Installing Plugins

To install a plugin, copy the WebAssembly module to the xvim plugins directory:

```bash
mkdir -p ~/.config/xvim/plugins
cp target/wasm32-wasi/release/my_plugin.wasm ~/.config/xvim/plugins/
```

## Plugin Configuration

Plugins can be configured in the xvim configuration file:

```toml
[plugins]
# Enable or disable plugins
enabled = true

# Plugin-specific configuration
[plugins.my_plugin]
option1 = "value1"
option2 = "value2"
```

## Advanced Features

### UI Manipulation API

The plugin system includes an advanced UI manipulation API that allows plugins to create custom UI elements:

- **Windows**: Create new editor windows with custom content
- **Dialogs**: Display modal dialogs for user interaction
- **Status Bar Items**: Add items to the status bar
- **Popup Menus**: Display context menus and dropdown menus
- **Floating Windows**: Create floating windows that can be positioned anywhere
- **Custom Elements**: Create custom UI elements with specific rendering

Example of creating a UI element:

```rust
// Create a dialog
let dialog_id = "my_dialog";
editor_create_ui_element(dialog_id, UiElementType::Dialog, "My Dialog");

// Set the dialog content
editor_set_ui_element_content(dialog_id, "This is a dialog message");

// Set the dialog position
editor_set_ui_element_position(dialog_id, 10, 20);

// Set the dialog size
editor_set_ui_element_size(dialog_id, 40, 10);

// Show the dialog
editor_show_ui_element(dialog_id);
```

### Network Access Security Model

The plugin system includes a comprehensive network access security model that allows plugins to make network requests in a controlled and secure way:

- **Permission Levels**: None, Localhost, Restricted, All
- **Domain Restrictions**: Limit which domains a plugin can access
- **Method Restrictions**: Limit which HTTP methods a plugin can use
- **Size Constraints**: Limit the size of requests and responses
- **Rate Limiting**: Limit the number of requests per minute

Example of making a network request:

```rust
// Create a network request
let request = NetworkRequest::new("https://api.example.com/data", HttpMethod::Get);

// Set a request header
request.set_header("Content-Type", "application/json");

// Send the request
let response = editor_send_network_request(request)?;

// Process the response
let status = response.status();
let body = response.body();
```

### Asynchronous Operation Support

The plugin system includes comprehensive support for asynchronous operations, allowing plugins to perform long-running tasks without blocking the editor:

- **Task-based System**: Create and manage background tasks
- **Progress Tracking**: Monitor and report task progress
- **Lifecycle Management**: Create, run, cancel, and complete tasks
- **Callback Mechanism**: Receive notifications when tasks complete
- **Status Monitoring**: Check task status and results

Example of running an asynchronous task:

```rust
// Create and run a task
let task_id = editor_create_task("My Task", "This is a long-running task", || {
    // This code runs in a background thread
    
    // Simulate a long-running operation
    for i in 0..100 {
        // Update progress
        editor_update_task_progress(i);
        
        // Sleep for a bit
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
    
    // Return the result
    Ok("Task completed successfully".as_bytes().to_vec())
}, Some(Box::new(|result| {
    // This callback is called when the task completes
    match result {
        TaskResult::Success(data) => {
            // Handle successful completion
            let message = String::from_utf8_lossy(&data);
            editor_message(&format!("Task completed: {}", message));
        }
        TaskResult::Error(err) => {
            // Handle error
            editor_message(&format!("Task failed: {}", err));
        }
        TaskResult::Cancelled => {
            // Handle cancellation
            editor_message("Task was cancelled");
        }
    }
})));

// Check task status later
let task_info = editor_get_task_info(task_id);
if let Some(info) = task_info {
    editor_message(&format!("Task status: {:?}, progress: {}%", info.status(), info.progress()));
}
```

### Plugin Dependency Management

The plugin system includes comprehensive dependency management, allowing plugins to depend on other plugins:

- **Dependency Declaration**: Plugins can declare dependencies in a JSON file
- **Version Requirements**: Specify required versions of dependencies
- **Optional Dependencies**: Mark dependencies as optional
- **Load Order**: Plugins are loaded in the correct order based on dependencies
- **Cycle Detection**: Detect and prevent circular dependencies

Example of a plugin dependency file (`my_plugin.dep.json`):

```json
{
  "name": "my_plugin",
  "version": "1.0.0",
  "dependencies": [
    {
      "name": "base_plugin",
      "version_req": "1.0.0",
      "optional": false
    },
    {
      "name": "optional_plugin",
      "version_req": "0.5.0",
      "optional": true
    }
  ]
}
```

The dependency manager ensures that all required dependencies are loaded before a plugin, and that version requirements are satisfied.

### Enhanced Debugging Support

The plugin system includes comprehensive debugging support to help developers diagnose and fix issues in their plugins:

- **Logging System**: Multi-level logging (Trace, Debug, Info, Warn, Error, Fatal)
- **Performance Tracing**: Measure and analyze the performance of plugin operations
- **State Snapshots**: Capture the state of plugins at specific points in time
- **Function Call Tracing**: Track function calls and their results
- **Event Tracing**: Monitor event dispatching and handling
- **Log Rotation**: Automatically rotate log files to manage disk space
- **Debug Configuration**: Configure debugging options through a JSON file
- **Export Tools**: Export logs, traces, and snapshots for offline analysis

Example of using the debugging API:

```rust
// Configure debugging
let mut config = DebugConfig::new();
config.enable();
config.set_log_level(LogLevel::Debug);
config.set_log_file(Some("plugin.log"));
config.enable_tracing();
config.enable_snapshots();

// Log a message
editor_log(LogLevel::Info, "Plugin initialized", None);

// Start a performance trace
let trace_id = editor_start_trace("load_data", Some("Loading user data"));

// ... perform the operation ...

// End the trace
editor_end_trace(trace_id);

// Take a state snapshot
let variables = HashMap::new();
variables.insert("user_count".to_string(), "42".to_string());
variables.insert("cache_size".to_string(), "1024".to_string());
editor_take_snapshot(1024 * 1024, 2, variables);

// Export logs
editor_export_logs("logs.json");
```

## Future Enhancements

The plugin system will be enhanced in the future with the following features:

- More comprehensive plugin API