#!/bin/bash

# Build all contracts
echo "Building contracts..."
stellar contract build

# Check if build was successful
if [ $? -eq 0 ]; then
    echo "✅ Contracts built successfully!"
    echo ""
    echo "WASM files generated:"
    ls -lh target/wasm32-unknown-unknown/release/*.wasm
else
    echo "❌ Build failed!"
    exit 1
fi
