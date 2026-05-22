# Frontend Development Guide

The frontend is built with Leptos, compiled to WebAssembly (WASM) via Trunk.

## Component Architecture

The application follows a service-driven component model.

### Creating a Page
1. Implement the view in `src/ui/pages/` as a standalone `.rs` file.
2. Register the page in `src/ui/page_registry.rs` to make it discoverable by the sidebar.
3. Inject required services via `use_context::<ServiceType>()`.
4. Use reactive signals from services to drive the UI.

### Using the Stdlib
The `src/ui/stdlib/` directory contains reusable UI primitives:
- **Components**: Standardized buttons, cards, and navigation elements.
- **Hooks**: Custom reactive logic (e.g., `use_storage` for persistence, `use_window_size` for responsiveness).
- **Layouts**: Higher-order components for structuring views (e.g., `Stack`).

## Styling and Theming

The project uses a CSS-variable based design system located in `src/styles/`.

### Theming System
- **Light Mode**: Default variables defined in `:root`.
- **Dark Mode**: Overriding variables defined in the `.dark` class.
- **Implementation**: The `ThemeService` toggles the `.dark` class on the `body` element.

### Standard Variables
- `--primary`: Accent color for buttons and links.
- `--bg-color`: Main application background.
- `--text-main`: Primary text color.
- `--border-color`: Subtle borders for panels.

## State Management

State is handled through a layered approach:
- **Local State**: Use `signal()` within components for ephemeral UI state.
- **Global State**: Use `services/` for application-wide state, provided via Context.
- **Derived State**: Use `Memo` or closures to compute values from signals.

## Build Pipeline

1. **Trunk**: Bundles WASM and assets.
2. **Optimization**: `wasm-opt` is used to reduce binary size.
3. **Deployment**: The resulting `dist/` folder is bundled by Tauri into the native installer.
