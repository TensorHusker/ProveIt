//! Basic proof tactics for TTT.
//!
//! This module provides fundamental tactics that correspond to the
//! introduction and elimination rules of TTT types.

use ttt_core::prelude::Term;
use ttt_kernel::prelude::TrustKernel;

use crate::tactic::{Goal, ProofBuilder, Tactic, TacticError, TacticResult};

/// The `intro` tactic: introduce a hypothesis for Pi or implication types.
///
/// For a goal `Γ ⊢ Π(A, B)`, produces subgoal `Γ, x:A ⊢ B`.
/// The proof term is `λ. <subproof>`.
pub struct Intro {
    /// Optional name for the introduced variable.
    pub name: Option<String>,
}

impl Intro {
    /// Create a new intro tactic.
    pub fn new() -> Self {
        Self { name: None }
    }

    /// Create an intro tactic with a specific name.
    pub fn named(name: impl Into<String>) -> Self {
        Self { name: Some(name.into()) }
    }
}

impl Default for Intro {
    fn default() -> Self {
        Self::new()
    }
}

impl Tactic for Intro {
    fn apply(&self, _kernel: &TrustKernel, goal: &Goal) -> TacticResult {
        let ty = goal.ty.whnf();

        match ty {
            Term::Pi { domain, codomain } => {
                // Create new context with the hypothesis
                let new_ctx = if let Some(name) = &self.name {
                    goal.ctx.extend_named(name.clone(), (*domain).clone())
                } else {
                    goal.ctx.extend((*domain).clone())
                };

                let subgoal = Goal::new(new_ctx, (*codomain).clone());

                // Builder wraps subproof in a lambda
                let builder = ProofBuilder::new(1, |subproofs| {
                    Some(Term::Lambda {
                        body: Box::new(subproofs[0].clone()),
                    })
                });

                TacticResult::with_subgoals(builder, vec![subgoal])
            }
            _ => TacticResult::fail(TacticError::GoalMismatch(
                "intro requires a Pi type goal".to_string(),
            )),
        }
    }

    fn name(&self) -> &str {
        "intro"
    }
}

/// The `exact` tactic: provide an exact proof term.
///
/// For a goal `Γ ⊢ T`, if the given term has type `T` in `Γ`,
/// the goal is solved.
pub struct Exact {
    /// The proof term to use.
    pub term: Term,
}

impl Exact {
    /// Create a new exact tactic with the given term.
    pub fn new(term: Term) -> Self {
        Self { term }
    }
}

impl Tactic for Exact {
    fn apply(&self, kernel: &TrustKernel, goal: &Goal) -> TacticResult {
        // Verify the term has the expected type
        match kernel.infer(&goal.ctx, &self.term) {
            Ok(inferred) => {
                if ttt_core::equality::definitionally_equal(&inferred, &goal.ty) {
                    TacticResult::complete(self.term.clone())
                } else {
                    TacticResult::fail(TacticError::TypeError(format!(
                        "Term has type {:?}, but goal requires {:?}",
                        inferred, goal.ty
                    )))
                }
            }
            Err(e) => TacticResult::fail(TacticError::from(e)),
        }
    }

    fn name(&self) -> &str {
        "exact"
    }
}

/// The `assumption` tactic: find a matching hypothesis in the context.
///
/// Searches the context for a hypothesis with the same type as the goal.
pub struct Assumption;

impl Tactic for Assumption {
    fn apply(&self, _kernel: &TrustKernel, goal: &Goal) -> TacticResult {
        // Search context for a matching type
        for (idx, entry) in goal.ctx.iter().enumerate() {
            if ttt_core::equality::definitionally_equal(&entry.ty, &goal.ty) {
                // Found a match - use de Bruijn index
                // Note: iter() goes oldest to newest, but de Bruijn counts from newest
                let db_idx = (goal.ctx.len() - 1 - idx) as u32;
                let var = Term::var(db_idx);
                return TacticResult::complete(var);
            }
        }

        TacticResult::fail(TacticError::HypothesisNotFound(
            format!("No hypothesis of type {:?} in context", goal.ty),
        ))
    }

    fn name(&self) -> &str {
        "assumption"
    }
}

