#!/usr/bin/env bash

# Remove frontend build artifacts to ensure fresh frontend build
rm -rf dist

# Start development mode (uses Cargo cache for backend)
npx @tauri-apps/cli dev

