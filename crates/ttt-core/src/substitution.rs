//! Capture-avoiding substitution for TTT terms.
//!
//! Substitution is the operation [N/x]M, which replaces all free occurrences
//! of variable x in M with term N. In de Bruijn notation, this becomes
//! [N/0]M where we substitute for index 0.
//!
//! # Key Invariant
//!
//! After substitution `M.subst(N)`, the result has one fewer free variable:
//! - Var(0) is replaced by N
//! - Var(n+1) becomes Var(n) for n >= 0

use crate::term::{DeBruijnIndex, Term};

impl Term {
    /// Substitute `replacement` for variable 0, then shift down.
    ///
    /// This is the standard capture-avoiding substitution:
    /// - Var(0) -> replacement
    /// - Var(n+1) -> Var(n) for n >= 0
    /// - Under binders, increment the cutoff
    ///
    /// # Examples
    ///
    /// ```
    /// use ttt_core::term::Term;
    ///
    /// // (lambda. #0) [Star/0] = lambda. #0  (bound variable unchanged)
    /// let lam = Term::lam(Term::var(0));
    /// assert_eq!(lam.subst(Term::Star), Term::lam(Term::var(0)));
    ///
    /// // #0 [Star/0] = Star
    /// assert_eq!(Term::var(0).subst(Term::Star), Term::Star);
    ///
    /// // #1 [Star/0] = #0  (shift down free variable)
    /// assert_eq!(Term::var(1).subst(Term::Star), Term::var(0));
    /// ```
    pub fn subst(&self, replacement: Term) -> Term {
        self.subst_at(0, replacement)
    }

    /// Substitute `replacement` for variable at `target_idx`.
    ///
    /// More general form of substitution that allows targeting any index.
    pub fn subst_at(&self, target_idx: u32, replacement: Term) -> Term {
        self.subst_inner(target_idx, &replacement, 0)
    }

