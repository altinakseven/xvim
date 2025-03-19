#!/bin/bash

# Script to fix common warnings in the xvim codebase
# This script will add underscores to unused variables and remove unused imports

# Exit on error
set -e

# Print what we're doing
set -x

# Function to fix unused variables by adding underscores
fix_unused_variables() {
    local file=$1
    
    # Get a list of unused variables from the compiler output
    unused_vars=$(cargo build 2>&1 | grep -A 1 "unused variable" | grep "help: if this is intentional, prefix it with an underscore" | sed -E "s/.*\`([^_][^']*)\`.*/\1/g" | sort | uniq)
    
    # Add underscores to unused variables
    for var in $unused_vars; do
        # Skip empty variables
        if [ -z "$var" ]; then
            continue
        fi
        
        # Replace the variable name with an underscore prefix
        find src -type f -name "*.rs" -exec sed -i "s/\b$var\b/_$var/g" {} \;
    done
}

# Function to fix unused imports by commenting them out
fix_unused_imports() {
    local file=$1
    
    # Get a list of unused imports from the compiler output
    unused_imports=$(cargo build 2>&1 | grep "unused import" | sed -E "s/.*\`([^']*)\`.*/\1/g" | sort | uniq)
    
    # Comment out unused imports
    for import in $unused_imports; do
        # Skip empty imports
        if [ -z "$import" ]; then
            continue
        fi
        
        # Comment out the import
        find src -type f -name "*.rs" -exec sed -i "s/use $import;/\/\/ use $import;/g" {} \;
    done
}

# Function to fix unused mutable variables
fix_unused_mut() {
    local file=$1
    
    # Get a list of unused mutable variables from the compiler output
    unused_muts=$(cargo build 2>&1 | grep -A 1 "variable does not need to be mutable" | grep "help: remove this" | sed -E "s/.*\`mut ([^']*)\`.*/\1/g" | sort | uniq)
    
    # Remove the mut keyword from unused mutable variables
    for var in $unused_muts; do
        # Skip empty variables
        if [ -z "$var" ]; then
            continue
        fi
        
        # Replace "mut var" with just "var"
        find src -type f -name "*.rs" -exec sed -i "s/mut $var/$var/g" {} \;
    done
}

# Fix unused variables
fix_unused_variables

# Fix unused imports
fix_unused_imports

# Fix unused mutable variables
fix_unused_mut

# Run cargo build again to see if we've fixed the issues
cargo build

echo "Fixed common warnings in the codebase!"