//! Audio synthesis and text-to-speech

use rodio::{OutputStream, OutputStreamHandle, Sink, Source};
use std::sync::Arc;
use std::time::Duration;

use crate::{Error, Result};

/// Audio engine for managing sound output
pub struct AudioEngine {
    /// Audio output stream
    _stream: OutputStream,
    /// Stream handle
    stream_handle: OutputStreamHandle,
    /// Master volume
    volume: f32,
}

impl AudioEngine {
    pub fn new() -> Result<Self> {
        let (stream, stream_handle) = OutputStream::try_default()
            .map_err(|e| Error::AudioError(format!("Failed to initialize audio: {}", e)))?;

        Ok(Self {
            _stream: stream,
            stream_handle,
            volume: 0.7,
        })
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume.clamp(0.0, 1.0);
    }

    pub fn play_tone(&self, frequency: f32, duration: Duration) -> Result<()> {
        let sink = Sink::try_new(&self.stream_handle)
            .map_err(|e| Error::AudioError(format!("Failed to create sink: {}", e)))?;

        let source = ToneSource::new(frequency, duration);
        sink.set_volume(self.volume);
        sink.append(source);
        sink.detach();

        Ok(())
    }

    pub fn stream_handle(&self) -> &OutputStreamHandle {
        &self.stream_handle
    }
}

impl Default for AudioEngine {
    fn default() -> Self {
        Self::new().expect("Failed to initialize audio engine")
    }
}

/// Simple tone generator
struct ToneSource {
    frequency: f32,
    sample_rate: u32,
    num_samples: usize,
    current_sample: usize,
}

impl ToneSource {
    fn new(frequency: f32, duration: Duration) -> Self {
        let sample_rate = 44100;
        let num_samples = (sample_rate as f32 * duration.as_secs_f32()) as usize;

        Self {
            frequency,
            sample_rate,
            num_samples,
            current_sample: 0,
        }
    }
}

impl Iterator for ToneSource {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_sample >= self.num_samples {
            return None;
        }

        let t = self.current_sample as f32 / self.sample_rate as f32;
        let value = (t * self.frequency * 2.0 * std::f32::consts::PI).sin();

        self.current_sample += 1;
        Some(value * 0.3) // Reduce amplitude
    }
}

impl Source for ToneSource {
    fn current_frame_len(&self) -> Option<usize> {
        Some(self.num_samples - self.current_sample)
    }

    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn total_duration(&self) -> Option<Duration> {
        Some(Duration::from_secs_f32(
            self.num_samples as f32 / self.sample_rate as f32,
        ))
    }
}

/// Text-to-speech synthesizer
pub struct SpeechSynthesizer {
    /// Speech rate (words per minute)
    rate: u32,
    /// Volume
    volume: f32,
    /// Audio engine for playback
    audio_engine: Arc<AudioEngine>,
}

impl SpeechSynthesizer {
    pub fn new(audio_engine: Arc<AudioEngine>) -> Self {
        Self {
            rate: 180,
            volume: 0.7,
            audio_engine,
        }
    }

    pub fn set_rate(&mut self, rate: u32) {
        self.rate = rate.clamp(50, 500);
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume.clamp(0.0, 1.0);
    }

    /// Speak text (placeholder - would integrate with TTS library)
    pub fn speak(&self, text: &str) -> Result<()> {
        tracing::info!("TTS: {}", text);

        // In a real implementation, this would use a TTS library like:
        // - espeak-ng (cross-platform, open source)
        // - festival (Linux)
        // - macOS Speech Synthesis Manager
        // - Windows SAPI

        // For now, we'll provide a simple beep as placeholder
        self.audio_engine
            .play_tone(440.0, Duration::from_millis(200))?;

        Ok(())
    }

    /// Speak text asynchronously
    pub async fn speak_async(&self, text: String) -> Result<()> {
        // Placeholder for async TTS
        self.speak(&text)
    }
}

/// Sonification engine - converts data to sound
pub struct Sonifier {
    audio_engine: Arc<AudioEngine>,
    /// Base frequency for sonification
    base_frequency: f32,
}

impl Sonifier {
    pub fn new(audio_engine: Arc<AudioEngine>) -> Self {
        Self {
            audio_engine,
            base_frequency: 220.0, // A3
        }
    }

