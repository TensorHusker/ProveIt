//! Proof tactics for interactive theorem proving

use sctt_core::{check_type, eval, infer, normalize, Context, Expr, Name};
use std::collections::HashMap;

use crate::goals::{Goal, GoalId, GoalState, Hypothesis};
use crate::{Error, Result};

/// A proof tactic that transforms goals
pub trait Tactic {
    /// Apply the tactic to a goal
    fn apply(&self, goal_state: &GoalState) -> TacticResult;

    /// Name of the tactic
    fn name(&self) -> &str;

    /// Description of what the tactic does
    fn description(&self) -> &str;
}

/// Result of applying a tactic
#[derive(Debug, Clone)]
pub enum TacticResult {
    /// Tactic succeeded, producing new subgoals
    Success {
        /// New subgoals to prove
        subgoals: Vec<Goal>,
        /// Proof term constructor (takes proofs of subgoals)
        proof_builder: ProofBuilder,
    },
    /// Tactic failed with an error
    Failure(String),
}

/// Builder for constructing proof terms from subgoal proofs
#[derive(Debug, Clone)]
pub struct ProofBuilder {
    /// Function that combines subgoal proofs
    builder_fn: ProofBuilderFn,
}

#[derive(Debug, Clone)]
enum ProofBuilderFn {
    /// Identity (no transformation)
    Identity,
    /// Lambda introduction
    Lambda(Name),
    /// Application
    Apply(Box<Expr>),
    /// Custom builder (placeholder for complex constructions)
    Custom(String),
}

impl ProofBuilder {
    pub fn identity() -> Self {
        Self {
            builder_fn: ProofBuilderFn::Identity,
        }
    }

    pub fn lambda(name: Name) -> Self {
        Self {
            builder_fn: ProofBuilderFn::Lambda(name),
        }
    }

    pub fn apply(term: Expr) -> Self {
        Self {
            builder_fn: ProofBuilderFn::Apply(Box::new(term)),
        }
    }

    /// Build a proof from subgoal proofs
    pub fn build(&self, subproofs: &[Expr]) -> Result<Expr> {
        match &self.builder_fn {
            ProofBuilderFn::Identity => {
                if subproofs.len() == 1 {
                    Ok(subproofs[0].clone())
                } else {
                    Err(Error::TacticFailed(
                        "Identity builder expects exactly one subproof".to_string(),
                    ))
                }
            }
            ProofBuilderFn::Lambda(name) => {
                if subproofs.len() == 1 {
                    Ok(Expr::Lambda {
                        name: name.clone(),
                        body: Box::new(subproofs[0].clone()),
                    })
                } else {
                    Err(Error::TacticFailed(
                        "Lambda builder expects exactly one subproof".to_string(),
                    ))
                }
            }
            ProofBuilderFn::Apply(term) => {
                if subproofs.len() == 1 {
                    Ok(Expr::App {
                        func: term.clone(),
                        arg: Box::new(subproofs[0].clone()),
                    })
                } else {
                    Err(Error::TacticFailed(
                        "Apply builder expects exactly one subproof".to_string(),
                    ))
                }
            }
            ProofBuilderFn::Custom(desc) => Err(Error::TacticFailed(format!(
                "Custom builder not implemented: {}",
                desc
            ))),
        }
    }
}

/// Library of built-in tactics
pub struct TacticLibrary {
    tactics: HashMap<String, Box<dyn Tactic>>,
}

impl TacticLibrary {
    pub fn new() -> Self {
        let mut library = Self {
            tactics: HashMap::new(),
        };

        // Register built-in tactics
        library.register(Box::new(IntroTactic));
        library.register(Box::new(ExactTactic));
        library.register(Box::new(ApplyTactic));
        library.register(Box::new(AssumptionTactic));
        library.register(Box::new(ReflTactic));

        library
    }

    /// Register a new tactic
    pub fn register(&mut self, tactic: Box<dyn Tactic>) {
        self.tactics.insert(tactic.name().to_string(), tactic);
    }

