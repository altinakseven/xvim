#!/bin/bash

# Script to generate test files for xvim commands and implementations
# This script will create test files for commands that don't have tests yet

# Exit on error
set -e

# Print what we're doing
set -x

# Define source and destination directories
SRC_DIR="src"
TEST_DIR="tests"

# Create test directory if it doesn't exist
mkdir -p "$TEST_DIR"

# Function to generate a test file for a command
generate_command_test() {
    local command_name=$1
    local test_file="$TEST_DIR/${command_name}_test.rs"
    
    # Skip if test file already exists
    if [ -f "$test_file" ]; then
        echo "Test file for $command_name already exists: $test_file"
        return
    fi
    
    # Create test file
    cat > "$test_file" << EOF
//! Test for $command_name commands
//!
//! This file contains tests for the $command_name commands implemented in the xvim editor.

use std::path::Path;
use std::fs;
use std::io::Write;

// Import the necessary modules from the xvim crate
use xvim::command::{ExCommand, ExCommandRegistry, ExCommandParser};
use xvim::editor::Editor;
use xvim::command::handlers;
use xvim::insert::InsertFunctions;

/// Test the $command_name functionality
fn test_${command_name}_basic() {
    println!("Testing $command_name basic functionality...");
    
    // Create an editor instance
    let mut editor = Editor::new().unwrap();
    
    // Set the editor reference in the handlers
    handlers::set_editor(&mut editor);
    
    // Create a buffer with some content
    let buffer_id = editor.get_buffer_manager_mut().create_buffer().unwrap();
    editor.get_buffer_manager_mut().set_current_buffer(buffer_id).unwrap();
    editor.insert_text("Test content for $command_name test\\n").unwrap();
    
    // Create a command registry
    let mut registry = ExCommandRegistry::new();
    handlers::register_handlers(&mut registry, None);
    
    // Create a command parser
    let parser = ExCommandParser::new();
    
    // Execute the command
    // TODO: Replace with actual command
    let cmd = parser.parse("$command_name").unwrap();
    let result = registry.execute(&cmd);
    
    // Check the result
    assert!(result.is_ok(), "Failed to execute $command_name command: {:?}", result);
    
    println!("  $command_name basic test passed");
}

/// Run all tests
fn main() {
    println!("Running $command_name tests...");
    
    test_${command_name}_basic();
    
    println!("All $command_name tests passed!");
}
EOF
    
    echo "Created test file for $command_name: $test_file"
}

# Function to generate a test file for a mode
generate_mode_test() {
    local mode_name=$1
    local test_file="$TEST_DIR/${mode_name}_test.rs"
    
    # Skip if test file already exists
    if [ -f "$test_file" ]; then
        echo "Test file for $mode_name already exists: $test_file"
        return
    fi
    
    # Create test file
    cat > "$test_file" << EOF
//! Test for $mode_name mode
//!
//! This file contains tests for the $mode_name mode implemented in the xvim editor.

use xvim::editor::Editor;
use xvim::mode::Mode;
use xvim::insert::InsertFunctions;

/// Test basic $mode_name mode functionality
fn test_${mode_name}_mode_basic() {
    println!("Testing $mode_name mode basic functionality...");
    
    // Create an editor instance
    let mut editor = Editor::new().unwrap();
    
    // Create a buffer
    let buffer_id = editor.get_buffer_manager_mut().create_buffer().unwrap();
    editor.get_buffer_manager_mut().set_current_buffer(buffer_id).unwrap();
    
    // Insert some text
    editor.insert_text("Test content for $mode_name mode test\\n").unwrap();
    
    // TODO: Add mode-specific tests
    
    println!("  $mode_name mode basic test passed");
}

/// Run all tests
fn main() {
    println!("Running $mode_name mode tests...");
    
    test_${mode_name}_mode_basic();
    
    println!("All $mode_name mode tests passed!");
}
EOF
    
    echo "Created test file for $mode_name mode: $test_file"
}

# Function to update Cargo.toml with new test binaries
update_cargo_toml() {
    local test_files=("$@")
    local cargo_toml="Cargo.toml"
    
    # Create a temporary file
    local temp_file=$(mktemp)
    
    # Copy the content up to the last bin section
    awk '/\[\[bin\]\]/{last=$0; lastline=NR; next} {if(lastline && lastline==NR-1) {lastline=0} else {print}}' "$cargo_toml" > "$temp_file"
    
    # Add the bin sections for the test files
    for test_file in "${test_files[@]}"; do
        local test_name=$(basename "$test_file" .rs)
        echo -e "\n[[bin]]" >> "$temp_file"
        echo "name = \"$test_name\"" >> "$temp_file"
        echo "path = \"$test_file\"" >> "$temp_file"
    done
    
    # Replace the original file
    mv "$temp_file" "$cargo_toml"
    
    echo "Updated $cargo_toml with new test binaries"
}

# Generate tests for commands
echo "Generating tests for commands..."
commands=("normal" "insert" "visual" "operator" "text_object" "mark" "register" "macro" "search" "syntax")
for cmd in "${commands[@]}"; do
    generate_command_test "$cmd"
done

# Generate tests for modes
echo "Generating tests for modes..."
modes=("normal" "insert" "visual" "command")
for mode in "${modes[@]}"; do
    generate_mode_test "$mode"
done

# Find all test files
test_files=($(find "$TEST_DIR" -name "*_test.rs" -not -path "*/\.*"))

# Update Cargo.toml with new test binaries
update_cargo_toml "${test_files[@]}"

echo "Test generation complete!"