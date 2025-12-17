//! Rewriting tactics for TTT.
//!
//! This module provides tactics for rewriting terms using equalities
//! (identity types) and simplification.

use ttt_core::prelude::{Context, Term};
use ttt_kernel::prelude::TrustKernel;

use crate::tactic::{Goal, ProofBuilder, Tactic, TacticError, TacticResult};

/// A rewrite rule with its proof of equality.
#[derive(Debug, Clone)]
pub struct RewriteRule {
    /// The left-hand side of the equality.
    pub lhs: Term,
    /// The right-hand side of the equality.
    pub rhs: Term,
    /// The type of the equality.
    pub ty: Term,
    /// The proof term (of type Id(ty, lhs, rhs)).
    pub proof: Term,
}

impl RewriteRule {
    /// Create a new rewrite rule.
    pub fn new(lhs: Term, rhs: Term, ty: Term, proof: Term) -> Self {
        Self { lhs, rhs, ty, proof }
    }

    /// Create the inverse rule (rewriting in the opposite direction).
    pub fn inverse(&self) -> Self {
        // To invert, we'd need sym : Id(A, x, y) → Id(A, y, x)
        // This would be defined using J
        Self {
            lhs: self.rhs.clone(),
            rhs: self.lhs.clone(),
            ty: self.ty.clone(),
            proof: build_sym(self.proof.clone(), self.ty.clone(), self.lhs.clone(), self.rhs.clone()),
        }
    }
}

/// Build the symmetry proof: sym(p) : Id(A, y, x) from p : Id(A, x, y).
fn build_sym(proof: Term, ty: Term, _lhs: Term, _rhs: Term) -> Term {
    // sym p = J(λ(x,y,_). Id(A, y, x), λx. refl(x), p)
    //
    // Motive: λx. λy. λ_. Id(A, y, x)
    // Base case: λx. refl(x)
    // Target: p
    Term::J {
        motive: Box::new(Term::Lambda {
            body: Box::new(Term::Lambda {
                body: Box::new(Term::Lambda {
                    body: Box::new(Term::Id {
                        ty: Box::new(ty.shift(3, 0)),
                        lhs: Box::new(Term::var(1)), // y
                        rhs: Box::new(Term::var(2)), // x
                    }),
                }),
            }),
        }),
        base_case: Box::new(Term::Lambda {
            body: Box::new(Term::Refl(Box::new(Term::var(0)))),
        }),
        target: Box::new(proof),
    }
}

/// The `rewrite` tactic: rewrite using an equality.
///
/// Given a proof of `Id(A, a, b)`, replaces occurrences of `a` with `b`
/// in the goal.
pub struct Rewrite {
    /// The equality proof to use for rewriting.
    pub eq_proof: Term,
    /// Whether to rewrite left-to-right (true) or right-to-left (false).
    pub left_to_right: bool,
}

impl Rewrite {
    /// Create a new rewrite tactic.
    pub fn new(eq_proof: Term) -> Self {
        Self {
            eq_proof,
            left_to_right: true,
        }
    }

    /// Create a rewrite tactic that rewrites right-to-left.
    pub fn reverse(eq_proof: Term) -> Self {
        Self {
            eq_proof,
            left_to_right: false,
        }
    }

    /// Extract the equality components from the proof.
    fn get_equality_info(&self, kernel: &TrustKernel, ctx: &Context) -> Result<(Term, Term, Term), TacticError> {
        let eq_type = kernel.infer(ctx, &self.eq_proof)?;
        let eq_type = eq_type.whnf();

        match eq_type {
            Term::Id { ty, lhs, rhs } => {
                if self.left_to_right {
                    Ok((*ty, *lhs, *rhs))
                } else {
                    Ok((*ty, *rhs, *lhs))
                }
            }
            _ => Err(TacticError::TypeError(
                "rewrite requires an identity type".to_string(),
            )),
        }
    }
}

