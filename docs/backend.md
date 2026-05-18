# Backend Development Guide

The backend is powered by Tauri and provides the native system bridge for the application.

## Command Implementation

Tauri commands allow the frontend to trigger native Rust code.

### Adding a Command
1. Define a function in `src-tauri/src/lib.rs` or a specific module.
2. Annotate with `#[tauri::command]`.
3. Register the command in the `invoke_handler`:
   ```rust
   .invoke_handler(tauri::generate_handler![your_command_name])
   ```

## System Utilities

Utilities are the core functional units of the backend.

### Implementation Workflow
1. Create a Rust module in `src-tauri/src/utils/`.
2. Implement the functional logic.
3. Register the utility in the `get_registry()` function in `lib.rs`, mapping a unique ID to the utility.

## Error Handling Strategy

The backend utilizes a unified error handling system:
- All commands return `Result<T, AppError>`.
- `AppError` is a custom enum that implements `serde::Serialize`, allowing errors to be passed directly to the frontend as JSON.
- Detailed error messages are propagated to the UI for developer debugging.

## Security and File System

To prevent security vulnerabilities:
- All file system access is strictly validated.
- Path traversal checks are implemented to ensure the application cannot access files outside its designated scope.
- Minimal privileges are requested from the operating system.
