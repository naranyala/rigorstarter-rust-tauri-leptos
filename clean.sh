#!/usr/bin/env bash
set -e

echo "🧹 Cleaning all build artifacts..."
rm -rf dist
rm -rf target
rm -rf src-tauri/target

echo "✅ Clean complete. Your next build will be a completely fresh one."
