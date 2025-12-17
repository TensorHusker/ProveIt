//! Proof automation tactics for TTT.
//!
//! This module provides tactics that automatically search for proofs
//! using combinations of basic tactics.

use ttt_core::prelude::Term;
use ttt_kernel::prelude::TrustKernel;

use crate::basic::{Assumption, Intro, Reflexivity, Split, Trivial};
use crate::tactic::{Goal, ProofTerm, Tactic, TacticError, TacticResult};

/// Configuration for automated proof search.
#[derive(Debug, Clone)]
pub struct AutoConfig {
    /// Maximum search depth.
    pub max_depth: usize,
    /// Whether to use intro tactic.
    pub use_intro: bool,
    /// Whether to use split tactic.
    pub use_split: bool,
    /// Whether to try assumptions.
    pub use_assumption: bool,
    /// Whether to try reflexivity.
    pub use_reflexivity: bool,
}

impl Default for AutoConfig {
    fn default() -> Self {
        Self {
            max_depth: 10,
            use_intro: true,
            use_split: true,
            use_assumption: true,
            use_reflexivity: true,
        }
    }
}

/// The `auto` tactic: automated proof search.
///
/// Tries various tactics in sequence with backtracking to find a proof.
pub struct Auto {
    config: AutoConfig,
}

impl Auto {
    /// Create a new auto tactic with default configuration.
    pub fn new() -> Self {
        Self {
            config: AutoConfig::default(),
        }
    }

    /// Create an auto tactic with custom configuration.
    pub fn with_config(config: AutoConfig) -> Self {
        Self { config }
    }

    /// Recursive proof search.
    fn search(&self, kernel: &TrustKernel, goal: &Goal, depth: usize) -> Option<Term> {
        if depth > self.config.max_depth {
            return None;
        }

        // Try trivial first (Unit, reflexivity for Id)
        if let TacticResult::Success { proof, subgoals } = Trivial.apply(kernel, goal) {
            if subgoals.is_empty() {
                if let ProofTerm::Complete(term) = proof {
                    return Some(term);
                }
            }
        }

        // Try assumption
        if self.config.use_assumption {
            if let TacticResult::Success { proof, subgoals } = Assumption.apply(kernel, goal) {
                if subgoals.is_empty() {
                    if let ProofTerm::Complete(term) = proof {
                        return Some(term);
                    }
                }
            }
        }

        // Try reflexivity for identity types
        if self.config.use_reflexivity {
            if let TacticResult::Success { proof, subgoals } = Reflexivity.apply(kernel, goal) {
                if subgoals.is_empty() {
                    if let ProofTerm::Complete(term) = proof {
                        return Some(term);
                    }
                }
            }
        }

        // Try intro for Pi types
        if self.config.use_intro {
            if let TacticResult::Success { proof: ProofTerm::Partial(builder), subgoals } = Intro::new().apply(kernel, goal) {
                if subgoals.len() == 1 {
                    if let Some(subproof) = self.search(kernel, &subgoals[0], depth + 1) {
                        if let Some(combined) = builder.build(&[subproof]) {
                            return Some(combined);
                        }
                    }
                }
            }
        }

        // Try split for Sigma types
        if self.config.use_split {
            if let TacticResult::Success { proof: ProofTerm::Partial(builder), subgoals } = Split.apply(kernel, goal) {
                if subgoals.len() == 2 {
                    if let Some(proof1) = self.search(kernel, &subgoals[0], depth + 1) {
                        if let Some(proof2) = self.search(kernel, &subgoals[1], depth + 1) {
                            if let Some(combined) = builder.build(&[proof1, proof2]) {
                                return Some(combined);
                            }
                        }
                    }
                }
            }
        }

        None
    }
}

impl Default for Auto {
    fn default() -> Self {
        Self::new()
    }
}

impl Tactic for Auto {
    fn apply(&self, kernel: &TrustKernel, goal: &Goal) -> TacticResult {
        match self.search(kernel, goal, 0) {
            Some(term) => TacticResult::complete(term),
            None => TacticResult::fail(TacticError::NotApplicable(
                "auto: could not find a proof".to_string(),
            )),
        }
    }

    fn name(&self) -> &str {
        "auto"
    }
}

/// The `eauto` tactic: enhanced auto with hint databases.
///
/// Like auto but with support for user-defined hints and lemmas.
pub struct EAuto {
    /// Base auto configuration.
    config: AutoConfig,
    /// Hint terms to try applying.
    hints: Vec<Term>,
}

