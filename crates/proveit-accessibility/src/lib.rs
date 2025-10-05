//! Accessibility Module
//!
//! Provides spatial audio, customizable layouts, and screen reader support
//! for blind, visually impaired, neurodivergent, and brain injury users.

use proveit_core::Position;
use serde::{Deserialize, Serialize};

/// Spatial audio configuration for 3D sound positioning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpatialAudio {
    /// Listener position in 3D space
    pub listener_position: Position,
    /// Listener orientation (forward direction)
    pub listener_orientation: (f64, f64, f64),
    /// Audio rolloff factor (how quickly sound diminishes with distance)
    pub rolloff_factor: f64,
    /// Maximum audio distance
    pub max_distance: f64,
}

impl SpatialAudio {
    pub fn new() -> Self {
        Self {
            listener_position: Position::origin(),
            listener_orientation: (0.0, 0.0, -1.0), // Looking down negative Z
            rolloff_factor: 1.0,
            max_distance: 100.0,
        }
    }
    
    /// Calculate audio volume and pan based on position
    pub fn calculate_audio_params(&self, sound_position: &Position) -> AudioParams {
        let dx = sound_position.x - self.listener_position.x;
        let dy = sound_position.y - self.listener_position.y;
        let dz = sound_position.z - self.listener_position.z;
        
        let distance = (dx * dx + dy * dy + dz * dz).sqrt();
        
        // Calculate volume based on distance
        let volume = if distance < self.max_distance {
            1.0 - (distance / self.max_distance).powf(self.rolloff_factor)
        } else {
            0.0
        };
        
        // Calculate pan based on horizontal angle
        let angle = dx.atan2(dz);
        let pan = (angle / std::f64::consts::PI).clamp(-1.0, 1.0);
        
        AudioParams {
            volume: volume.clamp(0.0, 1.0),
            pan,
            distance,
        }
    }
    
    /// Update listener position
    pub fn set_listener(&mut self, position: Position, orientation: (f64, f64, f64)) {
        self.listener_position = position;
        self.listener_orientation = orientation;
    }
    
    /// Describe audio setup for screen readers
    pub fn describe(&self) -> String {
        format!(
            "Spatial audio: listener at {}, facing ({:.2}, {:.2}, {:.2}), max distance {:.2}",
            self.listener_position,
            self.listener_orientation.0,
            self.listener_orientation.1,
            self.listener_orientation.2,
            self.max_distance
        )
    }
}

impl Default for SpatialAudio {
    fn default() -> Self {
        Self::new()
    }
}

/// Audio parameters for a sound source
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioParams {
    /// Volume (0.0 to 1.0)
    pub volume: f64,
    /// Pan (-1.0 left to 1.0 right)
    pub pan: f64,
    /// Distance from listener
    pub distance: f64,
}

/// Layout configuration for customizable UI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LayoutMode {
    /// List-based layout for screen readers
    List,
    /// Grid layout with customizable spacing
    Grid { rows: usize, cols: usize },
    /// Spatial layout in 2D
    Spatial2D,
    /// Spatial layout in 3D
    Spatial3D,
    /// Custom layout with user-defined positions
    Custom,
}

/// Accessibility settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessibilitySettings {
    /// Enable screen reader support
    pub screen_reader_enabled: bool,
    /// Enable spatial audio
    pub spatial_audio_enabled: bool,
    /// Layout mode
    pub layout_mode: LayoutMode,
    /// Font size multiplier
    pub font_size_multiplier: f64,
    /// High contrast mode
    pub high_contrast: bool,
    /// Reduce motion for vestibular issues
    pub reduce_motion: bool,
    /// Audio cues for actions
    pub audio_cues: bool,
    /// Haptic feedback
    pub haptic_feedback: bool,
}

impl AccessibilitySettings {
    pub fn new() -> Self {
        Self {
            screen_reader_enabled: false,
            spatial_audio_enabled: false,
            layout_mode: LayoutMode::List,
            font_size_multiplier: 1.0,
            high_contrast: false,
            reduce_motion: false,
            audio_cues: true,
            haptic_feedback: true,
        }
    }
    
