//! Definitional equality for TTT terms.
//!
//! Two terms are definitionally equal (Gamma |- a = b : A) if they have the
//! same beta-normal form. This is the basis for type conversion in the
//! type checker.
//!
//! # Algorithm
//!
//! 1. Normalize both terms to beta-normal form
//! 2. Compare structurally (alpha-equivalence for de Bruijn terms)
//!
//! Since we use de Bruijn indices, alpha-equivalence is syntactic equality.

use crate::context::Context;
use crate::term::{Term, UniverseLevel};

/// Check if two terms are definitionally equal.
///
/// This normalizes both terms and compares structurally.
pub fn definitionally_equal(a: &Term, b: &Term) -> bool {
    let a_nf = a.normalize();
    let b_nf = b.normalize();
    structurally_equal(&a_nf, &b_nf)
}

/// Check if two terms are definitionally equal with fuel limit.
pub fn definitionally_equal_with_fuel(a: &Term, b: &Term, fuel: usize) -> Option<bool> {
    let a_nf = a.normalize_with_fuel(Some(fuel));
    let b_nf = b.normalize_with_fuel(Some(fuel));

    // If either term didn't fully normalize, we can't be sure
    if !a_nf.is_whnf() || !b_nf.is_whnf() {
        return None; // Indeterminate
    }

    Some(structurally_equal(&a_nf, &b_nf))
}

/// Check structural equality of two terms.
///
/// Assumes both terms are in normal form. With de Bruijn indices,
/// this is just recursive syntactic comparison.
pub fn structurally_equal(a: &Term, b: &Term) -> bool {
    match (a, b) {
        (Term::Var(i), Term::Var(j)) => i == j,

        (Term::Universe(l1), Term::Universe(l2)) => universe_level_eq(l1, l2),

        (
            Term::Pi {
                domain: d1,
                codomain: c1,
            },
            Term::Pi {
                domain: d2,
                codomain: c2,
            },
        ) => structurally_equal(d1, d2) && structurally_equal(c1, c2),

        (Term::Lambda { body: b1 }, Term::Lambda { body: b2 }) => structurally_equal(b1, b2),

        (
            Term::App {
                func: f1,
                arg: a1,
            },
            Term::App {
                func: f2,
                arg: a2,
            },
        ) => structurally_equal(f1, f2) && structurally_equal(a1, a2),

        (
            Term::Sigma {
                fst_type: f1,
                snd_type: s1,
            },
            Term::Sigma {
                fst_type: f2,
                snd_type: s2,
            },
        ) => structurally_equal(f1, f2) && structurally_equal(s1, s2),

        (Term::Pair { fst: f1, snd: s1 }, Term::Pair { fst: f2, snd: s2 }) => {
            structurally_equal(f1, f2) && structurally_equal(s1, s2)
        }

        (Term::Fst(t1), Term::Fst(t2)) => structurally_equal(t1, t2),
        (Term::Snd(t1), Term::Snd(t2)) => structurally_equal(t1, t2),

        (
            Term::Id {
                ty: t1,
                lhs: l1,
                rhs: r1,
            },
            Term::Id {
                ty: t2,
                lhs: l2,
                rhs: r2,
            },
        ) => structurally_equal(t1, t2) && structurally_equal(l1, l2) && structurally_equal(r1, r2),

        (Term::Refl(t1), Term::Refl(t2)) => structurally_equal(t1, t2),

        (
            Term::J {
                motive: m1,
                base_case: b1,
                target: t1,
            },
            Term::J {
                motive: m2,
                base_case: b2,
                target: t2,
            },
        ) => {
            structurally_equal(m1, m2)
                && structurally_equal(b1, b2)
                && structurally_equal(t1, t2)
        }

        (Term::Empty, Term::Empty) => true,
        (
            Term::EmptyElim {
                motive: m1,
                scrutinee: s1,
            },
            Term::EmptyElim {
                motive: m2,
                scrutinee: s2,
            },
        ) => structurally_equal(m1, m2) && structurally_equal(s1, s2),

        (Term::Unit, Term::Unit) => true,
        (Term::Star, Term::Star) => true,
        (
            Term::UnitElim {
                motive: m1,
                base_case: b1,
                scrutinee: s1,
            },
            Term::UnitElim {
                motive: m2,
                base_case: b2,
                scrutinee: s2,
            },
        ) => {
            structurally_equal(m1, m2)
                && structurally_equal(b1, b2)
                && structurally_equal(s1, s2)
        }

        (Term::Bool, Term::Bool) => true,
        (Term::True, Term::True) => true,
        (Term::False, Term::False) => true,
        (
            Term::BoolElim {
                motive: m1,
                true_case: t1,
                false_case: f1,
                scrutinee: s1,
            },
            Term::BoolElim {
                motive: m2,
                true_case: t2,
                false_case: f2,
                scrutinee: s2,
            },
        ) => {
            structurally_equal(m1, m2)
                && structurally_equal(t1, t2)
                && structurally_equal(f1, f2)
                && structurally_equal(s1, s2)
        }

        // Annotations should be erased during normalization
        (Term::Ann { term: t1, .. }, t2) => structurally_equal(t1, t2),
        (t1, Term::Ann { term: t2, .. }) => structurally_equal(t1, t2),

        _ => false,
    }
}

