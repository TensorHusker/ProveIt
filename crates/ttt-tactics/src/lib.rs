//! TTT Tactics - Proof Automation for Tiny Type Theory
//!
//! This crate provides tactics for automated proof construction in TTT.
//! Tactics transform proof goals into simpler subgoals or complete proofs.
//!
//! # Architecture
//!
//! The tactics system is built around the `Tactic` trait, which defines
//! how tactics transform goals. The main components are:
//!
//! - **Goals**: A context and type to prove
//! - **Tactics**: Transformations on goals
//! - **ProofState**: Tracks progress through a proof
//!
//! # Example
//!
//! ```
//! use ttt_tactics::prelude::*;
//! use ttt_core::prelude::*;
//! use ttt_kernel::prelude::*;
//!
//! let kernel = TrustKernel::new();
//!
//! // Goal: prove Unit
//! let goal = Goal::new(Context::empty(), Term::Unit);
//!
//! // The trivial tactic solves it
//! let result = Trivial.apply(&kernel, &goal);
//! assert!(result.is_success());
//! ```
//!
//! # Available Tactics
//!
//! ## Basic Tactics
//!
//! - `intro` - Introduce a hypothesis (for Pi types)
//! - `exact` - Provide an exact proof term
//! - `assumption` - Find proof in hypotheses
//! - `apply` - Apply a function
//! - `split` - Split a Sigma type goal
//! - `reflexivity` - Prove identity by reflexivity
//! - `trivial` - Solve trivial goals (Unit, reflexive Id)
//!
//! ## Automation Tactics
//!
//! - `auto` - Automated proof search
//! - `eauto` - Enhanced auto with hints
//! - `intuition` - Intuitionistic propositional solver
//! - `constructor` - Try type constructors
//!
//! ## Rewriting Tactics
//!
//! - `rewrite` - Rewrite using an equality
//! - `simp` - Simplification
//! - `unfold` - Unfold definitions

pub mod automation;
pub mod basic;
pub mod rewriting;
pub mod tactic;

pub mod prelude {
    //! Convenient re-exports for common usage.

    // Core tactic types
    pub use crate::tactic::{
        Goal, ProofBuilder, ProofState, ProofTerm, Tactic, TacticError, TacticResult,
    };

    // Basic tactics
    pub use crate::basic::{
        Apply, Assumption, Destruct, Exact, Intro, Left, Reflexivity, Right, Split, Trivial,
    };

    // Automation tactics
    pub use crate::automation::{Auto, AutoConfig, Constructor, Decide, EAuto, FirstOrder, Intuition};

    // Rewriting tactics
    pub use crate::rewriting::{Rewrite, RewriteRule, Simp, Unfold};

    // Convenient constructors
    pub use crate::basic::{apply, apply_hyp, exact, intro, intro_named};
    pub use crate::automation::{auto, eauto, intuition};
    pub use crate::rewriting::{rewrite, rewrite_rev, simp, unfold};
}

pub use prelude::*;

#[cfg(test)]
mod integration_tests {
    use super::prelude::*;
    use ttt_core::prelude::*;
    use ttt_kernel::prelude::*;

    fn make_kernel() -> TrustKernel {
        TrustKernel::new()
    }

    #[test]
    fn test_prove_unit() {
        let kernel = make_kernel();
        let goal = Goal::new(Context::empty(), Term::Unit);

        let mut state = ProofState::new(goal);
        state.apply_tactic(&kernel, &Trivial).unwrap();

        assert!(state.is_complete());
        assert!(matches!(state.proof_term(), Some(Term::Star)));
    }

    #[test]
    fn test_prove_identity_function() {
        let kernel = make_kernel();

        // Goal: ⊢ Unit → Unit
        let goal_ty = Term::arrow(Term::Unit, Term::Unit);
        let goal = Goal::new(Context::empty(), goal_ty);

        let mut state = ProofState::new(goal);

        // intro
        state.apply_tactic(&kernel, &Intro::new()).unwrap();

        // assumption (or trivial)
        state.apply_tactic(&kernel, &Trivial).unwrap();

        assert!(state.is_complete());
    }

    #[test]
    fn test_prove_reflexivity() {
        let kernel = make_kernel();

        // Goal: ⊢ Id(Unit, *, *)
        let goal_ty = Term::Id {
            ty: Box::new(Term::Unit),
            lhs: Box::new(Term::Star),
            rhs: Box::new(Term::Star),
        };
        let goal = Goal::new(Context::empty(), goal_ty);

        let mut state = ProofState::new(goal);
        state.apply_tactic(&kernel, &Reflexivity).unwrap();

        assert!(state.is_complete());
    }

    #[test]
    fn test_auto_proves_unit() {
        let kernel = make_kernel();
        let goal = Goal::new(Context::empty(), Term::Unit);

        let result = Auto::new().apply(&kernel, &goal);
        assert!(result.is_success());
    }

    #[test]
    fn test_auto_proves_nested_arrow() {
        let kernel = make_kernel();

        // Goal: ⊢ Unit → Unit → Unit
        let goal_ty = Term::arrow(Term::Unit, Term::arrow(Term::Unit, Term::Unit));
        let goal = Goal::new(Context::empty(), goal_ty);

        let result = Auto::new().apply(&kernel, &goal);
        assert!(result.is_success());
    }

    #[test]
    fn test_auto_proves_product() {
        let kernel = make_kernel();

        // Goal: ⊢ Unit × Unit
        let goal_ty = Term::Sigma {
            fst_type: Box::new(Term::Unit),
            snd_type: Box::new(Term::Unit),
        };
        let goal = Goal::new(Context::empty(), goal_ty);

        let result = Auto::new().apply(&kernel, &goal);
        assert!(result.is_success());
    }

    #[test]
    fn test_verified_proof() {
        let kernel = make_kernel();

        // Prove Unit and verify with the kernel
        let goal = Goal::new(Context::empty(), Term::Unit);
        let mut state = ProofState::new(goal);

        state.apply_tactic(&kernel, &Trivial).unwrap();
        assert!(state.is_complete());

        // Verify the proof
        let verified = state.verify(&kernel);
        assert!(verified.is_ok());
    }

    #[test]
    fn test_exact_tactic() {
        let kernel = make_kernel();

        // Goal: ⊢ Unit with exact term Star
        let goal = Goal::new(Context::empty(), Term::Unit);
        let result = Exact::new(Term::Star).apply(&kernel, &goal);

        assert!(result.is_success());
    }

    #[test]
    fn test_apply_tactic() {
        let kernel = make_kernel();

        // Context: f : Unit → Unit
        // Goal: ⊢ Unit
        // We should be able to apply f
        let ctx = Context::empty().extend(Term::arrow(Term::Unit, Term::Unit));
        let goal = Goal::new(ctx, Term::Unit);

        let result = Apply::hypothesis(0).apply(&kernel, &goal);

        match result {
            TacticResult::Success { subgoals, .. } => {
                // Should have subgoal for the argument
                assert_eq!(subgoals.len(), 1);
                assert!(matches!(subgoals[0].ty, Term::Unit));
            }
            TacticResult::Failure(e) => panic!("apply failed: {:?}", e),
        }
    }
}
