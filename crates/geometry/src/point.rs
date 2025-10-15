//! Points: Geometric representations of propositions

use nalgebra::Point2;
use sctt_core::{Expr, Value};
use serde::{Deserialize, Serialize};

/// A point in the geometric proof space
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Point {
    /// Unique identifier
    pub id: PointId,
    /// Spatial position (for visualization)
    pub position: Point2<f64>,
    /// The proposition this point represents
    pub proposition: Expr,
    /// Evaluated type (cached for efficiency)
    #[serde(skip)]
    pub ty: Option<Value>,
    /// Human-readable label
    pub label: String,
    /// Accessibility metadata
    pub accessibility: AccessibilityData,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PointId(pub u64);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessibilityData {
    /// Audio description for screen readers
    pub audio_description: String,
    /// Haptic feedback pattern (for tactile interfaces)
    pub haptic_pattern: HapticPattern,
    /// Sonification: musical pitch representing logical complexity
    pub pitch_hz: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HapticPattern {
    /// Single pulse
    Pulse { duration_ms: u32 },
    /// Rhythmic pattern
    Rhythm { intervals_ms: Vec<u32> },
    /// Continuous vibration
    Continuous { duration_ms: u32, intensity: f32 },
}

impl Point {
    pub fn new(id: PointId, position: Point2<f64>, proposition: Expr, label: String) -> Self {
        let audio_description = format!(
            "Point {} at position {:.1}, {:.1} representing {}",
            label, position.x, position.y, proposition
        );

        // Complexity-based pitch: simpler = lower pitch
        let complexity = estimate_complexity(&proposition);
        let pitch_hz = 200.0 + (complexity as f32 * 50.0);

        Self {
            id,
            position,
            proposition,
            ty: None,
            label,
            accessibility: AccessibilityData {
                audio_description,
                haptic_pattern: HapticPattern::Pulse { duration_ms: 100 },
                pitch_hz,
            },
        }
    }

    /// Convert point to SCTT proof term
    pub fn to_proof_term(&self) -> Expr {
        self.proposition.clone()
    }

    /// Update spatial position (for visualization)
    pub fn set_position(&mut self, pos: Point2<f64>) {
        self.position = pos;
        // Update audio description
        self.accessibility.audio_description = format!(
            "Point {} at position {:.1}, {:.1}",
            self.label, pos.x, pos.y
        );
    }
}

/// Estimate logical complexity for sonification
fn estimate_complexity(expr: &Expr) -> u32 {
    match expr {
        Expr::Type(_) | Expr::Var(_, _) => 1,
        Expr::Pi {
            domain, codomain, ..
        } => 1 + estimate_complexity(domain) + estimate_complexity(codomain),
        Expr::Lambda { body, .. } => 1 + estimate_complexity(body),
        Expr::App { func, arg } => 1 + estimate_complexity(func) + estimate_complexity(arg),
        Expr::Path { ty, left, right } => {
            1 + estimate_complexity(ty) + estimate_complexity(left) + estimate_complexity(right)
        }
        _ => 2,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sctt_core::Name;

    #[test]
    fn test_point_creation() {
        let point = Point::new(
            PointId(0),
            Point2::new(0.0, 0.0),
            Expr::Type(0),
            "A".to_string(),
        );

        assert_eq!(point.label, "A");
        assert!(point.accessibility.audio_description.contains("Point A"));
    }

    #[test]
    fn test_complexity_estimation() {
        let simple = Expr::Type(0);
        assert_eq!(estimate_complexity(&simple), 1);

        let complex = Expr::Pi {
            name: Name("x".to_string()),
            domain: Box::new(Expr::Type(0)),
            codomain: Box::new(Expr::Type(0)),
        };
        assert!(estimate_complexity(&complex) > 1);
    }
}
