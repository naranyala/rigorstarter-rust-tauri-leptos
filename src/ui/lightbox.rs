use crate::services::event_bus::AppEvent;
use crate::services::AppFileSystem;
use crate::ui::layout::PageLayout;
use leptos::prelude::*;
use std::path::PathBuf;
use std::sync::Arc;
use wasm_bindgen_futures::spawn_local;

#[component]
pub fn Lightbox() -> impl IntoView {
    let fs = use_context::<Arc<AppFileSystem>>().expect("AppFileSystem context missing");

    let (selected_dir, set_selected_dir) = signal(None::<PathBuf>);
    let (images, set_images) = signal(Vec::<PathBuf>::new());
    let (current_index, set_current_index) = signal(None::<usize>);
    let (current_image_data, set_current_image_data) = signal(None::<String>);
    let (is_lightbox_open, set_is_lightbox_open) = signal(false);
    let (new_folder_name, set_new_folder_name) = signal(String::new());

    let fs_clone = fs.clone();

    // Folder picker
    let fs_pick = fs_clone.clone();
    let pick_folder = Callback::new(move |_| {
        let fs = fs_pick.clone();
        let images = images;

        spawn_local(async move {
            // Mocking folder selection for WASM compatibility
            let path = PathBuf::from("/mock/images");
            set_selected_dir.set(Some(path.clone()));

            match fs.list_dir(&path).await {
                Ok(entries) => {
                    let image_files: Vec<PathBuf> = entries
                        .into_iter()
                        .filter(|p| {
                            p.extension()
                                .and_then(|s| s.to_str())
                                .map(|ext| {
                                    ["jpg", "jpeg", "png", "gif", "webp"]
                                        .contains(&ext.to_lowercase().as_str())
                                })
                                .unwrap_or(false)
                        })
                        .collect();

                    set_images.set(image_files);

                    if !images.get().is_empty() {
                        let first_image = images.get()[0].clone();
                        if let Ok(bytes) = fs.read_file(&first_image).await {
                            let data_uri = crate::shared::utils::encoding::to_image_data_uri(
                                &bytes,
                                first_image
                                    .extension()
                                    .and_then(|s| s.to_str())
                                    .unwrap_or("jpg"),
                            );
                            set_current_image_data.set(Some(data_uri));
                            set_current_index.set(Some(0));
                            set_is_lightbox_open.set(true);
                        }
                    }
                }
                Err(e) => {
                    let _ = crate::services::event_bus::EventBus::publish(
                        "error",
                        AppEvent::Error(format!("Failed to list directory: {:?}", e)),
                    )
                    .await;
                }
            }
        });
    });

    // Create folder
    let fs_create = fs_clone.clone();
    let create_folder = Callback::new(move |_| {
        let fs = fs_create.clone();
        let new_folder_name = new_folder_name;

        spawn_local(async move {
            if let Some(base_dir) = selected_dir.get() {
                let name = new_folder_name.get();
                if name.is_empty() {
                    return;
                }
                let new_path = base_dir.join(name);
                if let Err(e) = fs.create_dir_all(&new_path).await {
                    let _ = crate::services::event_bus::EventBus::publish(
                        "error",
                        AppEvent::Error(format!("Failed to create folder: {}", e)),
                    )
                    .await;
                } else {
                    set_new_folder_name.set(String::new());
                }
            }
        });
    });

    // Navigation helpers
    let fs_next = fs_clone.clone();
    let next_image = Callback::new(move |_| {
        let images_val = images.get();
        if let Some(idx) = current_index.get() {
            let len = images_val.len();
            if len > 0 {
                let new_idx = (idx + 1) % len;
                set_current_index.set(Some(new_idx));
                let p = images_val[new_idx].clone();
                let fs = fs_next.clone();

                spawn_local(async move {
                    match fs.read_file(&p).await {
                        Ok(bytes) => {
                            let data_uri = crate::shared::utils::encoding::to_image_data_uri(
                                &bytes,
                                p.extension().and_then(|s| s.to_str()).unwrap_or("jpg"),
                            );
                            set_current_image_data.set(Some(data_uri));
                            set_is_lightbox_open.set(true);
                        }
                        Err(_) => {
                            let _ = crate::services::event_bus::EventBus::publish(
                                "error",
                                AppEvent::Error(format!("Failed to read image: {:?}", p)),
                            )
                            .await;
                        }
                    }
                });
            }
        }
    });

    let fs_prev = fs_clone.clone();
    let prev_image = Callback::new(move |_| {
        let images_val = images.get();
        if let Some(idx) = current_index.get() {
            let len = images_val.len();
            if len > 0 {
                let new_idx = if idx == 0 { len - 1 } else { idx - 1 };
                set_current_index.set(Some(new_idx));
                let p = images_val[new_idx].clone();
                let fs = fs_prev.clone();

                spawn_local(async move {
                    match fs.read_file(&p).await {
                        Ok(bytes) => {
                            let data_uri = crate::shared::utils::encoding::to_image_data_uri(
                                &bytes,
                                p.extension().and_then(|s| s.to_str()).unwrap_or("jpg"),
                            );
                            set_current_image_data.set(Some(data_uri));
                            set_is_lightbox_open.set(true);
                        }
                        Err(_) => {
                            let _ = crate::services::event_bus::EventBus::publish(
                                "error",
                                AppEvent::Error(format!("Failed to read image: {:?}", p)),
                            )
                            .await;
                        }
                    }
                });
            }
        }
    });

    view! {
        <PageLayout>
            <h1>"Lightbox Image Viewer"</h1>

            {move || if selected_dir.get().is_none() {
                view! {
                    <div style="display:flex;flex-direction:column;align-items:center;justify-content:center;margin-top:100px;gap:20px;">
                        <p>"No directory selected. Please pick a folder containing images."</p>
                        <button class="btn-primary"
                            style="padding:15px 30px;font-size:1.1rem;cursor:pointer;"
                            on:click=move |_| pick_folder.run(())>
                            "📁 Select Image Folder"
                        </button>
                    </div>
                }.into_any()
            } else {
                view! {
                    <div style="display:flex;gap:20px;width:100%;max-width:1400px;align-items:flex-start;">
                        <div style="flex:1;display:flex;flex-direction:column;gap:20px;">
                            <div style="display:flex;justify-content:space-between;align-items:center;margin-bottom:20px;">
                                <span style="font-size:0.9rem;color:var(--text-secondary);">
                                    {format!("Viewing: {}", selected_dir.get().unwrap().display())}
                                </span>
                                <button class="btn-danger"
                                    style="padding:8px 15px;cursor:pointer;"
                                    on:click=move |_| {
                                        set_selected_dir.set(None);
                                        set_images.set(Vec::new());
                                        set_current_index.set(None);
                                        set_current_image_data.set(None);
                                        set_is_lightbox_open.set(false);
                                    }>
                                    "Change Folder"
                                </button>
                            </div>

                            {move || if images.get().is_empty() {
                                view! { <div style="text-align:center;color:var(--text-secondary);">"No supported image files found in this folder."</div> }.into_any()
                            } else {
                                view! { <div style="text-align:center;padding:40px;border:2px dashed var(--border-color);border-radius:var(--radius-md);color:var(--text-secondary);">"Image viewer is active. Use navigation arrows or pick a folder to start."</div> }.into_any()
                            }}.into_any()
                        </div>

                        <div style="width:300px;display:flex;flex-direction:column;gap:15px;">
                            <crate::ui::layout::Surface outline=true padding=true>
                                <div style="display:flex;flex-direction:column;gap:10px;">
                                    <h3 style="margin:0;font-size:1rem;color:var(--text-primary);">"New Subfolder"</h3>
                                    <input
                                        style="padding:8px;border:1px solid var(--border-color);border-radius:var(--radius-sm);background:var(--bg-color);color:var(--text-primary);"
                                        placeholder="Folder name..."
                                        prop:value=new_folder_name
                                        on:input=move |ev| set_new_folder_name.set(event_target_value(&ev))
                                    />
                                    <button class="btn-primary"
                                        style="padding:8px;cursor:pointer;"
                                        on:click=move |_| create_folder.run(())>
                                        "Create Folder"
                                    </button>
                                </div>
                            </crate::ui::layout::Surface>
                        </div>
                    </div>
                }.into_any()
            }}

            {move || if is_lightbox_open.get() {
                view! {
                    <div
                        style="position:fixed;top:0;left:0;width:100vw;height:100vh;background:rgba(0,0,0,0.9);display:flex;align-items:center;justify-content:center;z-index:2000;cursor:zoom-out;"
                        tabindex=0
                        on:keydown=move |ev| {
                            match ev.key().as_str() {
                                "ArrowLeft" => prev_image.run(()),
                                "ArrowRight" => next_image.run(()),
                                "Escape" => {
                                    set_is_lightbox_open.set(false);
                                    set_current_image_data.set(None);
                                    set_current_index.set(None);
                                }
                                _ => {}
                            }
                        }
                        on:click=move |_| {
                            set_is_lightbox_open.set(false);
                            set_current_image_data.set(None);
                            set_current_index.set(None);
                        }
                    >
                        <img
                            style="max-width:90%;max-height:90%;object-fit:contain;box-shadow:0 0 20px rgba(0,0,0,0.5);"
                            src={move || current_image_data.get().unwrap_or_default()}
                        />

                        <button
                            style="position:absolute;left:20px;top:50%;transform:translateY(-50%);background:rgba(255,255,255,0.2);border:none;border-radius:50%;width:50px;height:50px;cursor:pointer;font-size:2rem;color:white;display:flex;align-items:center;justify-content:center;transition:background 0.2s;"
                            on:click=move |ev| {
                                ev.stop_propagation();
                                prev_image.run(());
                            }
                        >
                            "‹"
                        </button>

                        <button
                            style="position:absolute;right:20px;top:50%;transform:translateY(-50%);background:rgba(255,255,255,0.2);border:none;border-radius:50%;width:50px;height:50px;cursor:pointer;font-size:2rem;color:white;display:flex;align-items:center;justify-content:center;transition:background 0.2s;"
                            on:click=move |ev| {
                                ev.stop_propagation();
                                next_image.run(());
                            }
                        >
                            "›"
                        </button>

                        <button
                            style="position:absolute;top:20px;right:20px;background:white;border:none;border-radius:50%;width:40px;height:40px;cursor:pointer;font-size:1.5rem;font-weight:bold;"
                            on:click=move |_| {
                                set_is_lightbox_open.set(false);
                                set_current_image_data.set(None);
                                set_current_index.set(None);
                            }
                        >
                            "×"
                        </button>
                    </div>
                }.into_any()
            } else {
                view! { <div /> }.into_any()
            }}
        </PageLayout>
    }
}