    /// Core substitution implementation.
    ///
    /// - `target_idx`: The variable index to replace (adjusted for binders)
    /// - `replacement`: The term to substitute
    /// - `depth`: Current binder depth (how many binders we've entered)
    fn subst_inner(&self, target_idx: u32, replacement: &Term, depth: u32) -> Term {
        match self {
            Term::Var(DeBruijnIndex(idx)) => {
                let adjusted_target = target_idx + depth;
                if *idx == adjusted_target {
                    // Found the variable to replace
                    // Shift the replacement to account for binders we've passed through
                    replacement.shift(depth as i32, 0)
                } else if *idx > adjusted_target {
                    // Free variable above target: shift down by 1
                    Term::Var(DeBruijnIndex(idx - 1))
                } else {
                    // Bound variable below target: unchanged
                    Term::Var(DeBruijnIndex(*idx))
                }
            }

            Term::Universe(l) => Term::Universe(l.clone()),

            Term::Pi { domain, codomain } => Term::Pi {
                domain: Box::new(domain.subst_inner(target_idx, replacement, depth)),
                codomain: Box::new(codomain.subst_inner(target_idx, replacement, depth + 1)),
            },

            Term::Lambda { body } => Term::Lambda {
                body: Box::new(body.subst_inner(target_idx, replacement, depth + 1)),
            },

            Term::App { func, arg } => Term::App {
                func: Box::new(func.subst_inner(target_idx, replacement, depth)),
                arg: Box::new(arg.subst_inner(target_idx, replacement, depth)),
            },

            Term::Sigma { fst_type, snd_type } => Term::Sigma {
                fst_type: Box::new(fst_type.subst_inner(target_idx, replacement, depth)),
                snd_type: Box::new(snd_type.subst_inner(target_idx, replacement, depth + 1)),
            },

            Term::Pair { fst, snd } => Term::Pair {
                fst: Box::new(fst.subst_inner(target_idx, replacement, depth)),
                snd: Box::new(snd.subst_inner(target_idx, replacement, depth)),
            },

            Term::Fst(t) => Term::Fst(Box::new(t.subst_inner(target_idx, replacement, depth))),
            Term::Snd(t) => Term::Snd(Box::new(t.subst_inner(target_idx, replacement, depth))),

            Term::Id { ty, lhs, rhs } => Term::Id {
                ty: Box::new(ty.subst_inner(target_idx, replacement, depth)),
                lhs: Box::new(lhs.subst_inner(target_idx, replacement, depth)),
                rhs: Box::new(rhs.subst_inner(target_idx, replacement, depth)),
            },

            Term::Refl(t) => Term::Refl(Box::new(t.subst_inner(target_idx, replacement, depth))),

            Term::J {
                motive,
                base_case,
                target,
            } => Term::J {
                motive: Box::new(motive.subst_inner(target_idx, replacement, depth + 3)),
                base_case: Box::new(base_case.subst_inner(target_idx, replacement, depth + 1)),
                target: Box::new(target.subst_inner(target_idx, replacement, depth)),
            },

            Term::Empty => Term::Empty,
            Term::EmptyElim { motive, scrutinee } => Term::EmptyElim {
                motive: Box::new(motive.subst_inner(target_idx, replacement, depth)),
                scrutinee: Box::new(scrutinee.subst_inner(target_idx, replacement, depth)),
            },

            Term::Unit => Term::Unit,
            Term::Star => Term::Star,
            Term::UnitElim {
                motive,
                base_case,
                scrutinee,
            } => Term::UnitElim {
                motive: Box::new(motive.subst_inner(target_idx, replacement, depth + 1)),
                base_case: Box::new(base_case.subst_inner(target_idx, replacement, depth)),
                scrutinee: Box::new(scrutinee.subst_inner(target_idx, replacement, depth)),
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
                motive: Box::new(motive.subst_inner(target_idx, replacement, depth + 1)),
                true_case: Box::new(true_case.subst_inner(target_idx, replacement, depth)),
                false_case: Box::new(false_case.subst_inner(target_idx, replacement, depth)),
                scrutinee: Box::new(scrutinee.subst_inner(target_idx, replacement, depth)),
            },

            Term::Ann { term, ty } => Term::Ann {
                term: Box::new(term.subst_inner(target_idx, replacement, depth)),
                ty: Box::new(ty.subst_inner(target_idx, replacement, depth)),
            },
        }
    }

    /// Simultaneous substitution for multiple variables.
    ///
    /// `substs[0]` replaces Var(0), `substs[1]` replaces Var(1), etc.
    /// Variables beyond the substitution list are shifted down.
    pub fn subst_many(&self, substs: &[Term]) -> Term {
        if substs.is_empty() {
            return self.clone();
        }

        self.subst_many_inner(substs, 0)
    }

