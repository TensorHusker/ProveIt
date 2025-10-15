//! Proof Bridge: Bidirectional mapping between geometric constructions and SCTT proofs

use sctt_core::{check_type, eval, infer, Context, Env, Expr, Value};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::construction::{Construction, ConstructionGraph};
use crate::line::{Line, LineId, ProofPath};
use crate::point::{Point, PointId};
use crate::{Error, Result};

/// The bridge between geometric and logical representations
pub struct ProofBridge {
    /// Type checking context
    context: Context,
    /// Evaluation environment
    environment: Env,
    /// Mapping from geometric elements to proof terms
    geo_to_proof: GeoToProofMap,
    /// Mapping from proof terms to geometric elements
    proof_to_geo: ProofToGeoMap,
}

#[derive(Debug, Clone, Default)]
struct GeoToProofMap {
    /// Point ID -> Proof term
    points: HashMap<PointId, Expr>,
    /// Line ID -> Proof term
    lines: HashMap<LineId, Expr>,
}

#[derive(Debug, Clone, Default)]
struct ProofToGeoMap {
    /// Proof term -> Point ID (using string representation as key)
    points: HashMap<String, PointId>,
    /// Proof term -> Line ID
    lines: HashMap<String, LineId>,
}

impl ProofBridge {
    pub fn new() -> Self {
        Self {
            context: Context::new(),
            environment: Env::new(),
            geo_to_proof: GeoToProofMap::default(),
            proof_to_geo: ProofToGeoMap::default(),
        }
    }

    /// Register a point with its corresponding proof term
    pub fn register_point(&mut self, point: &Point) -> Result<()> {
        // Type check the proposition
        let ty = infer(&self.context, &point.proposition)
            .map_err(|e| Error::ProofCorrespondence(format!("Type checking failed: {}", e)))?;

        // Store mapping
        self.geo_to_proof
            .points
            .insert(point.id, point.proposition.clone());
        self.proof_to_geo
            .points
            .insert(format!("{:?}", point.proposition), point.id);

        // Add to context
        self.context = self.context.extend(sctt_core::Name(point.label.clone()), ty);

        Ok(())
    }

    /// Register a line with its corresponding proof term
    pub fn register_line(&mut self, line: &Line, from: &Point, to: &Point) -> Result<()> {
        // Get the propositions
        let from_prop =
            self.geo_to_proof.points.get(&from.id).ok_or_else(|| {
                Error::ProofCorrespondence("Source point not registered".to_string())
            })?;
        let to_prop =
            self.geo_to_proof.points.get(&to.id).ok_or_else(|| {
                Error::ProofCorrespondence("Target point not registered".to_string())
            })?;

        // The line represents a function from_prop -> to_prop
        // Type check the proof term
        let expected_ty_expr = Expr::Pi {
            name: sctt_core::Name("_".to_string()),
            domain: Box::new(from_prop.clone()),
            codomain: Box::new(to_prop.clone()),
        };

        // Evaluate the expected type to a Value
        let expected_ty = eval(&expected_ty_expr, self.context.env());

        // Check that the proof term has the expected type
        check_type(&self.context, &line.proof_term, &expected_ty)
            .map_err(|e| Error::ProofCorrespondence(format!("Line type mismatch: {}", e)))?;

        // Store mapping
        self.geo_to_proof
            .lines
            .insert(line.id, line.proof_term.clone());
        self.proof_to_geo
            .lines
            .insert(format!("{:?}", line.proof_term), line.id);

        Ok(())
    }

    /// Convert a construction to a complete proof term
    pub fn construction_to_proof(&self, construction: &Construction) -> Result<Expr> {
        // Find the main theorem (target)
        if let Some(target) = &construction.target {
            // Find a proof path from axioms to target
            let target_point = construction
                .graph
                .points
                .values()
                .find(|p| &p.proposition == target)
                .ok_or_else(|| {
                    Error::ProofCorrespondence("Target not found in construction".to_string())
                })?;

            // Find an axiom that leads to this theorem
            let axioms = construction.axioms();
            for axiom_id in axioms {
                if let Some(path) = construction.find_proof_path(axiom_id, target_point.id) {
                    return self.proof_path_to_term(&path, &construction.graph);
                }
            }

            Err(Error::ProofCorrespondence(
                "No proof path found".to_string(),
            ))
        } else {
            Err(Error::ProofCorrespondence(
                "Construction has no target".to_string(),
            ))
        }
    }

