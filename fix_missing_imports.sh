#!/bin/bash

# Script to fix missing imports in the xvim codebase
# This script will add missing imports for common types like HashMap and Arc

# Exit on error
set -e

# Print what we're doing
set -x

# Function to add missing imports to a file
add_missing_imports() {
    local file=$1
    
    # Check if the file needs HashMap import
    if grep -q "HashMap" "$file" && ! grep -q "use std::collections::HashMap" "$file"; then
        # Add the import after the last use statement or at the beginning of the file
        if grep -q "^use " "$file"; then
            sed -i '/^use /a use std::collections::HashMap;' "$file"
        else
            sed -i '1i use std::collections::HashMap;' "$file"
        fi
        echo "Added HashMap import to $file"
    fi
    
    # Check if the file needs Arc import
    if grep -q "Arc" "$file" && ! grep -q "use std::sync::Arc" "$file"; then
        # Add the import after the last use statement or at the beginning of the file
        if grep -q "^use " "$file"; then
            sed -i '/^use /a use std::sync::Arc;' "$file"
        else
            sed -i '1i use std::sync::Arc;' "$file"
        fi
        echo "Added Arc import to $file"
    fi
    
    # Check if the file needs Mutex import
    if grep -q "Mutex" "$file" && ! grep -q "use std::sync::Mutex" "$file"; then
        # Add the import after the last use statement or at the beginning of the file
        if grep -q "^use " "$file"; then
            sed -i '/^use /a use std::sync::Mutex;' "$file"
        else
            sed -i '1i use std::sync::Mutex;' "$file"
        fi
        echo "Added Mutex import to $file"
    fi
    
    # Check if the file needs Path import
    if grep -q "Path" "$file" && ! grep -q "use std::path::Path" "$file"; then
        # Add the import after the last use statement or at the beginning of the file
        if grep -q "^use " "$file"; then
            sed -i '/^use /a use std::path::Path;' "$file"
        else
            sed -i '1i use std::path::Path;' "$file"
        fi
        echo "Added Path import to $file"
    fi
    
    # Check if the file needs PathBuf import
    if grep -q "PathBuf" "$file" && ! grep -q "use std::path::PathBuf" "$file"; then
        # Add the import after the last use statement or at the beginning of the file
        if grep -q "^use " "$file"; then
            sed -i '/^use /a use std::path::PathBuf;' "$file"
        else
            sed -i '1i use std::path::PathBuf;' "$file"
        fi
        echo "Added PathBuf import to $file"
    fi
    
    # Check if the file needs File import
    if grep -q "File" "$file" && ! grep -q "use std::fs::File" "$file"; then
        # Add the import after the last use statement or at the beginning of the file
        if grep -q "^use " "$file"; then
            sed -i '/^use /a use std::fs::File;' "$file"
        else
            sed -i '1i use std::fs::File;' "$file"
        fi
        echo "Added File import to $file"
    fi
    
    # Check if the file needs io imports
    if grep -q "Read" "$file" && ! grep -q "use std::io::Read" "$file"; then
        # Add the import after the last use statement or at the beginning of the file
        if grep -q "^use " "$file"; then
            sed -i '/^use /a use std::io::Read;' "$file"
        else
            sed -i '1i use std::io::Read;' "$file"
        fi
        echo "Added Read import to $file"
    fi
    
    # Check if the file needs Write import
    if grep -q "Write" "$file" && ! grep -q "use std::io::Write" "$file"; then
        # Add the import after the last use statement or at the beginning of the file
        if grep -q "^use " "$file"; then
            sed -i '/^use /a use std::io::Write;' "$file"
        else
            sed -i '1i use std::io::Write;' "$file"
        fi
        echo "Added Write import to $file"
    fi
}

# Find all Rust files in the src directory
find src -name "*.rs" | while read -r file; do
    add_missing_imports "$file"
done

# Run cargo build to see if we've fixed the issues
cargo build

echo "Fixed missing imports in the codebase!"