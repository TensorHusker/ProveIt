//! Core tactic infrastructure for TTT proof automation.
//!
//! This module defines the `Tactic` trait and supporting types that form
//! the foundation of the proof automation system.

use std::sync::Arc;

use ttt_core::prelude::{Context, Term};
use ttt_kernel::prelude::{TrustKernel, TypeError, VerifiedJudgment};

/// A proof goal consisting of a context and a type to prove.
#[derive(Debug, Clone)]
pub struct Goal {
    /// The typing context with available hypotheses.
    pub ctx: Context,
    /// The type/proposition to prove.
    pub ty: Term,
    /// Optional name for this goal (for display purposes).
    pub name: Option<String>,
}

impl Goal {
    /// Create a new goal.
    pub fn new(ctx: Context, ty: Term) -> Self {
        Self {
            ctx,
            ty,
            name: None,
        }
    }

    /// Create a named goal.
    pub fn named(ctx: Context, ty: Term, name: impl Into<String>) -> Self {
        Self {
            ctx,
            ty,
            name: Some(name.into()),
        }
    }
}

/// The result of applying a tactic.
#[derive(Debug)]
pub enum TacticResult {
    /// Tactic succeeded, producing a proof term and possibly new subgoals.
    Success {
        /// The (partial) proof term. May contain holes if subgoals remain.
        proof: ProofTerm,
        /// New subgoals that need to be solved.
        subgoals: Vec<Goal>,
    },
    /// Tactic failed with an error message.
    Failure(TacticError),
}

impl TacticResult {
    /// Create a successful result with no subgoals.
    pub fn complete(term: Term) -> Self {
        TacticResult::Success {
            proof: ProofTerm::Complete(term),
            subgoals: vec![],
        }
    }

    /// Create a successful result with subgoals.
    pub fn with_subgoals(builder: ProofBuilder, subgoals: Vec<Goal>) -> Self {
        TacticResult::Success {
            proof: ProofTerm::Partial(builder),
            subgoals,
        }
    }

    /// Create a failure result.
    pub fn fail(error: TacticError) -> Self {
        TacticResult::Failure(error)
    }

    /// Check if the tactic succeeded.
    pub fn is_success(&self) -> bool {
        matches!(self, TacticResult::Success { .. })
    }
}

/// A proof term, either complete or partial with holes.
#[derive(Debug, Clone)]
pub enum ProofTerm {
    /// A complete proof term.
    Complete(Term),
    /// A partial proof with a builder to combine subproofs.
    Partial(ProofBuilder),
}

/// A combiner function that takes subproofs and produces a combined term.
pub type CombinerFn = Arc<dyn Fn(&[Term]) -> Option<Term> + Send + Sync>;

/// A builder that combines subproofs into a complete proof.
#[derive(Clone)]
pub struct ProofBuilder {
    /// Function to combine subproofs.
    combiner: CombinerFn,
    /// Number of expected subproofs.
    arity: usize,
}

impl std::fmt::Debug for ProofBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ProofBuilder")
            .field("arity", &self.arity)
            .finish()
    }
}

impl ProofBuilder {
    /// Create a new proof builder.
    pub fn new<F>(arity: usize, combiner: F) -> Self
    where
        F: Fn(&[Term]) -> Option<Term> + Send + Sync + 'static,
    {
        Self {
            combiner: Arc::new(combiner),
            arity,
        }
    }

    /// Get the expected number of subproofs.
    pub fn arity(&self) -> usize {
        self.arity
    }

    /// Build the combined proof from subproofs.
    pub fn build(&self, subproofs: &[Term]) -> Option<Term> {
        if subproofs.len() != self.arity {
            return None;
        }
        (self.combiner)(subproofs)
    }
}

/// Errors that can occur during tactic application.
#[derive(Debug, Clone)]
pub enum TacticError {
    /// The goal doesn't match the expected shape.
    GoalMismatch(String),
    /// A required hypothesis wasn't found.
    HypothesisNotFound(String),
    /// The tactic is not applicable to this goal.
    NotApplicable(String),
    /// A type error occurred during tactic execution.
    TypeError(String),
    /// The tactic requires more arguments.
    MissingArgument(String),
    /// A custom error message.
    Custom(String),
}

