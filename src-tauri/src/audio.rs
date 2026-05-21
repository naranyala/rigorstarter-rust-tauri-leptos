use anyhow::Context;
use hound;
use std::path::PathBuf;
use tracing::info;

pub struct AudioService;

#[allow(clippy::new_without_default)]
impl AudioService {
    pub fn new() -> Self {
        Self
    }

    pub async fn load_wav(&self, path: PathBuf) -> anyhow::Result<Vec<f32>> {
        info!("Loading audio file: {:?}", path);
        let mut reader = hound::WavReader::open(path).context("Failed to open WAV file")?;

        let spec = reader.spec();
        if spec.sample_rate != 16000 {
            return Err(anyhow::anyhow!(
                "Unsupported sample rate: {}. Whisper requires 16000Hz",
                spec.sample_rate
            ));
        }
        if spec.channels != 1 {
            return Err(anyhow::anyhow!(
                "Unsupported channels: {}. Whisper requires mono audio",
                spec.channels
            ));
        }

        let samples: Vec<f32> = match spec.sample_format {
            hound::SampleFormat::Float => {
                reader.samples::<f32>().map(|s| s.unwrap_or(0.0)).collect()
            }
            hound::SampleFormat::Int => {
                let bit_depth = spec.bits_per_sample;
                reader
                    .samples::<i32>()
                    .map(|s| {
                        let sample = s.unwrap_or(0);
                        sample as f32
                            / match bit_depth {
                                16 => 32768.0,
                                24 => 8388608.0,
                                32 => 2147483648.0,
                                _ => 1.0,
                            }
                    })
                    .collect()
            }
        };

        info!("Successfully loaded {} audio samples", samples.len());
        Ok(samples)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    fn create_wav(
        path: &PathBuf,
        sample_rate: u32,
        channels: u16,
        bit_depth: u16,
        samples: &[f32],
    ) {
        let spec = hound::WavSpec {
            channels,
            sample_rate,
            bits_per_sample: bit_depth,
            sample_format: hound::SampleFormat::Float,
        };
        let mut writer = hound::WavWriter::create(path, spec).unwrap();
        for &s in samples {
            writer.write_sample(s).unwrap();
        }
        writer.finalize().unwrap();
    }

    #[tokio::test]
    async fn test_load_wav_valid() {
        let temp_dir = env::temp_dir();
        let file_path = temp_dir.join("test_valid.wav");
        let samples = vec![0.0, 0.5, -0.5, 1.0, -1.0];
        create_wav(&file_path, 16000, 1, 32, &samples);

        let service = AudioService::new();
        let loaded_samples = service.load_wav(file_path.clone()).await.unwrap();
        assert_eq!(loaded_samples, samples);

        let _ = std::fs::remove_file(file_path);
    }

    #[tokio::test]
    async fn test_load_wav_invalid_sample_rate() {
        let temp_dir = env::temp_dir();
        let file_path = temp_dir.join("test_invalid_rate.wav");
        let samples = vec![0.0, 0.5];
        create_wav(&file_path, 44100, 1, 32, &samples);

        let service = AudioService::new();
        let result = service.load_wav(file_path.clone()).await;
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Unsupported sample rate"));

        let _ = std::fs::remove_file(file_path);
    }

    #[tokio::test]
    async fn test_load_wav_invalid_channels() {
        let temp_dir = env::temp_dir();
        let file_path = temp_dir.join("test_invalid_channels.wav");
        let samples = vec![0.0, 0.5, 0.0, 0.5];
        create_wav(&file_path, 16000, 2, 32, &samples);

        let service = AudioService::new();
        let result = service.load_wav(file_path.clone()).await;
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Unsupported channels"));

        let _ = std::fs::remove_file(file_path);
    }

    #[tokio::test]
    async fn test_load_wav_non_existent() {
        let service = AudioService::new();
        let result = service.load_wav(PathBuf::from("non_existent.wav")).await;
        assert!(result.is_err());
    }
}
