# Backend Development Guide

The backend is built with Tauri and provides native system capabilities.

## Adding a New Tauri Command

To expose new functionality to the frontend:

1. Define a function in `src-tauri/src/lib.rs` (or a separate module).
2. Annotate the function with `#[tauri::command]`.
3. Register the command in the `run()` function using the `invoke_handler` macro:
   ```rust
   .invoke_handler(tauri::generate_handler![your_command_name])
   ```
4. Call the command from the frontend using the `invoke` function.

## Adding System Utilities

System utilities are standalone Rust files that provide specific functionality.

1. Create a new Rust file in `src-tauri/src/utils/`.
2. Implement the utility logic.
3. Add the utility to the registry in `get_registry()` within `src-tauri/src/lib.rs`, ensuring the ID matches the filename.

## Error Handling

The backend uses a custom `AppError` enum to handle various failure states. Always return `Result<T, AppError>` from commands to ensure errors are correctly propagated to the frontend.

## File System Access

Access to the file system is restricted for security. All file paths are validated to prevent directory traversal attacks.
