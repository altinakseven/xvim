#!/bin/bash

# Run the simple test
echo "Running Ex command tests..."
cargo run --bin ex_commands_test

# Run the search test
echo "Running search tests..."
cargo test --test search_test

# Run the visual mode test
echo "Running visual mode tests..."
cargo test --test visual_test

# Run the insert mode test
echo "Running insert mode tests..."
cargo test --test insert_test

echo "All tests completed!"