/// Check equality of universe levels.
pub fn universe_level_eq(l1: &UniverseLevel, l2: &UniverseLevel) -> bool {
    match (l1, l2) {
        (UniverseLevel::Zero, UniverseLevel::Zero) => true,
        (UniverseLevel::Succ(a), UniverseLevel::Succ(b)) => universe_level_eq(a, b),
        (UniverseLevel::Max(a1, b1), UniverseLevel::Max(a2, b2)) => {
            universe_level_eq(a1, a2) && universe_level_eq(b1, b2)
        }
        (UniverseLevel::Var(v1), UniverseLevel::Var(v2)) => v1 == v2,
        // Try to simplify to concrete numbers
        _ => l1.to_nat() == l2.to_nat() && l1.to_nat().is_some(),
    }
}

/// Context-aware equality checking.
///
/// This extends definitional equality to handle let-definitions in context.
#[allow(dead_code)]
pub fn equal_in_context(_ctx: &Context, a: &Term, b: &Term) -> bool {
    // WIP: Implement delta-unfolding for let-definitions
    // For now, just use basic definitional equality
    definitionally_equal(a, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equal_simple() {
        assert!(definitionally_equal(&Term::Star, &Term::Star));
        assert!(definitionally_equal(&Term::var(0), &Term::var(0)));
        assert!(!definitionally_equal(&Term::Star, &Term::True));
    }

    #[test]
    fn test_equal_after_reduction() {
        // (lambda. #0) * = *
        let app = Term::app(Term::lam(Term::var(0)), Term::Star);
        assert!(definitionally_equal(&app, &Term::Star));
    }

    #[test]
    fn test_equal_nested_reduction() {
        // (lambda. lambda. #0) true * = *
        let term = Term::apps(Term::lam(Term::lam(Term::var(0))), [Term::True, Term::Star]);
        assert!(definitionally_equal(&term, &Term::Star));
    }

    #[test]
    fn test_equal_projections() {
        // fst<*, true> = *
        let fst = Term::Fst(Box::new(Term::pair(Term::Star, Term::True)));
        assert!(definitionally_equal(&fst, &Term::Star));

        // snd<*, true> = true
        let snd = Term::Snd(Box::new(Term::pair(Term::Star, Term::True)));
        assert!(definitionally_equal(&snd, &Term::True));
    }

    #[test]
    fn test_universe_level_equality() {
        let u0 = Term::Universe(UniverseLevel::zero());
        let u0_again = Term::Universe(UniverseLevel::from_nat(0));
        let u1 = Term::Universe(UniverseLevel::from_nat(1));

        assert!(definitionally_equal(&u0, &u0_again));
        assert!(!definitionally_equal(&u0, &u1));
    }

    #[test]
    fn test_equal_lambdas() {
        // lambda. #0 = lambda. #0
        let lam1 = Term::lam(Term::var(0));
        let lam2 = Term::lam(Term::var(0));
        assert!(definitionally_equal(&lam1, &lam2));

        // lambda. #0 != lambda. *
        let lam3 = Term::lam(Term::Star);
        assert!(!definitionally_equal(&lam1, &lam3));
    }

    #[test]
    fn test_equal_pi_types() {
        // Pi(Bool).Unit = Pi(Bool).Unit
        let pi1 = Term::Pi {
            domain: Box::new(Term::Bool),
            codomain: Box::new(Term::Unit),
        };
        let pi2 = Term::Pi {
            domain: Box::new(Term::Bool),
            codomain: Box::new(Term::Unit),
        };
        assert!(definitionally_equal(&pi1, &pi2));
    }

    #[test]
    fn test_equal_sigma_types() {
        // Sigma(Bool).Unit = Sigma(Bool).Unit
        let sig1 = Term::Sigma {
            fst_type: Box::new(Term::Bool),
            snd_type: Box::new(Term::Unit),
        };
        let sig2 = Term::Sigma {
            fst_type: Box::new(Term::Bool),
            snd_type: Box::new(Term::Unit),
        };
        assert!(definitionally_equal(&sig1, &sig2));
    }

    #[test]
    fn test_equal_id_types() {
        // Id[Bool](true, true) = Id[Bool](true, true)
        let id1 = Term::id(Term::Bool, Term::True, Term::True);
        let id2 = Term::id(Term::Bool, Term::True, Term::True);
        assert!(definitionally_equal(&id1, &id2));

        // Id[Bool](true, true) != Id[Bool](true, false)
        let id3 = Term::id(Term::Bool, Term::True, Term::False);
        assert!(!definitionally_equal(&id1, &id3));
    }

    #[test]
    fn test_equal_refl() {
        // refl(true) = refl(true)
        let refl1 = Term::refl(Term::True);
        let refl2 = Term::refl(Term::True);
        assert!(definitionally_equal(&refl1, &refl2));
    }

    #[test]
    fn test_equal_with_annotation() {
        // (true : Bool) = true
        let ann = Term::ann(Term::True, Term::Bool);
        assert!(definitionally_equal(&ann, &Term::True));
    }

    #[test]
    fn test_structural_equal_finite_types() {
        assert!(structurally_equal(&Term::Empty, &Term::Empty));
        assert!(structurally_equal(&Term::Unit, &Term::Unit));
        assert!(structurally_equal(&Term::Bool, &Term::Bool));
        assert!(structurally_equal(&Term::Star, &Term::Star));
        assert!(structurally_equal(&Term::True, &Term::True));
        assert!(structurally_equal(&Term::False, &Term::False));

        assert!(!structurally_equal(&Term::Empty, &Term::Unit));
        assert!(!structurally_equal(&Term::True, &Term::False));
    }

    #[test]
    fn test_universe_level_eq() {
        assert!(universe_level_eq(
            &UniverseLevel::Zero,
            &UniverseLevel::Zero
        ));
        assert!(universe_level_eq(
            &UniverseLevel::from_nat(5),
            &UniverseLevel::from_nat(5)
        ));
        assert!(!universe_level_eq(
            &UniverseLevel::from_nat(3),
            &UniverseLevel::from_nat(4)
        ));
    }

    #[test]
    fn test_reflexivity() {
        let terms = vec![
            Term::var(0),
            Term::Star,
            Term::True,
            Term::lam(Term::var(0)),
            Term::universe(0),
            Term::pair(Term::True, Term::False),
        ];

        for term in terms {
            assert!(
                definitionally_equal(&term, &term),
                "Reflexivity failed for {:?}",
                term
            );
        }
    }

    #[test]
    fn test_symmetry() {
        let pairs = vec![
            (Term::Star, Term::Star),
            (Term::True, Term::True),
            (Term::Star, Term::True),
            (
                Term::app(Term::lam(Term::var(0)), Term::Star),
                Term::Star,
            ),
        ];

        for (a, b) in pairs {
            assert_eq!(
                definitionally_equal(&a, &b),
                definitionally_equal(&b, &a),
                "Symmetry failed for {:?} and {:?}",
                a,
                b
            );
        }
    }
}
