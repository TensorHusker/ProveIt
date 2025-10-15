//! Haptic feedback for tactile interfaces

use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::{Error, Result};

/// Haptic feedback engine
pub struct HapticEngine {
    /// Whether haptics are enabled
    enabled: bool,
    /// Global intensity multiplier
    intensity: f32,
    /// Connected haptic devices
    devices: Vec<HapticDevice>,
}

impl HapticEngine {
    pub fn new() -> Self {
        Self {
            enabled: true,
            intensity: 0.5,
            devices: Vec::new(),
        }
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn set_intensity(&mut self, intensity: f32) {
        self.intensity = intensity.clamp(0.0, 1.0);
    }

    /// Discover and connect to haptic devices
    pub fn discover_devices(&mut self) -> Result<usize> {
        // Placeholder for device discovery
        // Would use platform-specific APIs:
        // - macOS: Core Haptics
        // - iOS: UIFeedbackGenerator
        // - Android: Vibrator API
        // - Windows: DirectInput
        // - Linux: evdev

        tracing::info!("Discovering haptic devices...");
        // For now, create a virtual device
        self.devices.push(HapticDevice::virtual_device());
        Ok(self.devices.len())
    }

    /// Play a haptic pattern
    pub fn play(&self, pattern: &HapticPattern) -> Result<()> {
        if !self.enabled {
            return Ok(());
        }

        for device in &self.devices {
            device.play_pattern(pattern, self.intensity)?;
        }

        Ok(())
    }

    /// Play a simple pulse
    pub fn pulse(&self, duration_ms: u32) -> Result<()> {
        self.play(&HapticPattern::Pulse { duration_ms })
    }

    /// Play a double pulse (confirmation)
    pub fn double_pulse(&self) -> Result<()> {
        self.play(&HapticPattern::Rhythm {
            intervals_ms: vec![100, 100, 100],
        })
    }

    /// Play a success pattern
    pub fn success(&self) -> Result<()> {
        self.play(&HapticPattern::Rhythm {
            intervals_ms: vec![50, 50, 50, 100, 200],
        })
    }

    /// Play an error pattern
    pub fn error(&self) -> Result<()> {
        self.play(&HapticPattern::Rhythm {
            intervals_ms: vec![300, 100, 300],
        })
    }

    /// Play a notification pattern
    pub fn notification(&self) -> Result<()> {
        self.play(&HapticPattern::Pulse { duration_ms: 150 })
    }
}

impl Default for HapticEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Haptic feedback pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HapticPattern {
    /// Single pulse
    Pulse { duration_ms: u32 },
    /// Rhythmic pattern
    Rhythm { intervals_ms: Vec<u32> },
    /// Continuous vibration
    Continuous { duration_ms: u32, intensity: f32 },
    /// Custom waveform
    Waveform { samples: Vec<f32>, sample_rate: u32 },
}

impl HapticPattern {
    /// Create a simple pulse
    pub fn pulse(duration_ms: u32) -> Self {
        Self::Pulse { duration_ms }
    }

    /// Create a rhythm from intervals
    pub fn rhythm(intervals_ms: Vec<u32>) -> Self {
        Self::Rhythm { intervals_ms }
    }

    /// Create a continuous vibration
    pub fn continuous(duration_ms: u32, intensity: f32) -> Self {
        Self::Continuous {
            duration_ms,
            intensity,
        }
    }

    /// Get the total duration of the pattern
    pub fn duration(&self) -> Duration {
        match self {
            Self::Pulse { duration_ms } => Duration::from_millis(*duration_ms as u64),
            Self::Rhythm { intervals_ms } => {
                Duration::from_millis(intervals_ms.iter().sum::<u32>() as u64)
            }
            Self::Continuous { duration_ms, .. } => Duration::from_millis(*duration_ms as u64),
            Self::Waveform {
                samples,
                sample_rate,
            } => Duration::from_secs_f32(samples.len() as f32 / *sample_rate as f32),
        }
    }
}

/// Haptic device representation
struct HapticDevice {
    name: String,
    device_type: HapticDeviceType,
}

#[derive(Debug, Clone, Copy)]
enum HapticDeviceType {
    Virtual,
    GameController,
    Touchscreen,
    WearableDevice,
}

impl HapticDevice {
    fn virtual_device() -> Self {
        Self {
            name: "Virtual Haptic Device".to_string(),
            device_type: HapticDeviceType::Virtual,
        }
    }

    fn play_pattern(&self, pattern: &HapticPattern, intensity: f32) -> Result<()> {
        // Placeholder implementation
        tracing::debug!(
            "Playing haptic pattern on {}: {:?} (intensity: {})",
            self.name,
            pattern,
            intensity
        );

        // In a real implementation, this would interface with:
        // - Platform-specific haptic APIs
        // - Game controller rumble
        // - Mobile device vibration motors
        // - Custom haptic hardware

        Ok(())
    }
}

/// Haptic feedback for specific proof events
pub struct HapticFeedback {
    engine: HapticEngine,
}

impl HapticFeedback {
    pub fn new(engine: HapticEngine) -> Self {
        Self { engine }
    }