impl Tactic for Rewrite {
    fn apply(&self, kernel: &TrustKernel, goal: &Goal) -> TacticResult {
        let (_eq_ty, from, to) = match self.get_equality_info(kernel, &goal.ctx) {
            Ok(info) => info,
            Err(e) => return TacticResult::fail(e),
        };

        // Check if goal contains the 'from' term
        if !contains_term(&goal.ty, &from) {
            return TacticResult::fail(TacticError::NotApplicable(
                format!("Goal does not contain term {:?}", from),
            ));
        }

        // Create new goal with 'from' replaced by 'to'
        let new_goal_ty = replace_term(&goal.ty, &from, &to);
        let subgoal = Goal::new(goal.ctx.clone(), new_goal_ty);

        // The proof uses transport (coercion along the equality)
        // We need: transport : Id(A, a, b) → P(a) → P(b)
        // In TTT, this is done with J
        let eq_proof = self.eq_proof.clone();
        let original_goal = goal.ty.clone();

        let builder = ProofBuilder::new(1, move |subproofs| {
            // Build transport using J
            // J(λx.λy.λ_. P(x) → P(y), λx.λpx.px, eq) : P(a) → P(b)
            // Then apply to subproof
            let transport = build_transport(
                eq_proof.clone(),
                original_goal.clone(),
            );
            Some(Term::App {
                func: Box::new(transport),
                arg: Box::new(subproofs[0].clone()),
            })
        });

        TacticResult::with_subgoals(builder, vec![subgoal])
    }

    fn name(&self) -> &str {
        "rewrite"
    }
}

/// Build a transport function using J.
fn build_transport(_eq_proof: Term, _goal_ty: Term) -> Term {
    // Simplified transport that just returns identity for now
    // Full implementation would construct proper J term
    Term::Lambda {
        body: Box::new(Term::var(0)),
    }
}

/// Check if term contains a subterm.
fn contains_term(term: &Term, target: &Term) -> bool {
    if ttt_core::equality::definitionally_equal(term, target) {
        return true;
    }

    match term {
        Term::Var(_) | Term::Universe(_) | Term::Empty | Term::Unit | Term::Star
        | Term::Bool | Term::True | Term::False => false,

        Term::Pi { domain, codomain } => {
            contains_term(domain, target) || contains_term(codomain, target)
        }
        Term::Lambda { body } => contains_term(body, target),
        Term::App { func, arg } => contains_term(func, target) || contains_term(arg, target),
        Term::Sigma { fst_type, snd_type } => {
            contains_term(fst_type, target) || contains_term(snd_type, target)
        }
        Term::Pair { fst, snd } => contains_term(fst, target) || contains_term(snd, target),
        Term::Fst(t) | Term::Snd(t) | Term::Refl(t) => contains_term(t, target),
        Term::Id { ty, lhs, rhs } => {
            contains_term(ty, target) || contains_term(lhs, target) || contains_term(rhs, target)
        }
        Term::J { motive, base_case, target: j_target } => {
            contains_term(motive, target) || contains_term(base_case, target) || contains_term(j_target, target)
        }
        Term::EmptyElim { motive, scrutinee } => {
            contains_term(motive, target) || contains_term(scrutinee, target)
        }
        Term::UnitElim { motive, base_case, scrutinee } => {
            contains_term(motive, target) || contains_term(base_case, target) || contains_term(scrutinee, target)
        }
        Term::BoolElim { motive, true_case, false_case, scrutinee } => {
            contains_term(motive, target) || contains_term(true_case, target)
            || contains_term(false_case, target) || contains_term(scrutinee, target)
        }
        Term::Ann { term: t, ty } => contains_term(t, target) || contains_term(ty, target),
    }
}