impl std::fmt::Display for TacticError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TacticError::GoalMismatch(msg) => write!(f, "Goal mismatch: {}", msg),
            TacticError::HypothesisNotFound(msg) => write!(f, "Hypothesis not found: {}", msg),
            TacticError::NotApplicable(msg) => write!(f, "Tactic not applicable: {}", msg),
            TacticError::TypeError(msg) => write!(f, "Type error: {}", msg),
            TacticError::MissingArgument(msg) => write!(f, "Missing argument: {}", msg),
            TacticError::Custom(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for TacticError {}

impl From<TypeError> for TacticError {
    fn from(err: TypeError) -> Self {
        TacticError::TypeError(format!("{:?}", err))
    }
}

/// The main tactic trait.
///
/// A tactic transforms a proof goal into either a proof term or
/// a set of simpler subgoals.
pub trait Tactic {
    /// Apply this tactic to a goal.
    fn apply(&self, kernel: &TrustKernel, goal: &Goal) -> TacticResult;

    /// Get the name of this tactic (for debugging/display).
    fn name(&self) -> &str;
}

/// A proof state tracks the current goals and accumulated proof.
#[derive(Debug)]
pub struct ProofState {
    /// The remaining goals to prove.
    goals: Vec<Goal>,
    /// Stack of proof builders waiting for subproofs.
    builders: Vec<(ProofBuilder, Vec<Option<Term>>)>,
    /// The original goal we're trying to prove.
    original_goal: Goal,
    /// Completed proof term (if finished).
    completed: Option<Term>,
}

impl ProofState {
    /// Create a new proof state for a goal.
    pub fn new(goal: Goal) -> Self {
        Self {
            original_goal: goal.clone(),
            goals: vec![goal],
            builders: vec![],
            completed: None,
        }
    }

    /// Get the current goal, if any.
    pub fn current_goal(&self) -> Option<&Goal> {
        self.goals.first()
    }

    /// Get all remaining goals.
    pub fn goals(&self) -> &[Goal] {
        &self.goals
    }

    /// Check if the proof is complete.
    pub fn is_complete(&self) -> bool {
        self.goals.is_empty() && self.completed.is_some()
    }

    /// Get the completed proof term.
    pub fn proof_term(&self) -> Option<&Term> {
        self.completed.as_ref()
    }

    /// Apply a tactic to the current goal.
    pub fn apply_tactic(&mut self, kernel: &TrustKernel, tactic: &dyn Tactic) -> Result<(), TacticError> {
        let goal = self.goals.first().ok_or_else(|| {
            TacticError::Custom("No goals remaining".to_string())
        })?;

        match tactic.apply(kernel, goal) {
            TacticResult::Success { proof, subgoals } => {
                // Remove the current goal
                self.goals.remove(0);

                match proof {
                    ProofTerm::Complete(term) => {
                        self.integrate_proof(term);
                    }
                    ProofTerm::Partial(builder) => {
                        // Add new subgoals at the front
                        let arity = builder.arity();
                        for subgoal in subgoals.into_iter().rev() {
                            self.goals.insert(0, subgoal);
                        }
                        // Track the builder
                        self.builders.push((builder, vec![None; arity]));
                    }
                }
                Ok(())
            }
            TacticResult::Failure(err) => Err(err),
        }
    }

    /// Integrate a completed subproof.
    fn integrate_proof(&mut self, term: Term) {
        if self.builders.is_empty() {
            // This is the final proof
            self.completed = Some(term);
            return;
        }

        // Find the first builder that needs this proof
        let mut current_term = term;
        while let Some((builder, subproofs)) = self.builders.last_mut() {
            // Find the first empty slot
            if let Some(slot) = subproofs.iter_mut().find(|s| s.is_none()) {
                *slot = Some(current_term);

                // Check if builder is complete
                if subproofs.iter().all(|s| s.is_some()) {
                    let terms: Vec<Term> = subproofs.iter()
                        .map(|s| s.clone().unwrap())
                        .collect();
                    if let Some(combined) = builder.build(&terms) {
                        current_term = combined;
                        self.builders.pop();
                        continue;
                    }
                }
                return;
            }
        }

        // No more builders, this is the final proof
        self.completed = Some(current_term);
    }

    /// Verify the completed proof with the kernel.
    #[allow(clippy::result_large_err)] // TypeError is defined in ttt-kernel
    pub fn verify(&self, kernel: &TrustKernel) -> Result<VerifiedJudgment, TypeError> {
        let term = self.completed.as_ref()
            .ok_or(TypeError::TypeMismatch {
                expected: self.original_goal.ty.clone(),
                found: Term::Empty,
                term: Term::Empty,
            })?;

        let judgment = ttt_kernel::prelude::Judgment::inhabitation(
            self.original_goal.ctx.clone(),
            term.clone(),
            self.original_goal.ty.clone(),
        );

        kernel.check(judgment)
    }
}

/// Helper function to create a tactic from a closure.
pub fn tactic_fn<F>(name: &'static str, f: F) -> impl Tactic
where
    F: Fn(&TrustKernel, &Goal) -> TacticResult,
{
    struct FnTactic<F> {
        name: &'static str,
        f: F,
    }

    impl<F> Tactic for FnTactic<F>
    where
        F: Fn(&TrustKernel, &Goal) -> TacticResult,
    {
        fn apply(&self, kernel: &TrustKernel, goal: &Goal) -> TacticResult {
            (self.f)(kernel, goal)
        }

        fn name(&self) -> &str {
            self.name
        }
    }

    FnTactic { name, f }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_goal_creation() {
        let goal = Goal::new(Context::empty(), Term::Unit);
        assert!(goal.name.is_none());

        let named = Goal::named(Context::empty(), Term::Unit, "test");
        assert_eq!(named.name.as_deref(), Some("test"));
    }

    #[test]
    fn test_tactic_result() {
        let result = TacticResult::complete(Term::Star);
        assert!(result.is_success());

        let failure = TacticResult::fail(TacticError::NotApplicable("test".into()));
        assert!(!failure.is_success());
    }

    #[test]
    fn test_proof_state_creation() {
        let goal = Goal::new(Context::empty(), Term::Unit);
        let state = ProofState::new(goal);
        assert!(!state.is_complete());
        assert!(state.current_goal().is_some());
    }
}