    fn subst_many_inner(&self, substs: &[Term], depth: u32) -> Term {
        match self {
            Term::Var(DeBruijnIndex(idx)) => {
                if *idx < depth {
                    // Bound by a binder we entered
                    Term::Var(DeBruijnIndex(*idx))
                } else {
                    let adjusted = *idx - depth;
                    if (adjusted as usize) < substs.len() {
                        // In range of substitution
                        substs[adjusted as usize].shift(depth as i32, 0)
                    } else {
                        // Beyond substitution: shift down by |substs|
                        Term::Var(DeBruijnIndex(*idx - substs.len() as u32))
                    }
                }
            }

            Term::Universe(l) => Term::Universe(l.clone()),

            Term::Pi { domain, codomain } => Term::Pi {
                domain: Box::new(domain.subst_many_inner(substs, depth)),
                codomain: Box::new(codomain.subst_many_inner(substs, depth + 1)),
            },

            Term::Lambda { body } => Term::Lambda {
                body: Box::new(body.subst_many_inner(substs, depth + 1)),
            },

            Term::App { func, arg } => Term::App {
                func: Box::new(func.subst_many_inner(substs, depth)),
                arg: Box::new(arg.subst_many_inner(substs, depth)),
            },

            Term::Sigma { fst_type, snd_type } => Term::Sigma {
                fst_type: Box::new(fst_type.subst_many_inner(substs, depth)),
                snd_type: Box::new(snd_type.subst_many_inner(substs, depth + 1)),
            },

            Term::Pair { fst, snd } => Term::Pair {
                fst: Box::new(fst.subst_many_inner(substs, depth)),
                snd: Box::new(snd.subst_many_inner(substs, depth)),
            },

            Term::Fst(t) => Term::Fst(Box::new(t.subst_many_inner(substs, depth))),
            Term::Snd(t) => Term::Snd(Box::new(t.subst_many_inner(substs, depth))),

            Term::Id { ty, lhs, rhs } => Term::Id {
                ty: Box::new(ty.subst_many_inner(substs, depth)),
                lhs: Box::new(lhs.subst_many_inner(substs, depth)),
                rhs: Box::new(rhs.subst_many_inner(substs, depth)),
            },

            Term::Refl(t) => Term::Refl(Box::new(t.subst_many_inner(substs, depth))),

            Term::J {
                motive,
                base_case,
                target,
            } => Term::J {
                motive: Box::new(motive.subst_many_inner(substs, depth + 3)),
                base_case: Box::new(base_case.subst_many_inner(substs, depth + 1)),
                target: Box::new(target.subst_many_inner(substs, depth)),
            },

            Term::Empty => Term::Empty,
            Term::EmptyElim { motive, scrutinee } => Term::EmptyElim {
                motive: Box::new(motive.subst_many_inner(substs, depth)),
                scrutinee: Box::new(scrutinee.subst_many_inner(substs, depth)),
            },

            Term::Unit => Term::Unit,
            Term::Star => Term::Star,
            Term::UnitElim {
                motive,
                base_case,
                scrutinee,
            } => Term::UnitElim {
                motive: Box::new(motive.subst_many_inner(substs, depth + 1)),
                base_case: Box::new(base_case.subst_many_inner(substs, depth)),
                scrutinee: Box::new(scrutinee.subst_many_inner(substs, depth)),
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
                motive: Box::new(motive.subst_many_inner(substs, depth + 1)),
                true_case: Box::new(true_case.subst_many_inner(substs, depth)),
                false_case: Box::new(false_case.subst_many_inner(substs, depth)),
                scrutinee: Box::new(scrutinee.subst_many_inner(substs, depth)),
            },

            Term::Ann { term, ty } => Term::Ann {
                term: Box::new(term.subst_many_inner(substs, depth)),
                ty: Box::new(ty.subst_many_inner(substs, depth)),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subst_var_hit() {
        // #0 [*/0] = *
        let result = Term::var(0).subst(Term::Star);
        assert_eq!(result, Term::Star);
    }

    #[test]
    fn test_subst_var_miss_above() {
        // #1 [*/0] = #0  (shift down)
        let result = Term::var(1).subst(Term::Star);
        assert_eq!(result, Term::var(0));
    }

    #[test]
    fn test_subst_under_lambda() {
        // lambda. #0 [*/0] = lambda. #0  (bound var unchanged)
        let lam = Term::lam(Term::var(0));
        let result = lam.subst(Term::Star);
        assert_eq!(result, Term::lam(Term::var(0)));
    }

    #[test]
    fn test_subst_free_under_lambda() {
        // lambda. #1 [*/0] = lambda. *
        let lam = Term::lam(Term::var(1));
        let result = lam.subst(Term::Star);
        assert_eq!(result, Term::lam(Term::Star));
    }

    #[test]
    fn test_subst_shifts_replacement() {
        // lambda. #1 [#0/0] = lambda. #1
        // - #1 inside the lambda refers to index 0 outside (a free variable)
        // - We substitute #0 (another free variable) for that
        // - Inside the lambda, #0 gets shifted to #1
        // - So the result is still lambda. #1
        let lam = Term::lam(Term::var(1));
        let result = lam.subst(Term::var(0));
        assert_eq!(result, Term::lam(Term::var(1)));
    }

    #[test]
    fn test_subst_captures_correctly() {
        // Verify that substitution correctly shifts replacements
        // lambda. #1 [true/0] = lambda. true
        // - #1 inside lambda refers to a free variable (index 0 outside)
        // - We substitute true for it
        // - true has no variables so shifting doesn't change it
        let lam = Term::lam(Term::var(1));
        let result = lam.subst(Term::True);
        assert_eq!(result, Term::lam(Term::True));
    }

    #[test]
    fn test_subst_nested_lambda() {
        // lambda. lambda. #2 [*/0] = lambda. lambda. *
        let nested = Term::lam(Term::lam(Term::var(2)));
        let result = nested.subst(Term::Star);
        assert_eq!(result, Term::lam(Term::lam(Term::Star)));
    }

    #[test]
    fn test_subst_application() {
        // (#0 #1) [*/0] = (* #0)
        let app = Term::app(Term::var(0), Term::var(1));
        let result = app.subst(Term::Star);
        assert_eq!(result, Term::app(Term::Star, Term::var(0)));
    }

    #[test]
    fn test_subst_pair() {
        // <#0, #1> [*/0] = <*, #0>
        let pair = Term::pair(Term::var(0), Term::var(1));
        let result = pair.subst(Term::Star);
        assert_eq!(result, Term::pair(Term::Star, Term::var(0)));
    }

    #[test]
    fn test_subst_at_different_index() {
        // #1 [*/1] = *
        let result = Term::var(1).subst_at(1, Term::Star);
        assert_eq!(result, Term::Star);

        // #0 [*/1] = #0 (unchanged, below target)
        let result2 = Term::var(0).subst_at(1, Term::Star);
        assert_eq!(result2, Term::var(0));

        // #2 [*/1] = #1 (shifted down)
        let result3 = Term::var(2).subst_at(1, Term::Star);
        assert_eq!(result3, Term::var(1));
    }

    #[test]
    fn test_subst_many_empty() {
        let term = Term::var(0);
        assert_eq!(term.subst_many(&[]), term);
    }

    #[test]
    fn test_subst_many_single() {
        // Same as regular subst
        let result = Term::var(0).subst_many(&[Term::Star]);
        assert_eq!(result, Term::Star);
    }

    #[test]
    fn test_subst_many_multiple() {
        // #0 [*, true, false] = *
        // #1 [*, true, false] = true
        // #2 [*, true, false] = false
        // #3 [*, true, false] = #0 (shifted)
        assert_eq!(
            Term::var(0).subst_many(&[Term::Star, Term::True, Term::False]),
            Term::Star
        );
        assert_eq!(
            Term::var(1).subst_many(&[Term::Star, Term::True, Term::False]),
            Term::True
        );
        assert_eq!(
            Term::var(2).subst_many(&[Term::Star, Term::True, Term::False]),
            Term::False
        );
        assert_eq!(
            Term::var(3).subst_many(&[Term::Star, Term::True, Term::False]),
            Term::var(0)
        );
    }

    #[test]
    fn test_subst_preserves_type_formers() {
        assert_eq!(Term::Unit.subst(Term::Star), Term::Unit);
        assert_eq!(Term::Bool.subst(Term::Star), Term::Bool);
        assert_eq!(Term::Empty.subst(Term::Star), Term::Empty);
        assert_eq!(Term::universe(0).subst(Term::Star), Term::universe(0));
    }

    #[test]
    fn test_subst_preserves_intro_forms() {
        assert_eq!(Term::Star.subst(Term::True), Term::Star);
        assert_eq!(Term::True.subst(Term::Star), Term::True);
        assert_eq!(Term::False.subst(Term::Star), Term::False);
    }
}
