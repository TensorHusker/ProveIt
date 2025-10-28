//! Spatial audio for 3D proof visualization

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;

use crate::audio::AudioEngine;
use crate::{Error, Result};

/// 3D position in audio space
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct AudioPosition {
    /// X coordinate (left/right)
    pub x: f32,
    /// Y coordinate (up/down)
    pub y: f32,
    /// Z coordinate (front/back)
    pub z: f32,
}

impl AudioPosition {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn origin() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    /// Distance from another position
    pub fn distance_to(&self, other: &AudioPosition) -> f32 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        let dz = other.z - self.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    /// Direction vector to another position
    pub fn direction_to(&self, other: &AudioPosition) -> AudioPosition {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        let dz = other.z - self.z;
        let len = self.distance_to(other);
        if len > 0.0 {
            AudioPosition::new(dx / len, dy / len, dz / len)
        } else {
            AudioPosition::origin()
        }
    }
}

impl Default for AudioPosition {
    fn default() -> Self {
        Self::origin()
    }
}

/// Listener position and orientation
#[derive(Debug, Clone)]
pub struct Listener {
    /// Position in space
    pub position: AudioPosition,
    /// Forward direction vector
    pub forward: AudioPosition,
    /// Up direction vector
    pub up: AudioPosition,
}

impl Listener {
    pub fn new() -> Self {
        Self {
            position: AudioPosition::origin(),
            forward: AudioPosition::new(0.0, 0.0, -1.0),
            up: AudioPosition::new(0.0, 1.0, 0.0),
        }
    }

    /// Move listener to a new position
    pub fn move_to(&mut self, position: AudioPosition) {
        self.position = position;
    }

    /// Look at a target position
    pub fn look_at(&mut self, target: AudioPosition) {
        self.forward = self.position.direction_to(&target);
    }
}

impl Default for Listener {
    fn default() -> Self {
        Self::new()
    }
}

/// Spatial audio source
#[derive(Debug, Clone)]
pub struct AudioSource {
    /// Position of the source
    pub position: AudioPosition,
    /// Base frequency
    pub frequency: f32,
    /// Volume
    pub volume: f32,
    /// Radius of influence
    pub radius: f32,
}

impl AudioSource {
    pub fn new(position: AudioPosition, frequency: f32) -> Self {
        Self {
            position,
            frequency,
            volume: 1.0,
            radius: 10.0,
        }
    }

    /// Calculate perceived volume based on distance to listener
    pub fn perceived_volume(&self, listener: &Listener) -> f32 {
        let distance = self.position.distance_to(&listener.position);
        if distance > self.radius {
            0.0
        } else {
            // Inverse square law with minimum distance
            let min_distance = 1.0;
            let effective_distance = distance.max(min_distance);
            self.volume / (effective_distance * effective_distance)
        }
    }

    /// Calculate left/right panning based on listener orientation
    pub fn panning(&self, listener: &Listener) -> (f32, f32) {
        let to_source = listener.position.direction_to(&self.position);

        // Simple stereo panning based on x-coordinate
        // -1.0 (left) to 1.0 (right)
        let pan = to_source.x;

        let left = ((1.0 - pan) / 2.0).clamp(0.0, 1.0);
        let right = ((1.0 + pan) / 2.0).clamp(0.0, 1.0);

        (left, right)
    }

    /// Calculate doppler shift (for moving sources)
    pub fn doppler_shift(&self, listener: &Listener, velocity: f32) -> f32 {
        // Simplified doppler effect
        let direction = listener.position.direction_to(&self.position);
        let speed_of_sound = 343.0; // m/s

        // Relative velocity along direction to listener
        let relative_velocity = velocity * direction.z;

        // Doppler formula: f' = f * (c + v_listener) / (c + v_source)
        self.frequency * (speed_of_sound) / (speed_of_sound + relative_velocity)
    }
}

