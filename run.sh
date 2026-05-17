#!/usr/bin/env bash

# Ensure we are in the project root
set -e

echo "Checking frontend build health..."
# Run a fast build check without serving
if ! trunk build --quiet; then
    echo "------------------------------------------------------------"
    echo "❌ FRONTEND BUILD FAILED"
    echo "Please fix the Rust compilation errors in the terminal above."
    echo "------------------------------------------------------------"
    exit 1
fi

echo "✅ Frontend build healthy. Starting development mode..."
rm -rf dist
npx @tauri-apps/cli dev

