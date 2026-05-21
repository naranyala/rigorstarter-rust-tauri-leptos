use leptos::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct AudioState {
    pub current_track: Option<String>,
    pub is_playing: bool,
    pub is_recording: bool,
    pub recording_duration: f32,
    pub playback_progress: f32,
    pub current_playback_duration: f32,
    pub status: String,
    pub recordings: Vec<String>,
}

impl Default for AudioState {
    fn default() -> Self {
        Self {
            current_track: None,
            is_playing: false,
            is_recording: false,
            recording_duration: 0.0,
            playback_progress: 0.0,
            current_playback_duration: 0.0,
            status: "Ready".to_string(),
            recordings: vec![],
        }
    }
}

#[derive(Clone, Copy)]
pub struct AudioViewModel {
    pub state: RwSignal<AudioState>,
}

impl AudioViewModel {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            state: RwSignal::new(AudioState::default()),
        }
    }

    pub fn play_file(&self, path: std::path::PathBuf) {
        self.state.update(|s| {
            s.current_track = Some(path.to_string_lossy().into_owned());
            s.is_playing = true;
            s.status = format!("Playing: {}", path.display());
            s.playback_progress = 0.0;
            s.current_playback_duration = 10.0; // Mock duration
        });
    }

    pub fn start_recording(&self) {
        self.state.update(|s| {
            s.is_recording = true;
            s.recording_duration = 0.0;
            s.status = "Recording...".to_string();
        });
    }

    pub fn stop_recording(&self) {
        self.state.update(|s| {
            s.is_recording = false;
            s.status = "Recording saved.".to_string();
            s.recordings
                .push(format!("rec_{}.wav", s.recording_duration));
        });
    }

    pub fn tick_recording(&self, delta: f32) {
        self.state.update(|s| {
            if s.is_recording {
                s.recording_duration += delta;
            }
        });
    }

    pub fn tick_playback(&self, delta: f32) {
        self.state.update(|s| {
            if s.is_playing {
                s.playback_progress += delta;
                if s.playback_progress >= s.current_playback_duration {
                    s.is_playing = false;
                    s.status = "Playback finished.".to_string();
                }
            }
        });
    }
}