impl EAuto {
    /// Create a new eauto tactic.
    pub fn new() -> Self {
        Self {
            config: AutoConfig::default(),
            hints: vec![],
        }
    }

    /// Add a hint term.
    pub fn with_hint(mut self, hint: Term) -> Self {
        self.hints.push(hint);
        self
    }

    /// Add multiple hint terms.
    pub fn with_hints(mut self, hints: impl IntoIterator<Item = Term>) -> Self {
        self.hints.extend(hints);
        self
    }
}

impl Default for EAuto {
    fn default() -> Self {
        Self::new()
    }
}

impl Tactic for EAuto {
    fn apply(&self, kernel: &TrustKernel, goal: &Goal) -> TacticResult {
        // First try regular auto
        let auto_result = Auto::with_config(self.config.clone()).apply(kernel, goal);
        if auto_result.is_success() {
            return auto_result;
        }

        // Try applying each hint
        for hint in &self.hints {
            // Try exact with the hint
            if let Ok(hint_type) = kernel.infer(&goal.ctx, hint) {
                if ttt_core::equality::definitionally_equal(&hint_type, &goal.ty) {
                    return TacticResult::complete(hint.clone());
                }
            }
        }

        TacticResult::fail(TacticError::NotApplicable(
            "eauto: could not find a proof".to_string(),
        ))
    }

    fn name(&self) -> &str {
        "eauto"
    }
}

/// The `decide` tactic: decision procedures for decidable types.
///
/// Handles Bool, finite types, and types with decidable equality.
pub struct Decide;

impl Tactic for Decide {
    fn apply(&self, _kernel: &TrustKernel, goal: &Goal) -> TacticResult {
        let ty = goal.ty.whnf();

        match ty {
            Term::Bool => {
                // For Bool goals, we can't just pick one - need user input
                TacticResult::fail(TacticError::NotApplicable(
                    "decide: Bool goal requires explicit choice (use 'left' or 'right')".to_string(),
                ))
            }
            Term::Unit => TacticResult::complete(Term::Star),
            Term::Empty => {
                // Empty type cannot be inhabited directly
                TacticResult::fail(TacticError::NotApplicable(
                    "decide: Empty type cannot be inhabited".to_string(),
                ))
            }
            _ => TacticResult::fail(TacticError::NotApplicable(
                "decide: not a decidable type".to_string(),
            )),
        }
    }

    fn name(&self) -> &str {
        "decide"
    }
}

/// The `constructor` tactic: try all constructors of an inductive type.
///
/// For product types, this is like split.
/// For identity types, this tries reflexivity.
pub struct Constructor;

impl Tactic for Constructor {
    fn apply(&self, kernel: &TrustKernel, goal: &Goal) -> TacticResult {
        let ty = goal.ty.whnf();

        match &ty {
            Term::Unit => TacticResult::complete(Term::Star),
            Term::Sigma { .. } => Split.apply(kernel, goal),
            Term::Id { lhs, rhs, .. } => {
                if ttt_core::equality::definitionally_equal(lhs, rhs) {
                    Reflexivity.apply(kernel, goal)
                } else {
                    TacticResult::fail(TacticError::NotApplicable(
                        "constructor: Id endpoints not equal".to_string(),
                    ))
                }
            }
            Term::Bool => {
                // Could return True or False - we choose True by convention
                TacticResult::complete(Term::True)
            }
            _ => TacticResult::fail(TacticError::NotApplicable(
                format!("constructor: don't know constructors for {:?}", ty),
            )),
        }
    }

    fn name(&self) -> &str {
        "constructor"
    }
}

/// The `intuition` tactic: prove intuitionistic propositional goals.
///
/// Uses a simplified decision procedure for intuitionistic propositional logic.
pub struct Intuition {
    max_depth: usize,
}

impl Intuition {
    /// Create a new intuition tactic.
    pub fn new() -> Self {
        Self { max_depth: 20 }
    }

    /// Create with custom depth limit.
    pub fn with_depth(max_depth: usize) -> Self {
        Self { max_depth }
    }
}

impl Default for Intuition {
    fn default() -> Self {
        Self::new()
    }
}

impl Tactic for Intuition {
    fn apply(&self, kernel: &TrustKernel, goal: &Goal) -> TacticResult {
        // Intuition is essentially auto specialized for propositional logic
        let config = AutoConfig {
            max_depth: self.max_depth,
            use_intro: true,
            use_split: true,
            use_assumption: true,
            use_reflexivity: true,
        };

        Auto::with_config(config).apply(kernel, goal)
    }

