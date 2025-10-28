//! # Accessibility: Making formal verification accessible to everyone
//!
//! This crate provides comprehensive accessibility features including:
//! - Text-to-speech for proof narration
//! - Sonification of proof structure
//! - Haptic feedback for tactile interfaces
//! - Natural language descriptions
//! - Spatial audio for geometric proofs

pub mod audio;
pub mod description;
pub mod haptic;
pub mod spatial_audio;

pub use audio::{AudioEngine, Sonifier, SpeechSynthesizer};
pub use description::{DescriptionGenerator, ProofNarrator};
pub use haptic::{HapticEngine, HapticFeedback, HapticPattern};
pub use spatial_audio::{AudioPosition, SpatialAudioEngine};

/// Result type for accessibility operations
pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can occur in accessibility subsystems
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Audio error: {0}")]
    AudioError(String),

    #[error("Speech synthesis failed: {0}")]
    SpeechError(String),

    #[error("Haptic feedback error: {0}")]
    HapticError(String),

    #[error("Description generation failed: {0}")]
    DescriptionError(String),

    #[error("Spatial audio error: {0}")]
    SpatialAudioError(String),

    #[error("Device not available: {0}")]
    DeviceNotAvailable(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

/// Accessibility preferences
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AccessibilityPreferences {
    /// Enable text-to-speech
    pub enable_tts: bool,
    /// Enable sonification
    pub enable_sonification: bool,
    /// Enable haptic feedback
    pub enable_haptic: bool,
    /// Enable spatial audio
    pub enable_spatial_audio: bool,
    /// Speech rate (words per minute)
    pub speech_rate: u32,
    /// Audio volume (0.0 - 1.0)
    pub volume: f32,
    /// Haptic intensity (0.0 - 1.0)
    pub haptic_intensity: f32,
    /// Verbosity level (1-5)
    pub verbosity: u8,
}

impl Default for AccessibilityPreferences {
    fn default() -> Self {
        Self {
            enable_tts: true,
            enable_sonification: true,
            enable_haptic: false, // Disabled by default (not all devices support it)
            enable_spatial_audio: true,
            speech_rate: 180,
            volume: 0.7,
            haptic_intensity: 0.5,
            verbosity: 3,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_preferences() {
        let prefs = AccessibilityPreferences::default();
        assert!(prefs.enable_tts);
        assert_eq!(prefs.verbosity, 3);
    }
}
