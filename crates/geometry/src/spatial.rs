//! Spatial relations: Geometric relationships between proof elements

use nalgebra::{Point2, Vector2};
use serde::{Deserialize, Serialize};

use crate::line::{Line, LineId};
use crate::point::{Point, PointId};

/// Spatial relation between geometric elements
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SpatialRelation {
    /// Two lines are parallel
    Parallel { line1: LineId, line2: LineId },
    /// Two lines are perpendicular
    Perpendicular { line1: LineId, line2: LineId },
    /// A point lies on a line
    Incidence { point: PointId, line: LineId },
    /// Lines intersect at a point
    Intersection {
        line1: LineId,
        line2: LineId,
        point: PointId,
    },
    /// Points are collinear
    Collinear { points: Vec<PointId> },
    /// Points form a closed region
    Region { points: Vec<PointId> },
}

/// Position descriptor for accessibility
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    /// Cartesian coordinates
    pub coords: Point2<f64>,
    /// Relative position description
    pub description: String,
    /// Polar coordinates (for circular arrangements)
    pub polar: Option<(f64, f64)>, // (radius, angle in radians)
}

impl Position {
    pub fn new(x: f64, y: f64) -> Self {
        let description = describe_position(x, y);
        let polar = Some(cartesian_to_polar(x, y));

        Self {
            coords: Point2::new(x, y),
            description,
            polar,
        }
    }

    pub fn from_point(point: &Point) -> Self {
        Self::new(point.position.x, point.position.y)
    }

    /// Distance from origin
    pub fn distance_from_origin(&self) -> f64 {
        (self.coords.x * self.coords.x + self.coords.y * self.coords.y).sqrt()
    }

    /// Distance to another position
    pub fn distance_to(&self, other: &Position) -> f64 {
        let dx = other.coords.x - self.coords.x;
        let dy = other.coords.y - self.coords.y;
        (dx * dx + dy * dy).sqrt()
    }

    /// Bearing to another position (in radians)
    pub fn bearing_to(&self, other: &Position) -> f64 {
        let dx = other.coords.x - self.coords.x;
        let dy = other.coords.y - self.coords.y;
        dy.atan2(dx)
    }
}

/// Spatial analyzer for geometric constructions
pub struct SpatialAnalyzer {
    /// Tolerance for floating-point comparisons
    pub epsilon: f64,
}

impl SpatialAnalyzer {
    pub fn new() -> Self {
        Self { epsilon: 1e-6 }
    }

    pub fn with_epsilon(epsilon: f64) -> Self {
        Self { epsilon }
    }

    /// Check if two lines are parallel
    pub fn are_parallel(
        &self,
        line1: &Line,
        p1_from: &Point,
        p1_to: &Point,
        line2: &Line,
        p2_from: &Point,
        p2_to: &Point,
    ) -> bool {
        let d1 = direction_vector(p1_from, p1_to);
        let d2 = direction_vector(p2_from, p2_to);

        // Parallel if cross product is near zero
        let cross = d1.x * d2.y - d1.y * d2.x;
        cross.abs() < self.epsilon
    }

    /// Check if two lines are perpendicular
    pub fn are_perpendicular(
        &self,
        line1: &Line,
        p1_from: &Point,
        p1_to: &Point,
        line2: &Line,
        p2_from: &Point,
        p2_to: &Point,
    ) -> bool {
        let d1 = direction_vector(p1_from, p1_to);
        let d2 = direction_vector(p2_from, p2_to);

        // Perpendicular if dot product is near zero
        let dot = d1.x * d2.x + d1.y * d2.y;
        dot.abs() < self.epsilon
    }

    /// Check if a point lies on a line (within epsilon)
    pub fn is_incident(
        &self,
        point: &Point,
        line: &Line,
        line_from: &Point,
        line_to: &Point,
    ) -> bool {
        // Use point-to-line distance
        let distance = point_to_line_distance(point, line_from, line_to);
        distance < self.epsilon
    }

    /// Find intersection point of two lines
    pub fn find_intersection(
        &self,
        line1: &Line,
        p1_from: &Point,
        p1_to: &Point,
        line2: &Line,
        p2_from: &Point,
        p2_to: &Point,
    ) -> Option<Point2<f64>> {
        let d1 = direction_vector(p1_from, p1_to);
        let d2 = direction_vector(p2_from, p2_to);

        let det = d1.x * d2.y - d1.y * d2.x;
        if det.abs() < self.epsilon {
            return None; // Parallel lines
        }

        let dx = p2_from.position.x - p1_from.position.x;
        let dy = p2_from.position.y - p1_from.position.y;

        let t = (dx * d2.y - dy * d2.x) / det;

        Some(Point2::new(
            p1_from.position.x + t * d1.x,
            p1_from.position.y + t * d1.y,
        ))
    }

