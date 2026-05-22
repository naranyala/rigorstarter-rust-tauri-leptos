use leptos::prelude::*;

#[component]
pub fn TooltipDemo() -> impl IntoView {
    view! {
        <div class="tooltip-demo-container">
            <p>
                "Hover over the " <span class="tooltip-trigger" style="text-decoration: underline; cursor: help;">"underlined text"</span> " to see a tooltip."
            </p>
            <style>
                ".tooltip-trigger { position: relative; }"
                ".tooltip-trigger:hover::after {
                    content: 'This is a tooltip!';
                    position: absolute;
                    bottom: 125%;
                    left: 50%;
                    transform: translateX(-50%);
                    background-color: #333;
                    color: white;
                    padding: 4px 8px;
                    border-radius: 4px;
                    font-size: 0.75rem;
                    white-space: nowrap;
                    z-index: 10;
                }"
            </style>
        </div>
    }
}
