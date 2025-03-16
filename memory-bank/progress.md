# xvim Progress Tracker

This document tracks the current state of the xvim project, highlighting what has been implemented, what's in progress, and what's planned for the future.

## Implemented Features

### Buffer Management
- [x] Basic text buffer implementation with ropey
- [x] Line-based operations (get, insert, delete)
- [x] Unicode support
- [x] Buffer ID system for multiple buffers

### Command Mode
- [x] Ex command parser
- [x] Command registry
- [x] Basic command execution flow
- [x] Error handling for commands

### Ex Commands
- [x] File Operations
  - [x] `:write`, `:w` - Write buffer to file
  - [x] `:quit`, `:q` - Quit
  - [x] `:wquit`, `:wq`, `:xit`, `:x` - Write and quit
  - [x] `:edit`, `:e` - Edit file
  - [x] `:read`, `:r` - Read file into buffer

- [x] Window Operations
  - [x] `:split`, `:sp` - Split window horizontally (stub)
  - [x] `:vsplit`, `:vs` - Split window vertically (stub)
  - [x] `:close`, `:clo` - Close window (stub)
  - [x] `:only`, `:on` - Close all windows except current (stub)
  - [x] `:wnext`, `:wn` - Go to next window (stub)
  - [x] `:wprevious`, `:wp` - Go to previous window (stub)

- [x] Tab Operations
  - [x] `:tabedit`, `:tabe`, `:tabnew` - Open file in new tab (stub)
  - [x] `:tabclose`, `:tabc` - Close current tab (stub)
  - [x] `:tabnext`, `:tabn` - Go to next tab (stub)
  - [x] `:tabprevious`, `:tabp` - Go to previous tab (stub)

- [x] Editing Operations
  - [x] `:delete`, `:d` - Delete lines
  - [x] `:yank`, `:y` - Yank lines
  - [x] `:put`, `:p` - Put yanked text
  - [x] `:copy`, `:co`, `:t` - Copy lines
  - [x] `:move`, `:m` - Move lines
  - [x] `:substitute`, `:s` - Search and replace
  - [x] `:global`, `:g` - Execute command on matching lines (partial)
  - [x] `:vglobal`, `:v` - Execute command on non-matching lines (partial)

- [x] Other Operations
  - [x] `:undo`, `:u` - Undo changes (stub)
  - [x] `:redo`, `:red` - Redo changes (stub)
  - [x] `:set`, `:se` - Set options
  - [x] `:map` - Create key mappings
  - [x] `:unmap` - Remove key mappings
  - [x] `:marks` - Display marks
  - [x] `:jumps` - Display jump list
  - [x] `:registers`, `:reg` - Display registers
  - [x] `:buffers`, `:ls`, `:files` - Display buffers
  - [x] `:windows` - Display windows
  - [x] `:tabs` - Display tabs
  - [x] `:help`, `:h` - Display help

### UI Framework
- [x] Basic terminal UI setup with crossterm and ratatui
- [x] Status line
- [x] Command line
- [x] Basic buffer rendering

### Testing
- [x] Unit tests for buffer operations
- [x] Tests for Ex commands

## In Progress

### Normal Mode
- [x] Basic movement commands (h, j, k, l)
- [x] Word movement (w, b, e)
- [x] Line navigation (0, $, ^)
- [x] Page navigation (Ctrl-F, Ctrl-B)
- [x] Search navigation (/, ?, n, N)

### Visual Mode
- [x] Character-wise selection
- [x] Line-wise selection
- [x] Block-wise selection
- [x] Visual mode operations (delete, yank, change)
- [x] Visual mode navigation
- [x] Reselect previous visual area with 'gv'
- [x] Swap selection corners with 'o' and 'O'

### Insert Mode
- [ ] Basic text insertion
- [ ] Auto-indentation
- [ ] Completion

## Planned Features

### Plugin System
- [x] Basic WASM runtime integration (placeholder)
- [x] Initial Plugin API definition
- [x] Plugin loading and management
- [x] Event system for plugins
- [x] NoxChat command for AI assistance with split window interface
- [ ] Full WASM runtime implementation
- [ ] Comprehensive Plugin API

### Advanced Features
- [ ] Syntax highlighting
- [ ] Code folding
- [ ] Multiple windows
- [ ] Tab pages
- [ ] Macros
- [ ] Marks
- [ ] Tags
- [ ] Completion

### Configuration
- [ ] User configuration file
- [ ] Color schemes
- [ ] Key mapping configuration

## Recent Updates

### March 15, 2025
- Completed the implementation of the `:read` command to fully insert file content
- Improved window navigation with `:wnext` and `:wprev` commands
- Completely redesigned the `:noxchat` command to fix persistent cursor positioning issues:
  - Replaced the chat interface with a read-only information buffer
  - Simplified the command implementation to avoid cursor positioning and insert mode
  - Removed complex lock management that was causing deadlocks
  - Set the buffer as read-only to prevent editing attempts
  - Added informative content about the AI functionality being under development

### March 14, 2025
- Installed Rust toolchain (rustc 1.85.0, cargo 1.85.0) for running tests
- Updated run_tests.sh script to use Cargo for running tests instead of rustc directly
- Implemented Visual mode with character-wise, line-wise, and block-wise selection
- Added Visual mode operations (delete, yank, change)
- Implemented 'gv' command to reselect previous visual area
- Added 'o' and 'O' commands to swap selection corners
- Implemented search navigation (/, ?, n, N) with search history and result navigation
- Enhanced `:jumps` command to display jump list with current position marker
- Enhanced `:windows` command to display window dimensions and buffer information
- Enhanced `:tabs` command to display tab list with windows in each tab
- Enhanced `:help` command to provide detailed help on various topics
- Fixed issues with the NoxChat command in the plugin system:
  - Removed debug print statements to improve performance
  - Fixed potential deadlocks in the plugin command handling
  - Simplified the chat interface creation to avoid UI-related deadlocks

### Previous Updates
- Implemented basic Ex commands with proper error handling
- Set up project structure and core components
- Implemented buffer management system
- Created terminal UI framework

## Next Steps

1. Complete the implementation of Normal mode commands
2. ~~Implement Visual mode selection~~ ✓ Completed
3. Implement Insert mode text entry
4. Begin work on the WASM plugin system
5. Add syntax highlighting support
6. Implement search and replace functionality
7. Add configuration system
8. Enhance Visual mode with more operations and text objects

## Known Issues

1. Some commands are implemented as stubs and need full implementation
2. Error handling could be improved in some areas
3. Need more comprehensive tests for edge cases
4. There are compilation warnings throughout the codebase that should be addressed
5. There are still persistent compilation errors in the codebase that need to be fixed before the editor can be run
6. The AI conversation functionality has been temporarily replaced with a read-only information buffer due to cursor positioning issues