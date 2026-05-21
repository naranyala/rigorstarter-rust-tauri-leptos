use crate::ui::pages::*;
use crate::ui::welcome::WelcomeDashboard;
use leptos::prelude::*;

#[derive(Clone)]
pub struct PageInfo {
    pub name: &'static str,
    pub id: &'static str,
    pub category: &'static str,
    pub desc: &'static str,
}

pub struct Page {
    pub info: PageInfo,
    pub component: fn() -> AnyView,
}

pub const PAGES: &[Page] = &[
    Page {
        info: PageInfo {
            name: "Welcome",
            id: "welcome",
            category: "General",
            desc: "Main dashboard",
        },
        component: || view! { <WelcomeDashboard /> }.into_any(),
    },
    Page {
        info: PageInfo {
            name: "Accordion",
            id: "accordion",
            category: "Components",
            desc: "Collapsible content panels",
        },
        component: || view! { <AccordionDemo /> }.into_any(),
    },
    Page {
        info: PageInfo {
            name: "Tabs",
            id: "tabs",
            category: "Components",
            desc: "Tabbed content switcher",
        },
        component: || view! { <TabsDemo /> }.into_any(),
    },
    Page {
        info: PageInfo {
            name: "Drawer",
            id: "drawer",
            category: "Components",
            desc: "Slide-out side panel",
        },
        component: || view! { <DrawerDemo /> }.into_any(),
    },
    Page {
        info: PageInfo {
            name: "Tree View",
            id: "tree_view",
            category: "Components",
            desc: "Nested tree navigation",
        },
        component: || view! { <TreeViewDemo /> }.into_any(),
    },
    Page {
        info: PageInfo {
            name: "Table",
            id: "table_demo",
            category: "Components",
            desc: "Data table with sorting",
        },
        component: || view! { <TableDemo /> }.into_any(),
    },
    Page {
        info: PageInfo {
            name: "Calendar",
            id: "calendar",
            category: "Components",
            desc: "Date picker and calendar view",
        },
        component: || view! { <Calendar /> }.into_any(),
    },
    Page {
        info: PageInfo {
            name: "Image Viewer",
            id: "image_viewer",
            category: "Components",
            desc: "Lightbox image gallery",
        },
        component: || view! { <ImageViewer /> }.into_any(),
    },
    Page {
        info: PageInfo {
            name: "Toast",
            id: "toast_demo",
            category: "Components",
            desc: "Notification toast messages",
        },
        component: || view! { <ToastDemo /> }.into_any(),
    },
    Page {
        info: PageInfo {
            name: "FFI Demo",
            id: "ffi_demo",
            category: "Exploration",
            desc: "Rust FFI interop example",
        },
        component: || view! { <FfiDemo /> }.into_any(),
    },
    Page {
        info: PageInfo {
            name: "Todo Demo",
            id: "todo_demo",
            category: "Exploration",
            desc: "Interactive todo list",
        },
        component: || view! { <TodoDemo /> }.into_any(),
    },
    Page {
        info: PageInfo {
            name: "JSON Todo",
            id: "json_todo",
            category: "Exploration",
            desc: "JSON-serialized todo manager",
        },
        component: || view! { <JsonTodoDemo /> }.into_any(),
    },
    Page {
        info: PageInfo {
            name: "Markdown",
            id: "markdown_demo",
            category: "Exploration",
            desc: "Markdown rendering demo",
        },
        component: || view! { <MarkdownDemo /> }.into_any(),
    },
    Page {
        info: PageInfo {
            name: "Audio Player",
            id: "audio_player",
            category: "Exploration",
            desc: "Audio file playback",
        },
        component: || view! { <AudioPlayerDemoView /> }.into_any(),
    },
    Page {
        info: PageInfo {
            name: "Audio Recorder",
            id: "audio_recorder",
            category: "Exploration",
            desc: "Audio recording with Tauri",
        },
        component: || view! { <AudioRecorderView /> }.into_any(),
    },
    Page {
        info: PageInfo {
            name: "Microphone",
            id: "microphone",
            category: "Exploration",
            desc: "Live microphone input",
        },
        component: || view! { <MicrophoneDemo /> }.into_any(),
    },
];

pub fn filter_pages(query: &str) -> Vec<PageInfo> {
    let q = query.to_lowercase();
    if q.is_empty() {
        return PAGES
            .iter()
            .filter(|p| p.info.id != "welcome")
            .map(|p| p.info.clone())
            .collect();
    }
    PAGES
        .iter()
        .filter(|p| p.info.id != "welcome")
        .filter(|p| {
            p.info.name.to_lowercase().contains(&q)
                || p.info.category.to_lowercase().contains(&q)
                || p.info.desc.to_lowercase().contains(&q)
        })
        .map(|p| p.info.clone())
        .collect()
}

pub fn grouped_pages() -> Vec<(&'static str, Vec<&'static PageInfo>)> {
    let mut groups: Vec<(&str, Vec<&'static PageInfo>)> = Vec::new();
    for page in PAGES {
        // Skip the welcome page for sidebar grouping
        if page.info.id == "welcome" {
            continue;
        }
        if let Some((_, list)) = groups.iter_mut().find(|(c, _)| *c == page.info.category) {
            list.push(&page.info);
        } else {
            groups.push((page.info.category, vec![&page.info]));
        }
    }
    groups
}
