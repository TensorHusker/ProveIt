//! Natural language description generation for proofs

use geometry::{Construction, Line, Point};
use proof_engine::{Goal, ProofState};
use sctt_core::Expr;

use crate::{Error, Result};

/// Generates natural language descriptions of proofs
pub struct DescriptionGenerator {
    /// Verbosity level (1-5)
    verbosity: u8,
    /// Use technical language
    technical: bool,
}

impl DescriptionGenerator {
    pub fn new() -> Self {
        Self {
            verbosity: 3,
            technical: false,
        }
    }

    pub fn with_verbosity(mut self, level: u8) -> Self {
        self.verbosity = level.clamp(1, 5);
        self
    }

    pub fn with_technical_language(mut self, technical: bool) -> Self {
        self.technical = technical;
        self
    }

    /// Describe a proof term
    pub fn describe_expr(&self, expr: &Expr) -> String {
        match expr {
            Expr::Var(name, _) => {
                if self.verbosity >= 3 {
                    format!("the variable {}", name.0)
                } else {
                    name.0.clone()
                }
            }

            Expr::Type(level) => {
                if self.technical {
                    format!("Type level {}", level)
                } else if self.verbosity >= 3 {
                    format!("a type at universe level {}", level)
                } else {
                    "Type".to_string()
                }
            }

            Expr::Pi {
                name,
                domain,
                codomain,
            } => {
                let domain_desc = self.describe_expr(domain);
                let codomain_desc = self.describe_expr(codomain);

                if self.technical {
                    format!("Π({}: {}). {}", name.0, domain_desc, codomain_desc)
                } else if self.verbosity >= 4 {
                    format!(
                        "a function that takes {} of type {} and returns {}",
                        name.0, domain_desc, codomain_desc
                    )
                } else {
                    format!("function from {} to {}", domain_desc, codomain_desc)
                }
            }

            Expr::Lambda { name, body } => {
                let body_desc = self.describe_expr(body);
                if self.technical {
                    format!("λ{}. {}", name.0, body_desc)
                } else if self.verbosity >= 3 {
                    format!("a lambda taking {} and returning {}", name.0, body_desc)
                } else {
                    format!("lambda with parameter {}", name.0)
                }
            }

            Expr::App { func, arg } => {
                let func_desc = self.describe_expr(func);
                let arg_desc = self.describe_expr(arg);
                if self.verbosity >= 3 {
                    format!("{} applied to {}", func_desc, arg_desc)
                } else {
                    format!("{}({})", func_desc, arg_desc)
                }
            }

            Expr::Path { ty, left, right } => {
                let left_desc = self.describe_expr(left);
                let right_desc = self.describe_expr(right);

                if self.technical {
                    format!("Path({}, {})", left_desc, right_desc)
                } else if self.verbosity >= 3 {
                    format!("a path showing {} equals {}", left_desc, right_desc)
                } else {
                    format!("{} = {}", left_desc, right_desc)
                }
            }

            Expr::PathLam { dim, body } => {
                if self.technical {
                    format!("λ^{:?}. {:?}", dim, body)
                } else {
                    format!("path lambda over dimension {:?}", dim)
                }
            }

            Expr::PathApp { path, dim } => {
                if self.technical {
                    format!("{:?} @ {:?}", path, dim)
                } else {
                    "path application".to_string()
                }
            }

            Expr::Comp { .. } => {
                if self.technical {
                    "composition operation".to_string()
                } else {
                    "composed path".to_string()
                }
            }

            Expr::Glue { .. } => {
                if self.technical {
                    "glue type for univalence".to_string()
                } else {
                    "glued types".to_string()
                }
            }

            Expr::SmoothPath { .. } => {
                if self.technical {
                    "smooth path (C^∞)".to_string()
                } else {
                    "smoothly varying path".to_string()
                }
            }

            Expr::Diff { .. } => {
                if self.technical {
                    "differential operator".to_string()
                } else {
                    "derivative".to_string()
                }
            }

            Expr::Integral { .. } => {
                if self.technical {
                    "integral operator".to_string()
                } else {
                    "integral".to_string()
                }
            }

            Expr::Coe { .. } => {
                if self.technical {
                    "coercion along type family".to_string()
                } else {
                    "type coercion".to_string()
                }
            }

            Expr::HComp { .. } => {
                if self.technical {
                    "homogeneous composition".to_string()
                } else {
                    "homogeneous composition".to_string()
                }
            }

            Expr::Taylor { .. } => {
                if self.technical {
                    "Taylor expansion".to_string()
                } else {
                    "polynomial approximation".to_string()
                }
            }
        }
    }

