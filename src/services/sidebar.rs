use leptos::prelude::*;

#[derive(Clone, Copy)]
pub struct SidebarService {
    pub is_open: ReadSignal<bool>,
    pub set_is_open: WriteSignal<bool>,
}

impl SidebarService {
    pub fn new() -> Self {
        let (is_open, set_is_open) = signal(false);
        Self {
            is_open,
            set_is_open,
        }
    }

    pub fn toggle_sidebar(&self) {
        self.set_is_open.update(|v| *v = !*v);
    }
}
