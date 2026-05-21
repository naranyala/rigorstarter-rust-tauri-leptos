use leptos::prelude::*;

#[component]
pub fn DrawerDemo() -> impl IntoView {
    let (is_open, set_is_open) = signal(false);
    view! {
        <div class="drawer-demo">
            <h2>"Drawer Demo"</h2>
            <button class="open-drawer-btn" on:click=move |_| set_is_open.set(true)>"Open Drawer"</button>
            <div class="drawer-overlay" style:display=move || if is_open.get() { "flex" } else { "none" } on:click=move |_| set_is_open.set(false)>
                <div class="drawer-panel" on:click=|ev| ev.stop_propagation() class:open=move || is_open.get()>
                    <div class="drawer-header">
                        <h3>"Settings"</h3>
                        <button class="close-drawer-btn" on:click=move |_| set_is_open.set(false)>"✕"</button>
                    </div>
                    <div class="drawer-body"><p>"Drawer content goes here."</p></div>
                </div>
            </div>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_runtime() -> Owner {
        let owner = Owner::new();
        owner.set();
        owner
    }

    #[test]
    fn test_drawer_toggle_logic() {
        let _rt = setup_runtime();
        let (is_open, set_is_open) = signal(false);

        assert!(!is_open.get());
        set_is_open.set(true);
        assert!(is_open.get());
        set_is_open.set(false);
        assert!(!is_open.get());
    }
}
