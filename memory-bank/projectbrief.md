# xvim Project Brief

## Project Overview

xvim is a ground-up rewrite of the Vim text editor in pure Rust with a WebAssembly (WASM) plugin system. The goal is to create a modern, extensible text editor that maintains the familiar keybindings and functionality of Vim while leveraging the safety and performance benefits of Rust.

## Core Objectives

1. **Vim Compatibility**: Maintain compatibility with core Vim functionality and keybindings
2. **Rust Implementation**: Utilize Rust's safety features and performance
3. **WASM Plugin System**: Create a flexible plugin system using WebAssembly
4. **Cross-Platform**: Support for multiple platforms (Linux, macOS, Windows)
5. **Modern Features**: Implement modern editor features while maintaining Vim's efficiency

## Key Components

1. **Buffer Management**: Efficient text storage and manipulation
2. **Command Mode**: Implementation of Ex commands
3. **Normal Mode**: Vim-style navigation and editing
4. **Visual Mode**: Text selection and manipulation
5. **Insert Mode**: Text input
6. **Window Management**: Split views and tab management
7. **Plugin System**: WASM-based plugin architecture
8. **UI Layer**: Terminal-based user interface using crossterm and ratatui

## Current Status

The project is in active development with the following components implemented:

1. **Buffer Management**: Basic implementation complete
2. **Command Mode**: Ex commands implemented with proper error handling
3. **UI Framework**: Basic terminal UI implemented

## Next Steps

1. Complete implementation of Normal mode
2. Implement Visual mode
3. Implement Insert mode
4. Enhance window and tab management
5. Develop the WASM plugin system
6. Add syntax highlighting
7. Implement search and replace functionality

## Technical Stack

- **Language**: Rust
- **Terminal UI**: crossterm, ratatui
- **Text Handling**: ropey, unicode-segmentation
- **Plugin System**: wasmtime, wasi-common, wit-bindgen
- **Configuration**: toml
- **Error Handling**: anyhow
- **Testing**: proptest, criterion, mockall