    /// Convert a proof path to a proof term
    pub fn proof_path_to_term(&self, path: &ProofPath, graph: &ConstructionGraph) -> Result<Expr> {
        if path.lines.is_empty() {
            // Trivial path - use identity
            let start_point = graph
                .points
                .get(&path.start)
                .ok_or_else(|| Error::ProofCorrespondence("Start point not found".to_string()))?;
            return Ok(start_point.proposition.clone());
        }

        // Compose the proof terms
        let mut composite = None;

        for &line_id in &path.lines {
            let line = graph
                .lines
                .get(&line_id)
                .ok_or_else(|| Error::ProofCorrespondence("Line not found".to_string()))?;

            let proof_term = self
                .geo_to_proof
                .lines
                .get(&line_id)
                .ok_or_else(|| Error::ProofCorrespondence("Line not registered".to_string()))?;

            composite = Some(match composite {
                None => proof_term.clone(),
                Some(prev) => {
                    // Function composition: current(prev)
                    Expr::App {
                        func: Box::new(proof_term.clone()),
                        arg: Box::new(prev),
                    }
                }
            });
        }

        composite.ok_or_else(|| Error::ProofCorrespondence("Empty path".to_string()))
    }

    /// Convert a proof term to a geometric construction
    pub fn proof_to_construction(&self, proof: &Expr, name: String) -> Result<Construction> {
        let mut construction = Construction::new(name);

        // Analyze the proof structure and build geometry
        self.analyze_and_build(proof, &mut construction)?;

        Ok(construction)
    }

    /// Recursively analyze a proof term and build geometric structure
    fn analyze_and_build(&self, expr: &Expr, construction: &mut Construction) -> Result<PointId> {
        match expr {
            Expr::Var(name, _) => {
                // Variable represents a point
                let point = Point::new(
                    PointId(0),
                    nalgebra::Point2::new(0.0, 0.0),
                    expr.clone(),
                    name.0.clone(),
                );
                Ok(construction.add_point(point))
            }

            Expr::Lambda { name, body, .. } => {
                // Lambda represents an implication
                let body_point = self.analyze_and_build(body, construction)?;

                // Create a point for the lambda itself
                let lambda_point = Point::new(
                    PointId(0),
                    nalgebra::Point2::new(0.0, 0.0),
                    expr.clone(),
                    format!("λ{}", name.0),
                );
                let lambda_id = construction.add_point(lambda_point);

                // Add line from body to lambda
                let line = Line::new(
                    LineId(0),
                    body_point,
                    lambda_id,
                    expr.clone(),
                    format!("λ{}", name.0),
                );
                construction.add_line(line)?;

                Ok(lambda_id)
            }

            Expr::App { func, arg } => {
                // Application represents modus ponens
                let func_point = self.analyze_and_build(func, construction)?;
                let arg_point = self.analyze_and_build(arg, construction)?;

                // Create result point
                let app_point = Point::new(
                    PointId(0),
                    nalgebra::Point2::new(0.0, 0.0),
                    expr.clone(),
                    "app".to_string(),
                );
                let app_id = construction.add_point(app_point);

                // Add lines
                let line1 = Line::new(
                    LineId(0),
                    func_point,
                    app_id,
                    func.as_ref().clone(),
                    "func".to_string(),
                );
                let line2 = Line::new(
                    LineId(0),
                    arg_point,
                    app_id,
                    arg.as_ref().clone(),
                    "arg".to_string(),
                );

                construction.add_line(line1)?;
                construction.add_line(line2)?;

                Ok(app_id)
            }

            Expr::Pi {
                name,
                domain,
                codomain,
            } => {
                // Pi type represents a proof obligation
                let domain_point = self.analyze_and_build(domain, construction)?;
                let codomain_point = self.analyze_and_build(codomain, construction)?;

                // Create Pi point
                let pi_point = Point::new(
                    PointId(0),
                    nalgebra::Point2::new(0.0, 0.0),
                    expr.clone(),
                    format!("Π{}", name.0),
                );
                let pi_id = construction.add_point(pi_point);

                Ok(pi_id)
            }

            _ => {
                // Other terms get represented as simple points
                let point = Point::new(
                    PointId(0),
                    nalgebra::Point2::new(0.0, 0.0),
                    expr.clone(),
                    "term".to_string(),
                );
                Ok(construction.add_point(point))
            }
        }
    }

    /// Verify that a construction correctly represents its corresponding proof
    pub fn verify_correspondence(&self, construction: &Construction) -> Result<()> {
        // Check all points are well-typed
        for point in construction.graph.points.values() {
            infer(&self.context, &point.proposition)
                .map_err(|e| Error::ProofCorrespondence(format!("Point ill-typed: {}", e)))?;
        }

        // Check all lines are well-typed implications
        for line in construction.graph.lines.values() {
            let from =
                construction.graph.points.get(&line.from).ok_or_else(|| {
                    Error::ProofCorrespondence("Line source not found".to_string())
                })?;
            let to =
                construction.graph.points.get(&line.to).ok_or_else(|| {
                    Error::ProofCorrespondence("Line target not found".to_string())
                })?;

            // Verify the proof term
            let expected_ty_expr = Expr::Pi {
                name: sctt_core::Name("_".to_string()),
                domain: Box::new(from.proposition.clone()),
                codomain: Box::new(to.proposition.clone()),
            };

            let expected_ty = eval(&expected_ty_expr, self.context.env());

            check_type(&self.context, &line.proof_term, &expected_ty)
                .map_err(|e| Error::ProofCorrespondence(format!("Line ill-typed: {}", e)))?;
        }

        Ok(())
    }

