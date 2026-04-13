#!/bin/bash

# Run all contract tests
echo "Running contract tests..."
cargo test --workspace

if [ $? -eq 0 ]; then
    echo "✅ All tests passed!"
else
    echo "❌ Tests failed!"
    exit 1
fi
