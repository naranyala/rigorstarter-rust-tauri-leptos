use leptos::prelude::*;

#[component]
fn AccordionItem(title: String, children: Children) -> impl IntoView {
    let (is_open, set_is_open) = signal(false);
    let toggle = move || set_is_open.update(|v| *v = !*v);

    view! {
        <div class="accordion-item">
            <button class="accordion-header" on:click=move |_| toggle()>
                {title}
                <span class="accordion-icon">{move || if is_open.get() { "−" } else { "+" }}</span>
            </button>
            <div style:display=move || if is_open.get() { "block" } else { "none" }>
                <div class="accordion-body">{children()}</div>
            </div>
        </div>
    }
}

#[component]
pub fn AccordionDemo() -> impl IntoView {
    view! {
        <div class="accordion-demo">
            <h2 style="color: var(--text-main); margin-bottom: 1.5rem;">"Accordion Demo"</h2>
            <AccordionItem title="Section 1".to_string()>
                <p>"This is the content for section 1."</p>
            </AccordionItem>
            <AccordionItem title="Section 2".to_string()>
                <p>"This is the content for section 2."</p>
            </AccordionItem>
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
    fn test_accordion_toggle_logic() {
        let _rt = setup_runtime();
        let (is_open, set_is_open) = signal(false);

        let toggle = move || set_is_open.update(|v| *v = !*v);

        assert!(!is_open.get());
        toggle();
        assert!(is_open.get());
        toggle();
        assert!(!is_open.get());
    }
}
