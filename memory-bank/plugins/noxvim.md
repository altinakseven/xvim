# Noxvim Plugin

## Overview

Noxvim is a core plugin for xvim that provides Neovim-like features and modern editor capabilities. It's designed to be loaded by default when xvim starts, providing an enhanced editing experience out of the box.

## Architecture

Noxvim is implemented as a WebAssembly (WASM) plugin using the xvim plugin API. It's loaded automatically by the editor during initialization.

### Integration Points

- **Plugin System**: Noxvim is loaded through the standard xvim plugin system
- **UI Integration**: Enhances the xvim UI with modern elements
- **Command Extensions**: Adds Neovim-style commands to xvim
- **API Usage**: Uses the full range of xvim plugin APIs

## Current Status

Noxvim is in early development. The basic plugin structure is in place, but most features are still planned for future implementation.

### Implemented Features

- Plugin auto-loading on editor startup
- Basic plugin structure and dependency management

### Planned Features

- Modern UI elements
- Enhanced syntax highlighting
- Improved completion
- LSP integration
- Treesitter support
- Telescope-like fuzzy finding
- Lua scripting support

## Development Notes

- The plugin is loaded from `plugins/noxvim.wasm`
- Dependency information is stored in `plugins/noxvim.dep.json`
- The plugin is designed to be compatible with all xvim versions