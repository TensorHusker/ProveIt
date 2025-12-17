//! Normalization for TTT terms.
//!
//! We implement call-by-name reduction to weak head normal form (WHNF),
//! followed by recursive normalization for full beta-normal form.
//!
//! # Reduction Rules
//!
//! Beta-reductions (computation):
//! - `(lambda.M) N --> M[N/0]`
//! - `fst<M, N> --> M`
//! - `snd<M, N> --> N`
//! - `J(C, c, refl(a)) --> c[a/0]`
//! - `unit-elim(C, c, *) --> c`
//! - `if-then-else(C, t, f, true) --> t`
//! - `if-then-else(C, t, f, false) --> f`
//!
//! Eta-expansions (extensionality):
//! - `M : Pi(x:A).B --> lambda.(M[up] #0)` when M is not a lambda
//! - `M : Sigma(x:A).B --> <fst(M), snd(M)>` when M is not a pair (optional)
//!
//! # Termination
//!
//! Strong normalization is guaranteed for well-typed terms by the type
//! structure of TTT. Untyped terms may loop; clients must ensure type
//! correctness before calling normalization.

use crate::term::Term;

/// The result of a single reduction step.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Step {
    /// Reduction succeeded, producing a new term.
    Reduced(Term),
    /// Term is already in normal form (no reduction possible).
    Stuck,
}

impl Step {
    pub fn is_reduced(&self) -> bool {
        matches!(self, Step::Reduced(_))
    }

    pub fn is_stuck(&self) -> bool {
        matches!(self, Step::Stuck)
    }
}

impl Term {
    // =========================================================================
    // WEAK HEAD NORMAL FORM
    // =========================================================================

    /// Reduce to weak head normal form.
    ///
    /// WHNF exposes the outermost constructor, reducing only as needed.
    /// This is the standard evaluation strategy for type checking.
    ///
    /// # Fuel
    ///
    /// The optional `fuel` parameter limits reduction steps to prevent
    /// non-termination on ill-typed terms. Pass `None` for unlimited.
    pub fn whnf(&self) -> Term {
        self.whnf_with_fuel(None)
    }

    pub fn whnf_with_fuel(&self, fuel: Option<usize>) -> Term {
        let mut current = self.clone();
        let mut steps = 0;

        loop {
            if let Some(max) = fuel {
                if steps >= max {
                    return current; // Out of fuel
                }
            }

            match current.step_whnf() {
                Step::Reduced(next) => {
                    current = next;
                    steps += 1;
                }
                Step::Stuck => return current,
            }
        }
    }

    /// Perform a single WHNF reduction step.
    pub fn step_whnf(&self) -> Step {
        match self {
            // Annotations: strip immediately
            Term::Ann { term, .. } => Step::Reduced(term.as_ref().clone()),

            // Beta-reduction for application
            Term::App { func, arg } => {
                // First, reduce func to WHNF
                match func.step_whnf() {
                    Step::Reduced(func_reduced) => Step::Reduced(Term::App {
                        func: Box::new(func_reduced),
                        arg: arg.clone(),
                    }),
                    Step::Stuck => {
                        // func is in WHNF; check if it's a lambda
                        if let Term::Lambda { body } = func.as_ref() {
                            Step::Reduced(body.subst(arg.as_ref().clone()))
                        } else {
                            Step::Stuck
                        }
                    }
                }
            }

            // Beta-reduction for first projection
            Term::Fst(t) => match t.step_whnf() {
                Step::Reduced(t_reduced) => Step::Reduced(Term::Fst(Box::new(t_reduced))),
                Step::Stuck => {
                    if let Term::Pair { fst, .. } = t.as_ref() {
                        Step::Reduced(fst.as_ref().clone())
                    } else {
                        Step::Stuck
                    }
                }
            },

            // Beta-reduction for second projection
            Term::Snd(t) => match t.step_whnf() {
                Step::Reduced(t_reduced) => Step::Reduced(Term::Snd(Box::new(t_reduced))),
                Step::Stuck => {
                    if let Term::Pair { snd, .. } = t.as_ref() {
                        Step::Reduced(snd.as_ref().clone())
                    } else {
                        Step::Stuck
                    }
                }
            },

            // J-elimination on refl
            Term::J {
                motive,
                base_case,
                target,
            } => match target.step_whnf() {
                Step::Reduced(target_reduced) => Step::Reduced(Term::J {
                    motive: motive.clone(),
                    base_case: base_case.clone(),
                    target: Box::new(target_reduced),
                }),
                Step::Stuck => {
                    if let Term::Refl(a) = target.as_ref() {
                        // J(C, c, refl(a)) --> c[a/0]
                        Step::Reduced(base_case.subst(a.as_ref().clone()))
                    } else {
                        Step::Stuck
                    }
                }
            },

            // Unit elimination on star
            Term::UnitElim {
                motive,
                base_case,
                scrutinee,
            } => match scrutinee.step_whnf() {
                Step::Reduced(s_reduced) => Step::Reduced(Term::UnitElim {
                    motive: motive.clone(),
                    base_case: base_case.clone(),
                    scrutinee: Box::new(s_reduced),
                }),
                Step::Stuck => {
                    if let Term::Star = scrutinee.as_ref() {
                        Step::Reduced(base_case.as_ref().clone())
                    } else {
                        Step::Stuck
                    }
                }
            },

            // Boolean elimination
            Term::BoolElim {
                motive,
                true_case,
                false_case,
                scrutinee,
            } => match scrutinee.step_whnf() {
                Step::Reduced(s_reduced) => Step::Reduced(Term::BoolElim {
                    motive: motive.clone(),
                    true_case: true_case.clone(),
                    false_case: false_case.clone(),
                    scrutinee: Box::new(s_reduced),
                }),
                Step::Stuck => match scrutinee.as_ref() {
                    Term::True => Step::Reduced(true_case.as_ref().clone()),
                    Term::False => Step::Reduced(false_case.as_ref().clone()),
                    _ => Step::Stuck,
                },
            },

            // EmptyElim: never reduces (no intro form for Empty)
            Term::EmptyElim { .. } => Step::Stuck,

            // All other forms are already in WHNF
            _ => Step::Stuck,
        }
    }

