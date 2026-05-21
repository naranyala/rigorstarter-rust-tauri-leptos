use leptos::prelude::*;
use web_sys::Storage;

/// A hook for synchronizing a signal with browser localStorage.
pub fn use_storage<T>(key: &'static str, default_value: T) -> RwSignal<T>
where
    T: 'static + Clone + serde::Serialize + for<'de> serde::Deserialize<'de>,
{
    let window = web_sys::window().expect("no global `window` exists");
    let storage = window
        .local_storage()
        .ok()
        .flatten()
        .expect("local storage not available");

    let initial_value = storage
        .get_item(key)
        .ok()
        .flatten()
        .and_then(|v| serde_json::from_str::<T>(&v).ok())
        .unwrap_or(default_value);

    let signal = RwSignal::new(initial_value);

    // Effect to sync changes from the signal to localStorage
    Effect::new(move |_| {
        let val = signal.get();
        if let Ok(serialized) = serde_json::to_string(&val) {
            let _ = storage.set_item(key, &serialized);
        }
    });

    signal
}