    /// Get a tactic by name
    pub fn get(&self, name: &str) -> Option<&dyn Tactic> {
        self.tactics.get(name).map(|t| t.as_ref())
    }

    /// List all available tactics
    pub fn list(&self) -> Vec<&str> {
        self.tactics.keys().map(|s| s.as_str()).collect()
    }
}

impl Default for TacticLibrary {
    fn default() -> Self {
        Self::new()
    }
}

// Built-in tactics

/// Introduction tactic for Pi types (proves A → B by assuming A)
struct IntroTactic;

impl Tactic for IntroTactic {
    fn apply(&self, goal_state: &GoalState) -> TacticResult {
        match &goal_state.goal.proposition {
            Expr::Pi {
                name,
                domain,
                codomain,
            } => {
                // Create new goal with hypothesis
                let mut new_goal = Goal::new(
                    GoalId(0), // ID will be reassigned
                    (**codomain).clone(),
                );

                // Add the domain as a hypothesis
                new_goal.add_hypothesis(Hypothesis::new(name.0.clone(), (**domain).clone()));

                // Copy existing hypotheses
                for hyp in &goal_state.goal.hypotheses {
                    new_goal.add_hypothesis(hyp.clone());
                }

                TacticResult::Success {
                    subgoals: vec![new_goal],
                    proof_builder: ProofBuilder::lambda(name.clone()),
                }
            }
            _ => TacticResult::Failure("intro: goal is not a Pi type".to_string()),
        }
    }

    fn name(&self) -> &str {
        "intro"
    }

    fn description(&self) -> &str {
        "Introduce a lambda abstraction for Pi types"
    }
}

/// Exact tactic (prove goal with an exact term)
struct ExactTactic;

impl Tactic for ExactTactic {
    fn apply(&self, goal_state: &GoalState) -> TacticResult {
        // This is a placeholder - the actual term would be provided by the user
        TacticResult::Failure("exact: requires a proof term argument".to_string())
    }

    fn name(&self) -> &str {
        "exact"
    }

    fn description(&self) -> &str {
        "Solve the goal with an exact proof term"
    }
}

/// Apply tactic (apply a function to the goal)
struct ApplyTactic;

impl Tactic for ApplyTactic {
    fn apply(&self, goal_state: &GoalState) -> TacticResult {
        // This is a placeholder - the actual function would be provided by the user
        TacticResult::Failure("apply: requires a function argument".to_string())
    }

    fn name(&self) -> &str {
        "apply"
    }

    fn description(&self) -> &str {
        "Apply a function to the goal"
    }
}

/// Assumption tactic (solve goal using a hypothesis)
struct AssumptionTactic;

impl Tactic for AssumptionTactic {
    fn apply(&self, goal_state: &GoalState) -> TacticResult {
        // Look for a hypothesis that matches the goal
        for hyp in &goal_state.goal.hypotheses {
            // Simple syntactic check (should be improved with normalization)
            if format!("{:?}", hyp.ty) == format!("{:?}", goal_state.goal.proposition) {
                // Found matching hypothesis
                let proof = Expr::Var(Name(hyp.name.clone()), 0);
                return TacticResult::Success {
                    subgoals: vec![],
                    proof_builder: ProofBuilder::identity(),
                };
            }
        }

        TacticResult::Failure("assumption: no matching hypothesis found".to_string())
    }

    fn name(&self) -> &str {
        "assumption"
    }

    fn description(&self) -> &str {
        "Solve the goal using a hypothesis from the context"
    }
}

/// Reflexivity tactic (prove x = x)
struct ReflTactic;

impl Tactic for ReflTactic {
    fn apply(&self, goal_state: &GoalState) -> TacticResult {
        match &goal_state.goal.proposition {
            Expr::Path { left, right, .. } => {
                // Check if left and right are the same
                if format!("{:?}", left) == format!("{:?}", right) {
                    TacticResult::Success {
                        subgoals: vec![],
                        proof_builder: ProofBuilder::identity(),
                    }
                } else {
                    TacticResult::Failure("refl: sides of path are not equal".to_string())
                }
            }
            _ => TacticResult::Failure("refl: goal is not a path type".to_string()),
        }
    }

