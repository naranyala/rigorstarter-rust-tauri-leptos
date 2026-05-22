use crate::ui::stdlib::hooks::use_storage::use_storage;
use leptos::prelude::*;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_use_storage_initialization() {
    let owner = Owner::new();
    owner.set();

    let key = "test_storage_key";
    let default = 42;
    let signal = use_storage(key, default);

    assert_eq!(signal.get(), 42);
}

#[wasm_bindgen_test]
fn test_use_storage_persistence() {
    let owner = Owner::new();
    owner.set();

    let key = "test_persistence_key";
    let default = 10;

    // First run: set value
    let signal1 = use_storage(key, default);
    signal1.set(20);

    // In a real browser, this would be in localStorage.
    // We would need to reload the page or create a new hook with the same key.
}
