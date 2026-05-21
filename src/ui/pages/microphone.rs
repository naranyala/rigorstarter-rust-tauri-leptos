use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::{MediaRecorder, MediaStream, MediaStreamConstraints};

#[component]
pub fn MicrophoneDemo() -> impl IntoView {
    let (is_recording, set_is_recording) = signal(false);
    let (audio_url, set_audio_url) = signal(None::<String>);
    let (status, set_status) = signal("Idle".to_string());

    let recorder = StoredValue::new(None::<MediaRecorder>);
    let stream_ref = StoredValue::new(None::<MediaStream>);
    let chunks_ref = StoredValue::new(Vec::<web_sys::Blob>::new());

    let start_recording = move |_| {
        set_is_recording.set(true);
        set_status.set("Requesting microphone access...".to_string());
        set_audio_url.set(None);
        chunks_ref.set_value(Vec::new());

        spawn_local(async move {
            let window = web_sys::window().unwrap();
            let navigator = window.navigator();

            let constraints = MediaStreamConstraints::new();
            constraints.set_audio(&wasm_bindgen::JsValue::from_bool(true));

            let media_devices = match navigator.media_devices() {
                Ok(md) => md,
                Err(_) => {
                    set_status.set("Error: Could not access media devices.".to_string());
                    set_is_recording.set(false);
                    return;
                }
            };

            let result = match media_devices.get_user_media_with_constraints(&constraints) {
                Ok(promise) => wasm_bindgen_futures::JsFuture::from(promise).await,
                Err(e) => {
                    set_status.set(format!("Error: {:?}", e));
                    set_is_recording.set(false);
                    return;
                }
            };

            match result {
                Ok(stream_js) => {
                    let stream: MediaStream = stream_js.dyn_into().unwrap();
                    stream_ref.set_value(Some(stream.clone()));
                    let media_recorder = MediaRecorder::new_with_media_stream(&stream).unwrap();

                    // Handle data available event
                    let chunks_clone = chunks_ref;
                    let on_data_available =
                        Closure::wrap(Box::new(move |e: web_sys::MessageEvent| {
                            if let Ok(blob) = e.data().dyn_into::<web_sys::Blob>() {
                                let mut chunks = chunks_clone.get_value();
                                chunks.push(blob);
                                chunks_clone.set_value(chunks);
                            }
                        })
                            as Box<dyn FnMut(web_sys::MessageEvent)>);

                    media_recorder
                        .set_ondataavailable(Some(on_data_available.as_ref().unchecked_ref()));
                    on_data_available.forget(); // Keep the closure alive

                    // Handle stop event
                    let on_stop = Closure::wrap(Box::new(move |_e: web_sys::Event| {
                        let chunks = chunks_ref.get_value();
                        if !chunks.is_empty() {
                            let blob = web_sys::Blob::new_with_u8_array_sequence(
                                &js_sys::Array::from_iter(
                                    chunks.iter().map(|c| JsValue::from(c.clone())),
                                ),
                            )
                            .unwrap();
                            let url = web_sys::Url::create_object_url_with_blob(&blob).unwrap();
                            set_audio_url.set(Some(url));
                        }
                        set_status.set("Recording finished.".to_string());
                        set_is_recording.set(false);
                    })
                        as Box<dyn FnMut(web_sys::Event)>);

                    media_recorder.set_onstop(Some(on_stop.as_ref().unchecked_ref()));
                    on_stop.forget(); // Keep the closure alive

                    let _ = media_recorder.start();
                    recorder.set_value(Some(media_recorder));
                    set_status.set("Recording...".to_string());
                }
                Err(e) => {
                    set_status.set(format!("Error: {:?}", e));
                    set_is_recording.set(false);
                }
            };
        });
    };

    let stop_recording = move |_| {
        if let Some(recorder) = recorder.get_value() {
            let _ = recorder.stop();
        }
        if let Some(stream) = stream_ref.get_value() {
            let tracks = stream.get_tracks();
            for i in 0..tracks.length() {
                let track_js = tracks.get(i);
                if let Ok(track) = track_js.dyn_into::<web_sys::MediaStreamTrack>() {
                    track.stop();
                }
            }
        }
        set_status.set("Stopping...".to_string());
    };

    view! {
        <div class="microphone-demo-container">
            <h2>"Microphone Recorder Demo"</h2>
            <p class="status-text">"Status: " {move || status.get()}</p>

            <div class="controls">
                <button
                    class="btn-start"
                    on:click=start_recording
                    disabled=is_recording
                >
                    "Start Recording"
                </button>

                <button
                    class="btn-stop"
                    on:click=stop_recording
                    disabled=move || !is_recording.get()
                >
                    "Stop Recording"
                </button>
            </div>

            <div class="playback-area">
                {move || if let Some(url) = audio_url.get() {
                    view! {
                        <div class="audio-result">
                            <p>"Playback:"</p>
                            <audio controls src=url></audio>
                        </div>
                    }.into_any()
                } else {
                    view! { <p class="empty-msg">"No recording available."</p> }.into_any()
                }}
            </div>
        </div>
    }
}