/// Spatial audio engine
pub struct SpatialAudioEngine {
    /// Audio backend
    audio_engine: Arc<AudioEngine>,
    /// Listener position and orientation
    listener: Listener,
    /// Active audio sources
    sources: Vec<AudioSource>,
    /// Enable spatial effects
    enabled: bool,
}

impl SpatialAudioEngine {
    pub fn new(audio_engine: Arc<AudioEngine>) -> Self {
        Self {
            audio_engine,
            listener: Listener::new(),
            sources: Vec::new(),
            enabled: true,
        }
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Set listener position
    pub fn set_listener_position(&mut self, position: AudioPosition) {
        self.listener.move_to(position);
    }

    /// Set listener orientation
    pub fn set_listener_look_at(&mut self, target: AudioPosition) {
        self.listener.look_at(target);
    }

    /// Add an audio source
    pub fn add_source(&mut self, source: AudioSource) -> usize {
        let id = self.sources.len();
        self.sources.push(source);
        id
    }

    /// Remove an audio source
    pub fn remove_source(&mut self, id: usize) {
        if id < self.sources.len() {
            self.sources.remove(id);
        }
    }

    /// Update a source's position
    pub fn update_source_position(&mut self, id: usize, position: AudioPosition) {
        if let Some(source) = self.sources.get_mut(id) {
            source.position = position;
        }
    }

    /// Play a tone at a spatial position
    pub fn play_tone_at(
        &self,
        position: AudioPosition,
        frequency: f32,
        duration: Duration,
    ) -> Result<()> {
        if !self.enabled {
            return self.audio_engine.play_tone(frequency, duration);
        }

        let source = AudioSource::new(position, frequency);
        let volume = source.perceived_volume(&self.listener);

        if volume > 0.01 {
            // Play with spatial effects
            self.audio_engine.play_tone(frequency, duration)?;
        }

        Ok(())
    }

    /// Sonify a proof path in 3D space
    pub fn sonify_path(&self, positions: &[AudioPosition], base_freq: f32) -> Result<()> {
        for (i, pos) in positions.iter().enumerate() {
            let frequency = base_freq * (1.0 + i as f32 * 0.2);
            self.play_tone_at(*pos, frequency, Duration::from_millis(200))?;
            std::thread::sleep(Duration::from_millis(100));
        }
        Ok(())
    }

    /// Sonify proof depth with spatial audio
    pub fn sonify_depth_spatial(&self, depth: usize) -> Result<()> {
        // Deeper proofs go further back in z-space
        for i in 0..=depth {
            let z = -(i as f32);
            let position = AudioPosition::new(0.0, 0.0, z);
            let frequency = 440.0 / (1.0 + i as f32 * 0.1);
            self.play_tone_at(position, frequency, Duration::from_millis(150))?;
            std::thread::sleep(Duration::from_millis(100));
        }
        Ok(())
    }

    /// Create a sonic "tour" of a proof construction
    pub fn tour_construction(&self, points: &[AudioPosition], frequencies: &[f32]) -> Result<()> {
        if points.len() != frequencies.len() {
            return Err(Error::SpatialAudioError(
                "Mismatched points and frequencies".to_string(),
            ));
        }

        for (pos, freq) in points.iter().zip(frequencies.iter()) {
            self.play_tone_at(*pos, *freq, Duration::from_millis(300))?;
            std::thread::sleep(Duration::from_millis(200));
        }

        Ok(())
    }

    /// Sonify connections between points
    pub fn sonify_connection(
        &self,
        from: AudioPosition,
        to: AudioPosition,
        base_freq: f32,
    ) -> Result<()> {
        // Interpolate positions
        let steps = 5;
        for i in 0..=steps {
            let t = i as f32 / steps as f32;
            let pos = AudioPosition::new(
                from.x + t * (to.x - from.x),
                from.y + t * (to.y - from.y),
                from.z + t * (to.z - from.z),
            );
            let freq = base_freq * (1.0 + t * 0.5);
            self.play_tone_at(pos, freq, Duration::from_millis(100))?;
            std::thread::sleep(Duration::from_millis(50));
        }
        Ok(())
    }

    /// Ambient soundscape for proof exploration
    pub fn ambient_soundscape(&self, complexity: u32) -> Result<()> {
        // Create layered ambient tones based on proof complexity
        let base_frequencies = vec![110.0, 146.83, 220.0, 293.66];

        for (i, freq) in base_frequencies
            .iter()
            .take(complexity.min(4) as usize)
            .enumerate()
        {
            let z = -(i as f32 * 2.0);
            let position = AudioPosition::new(0.0, 0.0, z);
            self.play_tone_at(position, *freq, Duration::from_millis(500))?;
        }

        Ok(())
    }
}

/// Spatial audio presets for common proof patterns
pub struct SpatialAudioPresets;

impl SpatialAudioPresets {
    /// Linear proof (axioms to theorem)
    pub fn linear_proof() -> Vec<AudioPosition> {
        (0..5)
            .map(|i| AudioPosition::new(0.0, 0.0, -(i as f32 * 2.0)))
            .collect()
    }

