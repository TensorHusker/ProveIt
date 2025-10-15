//! Lines: Geometric representations of implications and proof steps

use nalgebra::Point2;
use sctt_core::{Expr, Value};
use serde::{Deserialize, Serialize};

use crate::point::{Point, PointId};

/// A line connecting two points, representing an implication or proof step
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Line {
    /// Unique identifier
    pub id: LineId,
    /// Starting point (hypothesis/premise)
    pub from: PointId,
    /// Ending point (conclusion)
    pub to: PointId,
    /// The proof term that justifies this implication
    pub proof_term: Expr,
    /// Type of the proof term (cached)
    #[serde(skip)]
    pub ty: Option<Value>,
    /// Human-readable label
    pub label: String,
    /// Style information for rendering
    pub style: LineStyle,
    /// Accessibility metadata
    pub accessibility: LineAccessibilityData,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LineId(pub u64);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineStyle {
    /// Line color (RGB)
    pub color: [f32; 3],
    /// Line thickness
    pub thickness: f32,
    /// Dashed or solid
    pub dashed: bool,
    /// Arrow style
    pub arrow: ArrowStyle,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArrowStyle {
    /// No arrow
    None,
    /// Arrow at the end
    Forward,
    /// Arrow at the start
    Backward,
    /// Arrows at both ends
    Bidirectional,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineAccessibilityData {
    /// Audio description
    pub audio_description: String,
    /// Haptic feedback when line is selected
    pub haptic_intensity: f32,
    /// Sonification: frequency modulation representing proof complexity
    pub frequency_hz: f32,
}

impl Line {
    pub fn new(id: LineId, from: PointId, to: PointId, proof_term: Expr, label: String) -> Self {
        let audio_description = format!(
            "Implication {} from point {} to point {}",
            label, from.0, to.0
        );

        // More complex proofs get higher frequencies
        let complexity = estimate_proof_complexity(&proof_term);
        let frequency_hz = 300.0 + (complexity as f32 * 75.0);

        Self {
            id,
            from,
            to,
            proof_term,
            ty: None,
            label,
            style: LineStyle {
                color: [0.2, 0.6, 0.9],
                thickness: 2.0,
                dashed: false,
                arrow: ArrowStyle::Forward,
            },
            accessibility: LineAccessibilityData {
                audio_description,
                haptic_intensity: 0.5,
                frequency_hz,
            },
        }
    }

    /// Check if this line forms a direct connection
    pub fn is_direct(&self) -> bool {
        !self.style.dashed
    }

    /// Convert to SCTT proof term
    pub fn to_proof_term(&self) -> Expr {
        self.proof_term.clone()
    }

    /// Calculate the geometric direction vector
    pub fn direction(&self, from_point: &Point, to_point: &Point) -> Point2<f64> {
        Point2::new(
            to_point.position.x - from_point.position.x,
            to_point.position.y - from_point.position.y,
        )
    }

    /// Calculate the geometric length
    pub fn length(&self, from_point: &Point, to_point: &Point) -> f64 {
        let dx = to_point.position.x - from_point.position.x;
        let dy = to_point.position.y - from_point.position.y;
        (dx * dx + dy * dy).sqrt()
    }

    /// Check if this line intersects with another (for visualization)
    pub fn intersects(
        &self,
        other: &Line,
        from1: &Point,
        to1: &Point,
        from2: &Point,
        to2: &Point,
    ) -> bool {
        // Line segment intersection test
        let d1 = self.direction(from1, to1);
        let d2 = other.direction(from2, to2);

        let det = d1.x * d2.y - d1.y * d2.x;
        if det.abs() < 1e-10 {
            return false; // Parallel lines
        }

        let dx = from2.position.x - from1.position.x;
        let dy = from2.position.y - from1.position.y;

        let t1 = (dx * d2.y - dy * d2.x) / det;
        let t2 = (dx * d1.y - dy * d1.x) / det;

        t1 >= 0.0 && t1 <= 1.0 && t2 >= 0.0 && t2 <= 1.0
    }
}

/// Estimate the complexity of a proof term
fn estimate_proof_complexity(expr: &Expr) -> u32 {
    match expr {
        Expr::Var(_, _) => 1,
        Expr::Lambda { body, .. } => 1 + estimate_proof_complexity(body),
        Expr::App { func, arg } => {
            1 + estimate_proof_complexity(func) + estimate_proof_complexity(arg)
        }
        Expr::Pi {
            domain, codomain, ..
        } => 1 + estimate_proof_complexity(domain) + estimate_proof_complexity(codomain),
        Expr::Path { ty, left, right } => {
            2 + estimate_proof_complexity(ty)
                + estimate_proof_complexity(left)
                + estimate_proof_complexity(right)
        }
        Expr::PathLam { body, .. } => 2 + estimate_proof_complexity(body),
        Expr::PathApp { path, .. } => {
            2 + estimate_proof_complexity(path)
        }
        _ => 1,
    }
}

/// A collection of lines forming a proof path
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofPath {
    /// The lines in order
    pub lines: Vec<LineId>,
    /// Start point
    pub start: PointId,
    /// End point
    pub end: PointId,
    /// Combined proof term
    pub composite_proof: Option<Expr>,
}

impl ProofPath {
    pub fn new(start: PointId, end: PointId) -> Self {
        Self {
            lines: Vec::new(),
            start,
            end,
            composite_proof: None,
        }
    }

    /// Add a line to the path
    pub fn add_line(&mut self, line_id: LineId) {
        self.lines.push(line_id);
    }

    /// Check if the path is valid (all lines connect)
    pub fn is_valid(&self, lines: &[Line]) -> bool {
        if self.lines.is_empty() {
            return self.start == self.end;
        }

        let mut current = self.start;
        for line_id in &self.lines {
            if let Some(line) = lines.iter().find(|l| l.id == *line_id) {
                if line.from != current {
                    return false;
                }
                current = line.to;
            } else {
                return false;
            }
        }

        current == self.end
    }

    /// Compute the composite proof by composing all line proofs
    pub fn compute_composite_proof(&mut self, lines: &[Line]) -> Option<Expr> {
        if self.lines.is_empty() {
            return None;
        }

        // Start with the first proof
        let mut composite = lines
            .iter()
            .find(|l| l.id == self.lines[0])?
            .proof_term
            .clone();

        // Compose subsequent proofs
        for line_id in &self.lines[1..] {
            let line = lines.iter().find(|l| l.id == *line_id)?;
            // Function composition: g ∘ f
            composite = Expr::App {
                func: Box::new(line.proof_term.clone()),
                arg: Box::new(composite),
            };
        }

        self.composite_proof = Some(composite.clone());
        Some(composite)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sctt_core::Name;

    #[test]
    fn test_line_creation() {
        let line = Line::new(
            LineId(0),
            PointId(0),
            PointId(1),
            Expr::Var(Name("f".to_string()), 0),
            "A→B".to_string(),
        );

        assert_eq!(line.from, PointId(0));
        assert_eq!(line.to, PointId(1));
        assert!(line.accessibility.audio_description.contains("Implication"));
    }

    #[test]
    fn test_proof_path() {
        let mut path = ProofPath::new(PointId(0), PointId(2));
        path.add_line(LineId(0));
        path.add_line(LineId(1));

        let lines = vec![
            Line::new(
                LineId(0),
                PointId(0),
                PointId(1),
                Expr::Var(Name("f".to_string()), 0),
                "f".to_string(),
            ),
            Line::new(
                LineId(1),
                PointId(1),
                PointId(2),
                Expr::Var(Name("g".to_string()), 0),
                "g".to_string(),
            ),
        ];

        assert!(path.is_valid(&lines));
    }

    #[test]
    fn test_complexity_estimation() {
        let simple = Expr::Var(Name("x".to_string()), 0);
        assert_eq!(estimate_proof_complexity(&simple), 1);

        let complex = Expr::App {
            func: Box::new(Expr::Var(Name("f".to_string()), 0)),
            arg: Box::new(Expr::Var(Name("x".to_string()), 0)),
        };
        assert!(estimate_proof_complexity(&complex) > 1);
    }
}
