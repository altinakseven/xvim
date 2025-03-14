# xvim

A ground-up rewrite of the Vim text editor in pure Rust, maintaining full compatibility with Vim's CLI interface while modernizing its architecture and plugin system through WebAssembly.

## Project Overview

xvim aims to preserve Vim's efficiency and modal editing paradigm while enabling a more powerful, secure, and flexible extension ecosystem. By implementing the editor in Rust, xvim eliminates entire classes of bugs related to memory management and provides a more maintainable codebase.

## Core Goals

1. **Complete Vim Compatibility**: Implement all standard Vim CLI functionality with identical behavior and command syntax
2. **Pure Rust Implementation**: Build the entire codebase in Rust for memory safety, performance, and maintainability
3. **WASM Plugin System**: Create a plugin API supporting WebAssembly modules using the latest WASI standard
4. **Enhanced Plugin Capabilities**: Support both traditional Vim plugin patterns and Neovim's Lua-style advanced APIs
5. **Comprehensive Documentation**: Provide thorough documentation of the codebase, plugin API, and usage guides

## Current Status

The project is currently in the Foundation Phase, focusing on establishing the core architecture and implementing the fundamental components:

- Core architecture design
- Buffer management system
- Modal editing engine
- WASM plugin system research

## Building from Source

### Prerequisites

- Rust toolchain (1.75+)
- Cargo package manager

### Build Instructions

```bash
# Clone the repository
git clone https://github.com/yourusername/xvim.git
cd xvim

# Build in debug mode
cargo build

# Build in release mode
cargo build --release

# Run the editor
cargo run

# Run tests
cargo test
```

## Project Structure

```
xvim/
├── src/                  # Core Rust source code
│   ├── buffer/           # Buffer management
│   ├── command/          # Command parsing and execution
│   ├── editor/           # Core editor functionality
│   ├── mode/             # Modal editing implementation
│   ├── plugin/           # Plugin system
│   └── ui/               # User interface
├── wasm/                 # WASM runtime and interfaces
├── api/                  # Plugin API definitions
├── tests/                # Test suite
├── docs/                 # Documentation
├── examples/             # Example plugins and configurations
└── tools/                # Development tools and scripts
```

## Features

- Modal editing with full Vim keybinding support
- Text buffer management system
- Window/viewport management
- Configuration system compatible with .vimrc
- Command parser and executor
- WASM plugin runtime (planned)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.