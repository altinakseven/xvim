# xvim System Patterns

This document outlines the key design patterns and architectural decisions used in the xvim project.

## Core Design Patterns

### Command Pattern

The Ex command system uses the Command pattern to encapsulate commands as objects:

```rust
// Command registration
registry.register("write", make_handler(handle_write));
registry.register("w", make_handler(handle_write));

// Command execution
fn handle_write(cmd: &ExCommand) -> ExCommandResult<()> {
    // Implementation
}
```

Benefits:
- Decouples command invocation from implementation
- Allows for command aliasing (e.g., `:w` and `:write`)
- Centralizes error handling

### Registry Pattern

Commands, modes, and plugins are managed through registries:

```rust
pub struct ExCommandRegistry {
    commands: HashMap<String, Box<dyn Fn(&ExCommand) -> ExCommandResult<()> + Send + Sync>>,
}

impl ExCommandRegistry {
    pub fn register(&mut self, name: &str, handler: impl Fn(&ExCommand) -> ExCommandResult<()> + Send + Sync + 'static) {
        self.commands.insert(name.to_string(), Box::new(handler));
    }
    
    pub fn execute(&self, cmd: &ExCommand) -> ExCommandResult<()> {
        // Implementation
    }
}
```

Benefits:
- Centralized management of components
- Dynamic registration and lookup
- Extensibility through plugins

### Observer Pattern

The editor uses the Observer pattern for event handling:

```rust
// Event subscription
editor.subscribe(EventType::BufferChanged, handler);

// Event notification
editor.notify(Event::BufferChanged { buffer_id });
```

Benefits:
- Decouples components
- Allows for plugin integration
- Supports reactive UI updates

### Strategy Pattern

Different editor modes implement the same interface but with different behaviors:

```rust
trait Mode {
    fn handle_input(&mut self, input: Input, editor: &mut Editor) -> ModeResult;
    fn render(&self, editor: &Editor, frame: &mut Frame);
}

struct NormalMode { /* ... */ }
struct InsertMode { /* ... */ }
struct VisualMode { /* ... */ }

impl Mode for NormalMode {
    // Implementation
}
```

Benefits:
- Encapsulates mode-specific behavior
- Allows for easy mode switching
- Maintains consistent interface

### Trait-based Extension Pattern

The editor uses traits to extend functionality across components:

```rust
// Visual mode functionality for the Editor
trait VisualFunctions {
    fn start_visual_mode(&mut self, mode: VisualMode) -> EditorResult<()>;
    fn end_visual_mode(&mut self) -> EditorResult<()>;
    fn toggle_visual_mode(&mut self, mode: VisualMode) -> EditorResult<()>;
    fn reselect_visual_area(&mut self) -> EditorResult<()>;
    fn swap_visual_corners(&mut self, upper: bool) -> EditorResult<()>;
    fn visual_state(&self) -> &VisualState;
    fn visual_state_mut(&mut self) -> &mut VisualState;
}

impl VisualFunctions for Editor {
    // Implementation
}
```

Benefits:
- Organizes related functionality into cohesive units
- Allows for modular implementation of features
- Improves code organization and maintainability
- Enables clear separation of concerns

### Composite Pattern

The UI system uses the Composite pattern for widget hierarchy:

```rust
trait Widget {
    fn render(&self, frame: &mut Frame, area: Rect);
}

struct Container {
    children: Vec<Box<dyn Widget>>,
}

impl Widget for Container {
    fn render(&self, frame: &mut Frame, area: Rect) {
        // Render children
    }
}
```

Benefits:
- Hierarchical UI composition
- Consistent rendering interface
- Flexible layout management

## Architectural Patterns

### Model-View-Controller (MVC)

The editor follows an MVC architecture:

- **Model**: Buffer and document management
- **View**: Terminal UI rendering
- **Controller**: Input handling and command execution

### Plugin Architecture

The WASM plugin system follows a host-guest architecture:

- **Host**: The xvim editor core
- **Guest**: WASM plugins with defined interfaces
- **Bridge**: WIT (WebAssembly Interface Types) definitions

The plugin system is designed with these key components:

1. **Plugin Manager**: Central coordinator for plugin operations
   ```rust
   pub struct PluginManager {
       runtime: WasmRuntime,
       event_manager: EventManager,
       ui_manager: Option<UiManager>,
       network_manager: NetworkManager,
       task_manager: TaskManager,
       dependency_manager: DependencyManager,
       debug_manager: DebugManager,
       context: Arc<Mutex<PluginContext>>,
       plugins: HashMap<String, PluginInfo>,
       plugin_dir: PathBuf,
   }
   ```

2. **Plugin Context**: Shared state between the editor and plugins
   ```rust
   pub struct PluginContext {
       buffer_manager: Option<Arc<Mutex<BufferManager>>>,
       mode_manager: Option<Arc<Mutex<ModeManager>>>,
       command_registry: Option<Arc<Mutex<ExCommandRegistry>>>,
       terminal_ui: Option<Arc<Mutex<TerminalUi>>>,
       custom_data: HashMap<String, Vec<u8>>,
   }
   ```

