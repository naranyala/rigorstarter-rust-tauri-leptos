use anyhow::{Context, Result};
use hound;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::mpsc;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

enum RecorderCommand {
    Start,
    Stop(mpsc::UnboundedSender<Result<PathBuf>>),
    Play(PathBuf),
}

pub struct AudioRecorderService {
    command_tx: mpsc::UnboundedSender<RecorderCommand>,
}

impl AudioRecorderService {
    pub fn new() -> Self {
        let (command_tx, mut command_rx) = mpsc::unbounded_channel::<RecorderCommand>();

        std::thread::spawn(move || {
            let host = cpal::default_host();
            let input_device = host.default_input_device().expect("No input device found");
            let input_config: cpal::StreamConfig = input_device
                .default_input_config()
                .expect("Failed to get default input config")
                .into();
            let output_device = host
                .default_output_device()
                .expect("No output device found");
            let output_config: cpal::StreamConfig = output_device
                .default_output_config()
                .expect("Failed to get default output config")
                .into();

            let mut input_stream: Option<cpal::Stream> = None;
            let samples_mutex = Arc::new(Mutex::new(Vec::<f32>::new()));
            let is_recording = Arc::new(Mutex::new(false));

            let mut _output_stream: Option<cpal::Stream> = None;

            while let Some(cmd) = command_rx.blocking_recv() {
                match cmd {
                    RecorderCommand::Start => {
                        *is_recording.lock().unwrap() = true;
                        let samples_mutex_clone = Arc::clone(&samples_mutex);
                        let is_recording_clone = Arc::clone(&is_recording);
                        let input_device_clone = input_device.clone();
                        let config_clone = input_config.clone();

                        let stream = input_device_clone
                            .build_input_stream(
                                &config_clone,
                                move |data: &[f32], _| {
                                    if *is_recording_clone.lock().unwrap() {
                                        if let Ok(mut samples) = samples_mutex_clone.lock() {
                                            samples.extend_from_slice(data);
                                        }
                                    }
                                },
                                |err| eprintln!("Input stream error: {}", err),
                                None,
                            )
                            .expect("Failed to build input stream");

                        stream.play().expect("Failed to start input stream");
                        input_stream = Some(stream);
                    }
                    RecorderCommand::Stop(resp_tx) => {
                        *is_recording.lock().unwrap() = false;
                        if let Some(stream) = input_stream.take() {
                            drop(stream);
                        }

                        let final_samples = {
                            let mut samples = samples_mutex.lock().unwrap();
                            std::mem::take(&mut *samples)
                        };

                        if final_samples.is_empty() {
                            let _ = resp_tx.send(Err(anyhow::anyhow!("No audio recorded")));
                            continue;
                        }

                        let timestamp = SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .unwrap()
                            .as_millis();
                        let path =
                            std::env::temp_dir().join(format!("recording_{}.wav", timestamp));

                        let spec = hound::WavSpec {
                            channels: input_config.channels as u16,
                            sample_rate: input_config.sample_rate.0 as u32,
                            bits_per_sample: 32,
                            sample_format: hound::SampleFormat::Float,
                        };

                        let mut writer = hound::WavWriter::create(&path, spec)
                            .expect("Failed to create WAV writer");
                        for &sample in &final_samples {
                            writer.write_sample(sample).expect("Failed to write sample");
                        }
                        writer.finalize().expect("Failed to finalize WAV");

                        let _ = resp_tx.send(Ok(path));
                    }
                    RecorderCommand::Play(path) => {
                        let mut reader =
                            hound::WavReader::open(path).expect("Failed to open wav for playback");
                        let spec = reader.spec();
                        let samples = Arc::new(
                            reader
                                .samples::<f32>()
                                .map(|s| s.unwrap())
                                .collect::<Vec<f32>>(),
                        );
                        let samples_clone = Arc::clone(&samples);
                        let device = output_device.clone();
                        let config = output_config.clone();

                        let stream = device
                            .build_output_stream(
                                &config,
                                move |data: &mut [f32], _| {
                                    static mut INDEX: usize = 0;
                                    unsafe {
                                        for sample in data.iter_mut() {
                                            if INDEX < samples_clone.len() {
                                                *sample = samples_clone[INDEX];
                                                INDEX += 1;
                                            } else {
                                                *sample = 0.0;
                                            }
                                        }
                                    }
                                },
                                |err| eprintln!("Output stream error: {}", err),
                                None,
                            )
                            .expect("Failed to build output stream");

                        stream.play().expect("Failed to start output stream");
                        _output_stream = Some(stream);

                        std::thread::sleep(std::time::Duration::from_secs(
                            samples.len() as u64 / (spec.sample_rate as u64 * spec.channels as u64),
                        ));
                        _output_stream = None;
                    }
                }
            }
        });

        Self { command_tx }
    }

    pub async fn start_recording(&self) -> Result<()> {
        self.command_tx
            .send(RecorderCommand::Start)
            .map_err(|e| anyhow::anyhow!("Failed to send start command: {}", e))?;
        Ok(())
    }

    pub async fn stop_recording(&self) -> Result<PathBuf> {
        let (tx, mut rx) = mpsc::unbounded_channel();
        self.command_tx
            .send(RecorderCommand::Stop(tx))
            .map_err(|e| anyhow::anyhow!("Failed to send stop command: {}", e))?;
        rx.recv().await.context("Failed to receive stop response")?
    }

    pub async fn play_recording(&self, path: PathBuf) -> Result<()> {
        self.command_tx
            .send(RecorderCommand::Play(path))
            .map_err(|e| anyhow::anyhow!("Failed to send play command: {}", e))?;
        Ok(())
    }
}