    /// Get the type of a geometric element
    pub fn get_point_type(&self, point_id: PointId) -> Option<&Expr> {
        self.geo_to_proof.points.get(&point_id)
    }

    /// Get the proof term for a line
    pub fn get_line_proof(&self, line_id: LineId) -> Option<&Expr> {
        self.geo_to_proof.lines.get(&line_id)
    }

    /// Evaluate a geometric construction to a value
    pub fn evaluate_construction(&self, construction: &Construction) -> Result<Value> {
        let proof_term = self.construction_to_proof(construction)?;
        Ok(eval(&proof_term, &self.environment))
    }
}

impl Default for ProofBridge {
    fn default() -> Self {
        Self::new()
    }
}

/// Utility for automatic layout of constructions
pub struct ConstructionLayout {
    /// Width of the layout area
    pub width: f64,
    /// Height of the layout area
    pub height: f64,
}

impl ConstructionLayout {
    pub fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }

    /// Perform automatic layout using force-directed algorithm
    pub fn layout(&self, construction: &mut Construction) {
        let points = construction.graph.points.len();
        if points == 0 {
            return;
        }

        // Simple circular layout
        let center_x = self.width / 2.0;
        let center_y = self.height / 2.0;
        let radius = self.width.min(self.height) / 3.0;

        let mut point_ids: Vec<PointId> = construction.graph.points.keys().copied().collect();
        point_ids.sort_by_key(|id| id.0);

        for (i, point_id) in point_ids.iter().enumerate() {
            let angle = 2.0 * std::f64::consts::PI * (i as f64) / (points as f64);
            let x = center_x + radius * angle.cos();
            let y = center_y + radius * angle.sin();

            if let Some(point) = construction.graph.points.get_mut(point_id) {
                point.set_position(nalgebra::Point2::new(x, y));
            }
        }
    }

    /// Hierarchical layout for proofs (axioms at top, theorems at bottom)
    pub fn layout_hierarchical(&self, construction: &mut Construction) {
        let axioms = construction.axioms();
        let depth = construction.depth();

        if depth == 0 {
            return;
        }

        // Compute levels for each point
        let mut levels: HashMap<PointId, usize> = HashMap::new();
        for axiom in &axioms {
            self.compute_levels(*axiom, 0, &construction.graph, &mut levels);
        }

        // Group points by level
        let mut levels_vec: Vec<Vec<PointId>> = vec![Vec::new(); depth + 1];
        for (point_id, level) in levels {
            levels_vec[level].push(point_id);
        }

        // Layout each level
        let level_height = self.height / ((depth + 1) as f64);

        for (level, point_ids) in levels_vec.iter().enumerate() {
            let y = level_height * (level as f64 + 0.5);
            let level_width = self.width / ((point_ids.len() + 1) as f64);

            for (i, point_id) in point_ids.iter().enumerate() {
                let x = level_width * (i as f64 + 1.0);
                if let Some(point) = construction.graph.points.get_mut(point_id) {
                    point.set_position(nalgebra::Point2::new(x, y));
                }
            }
        }
    }

    fn compute_levels(
        &self,
        point: PointId,
        level: usize,
        graph: &ConstructionGraph,
        levels: &mut HashMap<PointId, usize>,
    ) {
        let current_level = levels.get(&point).copied().unwrap_or(usize::MAX);
        if level < current_level {
            levels.insert(point, level);

            // Recurse to children
            if let Some(outgoing) = graph.outgoing.get(&point) {
                for &line_id in outgoing {
                    if let Some(line) = graph.lines.get(&line_id) {
                        self.compute_levels(line.to, level + 1, graph, levels);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::Point2;
    use sctt_core::Name;

    #[test]
    fn test_proof_bridge_creation() {
        let bridge = ProofBridge::new();
        assert_eq!(bridge.geo_to_proof.points.len(), 0);
    }

    #[test]
    fn test_register_point() {
        let mut bridge = ProofBridge::new();
        let point = Point::new(
            PointId(0),
            Point2::new(0.0, 0.0),
            Expr::Type(0),
            "A".to_string(),
        );

        assert!(bridge.register_point(&point).is_ok());
    }

    #[test]
    fn test_construction_layout() {
        let mut construction = Construction::new("Test".to_string());

        let p1 = Point::new(
            PointId(0),
            Point2::new(0.0, 0.0),
            Expr::Type(0),
            "A".to_string(),
        );
        let p2 = Point::new(
            PointId(0),
            Point2::new(0.0, 0.0),
            Expr::Type(0),
            "B".to_string(),
        );

        construction.add_point(p1);
        construction.add_point(p2);

        let layout = ConstructionLayout::new(800.0, 600.0);
        layout.layout(&mut construction);

        // Verify points have been positioned
        for point in construction.graph.points.values() {
            assert!(point.position.x >= 0.0);
            assert!(point.position.y >= 0.0);
        }
    }
}
