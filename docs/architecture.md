# System Architecture

This project is a desktop application built with a Rust-based stack, combining Tauri for the windowing and system access and Leptos for the user interface.

## High-Level Design

The application follows a client-server model where the frontend (client) runs in a webview and the backend (server) runs as a native process.

### Frontend (Leptos)
The frontend is written in Rust and compiled to WebAssembly (WASM). It uses the Leptos framework for reactive UI components. The frontend handles:
- Routing and view management.
- State management for the UI.
- Invoking Tauri commands for system-level operations.

### Backend (Tauri)
The backend is a native Rust process. It manages the application lifecycle and provides access to the underlying operating system. The backend handles:
- System utility execution.
- File system access for source code retrieval.
- Window management and system tray integration.
- Communication with the frontend via the Tauri command system.

## Data Flow

1. Frontend requests data (e.g., a list of components) by invoking a Tauri command.
2. Backend processes the request, interacting with the file system or internal state.
3. Backend returns a serialized JSON response.
4. Frontend deserializes the response into Rust structs and updates the reactive UI.

## Registry System

A central registry maps human-readable names to unique identifiers. This allows the application to decouple the UI labels from the actual file paths or component IDs on disk.
