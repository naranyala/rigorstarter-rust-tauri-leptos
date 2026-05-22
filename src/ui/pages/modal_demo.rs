use leptos::prelude::*;

#[component]
pub fn ModalDemo() -> impl IntoView {
    let (is_open, set_is_open) = signal(false);

    view! {
        <div class="modal-demo-container">
            <button class="btn-primary" on:click=move |_| set_is_open.set(true)>
                "Open Modal"
            </button>

            {move || if is_open.get() {
                view! {
                    <div class="modal-overlay" on:click=move |_| set_is_open.set(false)>
                        <div class="modal-content" on:click=|ev| ev.stop_propagation()>
                            <h2>"Modal Title"</h2>
                            <p>"This is a simple modal demonstration."</p>
                            <button class="btn-secondary" on:click=move |_| set_is_open.set(false)>
                                "Close"
                            </button>
                        </div>
                    </div>
                }.into_any()
            } else {
                view! { <div /> }.into_any()
            }}
        </div>
    }
}
