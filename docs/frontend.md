# Frontend Development Guide

The frontend is built with Leptos, compiled to WebAssembly (WASM) via Trunk.

## Component Architecture

The application follows a service-driven component model.

### Creating a Component
1. Implement the view in `src/components/` using the `#[component]` attribute.
2. Inject required services via `use_context::<ServiceType>()`.
3. Use reactive signals from services to drive the UI.
4. Add the component to the `MainContent` slot for visibility.

## Styling and Theming

The project uses a CSS-variable based design system located in `styles/`.

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
