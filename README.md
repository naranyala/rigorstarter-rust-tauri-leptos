# Rigorstarter Rust Tauri Leptos

A professional starter template for building high-performance desktop applications using Tauri and Leptos.

## Technology Stack

- Tauri: Framework for secure and lightweight desktop applications with a Rust backend.
- Leptos: High-performance Rust web framework for the frontend.
- Trunk: WASM bundler for the Leptos frontend.

## Project Features

- MVVM Architecture: Separation of concerns between Core logic, Services, and Views.
- Integrated Registry: Dynamic discovery of components and system utilities.
- Source Code Viewer: Ability to view the Rust source of system utilities directly in the UI.
- Responsive Design: Modern UI with full light/dark theme support.
- Secure Backend: Validated file system access and command handling.

## Getting Started

### Development

To start the application in development mode:

```bash
sh run.sh
```

**Note on HTML Generation**: This project uses `index.html.default` as the source of truth for the frontend entry point. The `run.sh` script calls `scripts/generate_css_imports.sh` to programmatically scan the `styles/` directory and inject all CSS `<link>` tags into `index.html`. Please make any structural HTML changes in `index.html.default` rather than `index.html`.

### Production Build

To build the application for production:

```bash
npx @tauri-apps/cli build
```

## Project Structure

- `src/core/`: Domain models and pure business logic.
- `src/services/`: Stateful services and dependency injection.
- `src/ui/`: UI views and components (flattened structure).
- `src-tauri/`: Tauri backend Rust code and configuration.
- `docs/`: Comprehensive project documentation.
- `Trunk.toml`: Configuration for the Trunk WASM bundler.
- `index.html.default`: Source template for the frontend entry point.
- `index.html`: Generated entry point (Do not edit manually).

## Documentation

Detailed guides are available in the `docs/` directory:
- Architecture: Overview of the MVVM system design.
- Frontend: Guide for creating themed components.
- Backend: Guide for adding commands and utilities.
- Stdlib: Guide to reusable UI primitives.
- Testing: Strategy for ensuring app stability.
- Deployment: Build and distribution instructions.

## Contributing

To contribute to the project:
1. Clone the repository.
2. Install dependencies (Rust, Node.js, Tauri CLI).
3. Run the development environment:
   ```bash
   sh run.sh
   ```
4. Run the test suite:
   ```bash
   cargo test
   ```
