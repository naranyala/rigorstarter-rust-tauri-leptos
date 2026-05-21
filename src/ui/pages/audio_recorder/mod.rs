use crate::services::audio::AudioViewModel;
use js_sys::Promise;
use leptos::prelude::*;
use std::path::PathBuf;
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen_futures::JsFuture;

async fn sleep(ms: i32) {
    let promise = Promise::new(&mut |resolve, _| {
        web_sys::window()
            .unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, ms)
            .unwrap();
    });
    let _ = JsFuture::from(promise).await;
}

#[component]
fn RecordingItem(path: PathBuf, on_play: Callback<PathBuf>) -> impl IntoView {
    let path_for_text = path.clone();
    let path_for_click = path.clone();
    view! {
        <div style="display:flex;justify-content:space-between;align-items:center;padding:10px;background:var(--surface-color);border:1px solid var(--border-color);border-radius:var(--radius-sm);">
            <span style="font-size:0.9rem;">
                {move || {
                    let p = path_for_text.clone();
                    p.file_name().unwrap().to_string_lossy().to_string()
                }}
            </span>
            <button
                class="btn-play"
                style="padding:5px 10px;font-size:0.8rem;"
                on:click=move |_| on_play.run(path_for_click.clone())
            >
                "Play"
            </button>
        </div>
    }
}

#[component]
pub fn AudioRecorderView() -> impl IntoView {
    let vm = use_context::<AudioViewModel>().expect("AudioViewModel missing");
    let state = vm.state;

    // Recording tick
    {
        let vm_rec = vm;
        Effect::new(move |_| {
            let v = vm_rec;
            spawn_local(async move {
                loop {
                    sleep(100).await;
                    v.tick_recording(0.1);
                }
            });
        });
    }

    // Playback tick
    {
        let vm_play = vm;
        Effect::new(move |_| {
            let v = vm_play;
            spawn_local(async move {
                loop {
                    sleep(100).await;
                    v.tick_playback(0.1);
                }
            });
        });
    }

    let on_record_toggle = move |_| {
        let is_rec = vm.state.get().is_recording;
        if is_rec {
            vm.stop_recording();
        } else {
            vm.start_recording();
        }
    };

    view! {
        <div class="audio-recorder-container" style="max-width:800px;padding:20px;">
            <h2>"Audio Recorder Demo"</h2>

            <div style="display:flex;flex-direction:column;align-items:center;gap:15px;margin-bottom:30px;">
                <div style="font-size:2rem;font-weight:bold;font-family:monospace;">
                    {move || format!("{:.1}s", state.get().recording_duration)}
                </div>
                <p class="status">{move || state.get().status.clone()}</p>

                <div class="controls">
                    <button
                        class={move || if state.get().is_recording { "btn-stop" } else { "btn-record" }}
                        on:click=on_record_toggle
                    >
                        {move || if state.get().is_recording { "Stop Recording" } else { "Start Recording" }}
                    </button>
                </div>
            </div>

            {move || if state.get().is_playing {
                let s = state.get();
                view! {
                    <div style="background:var(--surface-color);padding:20px;border-radius:var(--radius-md);border:1px solid var(--border-color);margin-bottom:20px;">
                        <div style="display:flex;justify-content:space-between;font-size:0.8rem;margin-bottom:5px;">
                            <span>{format!("{:.1}s", s.playback_progress)}</span>
                            <span>{format!("{:.1}s", s.current_playback_duration)}</span>
                        </div>
                        <div style="width:100%;height:8px;background:var(--bg-color);border-radius:4px;overflow:hidden;">
                            <div
                                style={format!(
                                    "height:100%;background:var(--accent-color);transition:width 0.1s linear;width:{:.1}%;",
                                    (s.playback_progress / s.current_playback_duration * 100.0).clamp(0.0, 100.0)
                                )}
                            ></div>
                        </div>
                    </div>
                }.into_any()
            } else {
                view! { <div /> }.into_any()
            }}

            <div style="display:flex;flex-direction:column;gap:10px;">
                <h3 style="font-size:1.1rem;margin-bottom:5px;">"Saved Recordings"</h3>
                {move || {
                    let s = state.get();
                    if s.recordings.is_empty() {
                        view! { <p style="color:var(--text-secondary);font-style:italic;">"No recordings yet."</p> }.into_any()
                    } else {
                        view! {
                            <div style="display:flex;flex-direction:column;gap:8px;">
                                {s.recordings.iter().map(|path| {
                                    view! {
                                        <RecordingItem
                                            path={path.clone().into()}
                                            on_play=Callback::new(move |p| vm.play_file(p))
                                        />
                                    }
                                }).collect::<Vec<_>>()}
                            </div>
                        }.into_any()
                    }
                }}
            </div>
        </div>
    }
}
