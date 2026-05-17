# Frontend Development Guide

The frontend is built with Leptos and compiled to WASM using Trunk.

## Adding a New Component

To add a new UI component to the library:

1. Create a new Rust file in `src/components/`.
2. Define a Leptos component using the `#[component]` attribute.
3. Implement the view logic using the `view!` macro.
4. Import and integrate the component into `src/components/main_content.rs`.
5. Assign a unique ID to the component in the backend registry to make it discoverable.

## Styling

Styling is managed through a global `styles.css` file. The project uses CSS variables for consistent branding:
- `--primary`: Main accent color.
- `--secondary-bg`: Background for panels and sidebars.
- `--border-color`: Standard border color.

## State Management

State is managed using Leptos signals. Use `signal()` for local component state and `create_memo()` for derived state to optimize performance.

## Build Process

The project uses Trunk for bundling. Run `trunk build` to compile the WASM frontend and generate the distribution files in the `dist/` folder.