    fn name(&self) -> &str {
        "refl"
    }

    fn description(&self) -> &str {
        "Prove reflexivity of paths (x = x)"
    }
}

/// Tactic with an argument
pub struct TacticWithArg {
    pub name: String,
    pub arg: TacticArg,
}

#[derive(Debug, Clone)]
pub enum TacticArg {
    /// A proof term
    Term(Expr),
    /// A name
    Name(String),
    /// A number
    Number(i64),
}

impl TacticWithArg {
    pub fn exact(term: Expr) -> Self {
        Self {
            name: "exact".to_string(),
            arg: TacticArg::Term(term),
        }
    }

    pub fn apply(term: Expr) -> Self {
        Self {
            name: "apply".to_string(),
            arg: TacticArg::Term(term),
        }
    }

    pub fn intro_named(name: String) -> Self {
        Self {
            name: "intro".to_string(),
            arg: TacticArg::Name(name),
        }
    }

    /// Apply the tactic to a goal state
    pub fn apply_to(&self, goal_state: &GoalState) -> TacticResult {
        match (self.name.as_str(), &self.arg) {
            ("exact", TacticArg::Term(term)) => {
                // Check if the term proves the goal
                let expected_ty = eval(&goal_state.goal.proposition, goal_state.context.env());
                match check_type(&goal_state.context, term, &expected_ty) {
                    Ok(()) => TacticResult::Success {
                        subgoals: vec![],
                        proof_builder: ProofBuilder::identity(),
                    },
                    Err(e) => TacticResult::Failure(format!("exact: type error: {}", e)),
                }
            }
            ("apply", TacticArg::Term(func)) => {
                // Infer the type of the function
                match infer(&goal_state.context, func) {
                    Ok(func_ty) => {
                        // The function should be a Pi type
                        // Create a subgoal for the domain
                        if let sctt_core::Value::VPi { domain, .. } = func_ty {
                            let domain_expr = sctt_core::normalize(&domain);

                            let mut new_goal = Goal::new(GoalId(0), domain_expr);
                            for hyp in &goal_state.goal.hypotheses {
                                new_goal.add_hypothesis(hyp.clone());
                            }

                            TacticResult::Success {
                                subgoals: vec![new_goal],
                                proof_builder: ProofBuilder::apply(func.clone()),
                            }
                        } else {
                            TacticResult::Failure("apply: function is not a Pi type".to_string())
                        }
                    }
                    Err(e) => TacticResult::Failure(format!("apply: cannot infer type: {}", e)),
                }
            }
            _ => {
                TacticResult::Failure(format!("Unknown tactic or invalid argument: {}", self.name))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tactic_library() {
        let library = TacticLibrary::new();
        assert!(library.get("intro").is_some());
        assert!(library.get("exact").is_some());
    }

    #[test]
    fn test_intro_tactic() {
        let goal = Goal::new(
            GoalId(0),
            Expr::Pi {
                name: Name("x".to_string()),
                domain: Box::new(Expr::Type(0)),
                codomain: Box::new(Expr::Type(0)),
            },
        );

        let goal_state = GoalState::new(goal);
        let tactic = IntroTactic;

        match tactic.apply(&goal_state) {
            TacticResult::Success { subgoals, .. } => {
                assert_eq!(subgoals.len(), 1);
            }
            TacticResult::Failure(msg) => panic!("Tactic failed: {}", msg),
        }
    }

    #[test]
    fn test_proof_builder() {
        let builder = ProofBuilder::identity();
        let proof = Expr::Type(0);
        let result = builder.build(&[proof.clone()]).unwrap();
        assert_eq!(format!("{:?}", result), format!("{:?}", proof));
    }
}
