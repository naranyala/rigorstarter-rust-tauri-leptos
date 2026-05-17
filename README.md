# Rigorstarter Rust Tauri Leptos

A starter template for building high-performance desktop applications using Tauri and Leptos.

## Tech Stack

- Tauri 2.0: Framework for building secure and lightweight desktop applications with a Rust backend.
- Leptos 0.7: A modern, high-performance Rust web framework for the frontend.
- Trunk: WASM bundler used to manage the Leptos frontend build process.

## Prerequisites

Before you begin, ensure you have the following installed:

- Rust: Install via rustup.
- Node.js: Required for the Tauri CLI.
- Trunk: Install using `cargo install trunk`.
- WASM Target: Add the target using `rustup target add wasm32-unknown-unknown`.
- System Dependencies: Follow the official Tauri installation guide for your operating system.

## Getting Started

### Development

To start the application in development mode, run:

```bash
sh run.sh
```

Alternatively, you can use the Tauri CLI directly:

```bash
npx @tauri-apps/cli dev
```

This command will:
1. Start the Leptos frontend using `trunk serve`.
2. Launch the Tauri desktop window.

### Production Build

To build the application for production:

```bash
npx @tauri-apps/cli build
```

## Project Structure

- `src/`: Contains the Leptos frontend source code.
- `src-tauri/`: Contains the Tauri backend Rust code and configuration.
- `Trunk.toml`: Configuration for the Trunk WASM bundler.
- `src-tauri/tauri.conf.json`: Configuration for the Tauri application.
- `index.html`: Entry point for the Leptos frontend.

## Recommended IDE Setup

- Visual Studio Code
- Tauri Extension
- rust-analyzer
