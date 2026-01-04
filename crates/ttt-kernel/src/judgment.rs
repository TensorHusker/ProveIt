//! Judgment types for the TTT type system.
//!
//! A judgment is a statement that the type system can validate.
//! In TTT, we have three forms of judgment:
//!
//! 1. Type formation: Gamma |- A : U_i (A is a type at universe level i)
//! 2. Type inhabitation: Gamma |- a : A (term a has type A)
//! 3. Definitional equality: Gamma |- a = b : A (a and b are equal terms of type A)
//!
//! All judgments are validated against a context Gamma.

use ttt_core::term::UniverseLevel;
use ttt_core::{Context, Term};

/// A judgment that can be checked by the trust kernel.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Judgment {
    /// Gamma |- A : U_i
    ///
    /// Assert that `ty` is a well-formed type at universe level `level`
    /// in context `ctx`.
    TypeFormation {
        ctx: Context,
        ty: Term,
        level: UniverseLevel,
    },

    /// Gamma |- term : ty
    ///
    /// Assert that `term` inhabits type `ty` in context `ctx`.
    TypeInhabitation { ctx: Context, term: Term, ty: Term },

    /// Gamma |- lhs = rhs : ty
    ///
    /// Assert that `lhs` and `rhs` are definitionally equal terms
    /// of type `ty` in context `ctx`.
    DefEquality {
        ctx: Context,
        lhs: Term,
        rhs: Term,
        ty: Term,
    },
}

impl Judgment {
    /// Create a type formation judgment.
    pub fn type_formation(ctx: Context, ty: Term, level: UniverseLevel) -> Self {
        Judgment::TypeFormation { ctx, ty, level }
    }

    /// Create a type inhabitation judgment.
    pub fn inhabitation(ctx: Context, term: Term, ty: Term) -> Self {
        Judgment::TypeInhabitation { ctx, term, ty }
    }

    /// Create a definitional equality judgment.
    pub fn equality(ctx: Context, lhs: Term, rhs: Term, ty: Term) -> Self {
        Judgment::DefEquality { ctx, lhs, rhs, ty }
    }

    /// Get the context of this judgment.
    pub fn context(&self) -> &Context {
        match self {
            Judgment::TypeFormation { ctx, .. } => ctx,
            Judgment::TypeInhabitation { ctx, .. } => ctx,
            Judgment::DefEquality { ctx, .. } => ctx,
        }
    }
}

/// A judgment that has been verified by the kernel.
///
/// This type can only be constructed by the kernel's `check` method.
/// It serves as proof that the judgment was validated.
#[derive(Clone, Debug)]
pub struct VerifiedJudgment {
    judgment: Judgment,
    // Private field prevents external construction
    _private: (),
}

impl VerifiedJudgment {
    /// Create a verified judgment.
    ///
    /// This is `pub(crate)` - only the kernel can create these.
    pub(crate) fn new(judgment: Judgment) -> Self {
        VerifiedJudgment {
            judgment,
            _private: (),
        }
    }

    /// Get the underlying judgment.
    pub fn judgment(&self) -> &Judgment {
        &self.judgment
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_judgment_creation() {
        let ctx = Context::empty();

        // Type formation
        let tf = Judgment::type_formation(ctx.clone(), Term::Bool, UniverseLevel::zero());
        assert!(matches!(tf, Judgment::TypeFormation { .. }));

        // Type inhabitation
        let ti = Judgment::inhabitation(ctx.clone(), Term::True, Term::Bool);
        assert!(matches!(ti, Judgment::TypeInhabitation { .. }));

        // Definitional equality
        let eq = Judgment::equality(ctx, Term::True, Term::True, Term::Bool);
        assert!(matches!(eq, Judgment::DefEquality { .. }));
    }

    #[test]
    fn test_judgment_context() {
        let ctx = Context::empty().extend(Term::Bool);
        let judgment = Judgment::inhabitation(ctx.clone(), Term::var(0), Term::Bool);
        assert_eq!(judgment.context(), &ctx);
    }
}