    /// Describe a geometric point
    pub fn describe_point(&self, point: &Point) -> String {
        let prop_desc = self.describe_expr(&point.proposition);

        if self.verbosity >= 4 {
            format!(
                "Point {} at position ({:.1}, {:.1}) representing the proposition: {}",
                point.label, point.position.x, point.position.y, prop_desc
            )
        } else if self.verbosity >= 3 {
            format!("Point {} representing {}", point.label, prop_desc)
        } else {
            format!("Point {}", point.label)
        }
    }

    /// Describe a geometric line
    pub fn describe_line(&self, line: &Line, from: &Point, to: &Point) -> String {
        if self.verbosity >= 4 {
            format!(
                "Line {} connecting {} to {}, representing an implication from {} to {}",
                line.label,
                from.label,
                to.label,
                self.describe_expr(&from.proposition),
                self.describe_expr(&to.proposition)
            )
        } else if self.verbosity >= 3 {
            format!("Line {} from {} to {}", line.label, from.label, to.label)
        } else {
            format!("{} → {}", from.label, to.label)
        }
    }

    /// Describe a construction
    pub fn describe_construction(&self, construction: &Construction) -> String {
        let num_points = construction.graph.points.len();
        let num_lines = construction.graph.lines.len();
        let depth = construction.depth();

        if self.verbosity >= 4 {
            format!(
                "Construction \"{}\" with {} propositions, {} implications, and proof depth {}",
                construction.name, num_points, num_lines, depth
            )
        } else if self.verbosity >= 3 {
            format!(
                "\"{}\" containing {} propositions and {} implications",
                construction.name, num_points, num_lines
            )
        } else {
            format!("\"{}\" ({} steps)", construction.name, num_lines)
        }
    }

    /// Describe a goal
    pub fn describe_goal(&self, goal: &Goal) -> String {
        let prop_desc = self.describe_expr(&goal.proposition);

        if self.verbosity >= 4 {
            if goal.hypotheses.is_empty() {
                format!("Goal: prove {}", prop_desc)
            } else {
                let hyp_count = goal.hypotheses.len();
                format!(
                    "Goal: prove {} using {} hypothesis{}",
                    prop_desc,
                    hyp_count,
                    if hyp_count == 1 { "" } else { "es" }
                )
            }
        } else if self.verbosity >= 3 {
            format!("Prove: {}", prop_desc)
        } else {
            prop_desc
        }
    }

    /// Describe proof state
    pub fn describe_proof_state(&self, state: &ProofState) -> String {
        let num_goals = state.num_goals();
        let num_completed = state.completed_goals().len();

        if num_goals == 0 {
            if self.verbosity >= 3 {
                "Proof complete! No remaining goals.".to_string()
            } else {
                "Complete".to_string()
            }
        } else if self.verbosity >= 4 {
            format!(
                "Proof in progress: {} goal{} remaining, {} completed",
                num_goals,
                if num_goals == 1 { "" } else { "s" },
                num_completed
            )
        } else if self.verbosity >= 3 {
            format!(
                "{} goal{} remaining",
                num_goals,
                if num_goals == 1 { "" } else { "s" }
            )
        } else {
            format!("{}/{} goals", num_completed, num_completed + num_goals)
        }
    }
}

impl Default for DescriptionGenerator {
    fn default() -> Self {
        Self::new()
    }
}

/// Narrator for reading proofs aloud
pub struct ProofNarrator {
    generator: DescriptionGenerator,
}

impl ProofNarrator {
    pub fn new(verbosity: u8) -> Self {
        Self {
            generator: DescriptionGenerator::new().with_verbosity(verbosity),
        }
    }

