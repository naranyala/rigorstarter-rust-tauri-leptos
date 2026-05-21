use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::{AnalyserNode, AudioContext};

pub struct AudioEngine {
    context: AudioContext,
    analyser: AnalyserNode,
    source: Option<web_sys::MediaStreamAudioSourceNode>,
}

impl AudioEngine {
    pub fn new() -> Result<Self, JsValue> {
        let context = AudioContext::new()?;
        let analyser = context.create_analyser()?;
        analyser.set_fft_size(256);

        Ok(Self {
            context,
            analyser,
            source: None,
        })
    }

    pub fn connect_stream(&mut self, stream: &web_sys::MediaStream) -> Result<(), JsValue> {
        let source = self.context.create_media_stream_source(stream)?;
        source.connect_with_audio_node(&self.analyser)?;
        self.source = Some(source);
        Ok(())
    }

    pub fn get_frequency_data(&self) -> Vec<u8> {
        let mut buffer = vec![0u8; self.analyser.frequency_bin_count() as usize];
        self.analyser.get_byte_frequency_data(&mut buffer);
        buffer
    }

    pub fn resume(&self) -> Result<js_sys::Promise, JsValue> {
        self.context.resume()
    }
}