/// The `apply` tactic: apply a function to solve a goal.
///
/// For a goal `Γ ⊢ B` and a hypothesis `f : Π(A, B)` or `f : A → B`,
/// creates a subgoal `Γ ⊢ A`.
pub struct Apply {
    /// The function term to apply.
    pub func: Term,
}

impl Apply {
    /// Create a new apply tactic with the given function.
    pub fn new(func: Term) -> Self {
        Self { func }
    }

    /// Create an apply tactic using a hypothesis by index.
    pub fn hypothesis(idx: u32) -> Self {
        Self { func: Term::var(idx) }
    }
}

impl Tactic for Apply {
    fn apply(&self, kernel: &TrustKernel, goal: &Goal) -> TacticResult {
        // Infer the type of the function
        let func_type = match kernel.infer(&goal.ctx, &self.func) {
            Ok(ty) => ty.whnf(),
            Err(e) => return TacticResult::fail(TacticError::from(e)),
        };

        match func_type {
            Term::Pi { domain, codomain: _ } => {
                // Create subgoal for the domain
                let subgoal = Goal::new(goal.ctx.clone(), (*domain).clone());

                // Builder applies the function to the subproof
                let func = self.func.clone();
                let builder = ProofBuilder::new(1, move |subproofs| {
                    Some(Term::App {
                        func: Box::new(func.clone()),
                        arg: Box::new(subproofs[0].clone()),
                    })
                });

                TacticResult::with_subgoals(builder, vec![subgoal])
            }
            _ => TacticResult::fail(TacticError::NotApplicable(
                format!("apply requires a function, got {:?}", func_type),
            )),
        }
    }

    fn name(&self) -> &str {
        "apply"
    }
}

/// The `split` tactic: split a Sigma type goal into two subgoals.
///
/// For a goal `Γ ⊢ Σ(A, B)`, creates subgoals `Γ ⊢ A` and `Γ, x:A ⊢ B`.
pub struct Split;

impl Tactic for Split {
    fn apply(&self, _kernel: &TrustKernel, goal: &Goal) -> TacticResult {
        let ty = goal.ty.whnf();

        match ty {
            Term::Sigma { fst_type, snd_type } => {
                // First subgoal: prove the first component
                let subgoal1 = Goal::new(goal.ctx.clone(), (*fst_type).clone());

                // Second subgoal: prove the second component
                // Note: snd_type may depend on fst, so we extend context
                let extended_ctx = goal.ctx.extend((*fst_type).clone());
                let subgoal2 = Goal::new(extended_ctx, (*snd_type).clone());

                // Builder creates a pair from subproofs
                let builder = ProofBuilder::new(2, |subproofs| {
                    Some(Term::Pair {
                        fst: Box::new(subproofs[0].clone()),
                        snd: Box::new(subproofs[1].clone()),
                    })
                });

                TacticResult::with_subgoals(builder, vec![subgoal1, subgoal2])
            }
            _ => TacticResult::fail(TacticError::GoalMismatch(
                "split requires a Sigma type goal".to_string(),
            )),
        }
    }

    fn name(&self) -> &str {
        "split"
    }
}

/// The `left` tactic: prove a disjunction by proving the left side.
///
/// For a goal that can be interpreted as a sum type, proves the left case.
/// In TTT, this is represented using Sigma types with Bool.
pub struct Left;

impl Tactic for Left {
    fn apply(&self, _kernel: &TrustKernel, _goal: &Goal) -> TacticResult {
        // In TTT, we can encode sums as Σ(Bool, λb. if b then A else B)
        // For now, provide a simpler placeholder
        TacticResult::fail(TacticError::NotApplicable(
            "left tactic requires sum type encoding".to_string(),
        ))
    }

    fn name(&self) -> &str {
        "left"
    }
}

/// The `right` tactic: prove a disjunction by proving the right side.
pub struct Right;

impl Tactic for Right {
    fn apply(&self, _kernel: &TrustKernel, _goal: &Goal) -> TacticResult {
        // Similar to Left
        TacticResult::fail(TacticError::NotApplicable(
            "right tactic requires sum type encoding".to_string(),
        ))
    }

    fn name(&self) -> &str {
        "right"
    }
}