/// Replace all occurrences of a subterm with a replacement.
fn replace_term(term: &Term, from: &Term, to: &Term) -> Term {
    if ttt_core::equality::definitionally_equal(term, from) {
        return to.clone();
    }

    match term {
        Term::Var(_) | Term::Universe(_) | Term::Empty | Term::Unit | Term::Star
        | Term::Bool | Term::True | Term::False => term.clone(),

        Term::Pi { domain, codomain } => Term::Pi {
            domain: Box::new(replace_term(domain, from, to)),
            codomain: Box::new(replace_term(codomain, from, to)),
        },
        Term::Lambda { body } => Term::Lambda {
            body: Box::new(replace_term(body, from, to)),
        },
        Term::App { func, arg } => Term::App {
            func: Box::new(replace_term(func, from, to)),
            arg: Box::new(replace_term(arg, from, to)),
        },
        Term::Sigma { fst_type, snd_type } => Term::Sigma {
            fst_type: Box::new(replace_term(fst_type, from, to)),
            snd_type: Box::new(replace_term(snd_type, from, to)),
        },
        Term::Pair { fst, snd } => Term::Pair {
            fst: Box::new(replace_term(fst, from, to)),
            snd: Box::new(replace_term(snd, from, to)),
        },
        Term::Fst(t) => Term::Fst(Box::new(replace_term(t, from, to))),
        Term::Snd(t) => Term::Snd(Box::new(replace_term(t, from, to))),
        Term::Refl(t) => Term::Refl(Box::new(replace_term(t, from, to))),
        Term::Id { ty, lhs, rhs } => Term::Id {
            ty: Box::new(replace_term(ty, from, to)),
            lhs: Box::new(replace_term(lhs, from, to)),
            rhs: Box::new(replace_term(rhs, from, to)),
        },
        Term::J { motive, base_case, target } => Term::J {
            motive: Box::new(replace_term(motive, from, to)),
            base_case: Box::new(replace_term(base_case, from, to)),
            target: Box::new(replace_term(target, from, to)),
        },
        Term::EmptyElim { motive, scrutinee } => Term::EmptyElim {
            motive: Box::new(replace_term(motive, from, to)),
            scrutinee: Box::new(replace_term(scrutinee, from, to)),
        },
        Term::UnitElim { motive, base_case, scrutinee } => Term::UnitElim {
            motive: Box::new(replace_term(motive, from, to)),
            base_case: Box::new(replace_term(base_case, from, to)),
            scrutinee: Box::new(replace_term(scrutinee, from, to)),
        },
        Term::BoolElim { motive, true_case, false_case, scrutinee } => Term::BoolElim {
            motive: Box::new(replace_term(motive, from, to)),
            true_case: Box::new(replace_term(true_case, from, to)),
            false_case: Box::new(replace_term(false_case, from, to)),
            scrutinee: Box::new(replace_term(scrutinee, from, to)),
        },
        Term::Ann { term: t, ty } => Term::Ann {
            term: Box::new(replace_term(t, from, to)),
            ty: Box::new(replace_term(ty, from, to)),
        },
    }
}

/// The `simp` tactic: simplification using rewrite rules.
///
/// Applies a set of simplification rules repeatedly until no more
/// changes can be made.
pub struct Simp {
    /// Maximum number of simplification steps.
    max_steps: usize,
}

impl Simp {
    /// Create a new simp tactic.
    pub fn new() -> Self {
        Self {
            max_steps: 100,
        }
    }

    /// Create simp with custom step limit.
    pub fn with_max_steps(max_steps: usize) -> Self {
        Self {
            max_steps,
        }
    }

    /// Simplify a term using default rules.
    fn simplify(&self, term: &Term) -> Term {
        // Apply built-in simplifications
        let mut current = term.clone();
        let mut steps = 0;

        while steps < self.max_steps {
            let simplified = self.simplify_step(&current);
            if ttt_core::equality::structurally_equal(&simplified, &current) {
                break;
            }
            current = simplified;
            steps += 1;
        }

        current
    }

