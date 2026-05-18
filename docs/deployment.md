# Deployment and Distribution

This guide details the process of building and distributing the application for production.

## Production Build Process

To create a production-ready installer, run:

```bash
npx @tauri-apps/cli build
```

### Build Steps
1. **WASM Optimization**: The Leptos frontend is compiled and optimized using `wasm-opt`.
2. **Native Compilation**: The Rust backend is compiled with release optimizations (`--release`).
3. **Bundling**: Tauri packages the binary and assets into a platform-specific installer (MSI for Windows, DMG for macOS, DEB for Linux).

## Configuration

Application metadata and window settings are managed in `src-tauri/tauri.conf.json`:
- **Identifier**: Unique bundle ID for the OS.
- **Window**: Initial size, transparency, and resizable properties.
- **Bundle**: Versioning and application icons.

## Environment Requirements

### Build Machine
- Rust toolchain (latest stable).
- Node.js and npm/yarn.
- Platform-specific build tools (e.g., MSVC for Windows, Xcode for macOS, build-essential for Linux).

### Target Machine
The generated installers are standalone and do not require a Rust environment on the user's machine.
