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

The command system is organized into multiple handler modules:

```rust
// Core handlers in handlers.rs
pub fn register_handlers(registry: &mut ExCommandRegistry, plugin_manager: Option<Arc<Mutex<PluginManager>>>) {
    // Register core commands
    registry.register("write", make_handler(handle_write));
    registry.register("quit", make_handler(handle_quit));
    // ...
    
    // Register additional handlers
    additional_handlers::register_additional_handlers(registry);
    
    // Register quickfix handlers
    quickfix_handlers::register_quickfix_handlers(registry);
    
    // Register plugin commands
    if let Some(plugin_manager) = plugin_manager {
        plugin::commands::register_plugin_commands(registry, plugin_manager);
    }
}

// Additional handlers in additional_handlers.rs
pub fn register_additional_handlers(registry: &mut ExCommandRegistry) {
    // Register additional commands
    registry.register("delmarks", handle_delmarks);
    registry.register("history", handle_history);
    registry.register("source", handle_source);
    registry.register("let", handle_let);
    registry.register("unlet", handle_unlet);
    // ...
}

// Quickfix handlers in quickfix_handlers.rs
pub fn register_quickfix_handlers(registry: &mut ExCommandRegistry) {
    // Register quickfix commands
    registry.register("copen", handle_copen);
    registry.register("cclose", handle_cclose);
    registry.register("cnext", handle_cnext);
    registry.register("cprevious", handle_cprev);
    registry.register("make", handle_make);
    registry.register("grep", handle_grep);
    // ...
}

// Fold handlers in fold_handlers.rs
pub fn register_fold_handlers(registry: &mut ExCommandRegistry) {
    // Register fold commands
    registry.register("fold", handle_fold);
    registry.register("foldopen", handle_foldopen);
    registry.register("foldclose", handle_foldclose);
    registry.register("foldtoggle", handle_foldtoggle);
    registry.register("foldmethod", handle_foldmethod);
    // ...
}

// Tag handlers in tag_handlers.rs
pub fn register_tag_handlers(registry: &mut ExCommandRegistry) {
    // Register tag commands
    registry.register("tag", handle_tag);
    registry.register("tags", handle_tags);
    registry.register("tselect", handle_tselect);
    registry.register("tjump", handle_tjump);
    registry.register("tnext", handle_tnext);
    registry.register("tprevious", handle_tprev);
    // ...
}

// Window handlers in window_handlers.rs
pub fn register_window_handlers(registry: &mut ExCommandRegistry) {
    // Register window commands
    registry.register("wincmd", handle_wincmd);
    registry.register("winsize", handle_winsize);
    registry.register("winchoose", handle_winchoose);
    registry.register("winlayout", handle_winlayout);
    registry.register("windo", handle_windo);
    
    // Register tab commands
    registry.register("tabdo", handle_tabdo);
    registry.register("tabmove", handle_tabmove);
    registry.register("tabnew", handle_tabnew);
    registry.register("tabfind", handle_tabfind);
    // ...
}

// Window handlers in window_handlers.rs
pub fn register_window_handlers(registry: &mut ExCommandRegistry) {
    // Register window commands
    registry.register("wincmd", handle_wincmd);
    registry.register("winsize", handle_winsize);
    registry.register("winchoose", handle_winchoose);
    registry.register("winlayout", handle_winlayout);
    registry.register("windo", handle_windo);
    
    // Register tab commands
    registry.register("tabdo", handle_tabdo);
    registry.register("tabmove", handle_tabmove);
    registry.register("tabnew", handle_tabnew);
    registry.register("tabfind", handle_tabfind);
    // ...
}

// Search handlers in search_handlers.rs
pub fn register_search_handlers(registry: &mut ExCommandRegistry) {
    // Register search commands
    registry.register("search", handle_search);
    registry.register("/", handle_search);
    registry.register("?", handle_search_backward);
    registry.register("n", handle_search_next);
    registry.register("N", handle_search_prev);
    
    // Register replace commands
    registry.register("substitute", handle_substitute);
    registry.register("s", handle_substitute);
    registry.register("global", handle_global);
    registry.register("g", handle_global);
    // ...
}

// Macro handlers in macro_handlers.rs
pub fn register_macro_handlers(registry: &mut ExCommandRegistry) {
    // Register macro commands
    registry.register("record", handle_record);
    registry.register("stoprecord", handle_stoprecord);
    registry.register("playback", handle_playback);
    registry.register("let", handle_let);
    registry.register("registers", handle_registers);
    registry.register("display", handle_display);
    registry.register("normal", handle_normal);
    // ...
}

```

Benefits:
- Decouples command invocation from implementation
- Allows for command aliasing (e.g., `:w` and `:write`)
- Centralizes error handling
- Modular organization of commands by functionality
- Extensible through plugins

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

### Normal Mode Command Handling Architecture

The Normal mode command handling system follows a handler-registry-operator pattern:

```rust
// Normal command handler
pub struct NormalCommandHandler {
    // Current count prefix (e.g., 5 in 5dd)
    count: Option<usize>,
    // Current operator (e.g., d in dd)
    operator: Option<Operator>,
    // Current register (e.g., a in "ayy)
    register: Option<char>,
    // Command mappings
    command_map: HashMap<char, Box<dyn Fn(&mut Editor) -> EditorResult<()> + Send + Sync>>,
    // Operator mappings
    operator_map: HashMap<char, Operator>,
    // Motion mappings
    motion_map: HashMap<char, Direction>,
    // Text object mappings
    text_object_map: HashMap<char, TextObjectType>,
}

impl NormalCommandHandler {
    // Register a command
    pub fn register_command<F>(&mut self, key: char, handler: F)
    where
        F: Fn(&mut Editor) -> EditorResult<()> + Send + Sync + 'static,
    {
        self.command_map.insert(key, Box::new(handler));
    }
    
    // Handle a key press
    pub fn handle_key(&mut self, editor: &mut Editor, key: char) -> EditorResult<()> {
        // Implementation
    }
}
```

