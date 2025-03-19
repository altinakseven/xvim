#!/bin/bash

# Script to run tests for the commands and implementations integrated from vim
# This script will run all the test files in the tests directory

# Exit on error
set -e

# Print what we're doing
set -x

# Define colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Function to run a test file
run_test() {
    local test_file=$1
    local test_name=$(basename "$test_file" .rs)
    
    echo -e "${GREEN}Running test: $test_name${NC}"
    
    # Run the test
    if cargo test --test "$test_name"; then
        echo -e "${GREEN}Test $test_name passed!${NC}"
        return 0
    else
        echo -e "${RED}Test $test_name failed!${NC}"
        return 1
    fi
}

# Function to run all tests
run_all_tests() {
    local failed_tests=()
    local passed_tests=()
    
    # Find all test files in the tests directory
    find tests -name "*_test.rs" | while read -r test_file; do
        if run_test "$test_file"; then
            passed_tests+=("$(basename "$test_file" .rs)")
        else
            failed_tests+=("$(basename "$test_file" .rs)")
        fi
    done
    
    # Print summary
    echo -e "\n${GREEN}Test Summary:${NC}"
    echo -e "${GREEN}Passed tests: ${#passed_tests[@]}${NC}"
    for test in "${passed_tests[@]}"; do
        echo -e "${GREEN}- $test${NC}"
    done
    
    echo -e "${RED}Failed tests: ${#failed_tests[@]}${NC}"
    for test in "${failed_tests[@]}"; do
        echo -e "${RED}- $test${NC}"
    done
    
    # Return non-zero exit code if any tests failed
    if [ ${#failed_tests[@]} -gt 0 ]; then
        return 1
    fi
    
    return 0
}

# Run all tests
run_all_tests