    /// Branching proof (one axiom, multiple conclusions)
    pub fn branching_proof() -> Vec<AudioPosition> {
        vec![
            AudioPosition::new(0.0, 0.0, 0.0),
            AudioPosition::new(-2.0, 0.0, -2.0),
            AudioPosition::new(0.0, 0.0, -2.0),
            AudioPosition::new(2.0, 0.0, -2.0),
        ]
    }

    /// Converging proof (multiple axioms, one theorem)
    pub fn converging_proof() -> Vec<AudioPosition> {
        vec![
            AudioPosition::new(-2.0, 0.0, 0.0),
            AudioPosition::new(0.0, 0.0, 0.0),
            AudioPosition::new(2.0, 0.0, 0.0),
            AudioPosition::new(0.0, 0.0, -2.0),
        ]
    }

    /// Circular reasoning detection
    pub fn circular_pattern() -> Vec<AudioPosition> {
        let steps = 8;
        (0..steps)
            .map(|i| {
                let angle = 2.0 * std::f32::consts::PI * (i as f32) / (steps as f32);
                AudioPosition::new(2.0 * angle.cos(), 0.0, -2.0 * angle.sin())
            })
            .collect()
    }

    /// Deep proof (many levels)
    pub fn deep_proof(depth: usize) -> Vec<AudioPosition> {
        (0..depth)
            .map(|i| AudioPosition::new(0.0, -(i as f32 * 0.5), -(i as f32 * 2.0)))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_position() {
        let pos1 = AudioPosition::new(0.0, 0.0, 0.0);
        let pos2 = AudioPosition::new(1.0, 0.0, 0.0);
        let distance = pos1.distance_to(&pos2);
        assert!((distance - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_listener() {
        let mut listener = Listener::new();
        listener.move_to(AudioPosition::new(1.0, 2.0, 3.0));
        assert_eq!(listener.position.x, 1.0);
    }

    #[test]
    fn test_audio_source() {
        let source = AudioSource::new(AudioPosition::new(0.0, 0.0, 0.0), 440.0);
        let listener = Listener::new();
        let volume = source.perceived_volume(&listener);
        assert!(volume > 0.0);
    }

    #[test]
    fn test_panning() {
        let left_source = AudioSource::new(AudioPosition::new(-5.0, 0.0, 0.0), 440.0);
        let listener = Listener::new();
        let (left, right) = left_source.panning(&listener);
        assert!(left > right);
    }

    #[test]
    fn test_spatial_audio_presets() {
        let linear = SpatialAudioPresets::linear_proof();
        assert_eq!(linear.len(), 5);

        let branching = SpatialAudioPresets::branching_proof();
        assert!(branching.len() > 1);
    }
}
