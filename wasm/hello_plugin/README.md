# Hello World Plugin for xvim

This is a simple example plugin for the xvim editor that demonstrates the basic functionality of the plugin API.

## Features

- Adds a `:hello` command that displays a greeting message
- Demonstrates event handling for buffer creation and mode changes
- Shows how to register commands and event handlers

## Building

To build the plugin, you need to have Rust and the WebAssembly target installed:

```bash
# Install the WebAssembly target
rustup target add wasm32-wasi

# Build the plugin
cargo build --target wasm32-wasi --release
```

The compiled WebAssembly module will be located at `target/wasm32-wasi/release/hello_plugin.wasm`.

## Installation

To install the plugin, copy the WebAssembly module to the xvim plugins directory:

```bash
mkdir -p ~/.config/xvim/plugins
cp target/wasm32-wasi/release/hello_plugin.wasm ~/.config/xvim/plugins/
```

## Usage

Once the plugin is installed, you can use it in xvim:

1. Start xvim
2. Run the `:hello` command to see a simple greeting
3. Run the `:hello YourName` command to see a personalized greeting

## Event Handling

The plugin demonstrates how to handle events from the editor:

- When a buffer is created, the plugin logs a message
- When the editor mode changes, the plugin logs a message

## Plugin API

This example uses the following xvim plugin API functions:

- `register_plugin`: Register the plugin with the editor
- `register_command`: Register a command with the editor
- `register_event_handler`: Register an event handler
- `editor_message`: Display a message in the editor
- `log_message`: Log a message to the plugin log

## License

This plugin is licensed under the MIT License. See the LICENSE file for details.