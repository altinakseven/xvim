# NoxVim: AI Agent Plugin for xvim

## Overview

NoxVim is an AI agent plugin for xvim that provides similar functionality to Roo-Code in VSCode. It integrates AI-powered code assistance, automation, and chat capabilities directly within the xvim editor. The plugin leverages xvim's WASM plugin system to create a seamless and powerful development experience.

## Features

- **AI Chat Interface**: Split-pane interface for interacting with the AI assistant
- **Code Automation**: Generate, refactor, and improve code based on natural language descriptions
- **Context-Aware Assistance**: Understands your project structure and current editing context
- **Auto-Approve Mode**: Automatically applies suggested changes with real-time feedback
- **Realtime Feedback**: Provides immediate feedback as it operates

## Commands

- `:NoxChat` - Open the NoxVim chat interface
- `:NoxGenerate` - Generate code based on a description
- `:NoxRefactor` - Refactor the current selection or file
- `:NoxExplain` - Explain the selected code
- `:NoxFix` - Fix issues in the current file
- `:NoxTest` - Generate tests for the current file
- `:NoxToggleAutoApprove` - Toggle auto-approve mode

## Installation

1. Copy the `noxvim.wasm` file to your xvim plugins directory
2. Add the following to your xvim configuration:

```
" Load the NoxVim plugin
call xvim#plugin#load('noxvim')

" Optional: Configure NoxVim
let g:noxvim_auto_approve = 1  " Enable auto-approve mode by default
let g:noxvim_api_key = 'your-api-key-here'  " Set your API key
```

## Usage

1. Open a file in xvim
2. Use `:NoxChat` to open the chat interface
3. Type your request and press Ctrl-] to send it
4. View the AI's response in the output pane
5. Use specific commands like `:NoxGenerate` for targeted tasks

## Configuration

NoxVim can be configured through the following settings:

- `g:noxvim_auto_approve`: Enable/disable auto-approve mode (default: 1)
- `g:noxvim_api_key`: Your API key for the AI service
- `g:noxvim_api_endpoint`: Custom API endpoint (optional)
- `g:noxvim_model`: AI model to use (default: "gpt-4")
- `g:noxvim_max_context`: Maximum context size to send (default: 4000)

## License

MIT