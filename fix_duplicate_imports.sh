#!/bin/bash

# Script to fix duplicate imports in the xvim codebase
# This script will remove duplicate imports and fix syntax errors

# Exit on error
set -e

# Print what we're doing
set -x

# Function to fix duplicate imports in a file
fix_duplicate_imports() {
    local file=$1
    
    # Create a temporary file
    local temp_file=$(mktemp)
    
    # Check if the file has duplicate imports
    if grep -q "error\[E0252\]: the name .* is defined multiple times" "$file"; then
        # Get the list of imports
        local imports=$(grep "^use " "$file" | sort | uniq)
        
        # Remove all imports
        sed -i '/^use /d' "$file"
        
        # Add unique imports at the beginning of the file
        for import in $imports; do
            sed -i "1i $import" "$file"
        done
        
        echo "Fixed duplicate imports in $file"
    fi
}

# Function to fix syntax errors in a file
fix_syntax_errors() {
    local file=$1
    
    # Check if the file has syntax errors with use statements
    if grep -q "expected identifier, found keyword \`use\`" "$file" || grep -q "expected one of \`,\`, \`::\`, \`as\`, or \`}\`, found \`std\`" "$file"; then
        # Get the content of the file
        local content=$(cat "$file")
        
        # Create a new file with fixed imports
        local temp_file=$(mktemp)
        
        # Extract all imports
        local imports=$(grep "^use " "$file" | sort | uniq)
        
        # Remove all imports
        echo "$content" | grep -v "^use " > "$temp_file"
        
        # Add unique imports at the beginning of the file
        for import in $imports; do
            sed -i "1i $import" "$temp_file"
        done
        
        # Replace the original file
        mv "$temp_file" "$file"
        
        echo "Fixed syntax errors in $file"
    fi
}

# Function to fix all files with duplicate imports or syntax errors
fix_all_files() {
    # Get the build output with errors
    local temp_file=$(mktemp)
    cargo build 2>&1 > "$temp_file"
    
    # Extract all files with duplicate imports
    grep "error\[E0252\]: the name .* is defined multiple times" "$temp_file" | sed -E 's/.*src\/([^:]+):.*/src\/\1/g' | sort | uniq | while read -r file; do
        # Skip if file doesn't exist
        if [ ! -f "$file" ]; then
            continue
        fi
        
        fix_duplicate_imports "$file"
    done
    
    # Extract all files with syntax errors
    grep "expected identifier, found keyword \`use\`" "$temp_file" | sed -E 's/.*src\/([^:]+):.*/src\/\1/g' | sort | uniq | while read -r file; do
        # Skip if file doesn't exist
        if [ ! -f "$file" ]; then
            continue
        fi
        
        fix_syntax_errors "$file"
    done
    
    # Clean up
    rm "$temp_file"
}

# Fix all files with duplicate imports or syntax errors
fix_all_files

# Run cargo build to see if we've fixed the issues
cargo build

echo "Fixed duplicate imports and syntax errors in the codebase!"