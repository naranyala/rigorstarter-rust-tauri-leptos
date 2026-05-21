use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;

#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub enum PermissionState {
    Granted,
    Denied,
    Prompt,
}

#[allow(dead_code)]
pub struct PermissionManager;

#[allow(dead_code)]
impl PermissionManager {
    pub fn is_available() -> bool {
        web_sys::window()
            .and_then(|w| w.navigator().permissions().ok())
            .is_some()
    }

    pub async fn microphone() -> Result<PermissionState, String> {
        Self::check_media_permission("microphone").await
    }

    pub async fn camera() -> Result<PermissionState, String> {
        Self::check_media_permission("camera").await
    }

    async fn check_media_permission(name: &str) -> Result<PermissionState, String> {
        let window = web_sys::window().ok_or("Window not available")?;
        let navigator = window.navigator();
        let media_devices = navigator
            .media_devices()
            .map_err(|e| format!("MediaDevices not available: {:?}", e))?;

        let constraints = MediaStreamConstraints::new();
        if name == "microphone" {
            constraints.set_audio(&JsValue::from_bool(true));
        } else if name == "camera" {
            constraints.set_video(&JsValue::from_bool(true));
        }

        match media_devices.get_user_media_with_constraints(&constraints) {
            Ok(promise) => {
                let result = JsFuture::from(promise).await;
                match result {
                    Ok(stream) => {
                        let stream: web_sys::MediaStream = stream.unchecked_into();
                        for track in stream.get_tracks().iter() {
                            let track: web_sys::MediaStreamTrack = track.unchecked_into();
                            track.stop();
                        }
                        Ok(PermissionState::Granted)
                    }
                    Err(_) => Ok(PermissionState::Denied),
                }
            }
            Err(_) => Ok(PermissionState::Denied),
        }
    }
}

use web_sys::MediaStreamConstraints;

#[allow(dead_code)]
pub struct NotificationManager;

#[allow(dead_code)]
impl NotificationManager {
    pub fn permission() -> web_sys::NotificationPermission {
        web_sys::Notification::permission()
    }

    pub async fn request_permission() -> Result<web_sys::NotificationPermission, String> {
        match Self::permission() {
            web_sys::NotificationPermission::Granted => {
                Ok(web_sys::NotificationPermission::Granted)
            }
            web_sys::NotificationPermission::Denied => Ok(web_sys::NotificationPermission::Denied),
            _ => {
                let promise = web_sys::Notification::request_permission()
                    .map_err(|e| format!("Permission request failed: {:?}", e))?;
                let result = JsFuture::from(promise)
                    .await
                    .map_err(|e| format!("Permission request failed: {:?}", e))?;
                let perm_str = result.as_string().unwrap_or_default();
                match perm_str.as_str() {
                    "granted" => Ok(web_sys::NotificationPermission::Granted),
                    "denied" => Ok(web_sys::NotificationPermission::Denied),
                    _ => Ok(web_sys::NotificationPermission::Default),
                }
            }
        }
    }

    pub async fn show(title: &str, body: &str) -> Result<(), String> {
        if Self::permission() != web_sys::NotificationPermission::Granted {
            let perm = Self::request_permission().await?;
            if perm != web_sys::NotificationPermission::Granted {
                return Err("Notification permission denied".to_string());
            }
        }

        let opts = web_sys::NotificationOptions::new();
        opts.set_body(body);
        web_sys::Notification::new_with_options(title, &opts)
            .map(|_| ())
            .map_err(|e| format!("Failed to show notification: {:?}", e))
    }

    pub async fn show_with_icon(title: &str, body: &str, icon: &str) -> Result<(), String> {
        if Self::permission() != web_sys::NotificationPermission::Granted {
            let perm = Self::request_permission().await?;
            if perm != web_sys::NotificationPermission::Granted {
                return Err("Notification permission denied".to_string());
            }
        }

        let opts = web_sys::NotificationOptions::new();
        opts.set_body(body);
        opts.set_icon(icon);
        web_sys::Notification::new_with_options(title, &opts)
            .map(|_| ())
            .map_err(|e| format!("Failed to show notification: {:?}", e))
    }
}

#[allow(dead_code)]
pub struct SpeechManager;

#[allow(dead_code)]
impl SpeechManager {
    pub fn is_available() -> bool {
        web_sys::window()
            .and_then(|w| w.speech_synthesis().ok())
            .is_some()
    }

    pub fn speak(text: &str, lang: &str, rate: f32, pitch: f32, volume: f32) {
        let window = match web_sys::window() {
            Some(w) => w,
            None => return,
        };
        let synthesis = match window.speech_synthesis() {
            Ok(s) => s,
            Err(_) => return,
        };

        let utterance = match web_sys::SpeechSynthesisUtterance::new_with_text(text) {
            Ok(u) => u,
            Err(_) => return,
        };
        utterance.set_lang(lang);
        utterance.set_rate(rate);
        utterance.set_pitch(pitch);
        utterance.set_volume(volume);

        synthesis.speak(&utterance);
    }

    pub fn stop() {
        if let Some(window) = web_sys::window() {
            if let Ok(synthesis) = window.speech_synthesis() {
                synthesis.cancel();
            }
        }
    }

    pub fn pause() {
        if let Some(window) = web_sys::window() {
            if let Ok(synthesis) = window.speech_synthesis() {
                synthesis.pause();
            }
        }
    }

    pub fn resume() {
        if let Some(window) = web_sys::window() {
            if let Ok(synthesis) = window.speech_synthesis() {
                synthesis.resume();
            }
        }
    }

    pub fn is_speaking() -> bool {
        web_sys::window()
            .and_then(|w| w.speech_synthesis().ok())
            .map(|s| s.speaking())
            .unwrap_or(false)
    }

    pub fn is_paused() -> bool {
        web_sys::window()
            .and_then(|w| w.speech_synthesis().ok())
            .map(|s| s.paused())
            .unwrap_or(false)
    }

    pub fn get_voices() -> Vec<web_sys::SpeechSynthesisVoice> {
        web_sys::window()
            .and_then(|w| w.speech_synthesis().ok())
            .map(|s| {
                let voices = s.get_voices();
                let length = voices.length();
                (0..length)
                    .filter_map(|i| voices.get(i).dyn_into().ok())
                    .collect()
            })
            .unwrap_or_default()
    }
}