    /// Narrate a construction step by step
    pub fn narrate_construction(&self, construction: &Construction) -> Vec<String> {
        let mut narration = Vec::new();

        // Introduction
        narration.push(self.generator.describe_construction(construction));

        // Narrate axioms
        let axioms = construction.axioms();
        if !axioms.is_empty() {
            narration.push(format!(
                "Starting with {} axiom{}",
                axioms.len(),
                if axioms.len() == 1 { "" } else { "s" }
            ));

            for &axiom_id in &axioms {
                if let Some(point) = construction.graph.points.get(&axiom_id) {
                    narration.push(self.generator.describe_point(point));
                }
            }
        }

        // Narrate implications
        for line in construction.graph.lines.values() {
            if let (Some(from), Some(to)) = (
                construction.graph.points.get(&line.from),
                construction.graph.points.get(&line.to),
            ) {
                narration.push(self.generator.describe_line(line, from, to));
            }
        }

        // Conclusion
        let theorems = construction.theorems();
        if !theorems.is_empty() {
            narration.push(format!(
                "Concluding with {} theorem{}",
                theorems.len(),
                if theorems.len() == 1 { "" } else { "s" }
            ));

            for &theorem_id in &theorems {
                if let Some(point) = construction.graph.points.get(&theorem_id) {
                    narration.push(self.generator.describe_point(point));
                }
            }
        }

        narration
    }

    /// Narrate proof state changes
    pub fn narrate_state_change(
        &self,
        before: &ProofState,
        after: &ProofState,
        action: &str,
    ) -> String {
        let before_goals = before.num_goals();
        let after_goals = after.num_goals();

        if after_goals == 0 {
            "Proof complete!".to_string()
        } else if after_goals < before_goals {
            format!(
                "{}: Reduced from {} to {} goal{}",
                action,
                before_goals,
                after_goals,
                if after_goals == 1 { "" } else { "s" }
            )
        } else if after_goals > before_goals {
            format!(
                "{}: Generated {} new subgoal{}",
                action,
                after_goals - before_goals,
                if (after_goals - before_goals) == 1 {
                    ""
                } else {
                    "s"
                }
            )
        } else {
            format!("{}: State unchanged", action)
        }
    }

    /// Create an audio-friendly description (shorter, clearer)
    pub fn audio_description(&self, expr: &Expr) -> String {
        // Simplified descriptions optimized for speech
        match expr {
            Expr::Var(name, _) => name.0.clone(),
            Expr::Type(_) => "Type".to_string(),
            Expr::Pi {
                name,
                domain,
                codomain,
            } => {
                format!(
                    "Function from {} {} to {}",
                    name.0,
                    self.audio_description(domain),
                    self.audio_description(codomain)
                )
            }
            Expr::Lambda { name, .. } => format!("Lambda {}", name.0),
            Expr::App { func, arg } => {
                format!(
                    "{} of {}",
                    self.audio_description(func),
                    self.audio_description(arg)
                )
            }
            Expr::Path { left, right, .. } => {
                format!(
                    "{} equals {}",
                    self.audio_description(left),
                    self.audio_description(right)
                )
            }
            _ => "complex expression".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sctt_core::Name;

    #[test]
    fn test_description_generator() {
        let gen = DescriptionGenerator::new();
        let expr = Expr::Type(0);
        let desc = gen.describe_expr(&expr);
        assert!(desc.contains("Type") || desc.contains("type"));
    }

    #[test]
    fn test_verbosity_levels() {
        let expr = Expr::Var(Name("x".to_string()), 0);

        let low = DescriptionGenerator::new().with_verbosity(1);
        let high = DescriptionGenerator::new().with_verbosity(5);

        let low_desc = low.describe_expr(&expr);
        let high_desc = high.describe_expr(&expr);

        assert!(high_desc.len() >= low_desc.len());
    }

    #[test]
    fn test_technical_language() {
        let expr = Expr::Type(0);

        let simple = DescriptionGenerator::new().with_technical_language(false);
        let technical = DescriptionGenerator::new().with_technical_language(true);

        let simple_desc = simple.describe_expr(&expr);
        let tech_desc = technical.describe_expr(&expr);

        assert!(tech_desc.contains("level"));
    }

    #[test]
    fn test_proof_narrator() {
        let narrator = ProofNarrator::new(3);
        let construction = Construction::new("Test Proof".to_string());
        let narration = narrator.narrate_construction(&construction);

        assert!(!narration.is_empty());
        assert!(narration[0].contains("Test Proof"));
    }
}
