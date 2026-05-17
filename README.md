# Rigorstarter Rust Tauri Leptos

A professional starter template for building high-performance desktop applications using Tauri and Leptos.

## Technology Stack

- Tauri: Framework for secure and lightweight desktop applications with a Rust backend.
- Leptos: High-performance Rust web framework for the frontend.
- Trunk: WASM bundler for the Leptos frontend.

## Project Features

- Integrated Registry: Dynamic discovery of components and system utilities.
- Source Code Viewer: Ability to view the Rust source of system utilities directly in the UI.
- Responsive Design: Modern UI with a focus on developer experience.
- Secure Backend: Validated file system access and command handling.

## Getting Started

### Development

To start the application in development mode:

```bash
sh run.sh
```

### Production Build

To build the application for production:

```bash
npx @tauri-apps/cli build
```

## Project Structure

- `src/`: Leptos frontend source code.
- `src-tauri/`: Tauri backend Rust code and configuration.
- `docs/`: Comprehensive project documentation.
- `Trunk.toml`: Configuration for the Trunk WASM bundler.
- `index.html`: Entry point for the frontend.

## Documentation

Detailed guides are available in the `docs/` directory:
- Architecture: Overview of the system design.
- Frontend: Guide for adding components.
- Backend: Guide for adding commands and utilities.
- Deployment: Build and distribution instructions.