    /// Check if points are collinear
    pub fn are_collinear(&self, points: &[&Point]) -> bool {
        if points.len() < 3 {
            return true;
        }

        let p0 = points[0].position;
        let p1 = points[1].position;

        // Check each subsequent point
        for i in 2..points.len() {
            let p2 = points[i].position;

            // Calculate cross product
            let dx1 = p1.x - p0.x;
            let dy1 = p1.y - p0.y;
            let dx2 = p2.x - p0.x;
            let dy2 = p2.y - p0.y;

            let cross = dx1 * dy2 - dy1 * dx2;
            if cross.abs() > self.epsilon {
                return false;
            }
        }

        true
    }

    /// Compute the area of a polygon defined by points
    pub fn compute_area(&self, points: &[&Point]) -> f64 {
        if points.len() < 3 {
            return 0.0;
        }

        let mut area = 0.0;
        for i in 0..points.len() {
            let j = (i + 1) % points.len();
            let p1 = points[i].position;
            let p2 = points[j].position;
            area += p1.x * p2.y - p2.x * p1.y;
        }

        area.abs() / 2.0
    }

    /// Check if points form a convex polygon
    pub fn is_convex(&self, points: &[&Point]) -> bool {
        if points.len() < 3 {
            return false;
        }

        let mut sign: Option<bool> = None;

        for i in 0..points.len() {
            let p0 = points[i].position;
            let p1 = points[(i + 1) % points.len()].position;
            let p2 = points[(i + 2) % points.len()].position;

            let dx1 = p1.x - p0.x;
            let dy1 = p1.y - p0.y;
            let dx2 = p2.x - p1.x;
            let dy2 = p2.y - p1.y;

            let cross = dx1 * dy2 - dy1 * dx2;

            if cross.abs() > self.epsilon {
                let cross_positive = cross > 0.0;
                match sign {
                    None => sign = Some(cross_positive),
                    Some(s) => {
                        if s != cross_positive {
                            return false;
                        }
                    }
                }
            }
        }

        true
    }

    /// Detect all spatial relations in a construction
    pub fn detect_relations(&self, points: &[Point], lines: &[Line]) -> Vec<SpatialRelation> {
        let mut relations = Vec::new();

        // Check all pairs of lines
        for i in 0..lines.len() {
            for j in (i + 1)..lines.len() {
                let line1 = &lines[i];
                let line2 = &lines[j];

                let p1_from = points.iter().find(|p| p.id == line1.from).unwrap();
                let p1_to = points.iter().find(|p| p.id == line1.to).unwrap();
                let p2_from = points.iter().find(|p| p.id == line2.from).unwrap();
                let p2_to = points.iter().find(|p| p.id == line2.to).unwrap();

                if self.are_parallel(line1, p1_from, p1_to, line2, p2_from, p2_to) {
                    relations.push(SpatialRelation::Parallel {
                        line1: line1.id,
                        line2: line2.id,
                    });
                }

                if self.are_perpendicular(line1, p1_from, p1_to, line2, p2_from, p2_to) {
                    relations.push(SpatialRelation::Perpendicular {
                        line1: line1.id,
                        line2: line2.id,
                    });
                }
            }
        }

        // Check incidence relations
        for point in points {
            for line in lines {
                let line_from = points.iter().find(|p| p.id == line.from).unwrap();
                let line_to = points.iter().find(|p| p.id == line.to).unwrap();

                if point.id != line.from && point.id != line.to {
                    if self.is_incident(point, line, line_from, line_to) {
                        relations.push(SpatialRelation::Incidence {
                            point: point.id,
                            line: line.id,
                        });
                    }
                }
            }
        }

        relations
    }
}

impl Default for SpatialAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper: Compute direction vector of a line
fn direction_vector(from: &Point, to: &Point) -> Vector2<f64> {
    Vector2::new(
        to.position.x - from.position.x,
        to.position.y - from.position.y,
    )
}