/// The `reflexivity` tactic: prove identity types by reflexivity.
///
/// For a goal `Γ ⊢ Id(A, a, a)`, produces proof `refl(a)`.
pub struct Reflexivity;

impl Tactic for Reflexivity {
    fn apply(&self, _kernel: &TrustKernel, goal: &Goal) -> TacticResult {
        let ty = goal.ty.whnf();

        match ty {
            Term::Id { ty: _, lhs, rhs } => {
                // Check if lhs and rhs are definitionally equal
                if ttt_core::equality::definitionally_equal(&lhs, &rhs) {
                    TacticResult::complete(Term::Refl(lhs))
                } else {
                    TacticResult::fail(TacticError::NotApplicable(format!(
                        "reflexivity requires equal endpoints, got {:?} and {:?}",
                        lhs, rhs
                    )))
                }
            }
            _ => TacticResult::fail(TacticError::GoalMismatch(
                "reflexivity requires an identity type goal".to_string(),
            )),
        }
    }

    fn name(&self) -> &str {
        "reflexivity"
    }
}

/// The `trivial` tactic: solve trivial goals.
///
/// Handles Unit type (produces Star) and true propositions.
pub struct Trivial;

impl Tactic for Trivial {
    fn apply(&self, kernel: &TrustKernel, goal: &Goal) -> TacticResult {
        let ty = goal.ty.whnf();

        match &ty {
            Term::Unit => TacticResult::complete(Term::Star),
            Term::Id { lhs, rhs, .. } => {
                if ttt_core::equality::definitionally_equal(lhs, rhs) {
                    TacticResult::complete(Term::Refl(lhs.clone()))
                } else {
                    // Try assumption
                    Assumption.apply(kernel, goal)
                }
            }
            _ => {
                // Try assumption as last resort
                Assumption.apply(kernel, goal)
            }
        }
    }

    fn name(&self) -> &str {
        "trivial"
    }
}

/// The `destruct` tactic: eliminate a hypothesis.
///
/// For a hypothesis of product type, introduces its components.
pub struct Destruct {
    /// Index of the hypothesis to destruct.
    pub hyp_idx: u32,
}

impl Destruct {
    /// Create a new destruct tactic.
    pub fn new(hyp_idx: u32) -> Self {
        Self { hyp_idx }
    }
}

impl Tactic for Destruct {
    fn apply(&self, _kernel: &TrustKernel, goal: &Goal) -> TacticResult {
        let entry = goal.ctx.lookup_entry(self.hyp_idx)
            .ok_or_else(|| TacticError::HypothesisNotFound(
                format!("No hypothesis at index {}", self.hyp_idx)
            ));

        let entry = match entry {
            Ok(e) => e,
            Err(e) => return TacticResult::fail(e),
        };

        let hyp_ty = entry.ty.whnf();

        match hyp_ty {
            Term::Sigma { fst_type, snd_type } => {
                // Add two new hypotheses: π₁(h) : A and π₂(h) : B[π₁(h)/x]
                let hyp_var = Term::var(self.hyp_idx);
                let fst = Term::Fst(Box::new(hyp_var.clone()));
                let _snd = Term::Snd(Box::new(hyp_var));

                // Create new context with projections
                // This is a simplified version - full implementation would
                // substitute properly
                let new_ctx = goal.ctx
                    .extend((*fst_type).clone())
                    .extend(snd_type.subst(fst.clone()));

                let subgoal = Goal::new(new_ctx, goal.ty.shift(2, 0));

                // Builder needs to substitute back
                let builder = ProofBuilder::new(1, |subproofs| {
                    // Would need proper substitution here
                    Some(subproofs[0].clone())
                });

                TacticResult::with_subgoals(builder, vec![subgoal])
            }
            _ => TacticResult::fail(TacticError::NotApplicable(
                "destruct currently only supports Sigma types".to_string(),
            )),
        }
    }

    fn name(&self) -> &str {
        "destruct"
    }
}

/// Convenient function to create intro tactic.
pub fn intro() -> Intro {
    Intro::new()
}

/// Convenient function to create intro tactic with name.
pub fn intro_named(name: impl Into<String>) -> Intro {
    Intro::named(name)
}

/// Convenient function to create exact tactic.
pub fn exact(term: Term) -> Exact {
    Exact::new(term)
}