    // =========================================================================
    // FULL NORMALIZATION (beta-normal form)
    // =========================================================================

    /// Fully normalize to beta-normal form.
    ///
    /// This recursively normalizes all subterms after reaching WHNF.
    /// Required for definitional equality checking.
    pub fn normalize(&self) -> Term {
        self.normalize_with_fuel(None)
    }

    pub fn normalize_with_fuel(&self, fuel: Option<usize>) -> Term {
        // First reduce to WHNF
        let whnf = self.whnf_with_fuel(fuel);

        // Then recursively normalize subterms
        whnf.normalize_subterms(fuel)
    }

    /// Normalize subterms of a term already in WHNF.
    fn normalize_subterms(&self, fuel: Option<usize>) -> Term {
        match self {
            Term::Var(idx) => Term::Var(*idx),
            Term::Universe(l) => Term::Universe(l.clone()),

            Term::Pi { domain, codomain } => Term::Pi {
                domain: Box::new(domain.normalize_with_fuel(fuel)),
                codomain: Box::new(codomain.normalize_with_fuel(fuel)),
            },

            Term::Lambda { body } => Term::Lambda {
                body: Box::new(body.normalize_with_fuel(fuel)),
            },

            Term::App { func, arg } => Term::App {
                func: Box::new(func.normalize_with_fuel(fuel)),
                arg: Box::new(arg.normalize_with_fuel(fuel)),
            },

            Term::Sigma { fst_type, snd_type } => Term::Sigma {
                fst_type: Box::new(fst_type.normalize_with_fuel(fuel)),
                snd_type: Box::new(snd_type.normalize_with_fuel(fuel)),
            },

            Term::Pair { fst, snd } => Term::Pair {
                fst: Box::new(fst.normalize_with_fuel(fuel)),
                snd: Box::new(snd.normalize_with_fuel(fuel)),
            },

            Term::Fst(t) => Term::Fst(Box::new(t.normalize_with_fuel(fuel))),
            Term::Snd(t) => Term::Snd(Box::new(t.normalize_with_fuel(fuel))),

            Term::Id { ty, lhs, rhs } => Term::Id {
                ty: Box::new(ty.normalize_with_fuel(fuel)),
                lhs: Box::new(lhs.normalize_with_fuel(fuel)),
                rhs: Box::new(rhs.normalize_with_fuel(fuel)),
            },

            Term::Refl(t) => Term::Refl(Box::new(t.normalize_with_fuel(fuel))),

            Term::J {
                motive,
                base_case,
                target,
            } => Term::J {
                motive: Box::new(motive.normalize_with_fuel(fuel)),
                base_case: Box::new(base_case.normalize_with_fuel(fuel)),
                target: Box::new(target.normalize_with_fuel(fuel)),
            },

            Term::Empty => Term::Empty,
            Term::EmptyElim { motive, scrutinee } => Term::EmptyElim {
                motive: Box::new(motive.normalize_with_fuel(fuel)),
                scrutinee: Box::new(scrutinee.normalize_with_fuel(fuel)),
            },

            Term::Unit => Term::Unit,
            Term::Star => Term::Star,
            Term::UnitElim {
                motive,
                base_case,
                scrutinee,
            } => Term::UnitElim {
                motive: Box::new(motive.normalize_with_fuel(fuel)),
                base_case: Box::new(base_case.normalize_with_fuel(fuel)),
                scrutinee: Box::new(scrutinee.normalize_with_fuel(fuel)),
            },

            Term::Bool => Term::Bool,
            Term::True => Term::True,
            Term::False => Term::False,
            Term::BoolElim {
                motive,
                true_case,
                false_case,
                scrutinee,
            } => Term::BoolElim {
                motive: Box::new(motive.normalize_with_fuel(fuel)),
                true_case: Box::new(true_case.normalize_with_fuel(fuel)),
                false_case: Box::new(false_case.normalize_with_fuel(fuel)),
                scrutinee: Box::new(scrutinee.normalize_with_fuel(fuel)),
            },

            // Annotations should have been erased in WHNF
            Term::Ann { term, .. } => term.normalize_with_fuel(fuel),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_beta_reduction_simple() {
        // (lambda. #0) * --> *
        let app = Term::app(Term::lam(Term::var(0)), Term::Star);
        assert_eq!(app.normalize(), Term::Star);
    }

    #[test]
    fn test_beta_reduction_nested() {
        // (lambda. lambda. #1) * true --> *
        let inner = Term::lam(Term::lam(Term::var(1)));
        let app = Term::apps(inner, [Term::Star, Term::True]);
        assert_eq!(app.normalize(), Term::Star);
    }

    #[test]
    fn test_projection_fst() {
        // fst<*, true> --> *
        let pair = Term::pair(Term::Star, Term::True);
        let proj = Term::Fst(Box::new(pair));
        assert_eq!(proj.normalize(), Term::Star);
    }

    #[test]
    fn test_projection_snd() {
        // snd<*, true> --> true
        let pair = Term::pair(Term::Star, Term::True);
        let proj = Term::Snd(Box::new(pair));
        assert_eq!(proj.normalize(), Term::True);
    }

    #[test]
    fn test_bool_elim_true() {
        // if true then * else false --> *
        let if_term = Term::BoolElim {
            motive: Box::new(Term::Unit), // constant motive
            true_case: Box::new(Term::Star),
            false_case: Box::new(Term::False),
            scrutinee: Box::new(Term::True),
        };
        assert_eq!(if_term.normalize(), Term::Star);
    }

    #[test]
    fn test_bool_elim_false() {
        // if false then * else true --> true
        let if_term = Term::BoolElim {
            motive: Box::new(Term::Unit),
            true_case: Box::new(Term::Star),
            false_case: Box::new(Term::True),
            scrutinee: Box::new(Term::False),
        };
        assert_eq!(if_term.normalize(), Term::True);
    }

    #[test]
    fn test_j_elim_refl() {
        // J(C, c, refl(a)) --> c[a/0]
        // With base_case = *, target = refl(true), we get *[true/0] = *
        let j_term = Term::J {
            motive: Box::new(Term::Unit), // Simplified motive
            base_case: Box::new(Term::Star), // Base case is just Star (no lambda)
            target: Box::new(Term::refl(Term::True)),
        };
        // After J reduces: *[true/0] = * (Star has no free vars)
        assert_eq!(j_term.normalize(), Term::Star);
    }

    #[test]
    fn test_j_elim_refl_with_lambda() {
        // J(C, lambda.#0, refl(true)) --> (lambda.#0)[true/0] = lambda.#0
        // The substitution happens at depth 0, lambda body uses #0 (bound),
        // so no substitution inside the lambda body
        let j_term = Term::J {
            motive: Box::new(Term::Unit),
            base_case: Box::new(Term::lam(Term::var(0))), // identity function
            target: Box::new(Term::refl(Term::True)),
        };
        // After J reduces: (lambda.#0)[true/0] = lambda.#0
        assert_eq!(j_term.normalize(), Term::lam(Term::var(0)));
    }

    #[test]
    fn test_unit_elim() {
        // unit-elim(C, *, *) --> *
        let unit_elim = Term::UnitElim {
            motive: Box::new(Term::Unit),
            base_case: Box::new(Term::Star),
            scrutinee: Box::new(Term::Star),
        };
        assert_eq!(unit_elim.normalize(), Term::Star);
    }

    #[test]
    fn test_normalize_preserves_normal_form() {
        // Already normalized terms should be unchanged
        let normal_terms = vec![
            Term::var(0),
            Term::Star,
            Term::True,
            Term::lam(Term::var(0)),
            Term::universe(0),
        ];

        for term in normal_terms {
            assert_eq!(term.normalize(), term);
        }
    }

    #[test]
    fn test_whnf_stops_at_lambda() {
        // lambda. ((lambda. #0) *) should stay as-is in WHNF
        // (doesn't reduce under binders)
        let inner = Term::app(Term::lam(Term::var(0)), Term::Star);
        let outer = Term::lam(inner.clone());

        // WHNF: stops at lambda
        assert_eq!(outer.whnf(), outer);

        // Full normalize: reduces under lambda
        assert_eq!(outer.normalize(), Term::lam(Term::Star));
    }

    #[test]
    fn test_step_whnf() {
        // Single step: (lambda. #0) * --> *
        let app = Term::app(Term::lam(Term::var(0)), Term::Star);
        match app.step_whnf() {
            Step::Reduced(result) => assert_eq!(result, Term::Star),
            Step::Stuck => panic!("Expected reduction"),
        }
    }

    #[test]
    fn test_step_stuck() {
        // Variable is stuck
        assert!(Term::var(0).step_whnf().is_stuck());
        // Star is stuck
        assert!(Term::Star.step_whnf().is_stuck());
        // Lambda is stuck
        assert!(Term::lam(Term::var(0)).step_whnf().is_stuck());
    }

    #[test]
    fn test_fuel_limit() {
        // Create a term that takes multiple steps
        // (lambda. #0) ((lambda. #0) *)
        let inner = Term::app(Term::lam(Term::var(0)), Term::Star);
        let outer = Term::app(Term::lam(Term::var(0)), inner);

        // With 1 step of fuel, should not fully reduce
        let result = outer.whnf_with_fuel(Some(1));
        // After 1 step: (lambda. #0) *
        assert_eq!(result, Term::app(Term::lam(Term::var(0)), Term::Star));

        // With unlimited fuel, should fully reduce
        assert_eq!(outer.whnf(), Term::Star);
    }

    #[test]
    fn test_nested_projections() {
        // fst(snd(<*, <true, false>>)) --> true
        let inner_pair = Term::pair(Term::True, Term::False);
        let outer_pair = Term::pair(Term::Star, inner_pair);
        let snd_outer = Term::Snd(Box::new(outer_pair));
        let fst_result = Term::Fst(Box::new(snd_outer));

        assert_eq!(fst_result.normalize(), Term::True);
    }

    #[test]
    fn test_annotation_stripped() {
        // (true : Bool) --> true
        let ann = Term::ann(Term::True, Term::Bool);
        assert_eq!(ann.normalize(), Term::True);
    }

    #[test]
    fn test_nested_applications() {
        // ((lambda. lambda. #0) true) * --> *
        let lam = Term::lam(Term::lam(Term::var(0)));
        let app1 = Term::app(lam, Term::True);
        let app2 = Term::app(app1, Term::Star);
        assert_eq!(app2.normalize(), Term::Star);
    }

    #[test]
    fn test_whnf_idempotent() {
        let terms = vec![
            Term::app(Term::lam(Term::var(0)), Term::Star),
            Term::Fst(Box::new(Term::pair(Term::True, Term::False))),
            Term::var(0),
        ];

        for term in terms {
            let whnf1 = term.whnf();
            let whnf2 = whnf1.whnf();
            assert_eq!(whnf1, whnf2, "WHNF should be idempotent");
        }
    }

    #[test]
    fn test_normalize_idempotent() {
        let terms = vec![
            Term::app(Term::lam(Term::var(0)), Term::Star),
            Term::lam(Term::app(Term::lam(Term::var(0)), Term::Star)),
            Term::pair(
                Term::Fst(Box::new(Term::pair(Term::True, Term::False))),
                Term::Star,
            ),
        ];

        for term in terms {
            let nf1 = term.normalize();
            let nf2 = nf1.normalize();
            assert_eq!(nf1, nf2, "Normalization should be idempotent");
        }
    }
}