    fn name(&self) -> &str {
        "intuition"
    }
}

/// The `firstorder` tactic: prove first-order logic goals.
///
/// Extends intuition with quantifier handling.
pub struct FirstOrder {
    max_depth: usize,
}

impl FirstOrder {
    /// Create a new firstorder tactic.
    pub fn new() -> Self {
        Self { max_depth: 30 }
    }
}

impl Default for FirstOrder {
    fn default() -> Self {
        Self::new()
    }
}

impl Tactic for FirstOrder {
    fn apply(&self, kernel: &TrustKernel, goal: &Goal) -> TacticResult {
        // First-order extends intuition with better quantifier handling
        // For now, delegate to auto
        let config = AutoConfig {
            max_depth: self.max_depth,
            use_intro: true,
            use_split: true,
            use_assumption: true,
            use_reflexivity: true,
        };

        Auto::with_config(config).apply(kernel, goal)
    }

    fn name(&self) -> &str {
        "firstorder"
    }
}

/// Convenient function to create auto tactic.
pub fn auto() -> Auto {
    Auto::new()
}

/// Convenient function to create eauto tactic.
pub fn eauto() -> EAuto {
    EAuto::new()
}

/// Convenient function to create intuition tactic.
pub fn intuition() -> Intuition {
    Intuition::new()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ttt_core::prelude::Context;

    fn make_kernel() -> TrustKernel {
        TrustKernel::new()
    }

    #[test]
    fn test_auto_unit() {
        let kernel = make_kernel();
        let goal = Goal::new(Context::empty(), Term::Unit);

        let result = Auto::new().apply(&kernel, &goal);
        assert!(result.is_success());
    }

    #[test]
    fn test_auto_identity_arrow() {
        let kernel = make_kernel();

        // Goal: ⊢ Unit → Unit
        let goal_ty = Term::Pi {
            domain: Box::new(Term::Unit),
            codomain: Box::new(Term::Unit),
        };
        let goal = Goal::new(Context::empty(), goal_ty);

        let result = Auto::new().apply(&kernel, &goal);
        assert!(result.is_success());

        if let TacticResult::Success { proof: ProofTerm::Complete(term), .. } = result {
            // Should be λ. #0 or λ. *
            assert!(matches!(term, Term::Lambda { .. }));
        }
    }

    #[test]
    fn test_auto_product() {
        let kernel = make_kernel();

        // Goal: ⊢ Σ(Unit, λ_. Unit)
        let goal_ty = Term::Sigma {
            fst_type: Box::new(Term::Unit),
            snd_type: Box::new(Term::Unit),
        };
        let goal = Goal::new(Context::empty(), goal_ty);

        let result = Auto::new().apply(&kernel, &goal);
        assert!(result.is_success());

        if let TacticResult::Success { proof: ProofTerm::Complete(term), .. } = result {
            // Should be (*, *)
            assert!(matches!(term, Term::Pair { .. }));
        }
    }

    #[test]
    fn test_auto_nested_arrow() {
        let kernel = make_kernel();

        // Goal: ⊢ Unit → Unit → Unit
        let goal_ty = Term::Pi {
            domain: Box::new(Term::Unit),
            codomain: Box::new(Term::Pi {
                domain: Box::new(Term::Unit),
                codomain: Box::new(Term::Unit),
            }),
        };
        let goal = Goal::new(Context::empty(), goal_ty);

        let result = Auto::new().apply(&kernel, &goal);
        assert!(result.is_success());
    }

    #[test]
    fn test_auto_reflexivity() {
        let kernel = make_kernel();

        // Goal: ⊢ Id(Unit, *, *)
        let goal_ty = Term::Id {
            ty: Box::new(Term::Unit),
            lhs: Box::new(Term::Star),
            rhs: Box::new(Term::Star),
        };
        let goal = Goal::new(Context::empty(), goal_ty);

        let result = Auto::new().apply(&kernel, &goal);
        assert!(result.is_success());
    }

    #[test]
    fn test_constructor() {
        let kernel = make_kernel();

        let goal = Goal::new(Context::empty(), Term::Unit);
        let result = Constructor.apply(&kernel, &goal);
        assert!(result.is_success());
    }

    #[test]
    fn test_auto_from_hypothesis() {
        let kernel = make_kernel();

        // Goal: x:Unit ⊢ Unit
        let ctx = Context::empty().extend(Term::Unit);
        let goal = Goal::new(ctx, Term::Unit);

        let result = Auto::new().apply(&kernel, &goal);
        assert!(result.is_success());
    }
}