/// Convenient function to create apply tactic.
pub fn apply(func: Term) -> Apply {
    Apply::new(func)
}

/// Convenient function to create apply tactic from hypothesis.
pub fn apply_hyp(idx: u32) -> Apply {
    Apply::hypothesis(idx)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ttt_core::prelude::Context;

    fn make_kernel() -> TrustKernel {
        TrustKernel::new()
    }

    #[test]
    fn test_intro_pi() {
        let kernel = make_kernel();

        // Goal: ⊢ Unit → Unit
        let goal_ty = Term::Pi {
            domain: Box::new(Term::Unit),
            codomain: Box::new(Term::Unit),
        };
        let goal = Goal::new(Context::empty(), goal_ty);

        let result = Intro::new().apply(&kernel, &goal);

        match result {
            TacticResult::Success { subgoals, .. } => {
                assert_eq!(subgoals.len(), 1);
                // Subgoal should be: x:Unit ⊢ Unit
                assert!(matches!(subgoals[0].ty, Term::Unit));
                assert_eq!(subgoals[0].ctx.len(), 1);
            }
            TacticResult::Failure(e) => panic!("intro failed: {:?}", e),
        }
    }

    #[test]
    fn test_trivial_unit() {
        let kernel = make_kernel();

        // Goal: ⊢ Unit
        let goal = Goal::new(Context::empty(), Term::Unit);

        let result = Trivial.apply(&kernel, &goal);

        match result {
            TacticResult::Success { proof, subgoals } => {
                assert!(subgoals.is_empty());
                match proof {
                    crate::tactic::ProofTerm::Complete(term) => {
                        assert!(matches!(term, Term::Star));
                    }
                    _ => panic!("Expected complete proof"),
                }
            }
            TacticResult::Failure(e) => panic!("trivial failed: {:?}", e),
        }
    }

    #[test]
    fn test_assumption() {
        let kernel = make_kernel();

        // Goal: x:Unit ⊢ Unit
        let ctx = Context::empty().extend(Term::Unit);
        let goal = Goal::new(ctx, Term::Unit);

        let result = Assumption.apply(&kernel, &goal);

        match result {
            TacticResult::Success { proof, subgoals } => {
                assert!(subgoals.is_empty());
                match proof {
                    crate::tactic::ProofTerm::Complete(term) => {
                        assert!(matches!(term, Term::Var(_)));
                    }
                    _ => panic!("Expected complete proof"),
                }
            }
            TacticResult::Failure(e) => panic!("assumption failed: {:?}", e),
        }
    }

    #[test]
    fn test_reflexivity() {
        let kernel = make_kernel();

        // Goal: ⊢ Id(Unit, *, *)
        let goal_ty = Term::Id {
            ty: Box::new(Term::Unit),
            lhs: Box::new(Term::Star),
            rhs: Box::new(Term::Star),
        };
        let goal = Goal::new(Context::empty(), goal_ty);

        let result = Reflexivity.apply(&kernel, &goal);

        match result {
            TacticResult::Success { proof, subgoals } => {
                assert!(subgoals.is_empty());
                match proof {
                    crate::tactic::ProofTerm::Complete(term) => {
                        assert!(matches!(term, Term::Refl(_)));
                    }
                    _ => panic!("Expected complete proof"),
                }
            }
            TacticResult::Failure(e) => panic!("reflexivity failed: {:?}", e),
        }
    }

    #[test]
    fn test_split_sigma() {
        let kernel = make_kernel();

        // Goal: ⊢ Σ(Unit, λ_. Unit)
        let goal_ty = Term::Sigma {
            fst_type: Box::new(Term::Unit),
            snd_type: Box::new(Term::Unit),
        };
        let goal = Goal::new(Context::empty(), goal_ty);

        let result = Split.apply(&kernel, &goal);

        match result {
            TacticResult::Success { subgoals, .. } => {
                assert_eq!(subgoals.len(), 2);
                // Both subgoals should be Unit
                assert!(matches!(subgoals[0].ty, Term::Unit));
                assert!(matches!(subgoals[1].ty, Term::Unit));
            }
            TacticResult::Failure(e) => panic!("split failed: {:?}", e),
        }
    }
}
