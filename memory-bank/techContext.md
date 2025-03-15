# xvim Technical Context

## Architecture Overview

xvim follows a modular architecture with clear separation of concerns:

```
xvim
├── src/
│   ├── buffer/       # Text buffer management
│   ├── command/      # Ex command implementation
│   ├── editor/       # Core editor functionality
│   ├── input/        # Input handling
│   ├── mode/         # Editor modes (normal, insert, visual)
│   ├── plugin/       # WASM plugin system
│   ├── ui/           # Terminal UI components
│   └── main.rs       # Application entry point
└── tests/            # Test suite
```

## Key Technical Components

### Buffer Management

- Uses the `ropey` crate for efficient text storage and manipulation
- Implements a rope data structure for O(log n) insertions and deletions
- Handles Unicode correctly with `unicode-segmentation`

### Command System

- Implements Ex commands with a registry pattern
- Commands are registered with handlers that can be invoked by name
- Error handling with custom error types for clear user feedback

### UI System

- Terminal-based UI using `crossterm` for terminal manipulation
- Widget-based rendering with `ratatui` (formerly tui-rs)
- Event-driven architecture for handling user input

### Plugin System

- WebAssembly-based plugin system using `wasmtime`
- WASI implementation with `wasi-common`
- Interface Types binding generation with `wit-bindgen`

## Dependencies

### Core Dependencies
- `ropey`: Rope data structure for efficient text editing
- `unicode-segmentation`: Unicode-aware text manipulation
- `regex`: Pattern matching and search

### Terminal UI
- `crossterm`: Cross-platform terminal manipulation
- `ratatui`: Terminal user interface widgets

### WASM Runtime
- `wasmtime`: WebAssembly runtime
- `wasi-common`: WASI implementation
- `wit-bindgen`: Interface Types binding generator

### Utility Libraries
- `anyhow`: Error handling
- `serde` & `serde_json`: Serialization/deserialization
- `toml`: Configuration file parsing
- `dirs`: Finding standard directories
- `log`: Logging infrastructure
- `clap`: Command-line argument parsing

### Testing
- `proptest`: Property-based testing
- `criterion`: Benchmarking
- `mockall`: Mocking for unit tests
- `tempfile`: Temporary file creation for tests

## Development Environment

### Build System
- Cargo for building and package management (cargo 1.85.0)
- Rustc compiler (rustc 1.85.0)
- Feature flags for conditional compilation

### Testing Strategy
- Unit tests for individual components
- Integration tests for command functionality
- Property-based testing for buffer operations
- Benchmarks for performance-critical code

### Code Organization
- Clear module boundaries
- Trait-based interfaces for flexibility
- Error handling with custom error types
- Documentation with rustdoc

## Performance Considerations

- Text operations use rope data structure for O(log n) complexity
- Minimal allocations in hot paths
- Efficient rendering with terminal UI
- Lazy loading of plugins