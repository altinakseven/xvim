#!/bin/bash

# Script to fix unused variables in the xvim codebase
# This script will add underscores to unused variables

# Exit on error
set -e

# Print what we're doing
set -x

# Function to add underscores to unused variables in a file
fix_unused_vars() {
    local file=$1
    local temp_file=$(mktemp)
    
    # Get the build output with warnings
    cargo build 2>&1 > "$temp_file"
    
    # Extract unused variable warnings for this file
    grep "warning: unused variable" "$temp_file" | grep "$file" | while read -r line; do
        # Extract the variable name and line number
        var_name=$(echo "$line" | sed -E "s/.*\`([^']*)\`.*/\1/g")
        line_num=$(echo "$line" | grep -o -E '[0-9]+:[0-9]+' | cut -d':' -f1)
        
        # Skip if variable name is empty
        if [ -z "$var_name" ]; then
            continue
        fi
        
        # Add underscore to the variable name in the file
        sed -i "${line_num}s/\b${var_name}\b/_${var_name}/g" "$file"
        
        echo "Fixed unused variable '$var_name' in $file:$line_num"
    done
    
    # Clean up
    rm "$temp_file"
}

# Function to fix all unused variables in the codebase
fix_all_unused_vars() {
    # Get the build output with warnings
    local temp_file=$(mktemp)
    cargo build 2>&1 > "$temp_file"
    
    # Extract all files with unused variable warnings
    grep "warning: unused variable" "$temp_file" | sed -E 's/.*src\/([^:]+):.*/src\/\1/g' | sort | uniq | while read -r file; do
        # Skip if file doesn't exist
        if [ ! -f "$file" ]; then
            continue
        fi
        
        fix_unused_vars "$file"
    done
    
    # Clean up
    rm "$temp_file"
}

# Fix all unused variables
fix_all_unused_vars

# Run cargo build to see if we've fixed the issues
cargo build

echo "Fixed unused variables in the codebase!"