    /// One step of simplification.
    fn simplify_step(&self, term: &Term) -> Term {
        // First, normalize to WHNF
        let term = term.whnf();

        match &term {
            // π₁(a, b) → a
            Term::Fst(pair) => {
                if let Term::Pair { fst, .. } = pair.as_ref() {
                    return (**fst).clone();
                }
                term
            }

            // π₂(a, b) → b
            Term::Snd(pair) => {
                if let Term::Pair { snd, .. } = pair.as_ref() {
                    return (**snd).clone();
                }
                term
            }

            // Recurse into subterms
            Term::Pi { domain, codomain } => Term::Pi {
                domain: Box::new(self.simplify_step(domain)),
                codomain: Box::new(self.simplify_step(codomain)),
            },

            Term::Lambda { body } => Term::Lambda {
                body: Box::new(self.simplify_step(body)),
            },

            Term::App { func, arg } => {
                let func = self.simplify_step(func);
                let arg = self.simplify_step(arg);
                // Try beta reduction
                if let Term::Lambda { body } = &func {
                    return body.subst(arg);
                }
                Term::App {
                    func: Box::new(func),
                    arg: Box::new(arg),
                }
            }

            Term::Sigma { fst_type, snd_type } => Term::Sigma {
                fst_type: Box::new(self.simplify_step(fst_type)),
                snd_type: Box::new(self.simplify_step(snd_type)),
            },

            Term::Pair { fst, snd } => Term::Pair {
                fst: Box::new(self.simplify_step(fst)),
                snd: Box::new(self.simplify_step(snd)),
            },

            Term::Id { ty, lhs, rhs } => Term::Id {
                ty: Box::new(self.simplify_step(ty)),
                lhs: Box::new(self.simplify_step(lhs)),
                rhs: Box::new(self.simplify_step(rhs)),
            },

            _ => term,
        }
    }
}

impl Default for Simp {
    fn default() -> Self {
        Self::new()
    }
}

impl Tactic for Simp {
    fn apply(&self, _kernel: &TrustKernel, goal: &Goal) -> TacticResult {
        let simplified_goal = self.simplify(&goal.ty);

        // If goal simplified to something trivial, solve it
        match simplified_goal.whnf() {
            Term::Unit => {
                // Goal simplified to Unit, prove with Star
                TacticResult::complete(Term::Star)
            }
            Term::Id { lhs, rhs, .. } if ttt_core::equality::definitionally_equal(&lhs, &rhs) => {
                // Reflexivity case
                TacticResult::complete(Term::Refl(lhs))
            }
            _ => {
                // Check if simplification changed the goal
                if ttt_core::equality::definitionally_equal(&goal.ty, &simplified_goal) {
                    // No change, simp didn't help
                    TacticResult::fail(TacticError::NotApplicable(
                        "simp: no simplifications applicable".to_string(),
                    ))
                } else {
                    // Goal changed, create new subgoal
                    let subgoal = Goal::new(goal.ctx.clone(), simplified_goal);

                    // The proof is just the identity since they're definitionally equal
                    let builder = ProofBuilder::new(1, |subproofs| {
                        Some(subproofs[0].clone())
                    });

                    TacticResult::with_subgoals(builder, vec![subgoal])
                }
            }
        }
    }

    fn name(&self) -> &str {
        "simp"
    }
}

/// The `unfold` tactic: unfold a definition.
pub struct Unfold {
    /// Index of the definition to unfold.
    pub def_idx: u32,
}

impl Unfold {
    /// Create a new unfold tactic.
    pub fn new(def_idx: u32) -> Self {
        Self { def_idx }
    }
}

