#!/bin/bash

# Script to fix unused variables in the xvim codebase
# This script will add underscores to unused variables

# Exit on error
set -e

# Function to fix unused variables in a file
fix_unused_variables() {
    local file=$1
    local output_file=$(mktemp)
    
    # Get the unused variables from the compiler output
    cargo build 2>&1 | grep "unused variable" | grep "$file" | while read -r line; do
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
}

# Find all Rust files in the src directory
find src -name "*.rs" | while read -r file; do
    fix_unused_variables "$file"
done

echo "Fixed unused variables in the codebase!"