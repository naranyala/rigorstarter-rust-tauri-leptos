#[derive(Clone)]
pub struct PageInfo {
    pub name: &'static str,
    pub id: &'static str,
    pub category: &'static str,
    pub desc: &'static str,
}

pub const PAGES: &[PageInfo] = &[
    PageInfo {
        name: "Dashboard",
        id: "dashboard",
        category: "Components",
        desc: "Pinned and featured component overview",
    },
    PageInfo {
        name: "Accordion",
        id: "accordion",
        category: "Components",
        desc: "Collapsible content panels",
    },
    PageInfo {
        name: "Tabs",
        id: "tabs",
        category: "Components",
        desc: "Tabbed content switcher",
    },
    PageInfo {
        name: "Drawer",
        id: "drawer",
        category: "Components",
        desc: "Slide-out side panel",
    },
    PageInfo {
        name: "Tree View",
        id: "tree_view",
        category: "Components",
        desc: "Nested tree navigation",
    },
    PageInfo {
        name: "Table",
        id: "table_demo",
        category: "Components",
        desc: "Data table with sorting",
    },
    PageInfo {
        name: "Calendar",
        id: "calendar",
        category: "Components",
        desc: "Date picker and calendar view",
    },
    PageInfo {
        name: "Image Viewer",
        id: "image_viewer",
        category: "Components",
        desc: "Lightbox image gallery",
    },
    PageInfo {
        name: "Theme Demo",
        id: "theme_demo",
        category: "Components",
        desc: "Dark mode and theme showcase",
    },
    PageInfo {
        name: "Toast",
        id: "toast_demo",
        category: "Components",
        desc: "Notification toast messages",
    },
    PageInfo {
        name: "FFI Demo",
        id: "ffi_demo",
        category: "Demos",
        desc: "Rust FFI interop example",
    },
    PageInfo {
        name: "Todo Demo",
        id: "todo_demo",
        category: "Demos",
        desc: "Interactive todo list",
    },
    PageInfo {
        name: "JSON Todo",
        id: "json_todo",
        category: "Demos",
        desc: "JSON-serialized todo manager",
    },
    PageInfo {
        name: "Markdown",
        id: "markdown_demo",
        category: "Demos",
        desc: "Markdown rendering demo",
    },
    PageInfo {
        name: "Audio Player",
        id: "audio_player",
        category: "Media",
        desc: "Audio file playback",
    },
    PageInfo {
        name: "Audio Recorder",
        id: "audio_recorder",
        category: "Media",
        desc: "Audio recording with Tauri",
    },
    PageInfo {
        name: "Microphone",
        id: "microphone",
        category: "Media",
        desc: "Live microphone input",
    },
];

pub fn filter_pages(query: &str) -> Vec<PageInfo> {
    let q = query.to_lowercase();
    if q.is_empty() {
        return PAGES.to_vec();
    }
    PAGES
        .iter()
        .filter(|p| {
            p.name.to_lowercase().contains(&q)
                || p.category.to_lowercase().contains(&q)
                || p.desc.to_lowercase().contains(&q)
        })
        .cloned()
        .collect()
}

pub fn grouped_pages() -> Vec<(&'static str, Vec<&'static PageInfo>)> {
    let mut groups: Vec<(&str, Vec<&'static PageInfo>)> = Vec::new();
    for page in PAGES {
        if let Some((_, list)) = groups.iter_mut().find(|(c, _)| *c == page.category) {
            list.push(page);
        } else {
            groups.push((page.category, vec![page]));
        }
    }
    groups
}