impl Tactic for Unfold {
    fn apply(&self, _kernel: &TrustKernel, goal: &Goal) -> TacticResult {
        // Look up definition in context
        let entry = goal.ctx.lookup_entry(self.def_idx)
            .ok_or_else(|| TacticError::HypothesisNotFound(
                format!("No definition at index {}", self.def_idx)
            ));

        let entry = match entry {
            Ok(e) => e,
            Err(e) => return TacticResult::fail(e),
        };

        // Check if it has a definition
        let def = match &entry.def {
            Some(d) => d.clone(),
            None => return TacticResult::fail(TacticError::NotApplicable(
                format!("Entry at index {} is not a definition", self.def_idx)
            )),
        };

        // Replace occurrences of the variable with the definition
        let var = Term::var(self.def_idx);
        let new_goal_ty = replace_term(&goal.ty, &var, &def);

        if ttt_core::equality::definitionally_equal(&goal.ty, &new_goal_ty) {
            TacticResult::fail(TacticError::NotApplicable(
                "unfold: definition not found in goal".to_string()
            ))
        } else {
            let subgoal = Goal::new(goal.ctx.clone(), new_goal_ty);
            let builder = ProofBuilder::new(1, |subproofs| {
                Some(subproofs[0].clone())
            });
            TacticResult::with_subgoals(builder, vec![subgoal])
        }
    }

    fn name(&self) -> &str {
        "unfold"
    }
}

/// Convenient function to create rewrite tactic.
pub fn rewrite(eq_proof: Term) -> Rewrite {
    Rewrite::new(eq_proof)
}

/// Convenient function to create reverse rewrite tactic.
pub fn rewrite_rev(eq_proof: Term) -> Rewrite {
    Rewrite::reverse(eq_proof)
}

/// Convenient function to create simp tactic.
pub fn simp() -> Simp {
    Simp::new()
}

/// Convenient function to create unfold tactic.
pub fn unfold(def_idx: u32) -> Unfold {
    Unfold::new(def_idx)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_kernel() -> TrustKernel {
        TrustKernel::new()
    }

    #[test]
    fn test_contains_term() {
        let term = Term::App {
            func: Box::new(Term::var(0)),
            arg: Box::new(Term::Star),
        };

        assert!(contains_term(&term, &Term::Star));
        assert!(contains_term(&term, &Term::var(0)));
        assert!(!contains_term(&term, &Term::var(1)));
    }

    #[test]
    fn test_replace_term() {
        let term = Term::Pair {
            fst: Box::new(Term::Star),
            snd: Box::new(Term::Star),
        };

        let replaced = replace_term(&term, &Term::Star, &Term::True);

        match replaced {
            Term::Pair { fst, snd } => {
                assert!(matches!(*fst, Term::True));
                assert!(matches!(*snd, Term::True));
            }
            _ => panic!("Expected Pair"),
        }
    }

    #[test]
    fn test_simp_fst_pair() {
        let simp = Simp::new();

        // π₁(*, true) should simplify to *
        let term = Term::Fst(Box::new(Term::Pair {
            fst: Box::new(Term::Star),
            snd: Box::new(Term::True),
        }));

        let simplified = simp.simplify(&term);
        assert!(matches!(simplified, Term::Star));
    }

    #[test]
    fn test_simp_snd_pair() {
        let simp = Simp::new();

        // π₂(*, true) should simplify to true
        let term = Term::Snd(Box::new(Term::Pair {
            fst: Box::new(Term::Star),
            snd: Box::new(Term::True),
        }));

        let simplified = simp.simplify(&term);
        assert!(matches!(simplified, Term::True));
    }

    #[test]
    fn test_simp_beta() {
        let simp = Simp::new();

        // (λ. #0) * should simplify to *
        let term = Term::App {
            func: Box::new(Term::Lambda {
                body: Box::new(Term::var(0)),
            }),
            arg: Box::new(Term::Star),
        };

        let simplified = simp.simplify(&term);
        assert!(matches!(simplified, Term::Star));
    }

    #[test]
    fn test_simp_tactic_unit() {
        let kernel = make_kernel();

        // If goal is already Unit, simp should solve it
        let goal = Goal::new(Context::empty(), Term::Unit);
        let result = Simp::new().apply(&kernel, &goal);
        assert!(result.is_success());
    }
}
