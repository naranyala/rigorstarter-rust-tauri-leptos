use crate::ui::stdlib::components::card::Card;
use crate::ui::stdlib::hooks::use_window_size::use_window_size;
use crate::ui::stdlib::layout::stack::Stack;
use crate::ui::stdlib::layout::stack::StackDirection;
use leptos::prelude::*;

#[component]
fn MetricCard(label: String, value: String, color: String) -> impl IntoView {
    view! {
        <Card class="metric-card".to_string()>
            <div style="display: flex; flex-direction: column; gap: 0.5rem;">
                <span style="font-size: 0.8rem; color: var(--text-muted);">{label}</span>
                <span style=move || format!("font-size: 1.5rem; font-weight: 700; color: {};", color)>{value}</span>
            </div>
        </Card>
    }
}

#[component]
pub fn DashboardDemo() -> impl IntoView {
    let window_size = use_window_size();

    let direction = Memo::new(move |_| {
        if window_size.get().width < 768 {
            StackDirection::Vertical
        } else {
            StackDirection::Horizontal
        }
    });

    let gap = Memo::new(move |_| {
        if window_size.get().width < 768 {
            "1rem".to_string()
        } else {
            "2rem".to_string()
        }
    });

    view! {
        <div class="dashboard-demo" style="padding: 2rem;">
            <div style="margin-bottom: 2rem;">
                <h2>"Responsive System Dashboard"</h2>
                <p style="color: var(--text-muted);">
                    "This demo showcases the responsive layout using use_window_size and Stack. Watch the layout shift as you resize the window."
                </p>
                <div style="font-size: 0.8rem; background: var(--secondary-bg); padding: 0.5rem; border-radius: 4px; display: inline-block; margin-top: 0.5rem;">
                    "Current Width: " {move || window_size.get().width} "px"
                </div>
            </div>

            <Stack
                direction=direction
                gap=gap
            >
                <MetricCard label="CPU Usage".to_string() value="24%".to_string() color="var(--primary)".to_string() />
                <MetricCard label="Memory".to_string() value="4.2 GB / 16 GB".to_string() color="#4caf50".to_string() />
                <MetricCard label="Disk I/O".to_string() value="12 MB/s".to_string() color="#ff9800".to_string() />
                <MetricCard label="Network".to_string() value="120 Mbps".to_string() color="#2196f3".to_string() />
            </Stack>
        </div>
    }
}