The Normal mode command handling system provides:

1. **Command Registration**: Register handlers for specific keys
2. **Operator Handling**: Support for operators like delete, change, and yank
3. **Motion Integration**: Combine operators with motions (e.g., dw, cj, y$)
4. **Text Object Support**: Operate on text objects (e.g., diw, ca", yi()
5. **Count Prefixes**: Apply commands multiple times (e.g., 3dd, 5j)
6. **Register Selection**: Use specific registers for operations (e.g., "ayy, "bp)
7. **Composable Commands**: Build complex commands from simple components

This architecture allows for the implementation of Vim's powerful and composable command language, where operators, motions, and text objects can be combined in various ways to perform complex editing operations with minimal keystrokes.

The integration with the Mode Manager ensures that key presses are properly routed to the appropriate handler based on the current mode:

```rust
// Mode manager
pub struct ModeManager {
    // Current mode
    current_mode: Mode,
    // Previous mode
    previous_mode: Mode,
    // Normal mode command handler
    normal_handler: NormalCommandHandler,
}

impl ModeManager {
    // Handle a key press in the current mode
    pub fn handle_key(&mut self, editor: &mut Editor, key: char) -> Result<(), String> {
        match self.current_mode {
            Mode::Normal => {
                // Use the normal command handler
                self.normal_handler.handle_key(editor, key)
                    .map_err(|e| format!("Error handling key in normal mode: {}", e))
            },
            // Handle other modes
            // ...
        }
    }
}
```

This design provides a clean separation of concerns between the mode management system and the specific command handling logic for each mode, making it easier to extend and maintain the codebase.

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

### Quickfix System Architecture

The quickfix system follows a manager-list-entry architecture:

```rust
// Quickfix manager
pub struct QuickfixManager {
    // Quickfix lists
    pub lists: VecDeque<QuickfixList>,
    // Current quickfix list index
    pub current: usize,
    // Location lists (one per window)
    pub location_lists: HashMap<usize, VecDeque<QuickfixList>>,
    // Current location list index (per window)
    pub current_location: HashMap<usize, usize>,
    // Quickfix window ID
    pub quickfix_window: Option<usize>,
    // Location list window ID (per window)
    pub location_windows: HashMap<usize, usize>,
}

// Quickfix list
pub struct QuickfixList {
    // Entries in the quickfix list
    pub entries: Vec<QuickfixEntry>,
    // Current position in the quickfix list
    pub current: usize,
    // Title of the quickfix list
    pub title: String,
}

// Quickfix entry
pub struct QuickfixEntry {
    // File path
    pub file: PathBuf,
    // Line number
    pub line: usize,
    // Column number
    pub col: Option<usize>,
    // Error message
    pub text: String,
    // Error type (error, warning, info)
    pub error_type: QuickfixErrorType,
}
```

The quickfix system provides:

1. **Error Navigation**: Jump between errors, warnings, and other locations in the codebase
2. **External Command Integration**: Capture output from external commands like `make` and `grep`
3. **Search Results**: Store and navigate search results from commands like `vimgrep`
4. **Window Management**: Open and close quickfix windows to display errors
5. **Multiple Lists**: Maintain a history of quickfix lists that can be navigated

This architecture allows for efficient navigation of errors and search results, with a clear separation between the global quickfix list and window-specific location lists.

### Fold System Architecture

The fold system follows a manager-fold-state architecture:

```rust
// Fold manager
pub struct FoldManager {
    // Folds for each buffer
    pub folds: HashMap<usize, Vec<Fold>>,
    // Fold method for each buffer
    pub methods: HashMap<usize, FoldMethod>,
    // Fold column (used for displaying fold markers)
    pub fold_column: usize,
    // Fold minimum width
    pub fold_min_width: usize,
    // Fold text function
    pub fold_text: Option<String>,
}

// Fold
pub struct Fold {
    // Start line
    pub start: usize,
    // End line
    pub end: usize,
    // Fold level
    pub level: usize,
    // Fold state
    pub state: FoldState,
    // Fold text (displayed when fold is closed)
    pub text: Option<String>,
}

// Fold method
pub enum FoldMethod {
    // Manual folding
    Manual,
    // Indent-based folding
    Indent,
    // Expression-based folding
    Expr,
    // Syntax-based folding
    Syntax,
    // Marker-based folding
    Marker,
    // Diff-based folding
    Diff,
}
```

The fold system provides:

1. **Code Organization**: Hide and show sections of code to focus on relevant parts
2. **Multiple Folding Methods**: Different ways to create folds (manual, indent, marker)
3. **Nested Folds**: Support for hierarchical folding with different levels
4. **Visual Indicators**: Display fold markers in the fold column
5. **Customizable Display**: Options for how folds are displayed when closed

This architecture allows for flexible code folding with different methods for different file types, while maintaining a consistent interface for fold operations.

### Tag System Architecture

The tag system follows a database-entry-stack architecture:

```rust
// Tag database
pub struct TagDatabase {
    // Tags by name
    pub tags: HashMap<String, Vec<TagEntry>>,
    // Tag files
    pub tag_files: Vec<PathBuf>,
    // Tag stack
    pub tag_stack: VecDeque<(String, usize)>,
    // Current tag index
    pub current_tag: Option<(String, usize)>,
}

// Tag entry
pub struct TagEntry {
    // Tag name
    pub name: String,
    // File path
    pub file: PathBuf,
    // Search pattern
    pub pattern: String,
    // Tag kind (function, class, etc.)
    pub kind: Option<String>,
    // Line number (if available)
    pub line: Option<usize>,
    // Additional fields
    pub fields: HashMap<String, String>,
}
```

The tag system provides:

1. **Code Navigation**: Jump to symbol definitions across files
2. **Tag Selection**: Choose from multiple matching tags
3. **Tag Stack**: Track tag jumps with a stack for easy navigation back
4. **Tag Files**: Load tags from external tag files (e.g., generated by ctags)
5. **Tag Search**: Find tags by name or pattern

This architecture allows for efficient navigation between symbol definitions, which is essential for understanding and working with large codebases.

### Window and Tab System Architecture

The window and tab system follows a manager-layout-view architecture:

```rust
// Window manager
pub struct WindowManager {
    // Current layout
    pub layout: WindowLayout,
    // Minimum window width
    pub min_width: usize,
    // Minimum window height
    pub min_height: usize,
    // Equal size flag
    pub equal_size: bool,
    // Window focus history
    pub focus_history: Vec<usize>,
}

// Window layout
pub enum WindowLayout {
    // Horizontal layout (windows stacked vertically)
    Horizontal,
    // Vertical layout (windows side by side)
    Vertical,
    // Grid layout (windows in a grid)
    Grid,
}

// Window view
pub struct WindowView {
    // Cursor position
    pub cursor: CursorPosition,
    // Top line
    pub top_line: usize,
    // Left column
    pub left_col: usize,
    // Cursor line
    pub cursor_line: usize,
    // Cursor column
    pub cursor_col: usize,
    // Cursor line offset
    pub cursor_line_offset: usize,
    // Cursor column offset
    pub cursor_col_offset: usize,
}
```

The window and tab system provides:

1. **Multi-file Editing**: Edit multiple files simultaneously in different windows and tabs
2. **Window Navigation**: Move between windows with directional commands
3. **Window Layout**: Control window size, position, and arrangement
4. **Tab Management**: Create, close, and navigate between tabs
5. **View Preservation**: Save and restore window views (cursor position, scroll position, etc.)
6. **Batch Operations**: Execute commands across all windows or tabs

This architecture allows for flexible window and tab management, enabling efficient navigation and organization of multiple files within the editor.

### Search and Replace System Architecture

The search and replace system follows a manager-options-history architecture:

```rust
// Search manager
pub struct SearchManager {
    // Search options
    pub options: SearchOptions,
    // Last search pattern
    pub last_pattern: Option<String>,
    // Last replacement pattern
    pub last_replacement: Option<String>,
    // Search history
    pub history: SearchHistory,
    // Highlighted matches
    pub highlights: HashMap<usize, Vec<(usize, usize)>>,
}

// Search options
pub struct SearchOptions {
    // Case sensitivity
    pub case_sensitive: bool,
    // Whole word search
    pub whole_word: bool,
    // Regular expression search
    pub regex: bool,
    // Incremental search
    pub incremental: bool,
    // Highlight all matches
    pub highlight_all: bool,
    // Wrap around
    pub wrap_around: bool,
    // Search direction
    pub direction: SearchDirection,
}

// Search history
pub struct SearchHistory {
    // Search history entries
    pub entries: Vec<SearchHistoryEntry>,
    // Current position in the history
    pub current: usize,
    // Maximum history size
    pub max_size: usize,
}
```

The search and replace system provides:

1. **Pattern Searching**: Find text patterns in the buffer with forward and backward search
2. **Pattern Replacement**: Replace text patterns with substitution strings
3. **Global Operations**: Execute commands on lines matching or not matching a pattern
4. **Search Options**: Configure case sensitivity, regex support, and other search behaviors
5. **Search Highlighting**: Highlight all matches of the search pattern
6. **Search History**: Track and recall previous search patterns

This architecture allows for powerful text manipulation and navigation, with a flexible system for finding and replacing text patterns throughout the editor.

### Macro System Architecture

The macro system follows a manager-recording-execution architecture:

```rust
// Macro manager
pub struct MacroManager {
    // Macros by register name
    pub macros: HashMap<char, Vec<String>>,
    // Current recording
    pub recording: Option<MacroRecording>,
    // Currently executing macro
    pub executing: Option<char>,
    // Execution depth (to prevent infinite recursion)
    pub execution_depth: usize,
    // Maximum execution depth
    pub max_execution_depth: usize,
}

// Macro recording state
pub struct MacroRecording {
    // Register name
    pub register: char,
    // Recorded keystrokes
    pub keystrokes: Vec<String>,
    // Start time
    pub start_time: std::time::SystemTime,
}
```

The macro system provides:

1. **Macro Recording**: Record sequences of keystrokes into named registers
2. **Macro Playback**: Play back recorded keystrokes from registers
3. **Register Management**: Store and retrieve macro content from registers
4. **Recursion Protection**: Prevent infinite recursion when macros call themselves
5. **Normal Mode Execution**: Execute normal mode commands programmatically
6. **Register Display**: View the content of registers

This architecture allows for powerful automation of repetitive tasks, with a flexible system for recording, storing, and playing back sequences of commands.

### Undo/Redo System Architecture

The undo/redo system follows a manager-tree-branch architecture:

```rust
// Undo manager
pub struct UndoManager {
    // Undo tree for each buffer
    pub undo_trees: HashMap<usize, UndoTree>,
    // Auto-save settings
    pub auto_save: bool,
    // Auto-save interval (in seconds)
    pub auto_save_interval: u64,
    // Last auto-save time
    pub last_auto_save: std::time::SystemTime,
    // Persistent undo
    pub persistent_undo: bool,
    // Persistent undo directory
    pub persistent_undo_dir: Option<std::path::PathBuf>,
    // Maximum number of changes to remember
    pub max_changes: usize,
}

// Undo tree
pub struct UndoTree {
    // Undo branches
    pub branches: Vec<UndoBranch>,
    // Current branch index
    pub current_branch: usize,
    // Current position in the current branch
    pub current_position: usize,
    // Last save position
    pub last_save_position: Option<(usize, usize)>,
}

// Undo branch
pub struct UndoBranch {
    // Changes in this branch
    pub changes: Vec<UndoChange>,
    // Parent branch index
    pub parent: Option<usize>,
    // Parent position
    pub parent_position: Option<usize>,
    // Branch name
    pub name: Option<String>,
}
```

The undo/redo system provides:

1. **Change Tracking**: Record all changes made to buffers
2. **Undo/Redo Operations**: Revert or reapply changes
3. **Branching History**: Support for multiple undo branches
4. **Time-based Navigation**: Go back or forward in time
5. **Persistent History**: Save and load undo history across sessions
6. **Change Grouping**: Group related changes for atomic undo/redo
7. **History Display**: View the undo history tree

This architecture allows for powerful change management, with a flexible system for tracking, navigating, and manipulating the history of changes to buffers.

### Mark System Architecture

The mark system follows a manager-position-jump architecture:

```rust
// Mark manager
pub struct MarkManager {
    // Marks by name
    pub marks: HashMap<char, MarkPosition>,
    // Last jump position
    pub last_jump: Option<MarkPosition>,
    // Jump list
    pub jump_list: Vec<MarkPosition>,
    // Current position in the jump list
    pub jump_position: usize,
    // Maximum jump list size
    pub max_jump_list_size: usize,
    // Automatic marks
    pub auto_marks: bool,
}

// Mark position
pub struct MarkPosition {
    // Buffer ID
    pub buffer_id: usize,
    // Line number
    pub line: usize,
    // Column number
    pub column: usize,
    // Timestamp
    pub timestamp: std::time::SystemTime,
}
```

The mark system provides:

1. **Position Marking**: Set marks at specific positions in buffers
2. **Position Navigation**: Jump to marked positions
3. **Jump History**: Track jumps between positions
4. **Jump Navigation**: Navigate forward and backward through the jump history
5. **Mark Types**: Support for local marks (lowercase), file marks (uppercase), and special marks
6. **Mark Management**: List, set, and delete marks
7. **Automatic Marking**: Automatically set marks for certain operations

This architecture allows for efficient navigation within and between files, with a flexible system for marking and jumping to positions in the text.

### Completion System Architecture

The completion system follows a manager-item-context architecture:

```rust
// Completion manager
pub struct CompletionManager {
    // Completion items
    pub items: Vec<CompletionItem>,
    // Current completion index
    pub current_index: usize,
    // Completion context
    pub context: Option<CompletionContext>,
    // Completion history
    pub history: VecDeque<Vec<CompletionItem>>,
    // Maximum history size
    pub max_history_size: usize,
    // Keyword dictionaries
    pub keyword_dicts: HashMap<String, HashSet<String>>,
    // User defined completions
    pub user_defined_completions: HashMap<String, Box<dyn Fn(&str) -> Vec<CompletionItem> + Send + Sync>>,
    // Omni completions
    pub omni_completions: HashMap<String, Box<dyn Fn(&str) -> Vec<CompletionItem> + Send + Sync>>,
    // Completion options
    pub options: CompletionOptions,
}

// Completion item
pub struct CompletionItem {
    // Item text
    pub text: String,
    // Item kind
    pub kind: String,
    // Item menu
    pub menu: Option<String>,
    // Item info
    pub info: Option<String>,
    // Item source
    pub source: CompletionType,
}

// Completion context
pub struct CompletionContext {
    // Base word
    pub base: String,
    // Start position
    pub start_pos: usize,
    // End position
    pub end_pos: usize,
    // Completion type
    pub completion_type: CompletionType,
    // Buffer ID
    pub buffer_id: usize,
    // Line number
    pub line: usize,
    // Column number
    pub column: usize,
}
```

The completion system provides:

1. **Text Completion**: Suggest completions for partially typed text
2. **Multiple Sources**: Support for various completion sources (keywords, lines, files, tags, etc.)
3. **Context-Aware**: Completions based on the current context (buffer, position, filetype)
4. **User-Defined**: Support for custom completion functions
5. **Omni Completion**: Intelligent completion based on language semantics
6. **Completion Navigation**: Navigate through completion suggestions
7. **Completion Display**: Show completions in a menu or popup

This architecture allows for intelligent text completion, with a flexible system for suggesting and inserting completions based on various sources and contexts.

### Spell Checking System Architecture

The spell checking system follows a manager-dictionary-error architecture:

```rust
// Spell manager
pub struct SpellManager {
    // Spell dictionaries
    pub dictionaries: HashMap<String, SpellDictionary>,
    // Active dictionaries
    pub active_dictionaries: Vec<String>,
    // Spell errors
    pub errors: HashMap<usize, Vec<SpellError>>,
    // Spell options
    pub options: SpellOptions,
    // User dictionary
    pub user_dictionary: Option<PathBuf>,
    // Bad words
    pub bad_words: HashSet<String>,
    // Good words
    pub good_words: HashSet<String>,
    // Spell history
    pub history: VecDeque<SpellAction>,
    // Maximum history size
    pub max_history_size: usize,
}

// Spell dictionary
pub struct SpellDictionary {
    // Dictionary name
    pub name: String,
    // Dictionary path
    pub path: PathBuf,
    // Dictionary words
    pub words: HashSet<String>,
    // Dictionary encoding
    pub encoding: String,
    // Dictionary language
    pub language: String,
    // Dictionary region
    pub region: Option<String>,
    // Dictionary version
    pub version: Option<String>,
    // Dictionary format
    pub format: SpellDictionaryFormat,
}

// Spell error
pub struct SpellError {
    // Misspelled word
    pub word: String,
    // Line number
    pub line: usize,
    // Column number
    pub column: usize,
    // Suggestions
    pub suggestions: Vec<SpellSuggestion>,
    // Error type
    pub error_type: SpellErrorType,
}
```

The spell checking system provides:

1. **Spell Checking**: Identify misspelled words in text
2. **Multiple Dictionaries**: Support for multiple dictionaries and languages
3. **User Dictionaries**: Add custom words to a user dictionary
4. **Bad Words**: Mark specific words as incorrect
5. **Spell Suggestions**: Suggest corrections for misspelled words
6. **Error Highlighting**: Highlight misspelled words in the editor
7. **Spell Configuration**: Configure spell checking behavior

This architecture allows for flexible spell checking with support for multiple languages and dictionaries, providing a powerful tool for identifying and correcting spelling errors in text.

### Diff System Architecture

The diff system follows a manager-change-hunk architecture:

```rust
// Diff manager
pub struct DiffManager {
    // Diff mode enabled
    pub enabled: bool,
    // Diff options
    pub options: DiffOptions,
    // Diff buffers
    pub buffers: HashSet<usize>,
    // Diff changes
    pub changes: HashMap<(usize, usize), Vec<DiffChange>>,
    // Current change index
    pub current_change: usize,
    // Diff base directory
    pub base_dir: Option<PathBuf>,
    // Diff history
    pub history: VecDeque<DiffAction>,
    // Maximum history size
    pub max_history_size: usize,
}

// Diff change
pub struct DiffChange {
    // Change type
    pub change_type: DiffChangeType,
    // Start line in buffer A
    pub start_a: usize,
    // End line in buffer A
    pub end_a: usize,
    // Start line in buffer B
    pub start_b: usize,
    // End line in buffer B
    pub end_b: usize,
    // Hunks (for text changes)
    pub hunks: Vec<DiffHunk>,
}

// Diff hunk
pub struct DiffHunk {
    // Start column in buffer A
    pub start_a: usize,
    // End column in buffer A
    pub end_a: usize,
    // Start column in buffer B
    pub start_b: usize,
    // End column in buffer B
    pub end_b: usize,
}
```

The diff system provides:

1. **File Comparison**: Compare the content of multiple buffers
2. **Change Highlighting**: Highlight added, deleted, and changed lines
3. **Change Navigation**: Navigate between changes in the buffers
4. **Content Transfer**: Copy changes between buffers
5. **Patch Application**: Apply patches to buffers
6. **Diff Options**: Configure how differences are detected and displayed
7. **Window Management**: Split windows to display multiple buffers side by side

This architecture allows for powerful file comparison and merging, with a flexible system for detecting, displaying, and navigating differences between files.

### Session Management System Architecture

The session management system follows a manager-options-file architecture:

```rust
// Session manager
pub struct SessionManager {
    // Current session
    pub current_session: Option<PathBuf>,
    // Session options
    pub options: SessionOptions,
    // Auto-save session
    pub auto_save: bool,
    // Auto-save interval (in seconds)
    pub auto_save_interval: u64,
    // Last auto-save time
    pub last_auto_save: std::time::SystemTime,
    // Session directory
    pub session_dir: Option<PathBuf>,
    // Default session name
    pub default_session: String,
}

// Session options
pub struct SessionOptions {
    // Save buffers
    pub buffers: bool,
    // Save windows
    pub windows: bool,
    // Save tabs
    pub tabs: bool,
    // Save global variables
    pub globals: bool,
    // Save options
    pub options: bool,
    // Save folds
    pub folds: bool,
    // Save marks
    pub marks: bool,
    // Save jumps
    pub jumps: bool,
    // Save registers
    pub registers: bool,
    // Save terminal buffers
    pub terminal: bool,
    // Save blank buffers
    pub blank: bool,
    // Save cursor position
    pub cursor: bool,
    // Save undo history
    pub undo: bool,
    // Save quickfix lists
    pub quickfix: bool,
    // Save help buffers
    pub help: bool,
    // Save empty windows
    pub empty: bool,
    // Save session name
    pub sesdir: bool,
    // Skip session name
    pub skip_session: bool,
}
```

The session management system provides:

1. **Session Saving**: Save the current editor state to a file
2. **Session Loading**: Restore the editor state from a file
3. **View Saving**: Save the state of a single buffer to a file
4. **View Loading**: Restore the state of a single buffer from a file
5. **Session Options**: Configure what is saved in a session
6. **Auto-save**: Automatically save sessions at regular intervals
7. **Script Execution**: Execute commands from a script file

This architecture allows for saving and restoring the editor state, making it easy to resume work exactly where you left off. It also provides a way to save and restore the state of individual buffers, which is useful for complex editing tasks.

### Autocmd System Architecture

The autocmd system follows a manager-event-pattern architecture:

```rust
// Autocmd manager
pub struct AutocmdManager {
    // Groups
    pub groups: HashMap<String, AutocmdGroup>,
    // Default group
    pub default_group: String,
    // Current group
    pub current_group: Option<String>,
    // Autocmd history
    pub history: VecDeque<AutocmdAction>,
    // Maximum history size
    pub max_history_size: usize,
    // Nested level
    pub nested_level: usize,
    // Maximum nested level
    pub max_nested_level: usize,
}

// Autocmd group
pub struct AutocmdGroup {
    // Name
    pub name: String,
    // Autocmds
    pub autocmds: Vec<Autocmd>,
}

// Autocmd
pub struct Autocmd {
    // Event
    pub event: AutocmdEvent,
    // Pattern
    pub pattern: AutocmdPattern,
    // Command
    pub command: AutocmdCommand,
}

// Autocmd event
pub enum AutocmdEvent {
    // Buffer events
    BufEnter, BufLeave, BufWrite, BufWritePost, BufRead, BufReadPost,
    // File events
    FileType, FileRead, FileReadPost, FileWrite, FileWritePost,
    // Window events
    WinEnter, WinLeave,
    // Cursor events
    CursorMoved, CursorMovedI,
    // Mode events
    InsertEnter, InsertLeave, VisualEnter, VisualLeave,
    // Command line events
    CmdlineEnter, CmdlineLeave, CmdlineChanged,
    // Terminal events
    TerminalOpen, TerminalClose,
    // Editor events
    VimEnter, VimLeave, VimLeavePre, QuitPre, Quit,
}
```

The autocmd system provides:

1. **Event-Based Automation**: Execute commands when specific events occur
2. **Pattern Matching**: Match file patterns using glob or regex syntax
3. **Autocmd Groups**: Organize autocmds into logical groups
4. **Nested Execution**: Support for nested autocmds with recursion protection
5. **Command Execution**: Execute Ex commands when events are triggered
6. **Event Types**: Support for various event types (buffer, file, window, etc.)
7. **Conditional Execution**: Execute commands only when specific conditions are met

This architecture allows for powerful event-based automation, enabling the editor to respond to various events and execute commands automatically. It provides a way to customize the editor's behavior based on file types, buffer changes, and other events.

### Terminal System Architecture

The terminal system follows a manager-buffer-process architecture:

```rust
// Terminal manager
pub struct TerminalManager {
    // Terminals
    pub terminals: HashMap<usize, Arc<Mutex<TerminalBuffer>>>,
    // Next terminal ID
    pub next_id: usize,
    // Current terminal
    pub current_terminal: Option<usize>,
    // Terminal history
    pub history: VecDeque<TerminalAction>,
    // Maximum history size
    pub max_history_size: usize,
    // Update thread
    pub update_thread: Option<thread::JoinHandle<()>>,
    // Update channel
    pub update_channel: Option<mpsc::Sender<TerminalAction>>,
    // Update interval
    pub update_interval: Duration,
    // Is running
    pub running: bool,
}

// Terminal buffer
pub struct TerminalBuffer {
    // Process ID
    pub pid: u32,
    // Child process
    pub child: Child,
    // Command
    pub command: String,
    // Working directory
    pub cwd: PathBuf,
    // Environment variables
    pub env: HashMap<String, String>,
    // Output buffer
    pub output: String,
    // Input buffer
    pub input: String,
    // Cursor position
    pub cursor: (usize, usize),
    // Scroll position
    pub scroll: usize,
    // Is running
    pub running: bool,
    // Exit code
    pub exit_code: Option<i32>,
    // Terminal properties
    pub size: (usize, usize),
    pub title: String,
    pub term_type: String,
    pub colors: bool,
    pub bell: bool,
    pub scrollback: usize,
}

// Terminal action
pub enum TerminalAction {
    // Create terminal
    Create(String, PathBuf, HashMap<String, String>, bool),
    // Kill terminal
    Kill(usize),
    // Write input
    Write(usize, String),
    // Resize terminal
    Resize(usize, usize, usize),
    // Set current terminal
    SetCurrent(Option<usize>),
    // Set terminal properties
    SetVisible(usize, bool),
    SetTitle(usize, String),
    SetTermType(usize, String),
    SetColors(usize, bool),
    SetBell(usize, bool),
    SetScrollback(usize, usize),
    SetUpdateInterval(usize, Duration),
    // Stop update thread
    Stop,
}
```

The terminal system provides:

1. **Command Execution**: Run commands in a terminal within the editor
2. **Interactive Terminal**: Interact with running processes
3. **Terminal Output**: Capture and display terminal output
4. **Terminal Input**: Send input to terminal processes
5. **Multiple Terminals**: Support for multiple terminal instances
6. **Terminal Configuration**: Configure terminal properties
7. **Terminal Integration**: Integrate terminals with the editor's buffer and window system

This architecture allows for running commands within the editor, providing a seamless integration between the editor and the command line. It enables users to run commands, view their output, and interact with them without leaving the editor.

### Syntax Highlighting System Architecture

The syntax highlighting system follows a manager-definition-rule architecture:

```rust
// Syntax manager
pub struct SyntaxManager {
    // Syntax definitions
    pub definitions: HashMap<String, SyntaxDefinition>,
    // Runtime directory
    pub runtime_dir: Option<PathBuf>,
}

// Syntax definition
pub struct SyntaxDefinition {
    // Name
    pub name: String,
    // File extensions
    pub file_extensions: Vec<String>,
    // File names
    pub file_names: Vec<String>,
    // First line patterns
    pub first_line_patterns: Vec<String>,
    // Rules
    pub rules: Vec<Rule>,
    // Keywords
    pub keywords: HashMap<TokenType, HashSet<String>>,
    // Comments
    pub comments: Vec<(String, Option<String>)>,
    // Indentation
    pub indentation: Option<String>,
}

// Syntax rule
pub struct Rule {
    // Pattern
    pub pattern: String,
    // Token type
    pub token_type: TokenType,
    // Is regex
    pub is_regex: bool,
    // Is case sensitive
    pub is_case_sensitive: bool,
    // Is whole word
    pub is_whole_word: bool,
    // Is start of line
    pub is_start_of_line: bool,
    // Is end of line
    pub is_end_of_line: bool,
    // Is multiline
    pub is_multiline: bool,
    // Start pattern (for multiline)
    pub start_pattern: Option<String>,
    // End pattern (for multiline)
    pub end_pattern: Option<String>,
    // Include rules
    pub include_rules: Vec<String>,
    // Exclude rules
    pub exclude_rules: Vec<String>,
    // Priority
    pub priority: i32,
}

// Token type
pub enum TokenType {
    Keyword, Identifier, String, Character, Number, Comment, Preprocessor,
    Operator, Type, Function, Variable, Constant, Special, Error, Todo,
    Statement, Conditional, Repeat, Label, Include, Define, Macro,
    PreCondit, StorageClass, Structure, Typedef, Tag, SpecialChar,
    Delimiter, SpecialComment, Debug, Underlined, Ignore, None,
}

// Token
pub struct Token {
    // Token type
    pub token_type: TokenType,
    // Start position
    pub start: usize,
    // End position
    pub end: usize,
}
```

The syntax highlighting system provides:

1. **Language Detection**: Detect the language of a file based on its extension or content
2. **Token-Based Highlighting**: Highlight code based on token types (keywords, strings, comments, etc.)
3. **Rule-Based Matching**: Match code patterns using rules (regex, string, multiline)
4. **Vim Syntax Compatibility**: Load and parse Vim syntax files
5. **Built-in Language Support**: Support for common languages (Rust, C, Python, JavaScript, HTML, CSS, Markdown)
6. **Extensibility**: Add custom syntax definitions for new languages
7. **Integration**: Integrate with the buffer system for highlighting

This architecture allows for flexible and powerful syntax highlighting, making code more readable and easier to understand. It supports a wide range of languages and can be extended to support new languages as needed.

### Vim Script Execution System Architecture

The Vim script execution system follows an interpreter-context-value architecture:

```rust
// Vim script interpreter
pub struct VimScriptInterpreter {
    // Execution context
    pub context: VimContext,
    // Runtime directory
    pub runtime_dir: Option<PathBuf>,
    // Script names
    pub script_names: Vec<PathBuf>,
    // Next script ID
    pub next_script_id: usize,
}

// Vim script execution context
pub struct VimContext {
    // Variables by scope
    pub variables: HashMap<VimScope, HashMap<String, VimValue>>,
    // Functions
    pub functions: HashMap<String, VimFunction>,
    // Current script
    pub current_script: Option<PathBuf>,
    // Script ID
    pub script_id: usize,
    // Script stack
    pub script_stack: Vec<PathBuf>,
    // Function stack
    pub function_stack: Vec<String>,
    // Command output
    pub output: String,
    // Command registry
    pub command_registry: Arc<Mutex<ExCommandRegistry>>,
    // Editor
    pub editor: Option<*mut Editor>,
}

// Vim script value
pub enum VimValue {
    // String value
    String(String),
    // Integer value
    Integer(i64),
    // Float value
    Float(f64),
    // List value
    List(Vec<VimValue>),
    // Dictionary value
    Dict(HashMap<String, VimValue>),
    // Boolean value
    Boolean(bool),
    // Null value
    Null,
}

// Vim script scope
pub enum VimScope {
    // Global scope (g:)
    Global,
    // Buffer scope (b:)
    Buffer,
    // Window scope (w:)
    Window,
    // Tab scope (t:)
    Tab,
    // Script scope (s:)
    Script,
    // Function scope (l:)
    Function,
    // Argument scope (a:)
    Argument,
    // Vim scope (v:)
    Vim,
    // Environment scope (env:)
    Environment,
    // Option scope (&)
    Option,
    // Register scope (@)
    Register,
    // No scope
    None,
}

// Vim script function
pub struct VimFunction {
    // Function name
    pub name: String,
    // Function arguments
    pub args: Vec<String>,
    // Function body
    pub body: Vec<String>,
    // Is script-local
    pub is_script_local: bool,
    // Script ID
    pub script_id: Option<usize>,
    // Line number
    pub line: usize,
}
```

The Vim script execution system provides:

1. **Script Execution**: Execute Vim script files and commands
2. **Variable Management**: Manage variables with different scopes (global, buffer, window, etc.)
3. **Function Definition**: Define and execute functions with arguments and return values
4. **Control Flow**: Support for if statements, while loops, and for loops
5. **Expression Evaluation**: Evaluate expressions with operators and functions
6. **Data Types**: Support for strings, numbers, booleans, lists, and dictionaries
7. **Built-in Functions**: Provide common functions like len(), empty(), type(), etc.
8. **Integration**: Integrate with the Ex command system for executing commands

This architecture allows for powerful scripting capabilities, enabling users to customize the editor's behavior, automate tasks, and extend functionality through scripts. It provides a way to execute Vim script files and commands, which is essential for compatibility with existing Vim configurations and plugins.

### Editor Initialization Architecture

The editor initialization system follows a layered approach:

```rust
// Editor initialization
pub fn initialize_editor(editor: &mut Editor) -> io::Result<()> {
    // Set up the runtime directory
    let runtime_dir = setup_runtime_directory()?;
    
    // Initialize the Vim script interpreter
    initialize_vim_script_interpreter(editor, &runtime_dir)?;
    
    // Load the vimrc file
    load_vimrc(editor, &runtime_dir)?;
    
    Ok(())
}

// Environment initialization
pub fn initialize_environment(editor: &mut Editor) -> io::Result<()> {
    // Create necessary directories
    create_directories()?;
    
    // Set up the runtime directory
    let runtime_dir = setup_runtime_directory()?;
    
    // Load plugins
    load_plugins(editor, &runtime_dir)?;
    
    // Load filetype plugins
    load_filetype_plugins(editor, &runtime_dir)?;
    
    // Load syntax files
    load_syntax_files(editor, &runtime_dir)?;
    
    // Load indent files
    load_indent_files(editor, &runtime_dir)?;
    
    // Load color schemes
    load_color_schemes(editor, &runtime_dir)?;
    
    Ok(())
}
```

The initialization system provides:

1. **Runtime Directory Setup**: Detect and set up the runtime directory for Vim files
2. **Vim Script Initialization**: Initialize the Vim script interpreter for executing Vim scripts
3. **Configuration Loading**: Load the vimrc.vim file for user configuration
4. **Directory Creation**: Create necessary directories for backups, swaps, and undo files
5. **Plugin Loading**: Load and execute plugin files
6. **Filetype Detection**: Load filetype detection and filetype plugins
7. **Syntax Highlighting**: Load syntax highlighting definitions
8. **Indentation Rules**: Load indentation rules for different file types
9. **Color Schemes**: Load and apply color schemes

This architecture allows for a flexible and extensible initialization process, with clear separation of concerns between different initialization steps. It enables the editor to be configured and extended through Vim scripts, providing compatibility with existing Vim configurations and plugins.

### Diff System Architecture

The diff system follows a manager-change-hunk architecture:

```rust
// Diff manager
pub struct DiffManager {
    // Diff mode enabled
    pub enabled: bool,
    // Diff options
    pub options: DiffOptions,
    // Diff buffers
    pub buffers: HashSet<usize>,
    // Diff changes
    pub changes: HashMap<(usize, usize), Vec<DiffChange>>,
    // Current change index
    pub current_change: usize,
    // Diff base directory
    pub base_dir: Option<PathBuf>,
    // Diff history
    pub history: VecDeque<DiffAction>,
    // Maximum history size
    pub max_history_size: usize,
}

// Diff change
pub struct DiffChange {
    // Change type
    pub change_type: DiffChangeType,
    // Start line in buffer A
    pub start_a: usize,
    // End line in buffer A
    pub end_a: usize,
    // Start line in buffer B
    pub start_b: usize,
    // End line in buffer B
    pub end_b: usize,
    // Hunks (for text changes)
    pub hunks: Vec<DiffHunk>,
}

// Diff hunk
pub struct DiffHunk {
    // Start column in buffer A
    pub start_a: usize,
    // End column in buffer A
    pub end_a: usize,
    // Start column in buffer B
    pub start_b: usize,
    // End column in buffer B
    pub end_b: usize,
}
```

The diff system provides:

1. **File Comparison**: Compare the content of multiple buffers
2. **Change Highlighting**: Highlight added, deleted, and changed lines
3. **Change Navigation**: Navigate between changes in the buffers
4. **Content Transfer**: Copy changes between buffers
5. **Patch Application**: Apply patches to buffers
6. **Diff Options**: Configure how differences are detected and displayed
7. **Window Management**: Split windows to display multiple buffers side by side

This architecture allows for powerful file comparison and merging, with a flexible system for detecting, displaying, and navigating differences between files.

### Vim Runtime Integration

The editor integrates with Vim's runtime files using a layered approach:

1. **Runtime Directory Structure**: Mirrors Vim's directory structure for compatibility
   ```
   runtime/
   ├── filetype.vim    # Filetype detection
   ├── ftplugin.vim    # Filetype plugin loader
   ├── indent.vim      # Indentation settings loader
   ├── ftplugin/       # Filetype-specific plugins
   ├── indent/         # Filetype-specific indentation
   ├── syntax/         # Syntax highlighting definitions
   │   ├── syntax.vim  # Syntax system loader
   │   ├── c.vim       # C syntax
   │   ├── cpp.vim     # C++ syntax
   │   ├── python.vim  # Python syntax
   │   └── rust.vim    # Rust syntax
   └── doc/            # Documentation
   ```

2. **Runtime Loading**: Files are loaded at editor startup
   ```rust
   pub fn load_vim_runtime_files(&mut self) -> EditorResult<()> {
       // Load filetype.vim
       // Load syntax.vim
       // Load ftplugin.vim
       // Load indent.vim
       Ok(())
   }
   ```

3. **Dynamic Syntax Loading**: Syntax definitions are loaded from runtime files
   ```rust
   fn load_runtime_syntax_definitions(registry: &mut SyntaxRegistry) {
       // Scan runtime/syntax directory
       // Load each syntax file
       // Create syntax definitions
   }
   ```

This approach allows xvim to leverage Vim's extensive language support while maintaining a clean, modern implementation in Rust.

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