    /// Feedback for adding a point
    pub fn point_added(&self) -> Result<()> {
        self.engine.pulse(50)
    }

    /// Feedback for adding a line
    pub fn line_added(&self) -> Result<()> {
        self.engine.pulse(100)
    }

    /// Feedback for goal completion
    pub fn goal_completed(&self) -> Result<()> {
        self.engine.success()
    }

    /// Feedback for proof completion
    pub fn proof_completed(&self) -> Result<()> {
        self.engine.play(&HapticPattern::Rhythm {
            intervals_ms: vec![50, 50, 50, 50, 50, 100, 200],
        })
    }

    /// Feedback for error
    pub fn error_occurred(&self) -> Result<()> {
        self.engine.error()
    }

    /// Feedback for tactic application
    pub fn tactic_applied(&self) -> Result<()> {
        self.engine.pulse(75)
    }

    /// Feedback for undo
    pub fn undo(&self) -> Result<()> {
        self.engine.play(&HapticPattern::Rhythm {
            intervals_ms: vec![100, 50, 100],
        })
    }

    /// Feedback for redo
    pub fn redo(&self) -> Result<()> {
        self.engine.play(&HapticPattern::Rhythm {
            intervals_ms: vec![50, 100, 50],
        })
    }

    /// Feedback for navigation
    pub fn navigate(&self, direction: NavigationDirection) -> Result<()> {
        match direction {
            NavigationDirection::Forward => self.engine.pulse(50),
            NavigationDirection::Backward => self.engine.pulse(75),
            NavigationDirection::Up => self.engine.pulse(60),
            NavigationDirection::Down => self.engine.pulse(60),
        }
    }

    /// Feedback proportional to proof depth
    pub fn depth_feedback(&self, depth: usize) -> Result<()> {
        // More pulses for deeper proofs
        let pulses = (depth.min(5) + 1) as u32;
        let mut intervals = Vec::new();
        for _ in 0..pulses {
            intervals.push(50);
            intervals.push(50);
        }
        self.engine.play(&HapticPattern::Rhythm {
            intervals_ms: intervals,
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub enum NavigationDirection {
    Forward,
    Backward,
    Up,
    Down,
}

/// Haptic patterns for common mathematical concepts
pub struct MathematicalHaptics;

impl MathematicalHaptics {
    /// Pattern for implication (A → B)
    pub fn implication() -> HapticPattern {
        HapticPattern::Rhythm {
            intervals_ms: vec![100, 50, 150],
        }
    }

    /// Pattern for conjunction (A ∧ B)
    pub fn conjunction() -> HapticPattern {
        HapticPattern::Rhythm {
            intervals_ms: vec![100, 50, 100],
        }
    }

    /// Pattern for disjunction (A ∨ B)
    pub fn disjunction() -> HapticPattern {
        HapticPattern::Rhythm {
            intervals_ms: vec![100, 100, 50, 50],
        }
    }

    /// Pattern for negation (¬A)
    pub fn negation() -> HapticPattern {
        HapticPattern::Rhythm {
            intervals_ms: vec![200, 100, 200],
        }
    }

    /// Pattern for universal quantification (∀)
    pub fn universal() -> HapticPattern {
        HapticPattern::Continuous {
            duration_ms: 200,
            intensity: 0.8,
        }
    }

    /// Pattern for existential quantification (∃)
    pub fn existential() -> HapticPattern {
        HapticPattern::Pulse { duration_ms: 150 }
    }

    /// Pattern for equality (=)
    pub fn equality() -> HapticPattern {
        HapticPattern::Rhythm {
            intervals_ms: vec![100, 100],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_haptic_engine() {
        let mut engine = HapticEngine::new();
        assert!(engine.enabled);

        engine.set_intensity(0.7);
        assert_eq!(engine.intensity, 0.7);
    }

    #[test]
    fn test_haptic_pattern_duration() {
        let pulse = HapticPattern::Pulse { duration_ms: 100 };
        assert_eq!(pulse.duration(), Duration::from_millis(100));

        let rhythm = HapticPattern::Rhythm {
            intervals_ms: vec![50, 50, 100],
        };
        assert_eq!(rhythm.duration(), Duration::from_millis(200));
    }

    #[test]
    fn test_mathematical_haptics() {
        let pattern = MathematicalHaptics::implication();
        assert!(pattern.duration() > Duration::from_millis(0));
    }

    #[test]
    fn test_haptic_feedback() {
        let engine = HapticEngine::new();
        let feedback = HapticFeedback::new(engine);

        // These should not panic
        let _ = feedback.point_added();
        let _ = feedback.goal_completed();
    }
}
