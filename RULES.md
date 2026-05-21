# Development Rules & Guidelines

This document outlines the patterns and constraints for the Rigorstarter Rust-Tauri-Leptos project. Follow these rules to avoid common pitfalls related to WASM, Leptos reactivity, Tauri integration, and cross-platform compatibility.

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

### 6. Solving Rendering Stalls & UI Lag
- **The Problem:** In some environments, state changes may occur, but the UI doesn't repaint immediately (the app feels "frozen" until a global event like a theme toggle occurs).
- **The Fix (The "Wake Up" Pattern):**
  1. **Lazy Rendering:** Never render all pages and hide them with `display: none`. Only render the active component to keep the DOM lean.
  2. **Schedule Updates:** For critical UI transitions (like navigation), wrap the state update in `requestAnimationFrame` to sync with the browser's paint cycle.
     - ✅ `requestAnimationFrame(|_| active_page.set(Some(id)))`
  3. **Force Reflow:** If the UI still lags, use a "forced reflow" in an `Effect` to kick the renderer.
     - ✅ `Effect::new(move |_| { active_page.get(); let _ = document.body().offset_height(); })`

---

## 🛠️ Backend (Tauri)

### 1. Command Design
- **Error Handling:** All Tauri commands should return a `Result<T, AppError>` to ensure frontend errors are caught and handled.
- **Data Transfer:** Use `serde_wasm_bindgen` for efficient conversion between Rust types and JS values.
- **Single Responsibility:** Commands should perform a single task. If a command is doing complex logic (like registry construction + file reading), split it into smaller functions.

### 2. File System Access & Portability
- **Avoid Hardcoded Relative Paths:** Never use hardcoded paths relative to the project root (e.g., `src/utils/...`) in Tauri commands, as these paths will not exist in a distributed package.
- **Platform Abstraction:** Avoid direct access to OS-specific filesystems (like `/proc` on Linux). Use crates like `sysinfo` or `dirs` to achieve cross-platform compatibility.
- **Path Resolution:** Use `tauri::AppHandle` or `tauri::PathResolver` to resolve paths to application data or configuration files.

### 3. System Architecture
- **Modular Initialization:** Avoid "God functions" in `lib.rs`. Break down the `run()` function into smaller, testable setup modules (e.g., `db::init`, `menu::setup`, `tray::setup`).
- **FFI Safety:** Minimize use of `unsafe` and manual FFI. Prefer high-level, safe Rust wrappers for system calls.

---

## 🐞 Debugging Tips

- **Terminal Logging:** Use the `log_message` Tauri command to print frontend logs to the Rust terminal. 
- **Example:** `spawn_local(async move { log_to_terminal("Message").await; })`.
- **Browser Console:** Always check the browser's developer console for WASM panics or failed `invoke` calls.

## 📦 Build Process
- Use `./run.sh` for development.
- If the build fails with strange errors after a major change, try `rm -rf dist` to clear frontend artifacts.