    /// Preset for blind users
    pub fn blind_preset() -> Self {
        Self {
            screen_reader_enabled: true,
            spatial_audio_enabled: true,
            layout_mode: LayoutMode::List,
            font_size_multiplier: 1.0,
            high_contrast: false,
            reduce_motion: false,
            audio_cues: true,
            haptic_feedback: true,
        }
    }
    
    /// Preset for visually impaired users
    pub fn visually_impaired_preset() -> Self {
        Self {
            screen_reader_enabled: true,
            spatial_audio_enabled: false,
            layout_mode: LayoutMode::Grid { rows: 3, cols: 3 },
            font_size_multiplier: 2.0,
            high_contrast: true,
            reduce_motion: false,
            audio_cues: true,
            haptic_feedback: true,
        }
    }
    
    /// Preset for neurodivergent users
    pub fn neurodivergent_preset() -> Self {
        Self {
            screen_reader_enabled: false,
            spatial_audio_enabled: false,
            layout_mode: LayoutMode::Grid { rows: 2, cols: 2 },
            font_size_multiplier: 1.2,
            high_contrast: false,
            reduce_motion: true,
            audio_cues: false,
            haptic_feedback: false,
        }
    }
    
    /// Describe settings for screen readers
    pub fn describe(&self) -> String {
        let mut description = String::new();
        
        if self.screen_reader_enabled {
            description.push_str("Screen reader enabled. ");
        }
        if self.spatial_audio_enabled {
            description.push_str("Spatial audio enabled. ");
        }
        if self.high_contrast {
            description.push_str("High contrast mode. ");
        }
        if self.reduce_motion {
            description.push_str("Motion reduced. ");
        }
        
        description.push_str(&format!("Font size: {}x. ", self.font_size_multiplier));
        description.push_str(&format!("Layout: {:?}.", self.layout_mode));
        
        description
    }
}

impl Default for AccessibilitySettings {
    fn default() -> Self {
        Self::new()
    }
}

/// Screen reader announcement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Announcement {
    pub text: String,
    pub priority: AnnouncementPriority,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnnouncementPriority {
    /// Low priority, can be interrupted
    Polite,
    /// Medium priority
    Assertive,
    /// High priority, interrupts everything
    Alert,
}

impl Announcement {
    pub fn new(text: String, priority: AnnouncementPriority) -> Self {
        Self { text, priority }
    }
    
    pub fn polite(text: String) -> Self {
        Self::new(text, AnnouncementPriority::Polite)
    }
    
    pub fn assertive(text: String) -> Self {
        Self::new(text, AnnouncementPriority::Assertive)
    }
    
    pub fn alert(text: String) -> Self {
        Self::new(text, AnnouncementPriority::Alert)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spatial_audio() {
        let audio = SpatialAudio::new();
        let sound_pos = Position::new(10.0, 0.0, 0.0);
        
        let params = audio.calculate_audio_params(&sound_pos);
        assert!(params.volume > 0.0);
        assert!(params.pan > 0.0); // Sound to the right
    }

    #[test]
    fn test_accessibility_presets() {
        let blind = AccessibilitySettings::blind_preset();
        assert!(blind.screen_reader_enabled);
        assert!(blind.spatial_audio_enabled);
        
        let visually_impaired = AccessibilitySettings::visually_impaired_preset();
        assert!(visually_impaired.high_contrast);
        assert!(visually_impaired.font_size_multiplier > 1.0);
        
        let neurodivergent = AccessibilitySettings::neurodivergent_preset();
        assert!(neurodivergent.reduce_motion);
    }

    #[test]
    fn test_announcements() {
        let announcement = Announcement::alert("Test alert".to_string());
        assert_eq!(announcement.priority, AnnouncementPriority::Alert);
        assert_eq!(announcement.text, "Test alert");
    }
}
