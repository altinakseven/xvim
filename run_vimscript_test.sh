#!/bin/bash

# Build and run the vimscript_test example

# Ensure we're in the xvim directory
cd "$(dirname "$0")"

# Build the example
echo "Building vimscript_test example..."
cargo build --example vimscript_test

# Run the example
echo "Running vimscript_test example..."
cargo run --example vimscript_test

echo "Done!"