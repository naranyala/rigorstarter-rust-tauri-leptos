# Development Rules & Guidelines

This document outlines the patterns and constraints for the Rigorstarter Rust-Tauri-Leptos project. Follow these rules to avoid common pitfalls related to WASM, Leptos reactivity, and Tauri integration.

## 🚀 Frontend (Leptos 0.7)

### 1. Signal Management
- **State:** Use `signal()` for local state. Pass `ReadSignal` and `WriteSignal` to child components to maintain a single source of truth.
- **Updates:** Use `.set()` for direct updates and `.update(|v| ...)` for updates based on the current value.

### 2. Reactivity & Rendering
- **Conditional Rendering:** Prefer the `<Show />` component over `match` expressions that return `.into_any()`.
- **Avoiding Flicker:** Avoid returning `into_any()` inside a reactive closure in the root `App` component. This often causes the entire subtree to unmount and remount, leading to flickering and "endless re-renders."
- **Component Isolation:** Move complex switching logic (like the main content area) into a separate component (e.g., `MainContent`) to isolate re-renders and maintain component identity.

### 3. Callbacks
- **Invocation:** `Callback<T>` is a struct, not a function. To trigger it, you **must** use the `.run(value)` method.
  - ❌ `on_click(())`
  - ✅ `on_click.run(())`

### 4. Side Effects & Async
- **Effects:** Never trigger side effects (like `spawn_local` or DOM manipulation) directly inside a `view!` render closure. This can cause unstable rendering or runtime panics.
- **Effect Placement:** Place side effects inside `Effect::new(move |_| { ... })`.
- **Async Calls:** Use `spawn_local` for any `async` block (like Tauri `invoke` calls) to ensure they run on the WASM event loop.

### 5. Iterators & Lifetimes
- **Owned Data:** When mapping over signals (e.g., `registry.get().iter()`), avoid returning references (`&str`) in the resulting collection.
- **The Fix:** Use `.clone()` or `.to_string()` to return owned `String`s. Returning references to temporary values created during `.get()` will cause compilation errors (`E0515`).

---

## 🛠️ Backend (Tauri)

### 1. Command Design
- **Error Handling:** All Tauri commands should return a `Result<T, AppError>` to ensure frontend errors are caught and handled.
- **Data Transfer:** Use `serde_wasm_bindgen` for efficient conversion between Rust types and JS values.

### 2. File System Access
- **Path Accuracy:** When mapping files for the registry, ensure paths are absolute relative to the project root. 
- **Component Mapping:** Components should map to `src/components/{id}.rs`, and utilities to `src-tauri/src/utils/{id}.rs`.

---

## 🐞 Debugging Tips

- **Terminal Logging:** Use the `log_message` Tauri command to print frontend logs to the Rust terminal. 
- **Example:** `spawn_local(async move { log_to_terminal("Message").await; })`.
- **Browser Console:** Always check the browser's developer console for WASM panics or failed `invoke` calls.

## 📦 Build Process
- Use `./run.sh` for development.
- If the build fails with strange errors after a major change, try `rm -rf dist` to clear frontend artifacts.