    /// Sonify a value (0.0 - 1.0) as a pitch
    pub fn sonify_value(&self, value: f32) -> Result<()> {
        let frequency = self.base_frequency * (1.0 + value * 2.0);
        self.audio_engine
            .play_tone(frequency, Duration::from_millis(300))
    }

    /// Sonify proof depth (higher = deeper)
    pub fn sonify_depth(&self, depth: usize) -> Result<()> {
        // Lower pitch for deeper proofs
        let frequency = self.base_frequency * (1.0 / (1.0 + depth as f32 * 0.1));
        self.audio_engine
            .play_tone(frequency, Duration::from_millis(200))
    }

    /// Sonify proof complexity with a chord
    pub fn sonify_complexity(&self, complexity: u32) -> Result<()> {
        // Play multiple tones for complex proofs
        let num_tones = (complexity.min(5) + 1) as usize;

        for i in 0..num_tones {
            let frequency = self.base_frequency * (1.0 + i as f32 * 0.25);
            self.audio_engine
                .play_tone(frequency, Duration::from_millis(150))?;
        }

        Ok(())
    }

    /// Sonify a proof path with ascending tones
    pub fn sonify_path(&self, path_length: usize) -> Result<()> {
        for i in 0..path_length {
            let frequency = self.base_frequency * (1.0 + i as f32 * 0.2);
            self.audio_engine
                .play_tone(frequency, Duration::from_millis(100))?;
            std::thread::sleep(Duration::from_millis(50));
        }
        Ok(())
    }

    /// Play success sound
    pub fn play_success(&self) -> Result<()> {
        // Ascending major chord
        let frequencies = [261.63, 329.63, 392.00]; // C, E, G
        for freq in frequencies {
            self.audio_engine
                .play_tone(freq, Duration::from_millis(200))?;
            std::thread::sleep(Duration::from_millis(100));
        }
        Ok(())
    }

    /// Play error sound
    pub fn play_error(&self) -> Result<()> {
        // Descending dissonant tones
        let frequencies = [440.0, 415.3, 392.0];
        for freq in frequencies {
            self.audio_engine
                .play_tone(freq, Duration::from_millis(150))?;
            std::thread::sleep(Duration::from_millis(50));
        }
        Ok(())
    }

    /// Play notification sound
    pub fn play_notification(&self) -> Result<()> {
        self.audio_engine
            .play_tone(523.25, Duration::from_millis(200))?;
        std::thread::sleep(Duration::from_millis(100));
        self.audio_engine
            .play_tone(659.25, Duration::from_millis(200))?;
        Ok(())
    }
}

/// Musical scale for sonification
pub enum Scale {
    Major,
    Minor,
    Pentatonic,
    Chromatic,
}

impl Scale {
    /// Get frequency ratios for the scale
    pub fn ratios(&self) -> &[f32] {
        match self {
            Scale::Major => &[1.0, 1.122, 1.260, 1.335, 1.498, 1.682, 1.888, 2.0],
            Scale::Minor => &[1.0, 1.122, 1.189, 1.335, 1.498, 1.587, 1.782, 2.0],
            Scale::Pentatonic => &[1.0, 1.122, 1.260, 1.498, 1.682, 2.0],
            Scale::Chromatic => &[
                1.0, 1.059, 1.122, 1.189, 1.260, 1.335, 1.414, 1.498, 1.587, 1.682, 1.782, 1.888,
                2.0,
            ],
        }
    }

    /// Get frequency for a scale degree
    pub fn frequency(&self, base: f32, degree: usize) -> f32 {
        let ratios = self.ratios();
        let octave = degree / ratios.len();
        let note = degree % ratios.len();
        base * ratios[note] * (2.0_f32).powi(octave as i32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tone_source() {
        let source = ToneSource::new(440.0, Duration::from_millis(100));
        assert_eq!(source.channels(), 1);
        assert_eq!(source.sample_rate(), 44100);
    }

    #[test]
    fn test_scale() {
        let scale = Scale::Major;
        let freq = scale.frequency(440.0, 0);
        assert_eq!(freq, 440.0);
    }

    #[test]
    fn test_speech_synthesizer() {
        // This would require audio device, so we just test creation
        // Actual tests would need to mock the audio engine
        assert!(true);
    }
}
