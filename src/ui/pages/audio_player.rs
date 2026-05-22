use crate::services::audio::AudioViewModel;
use leptos::prelude::*;
use std::path::PathBuf;
use wasm_bindgen_futures::spawn_local;

#[component]
pub fn AudioPlayerDemoView() -> impl IntoView {
    let vm = use_context::<AudioViewModel>().expect("AudioViewModel missing");
    let state = vm.state;

    let vm_play_latest = vm;
    let vm_select_file = vm;

    view! {
        <div style="padding:20px;max-width:600px;margin:0 auto;display:flex;flex-direction:column;gap:20px;font-family:sans-serif;">
            <h1>"Mini Audio Player"</h1>

            <div style="height:100px;display:flex;align-items:flex-end;gap:4px;background-color:var(--surface-color);padding:10px;border-radius:4px;border:1px solid var(--border-color);overflow:hidden;">
                {move || {
                    let is_playing = state.get().is_playing;
                    (0..20).map(|i| {
                        let h = if is_playing { 20.0 + (i as f32 * 3.0) } else { 10.0 };
                        view! {
                            <div
                                style={format!("flex:1;background-color:var(--accent-color);height:{}%;transition:height 0.1s ease-out;", h)}
                            ></div>
                        }
                    }).collect::<Vec<_>>()
                }}
            </div>

            <div style="display:flex;gap:10px;justify-content:center;">
                <button
                    on:click=move |_| {
                        let vm = vm_play_latest;
                        let s = vm.state.get();
                        let playing = s.is_playing;
                        let last = s.recordings.last().cloned();
                        if playing {
                            vm.state.update(|s| s.is_playing = false);
                        } else if let Some(path) = last {
                            vm.play_file(path.into());
                        }
                    }
                    style="padding:10px 20px;background-color:var(--accent-color);color:white;border:none;border-radius:4px;cursor:pointer;"
                >
                    {move || if state.get().is_playing { "Stop".to_string() } else { "Play Latest".to_string() }}
                </button>

                <button
                    on:click=move |_| {
                        let vm = vm_select_file;
                        spawn_local(async move {
                            // Mocking file selection for WASM compatibility
                            let path = PathBuf::from("/mock/audio/recording.wav");
                            vm.play_file(path);
                        });
                    }
                    style="padding:10px 20px;background-color:var(--text-secondary);color:white;border:none;border-radius:4px;cursor:pointer;"
                >
                    "Select Audio File"
                </button>
            </div>

            <div style="padding:10px;background-color:var(--surface-color);border-radius:4px;border:1px solid var(--border-color);min-height:40px;text-align:center;">
                {move || state.get().status.clone()}
            </div>
        </div>
    }
}