3. **WASM Runtime**: Executes plugin code in a sandboxed environment
   ```rust
   pub struct WasmRuntime {
       plugins: HashMap<String, WasmPlugin>,
       context: Arc<Mutex<PluginContext>>,
   }
   ```

4. **Event System**: Allows plugins to react to editor events
   ```rust
   pub enum EventType {
       BufferCreated(usize),
       BufferDeleted(usize),
       BufferChanged(usize),
       ModeChanged(Mode),
       CursorMoved(usize, usize, usize),
       CommandExecuted(String),
       Custom(String, Vec<u8>),
   }
   ```

### Lock Management Pattern

The plugin system uses a careful lock acquisition pattern to prevent deadlocks:

1. **Lock Ordering**: Acquire locks in a consistent order
   - Plugin Manager → Plugin Context → Buffer Manager → Terminal UI
   
2. **Early Lock Release**: Release locks as soon as possible
   ```rust
   // Get the context
   let context_arc = plugin_manager.context();
   
   // Lock the context
   let context_result = context_arc.lock();
   if context_result.is_err() {
       return Err(anyhow!("Failed to lock plugin context"));
   }
   
   // Use the context
   let context = context_result.unwrap();
   let terminal_ui_option = context.terminal_ui();
   
   // Release the context lock before proceeding
   drop(context);
   
   // Now use the terminal UI
   if let Some(terminal_ui) = terminal_ui_option {
       // Lock and use the terminal UI
   }
   ```

3. **Minimal Lock Scope**: Keep critical sections as small as possible

This approach helps prevent deadlocks that can occur when multiple threads try to acquire locks in different orders.

### Known Issues with the Plugin System

The plugin system currently has several issues that need to be addressed:

1. **Deadlocks in UI Operations**: The NoxChat command can cause deadlocks when trying to split windows. This is because the terminal UI operations may require locks that are already held by other components.

2. **Circular Dependencies**: There are potential circular dependencies in the lock acquisition pattern, particularly between the plugin manager, plugin context, and terminal UI.

3. **Conversation Management**: The AI conversation functionality is implemented but not fully integrated with the editor. The current implementation creates buffers but doesn't properly handle the window layout.

4. **Compilation Errors**: There are persistent compilation errors in the plugin system that need to be fixed before the editor can be run.

5. **Numerous Warnings**: The codebase has many warnings about unused variables, imports, and other issues that should be addressed to improve code quality.

To address these issues, we need to:

1. Simplify the lock acquisition pattern to avoid potential deadlocks
2. Refactor the UI operations to be more independent of the plugin system
3. Implement a more robust conversation management system
4. Fix the compilation errors and address the warnings

### Error Handling Strategy

Error handling follows a consistent pattern:

```rust
// Custom error types
pub enum ExCommandError {
    InvalidCommand(String),
    MissingArgument(String),
    InvalidArgument(String),
    // ...
}

// Result type alias
pub type ExCommandResult<T> = Result<T, ExCommandError>;

// Error propagation
fn handle_command(cmd: &ExCommand) -> ExCommandResult<()> {
    // Implementation that may return errors
    if cmd.args.is_empty() {
        return Err(ExCommandError::MissingArgument("Required argument missing".to_string()));
    }
    
    // Success case
    Ok(())
}
```

Benefits:
- Clear error messages for users
- Type-safe error handling
- Consistent error propagation

## Code Organization Patterns

### Module Structure

The codebase is organized into modules by functionality:

```
src/
├── buffer/       # Text buffer management
├── command/      # Ex command implementation
├── cursor/       # Cursor management
├── editor/       # Core editor functionality
├── input/        # Input handling
├── mode/         # Editor modes
├── plugin/       # WASM plugin system
├── search/       # Search functionality
├── selection/    # Selection management
├── ui/           # Terminal UI components
└── visual/       # Visual mode implementation
```

### Interface Segregation

Interfaces are kept small and focused:

```rust
trait BufferReader {
    fn get_line(&self, line: usize) -> Option<&str>;
    fn line_count(&self) -> usize;
    // ...
}

trait BufferWriter {
    fn insert(&mut self, pos: Position, text: &str) -> Result<(), BufferError>;
    fn delete(&mut self, range: Range) -> Result<(), BufferError>;
    // ...
}
```

### Dependency Injection

Components receive their dependencies through constructors or setters:

```rust
struct Editor {
    buffer_manager: BufferManager,
    command_registry: ExCommandRegistry,
    // ...
}

impl Editor {
    pub fn new(
        buffer_manager: BufferManager,
        command_registry: ExCommandRegistry,
    ) -> Self {
        // ...
    }
}
```

These patterns work together to create a maintainable, extensible, and robust codebase for the xvim editor.