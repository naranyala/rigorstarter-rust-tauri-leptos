# Testing Strategy

This project employs a multi-layered testing strategy to ensure stability across the frontend and backend.

## Core Logic Tests
Located in `src/core/logic.rs`, these are pure Rust tests that validate business rules without any UI dependencies.
- **Focus**: Filtering, sorting, and data transformation.
- **Edge Cases**: Empty registries, special characters, and large datasets.

## Service Tests
Located in `src/services/mod.rs` and `src/tests/mod.rs`, these tests validate the application's state management.
- **Approach**: Uses a `test_runtime()` (a minimal Leptos `Owner`) to simulate the reactive environment.
- **Coverage**: State transitions, context provider/consumer patterns, and service orchestration.

## Component Tests
Located in `src/tests/component_tests.rs`, these tests use `wasm-bindgen-test` to verify UI behavior in a browser environment.
- **Focus**: Interaction logic, visibility toggles, and reactive updates.

## Model Tests
Located in `src/core/models.rs`, these tests ensure that data structures are correctly serialized and deserialized (round-trip testing).

## Running Tests
- **Standard Tests**: `cargo test`
- **WASM Tests**: `wasm-pack test --chrome --headless`
