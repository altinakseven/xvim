#!/bin/bash

# Script to build the project and check for errors
# This script will build the project and report any errors or warnings

# Exit on error
set -e

# Print what we're doing
set -x

# Define colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[0;33m'
NC='\033[0m' # No Color

# Function to build the project
build_project() {
    echo -e "${GREEN}Building project...${NC}"
    
    # Capture the output of cargo build
    local build_output=$(cargo build 2>&1)
    local build_status=$?
    
    # Count errors and warnings
    local error_count=$(echo "$build_output" | grep -c "error:")
    local warning_count=$(echo "$build_output" | grep -c "warning:")
    
    # Print the build output
    echo "$build_output"
    
    # Print summary
    echo -e "\n${GREEN}Build Summary:${NC}"
    if [ $build_status -eq 0 ]; then
        echo -e "${GREEN}Build successful!${NC}"
    else
        echo -e "${RED}Build failed!${NC}"
    fi
    
    echo -e "${RED}Errors: $error_count${NC}"
    echo -e "${YELLOW}Warnings: $warning_count${NC}"
    
    # Return the build status
    return $build_status
}

# Function to check for common issues
check_common_issues() {
    echo -e "\n${GREEN}Checking for common issues...${NC}"
    
    # Check for unused variables
    local unused_vars=$(cargo build 2>&1 | grep -c "unused variable")
    echo -e "${YELLOW}Unused variables: $unused_vars${NC}"
    
    # Check for unused imports
    local unused_imports=$(cargo build 2>&1 | grep -c "unused import")
    echo -e "${YELLOW}Unused imports: $unused_imports${NC}"
    
    # Check for unused mut
    local unused_mut=$(cargo build 2>&1 | grep -c "variable does not need to be mutable")
    echo -e "${YELLOW}Unused mut: $unused_mut${NC}"
    
    # Check for unreachable patterns
    local unreachable_patterns=$(cargo build 2>&1 | grep -c "unreachable pattern")
    echo -e "${YELLOW}Unreachable patterns: $unreachable_patterns${NC}"
    
    # Check for unused assignments
    local unused_assignments=$(cargo build 2>&1 | grep -c "value assigned to .* is never read")
    echo -e "${YELLOW}Unused assignments: $unused_assignments${NC}"
    
    # Total issues
    local total_issues=$((unused_vars + unused_imports + unused_mut + unreachable_patterns + unused_assignments))
    echo -e "${YELLOW}Total issues: $total_issues${NC}"
    
    # Return non-zero exit code if there are issues
    if [ $total_issues -gt 0 ]; then
        return 1
    fi
    
    return 0
}

# Build the project
build_project

# Check for common issues
check_common_issues