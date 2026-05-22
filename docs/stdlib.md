# UI Stdlib Guide

The `src/ui/stdlib/` directory provides a set of reusable, low-level UI primitives that ensure consistency across the application.

## Components

### Button
A versatile button component with support for various styles and sizes.
- **Variants**: `Primary`, `Secondary`, `Outline`, `Ghost`, `Danger`.
- **Sizes**: `Small`, `Medium`, `Large`.

### Card
A standardized container for grouping related content with a consistent background and border.

### NavCategory
A collapsible category header used in the sidebar, featuring an integrated item count that reacts to search filters.

## Hooks

### `use_storage<T>`
Synchronizes a Leptos signal with the browser's `localStorage`.
- **Behavior**: Loads the initial value from storage on mount; updates storage automatically whenever the signal changes.
- **Requirements**: `T` must implement `Serialize`, `Deserialize`, `Clone`, `Send`, and `Sync`.

### `use_window_size`
Provides a reactive `WindowSize` struct containing current `width` and `height`.
- **Behavior**: Listens to the window `resize` event and updates the signal.

## Layouts

### Stack
A flexbox-based layout component for arranging children vertically or horizontally.
- **Direction**: `Vertical` or `Horizontal`.
- **Gap**: Customizable spacing between items.