/// Helper: Distance from a point to a line
fn point_to_line_distance(point: &Point, line_from: &Point, line_to: &Point) -> f64 {
    let d = direction_vector(line_from, line_to);
    let length_sq = d.x * d.x + d.y * d.y;

    if length_sq < 1e-10 {
        // Line is actually a point
        let dx = point.position.x - line_from.position.x;
        let dy = point.position.y - line_from.position.y;
        return (dx * dx + dy * dy).sqrt();
    }

    // Project point onto line
    let px = point.position.x - line_from.position.x;
    let py = point.position.y - line_from.position.y;

    let t = ((px * d.x + py * d.y) / length_sq).clamp(0.0, 1.0);

    let proj_x = line_from.position.x + t * d.x;
    let proj_y = line_from.position.y + t * d.y;

    let dx = point.position.x - proj_x;
    let dy = point.position.y - proj_y;

    (dx * dx + dy * dy).sqrt()
}

/// Helper: Convert Cartesian to polar coordinates
fn cartesian_to_polar(x: f64, y: f64) -> (f64, f64) {
    let r = (x * x + y * y).sqrt();
    let theta = y.atan2(x);
    (r, theta)
}

/// Helper: Describe position in natural language
fn describe_position(x: f64, y: f64) -> String {
    let quadrant = match (x >= 0.0, y >= 0.0) {
        (true, true) => "upper right",
        (false, true) => "upper left",
        (false, false) => "lower left",
        (true, false) => "lower right",
    };

    let distance = (x * x + y * y).sqrt();
    let distance_desc = if distance < 1.0 {
        "near the center"
    } else if distance < 5.0 {
        "moderate distance"
    } else {
        "far from center"
    };

    format!("{}, {} at ({:.1}, {:.1})", distance_desc, quadrant, x, y)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::line::LineId;
    use sctt_core::Expr;

    fn make_point(id: u64, x: f64, y: f64) -> Point {
        Point::new(
            PointId(id),
            Point2::new(x, y),
            Expr::Type(0),
            format!("P{}", id),
        )
    }

    fn make_line(id: u64, from: u64, to: u64) -> Line {
        Line::new(
            LineId(id),
            PointId(from),
            PointId(to),
            Expr::Type(0),
            format!("L{}", id),
        )
    }

    #[test]
    fn test_position_creation() {
        let pos = Position::new(3.0, 4.0);
        assert_eq!(pos.distance_from_origin(), 5.0);
        assert!(pos.description.contains("upper right"));
    }

    #[test]
    fn test_parallel_detection() {
        let analyzer = SpatialAnalyzer::new();

        let p1 = make_point(0, 0.0, 0.0);
        let p2 = make_point(1, 1.0, 0.0);
        let p3 = make_point(2, 0.0, 1.0);
        let p4 = make_point(3, 1.0, 1.0);

        let line1 = make_line(0, 0, 1);
        let line2 = make_line(1, 2, 3);

        assert!(analyzer.are_parallel(&line1, &p1, &p2, &line2, &p3, &p4));
    }

    #[test]
    fn test_perpendicular_detection() {
        let analyzer = SpatialAnalyzer::new();

        let p1 = make_point(0, 0.0, 0.0);
        let p2 = make_point(1, 1.0, 0.0);
        let p3 = make_point(2, 0.0, 0.0);
        let p4 = make_point(3, 0.0, 1.0);

        let line1 = make_line(0, 0, 1);
        let line2 = make_line(1, 2, 3);

        assert!(analyzer.are_perpendicular(&line1, &p1, &p2, &line2, &p3, &p4));
    }

    #[test]
    fn test_collinearity() {
        let analyzer = SpatialAnalyzer::new();

        let p1 = make_point(0, 0.0, 0.0);
        let p2 = make_point(1, 1.0, 1.0);
        let p3 = make_point(2, 2.0, 2.0);

        let points = vec![&p1, &p2, &p3];
        assert!(analyzer.are_collinear(&points));
    }

    #[test]
    fn test_area_computation() {
        let analyzer = SpatialAnalyzer::new();

        // Unit square
        let p1 = make_point(0, 0.0, 0.0);
        let p2 = make_point(1, 1.0, 0.0);
        let p3 = make_point(2, 1.0, 1.0);
        let p4 = make_point(3, 0.0, 1.0);

        let points = vec![&p1, &p2, &p3, &p4];
        let area = analyzer.compute_area(&points);
        assert!((area - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_convexity() {
        let analyzer = SpatialAnalyzer::new();

        // Convex square
        let p1 = make_point(0, 0.0, 0.0);
        let p2 = make_point(1, 1.0, 0.0);
        let p3 = make_point(2, 1.0, 1.0);
        let p4 = make_point(3, 0.0, 1.0);

        let points = vec![&p1, &p2, &p3, &p4];
        assert!(analyzer.is_convex(&points));
    }
}
