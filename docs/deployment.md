# Deployment and Distribution

This guide explains how to build and distribute the application.

## Production Build

To generate a production-ready installer:

```bash
npx @tauri-apps/cli build
```

This command performs the following steps:
1. Compiles the Leptos frontend to optimized WASM.
2. Compiles the Rust backend with release optimizations.
3. Bundles all assets into a native installer (e.g., .msi, .dmg, .deb).

## Build Configuration

Tauri configuration is managed in `src-tauri/tauri.conf.json`. Here you can modify:
- Application identifier.
- Window dimensions and properties.
- Bundle icons and versioning.

## Prerequisites for Distribution

Ensure that the target machine has the necessary system libraries installed. Refer to the official Tauri documentation for specific OS requirements.
