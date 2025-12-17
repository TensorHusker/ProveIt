//! The Trust Kernel: minimal type checker for TTT.
//!
//! This module contains the ONLY code that can produce verified judgments.
//! Everything else in ProveIt is convenience; this is correctness.
//!
//! # Design Principles
//!
//! 1. **Minimality**: Include only what's necessary for soundness
//! 2. **Clarity**: Prefer obvious code over clever code
//! 3. **Auditability**: A careful reader should verify correctness
//! 4. **No Dependencies**: Only depend on `ttt-core` types
//!
//! # Typing Rules
//!
//! See `docs/theory/ttt-specification.md` for the formal rules.
//! Each function here corresponds to one or more inference rules.

use ttt_core::equality::definitionally_equal;
use ttt_core::term::{DeBruijnIndex, UniverseLevel};
use ttt_core::{Context, Term};

use crate::error::TypeError;
use crate::judgment::{Judgment, VerifiedJudgment};

/// The Trust Kernel.
///
/// A stateless type checker. All checking operations are pure functions
/// that produce verified judgments on success.
pub struct TrustKernel {
    // Currently stateless. May add:
    // - fuel limits for normalization
    // - metrics/logging (in debug builds only)
    _private: (),
}

impl TrustKernel {
    /// Create a new kernel instance.
    pub fn new() -> Self {
        TrustKernel { _private: () }
    }

    /// Check a judgment and produce a verified judgment on success.
    ///
    /// This is the primary entry point for type checking.
    pub fn check(&self, judgment: Judgment) -> Result<VerifiedJudgment, TypeError> {
        match &judgment {
            Judgment::TypeFormation { ctx, ty, level } => {
                self.check_type_formation(ctx, ty, level)?;
            }
            Judgment::TypeInhabitation { ctx, term, ty } => {
                self.check_inhabitation(ctx, term, ty)?;
            }
            Judgment::DefEquality { ctx, lhs, rhs, ty } => {
                self.check_equality(ctx, lhs, rhs, ty)?;
            }
        }

        Ok(VerifiedJudgment::new(judgment))
    }

    /// Infer the type of a term.
    ///
    /// Returns the inferred type, or an error if the term is ill-typed.
    pub fn infer(&self, ctx: &Context, term: &Term) -> Result<Term, TypeError> {
        self.infer_type(ctx, term)
    }

    // =========================================================================
    // TYPE FORMATION CHECKING
    // =========================================================================

    /// Check that `ty` is a well-formed type at level `level`.
    fn check_type_formation(
        &self,
        ctx: &Context,
        ty: &Term,
        level: &UniverseLevel,
    ) -> Result<(), TypeError> {
        let inferred_level = self.infer_universe_level(ctx, ty)?;

        // Check that inferred level <= declared level
        if !universe_leq(&inferred_level, level) {
            return Err(TypeError::UniverseLevelMismatch {
                expected: level.clone(),
                found: inferred_level,
            });
        }

        Ok(())
    }

    /// Infer the universe level of a type.
    fn infer_universe_level(&self, ctx: &Context, ty: &Term) -> Result<UniverseLevel, TypeError> {
        let ty_of_ty = self.infer_type(ctx, ty)?;
        let ty_of_ty_nf = ty_of_ty.normalize();

        match ty_of_ty_nf {
            Term::Universe(level) => Ok(level),
            _ => Err(TypeError::NotAType {
                term: ty.clone(),
                inferred: ty_of_ty,
            }),
        }
    }

    // =========================================================================
    // TYPE INHABITATION CHECKING
    // =========================================================================

    /// Check that `term` has type `ty`.
    fn check_inhabitation(
        &self,
        ctx: &Context,
        term: &Term,
        ty: &Term,
    ) -> Result<(), TypeError> {
        // First, verify that `ty` is actually a type
        let _ = self.infer_universe_level(ctx, ty)?;

        // Use bidirectional type checking
        self.check_against(ctx, term, ty)
    }

    /// Infer the type of a term.
    fn infer_type(&self, ctx: &Context, term: &Term) -> Result<Term, TypeError> {
        match term {
            // -----------------------------------------------------------------
            // VARIABLES
            // -----------------------------------------------------------------

            // Gamma(x) = A
            // ----------- Var
            // Gamma |- x : A
            Term::Var(DeBruijnIndex(idx)) => ctx
                .lookup(*idx)
                .cloned()
                .ok_or(TypeError::UnboundVariable { index: *idx }),

            // -----------------------------------------------------------------
            // UNIVERSES
            // -----------------------------------------------------------------

            // -------------------- Universe
            // Gamma |- U_i : U_{i+1}
            Term::Universe(level) => Ok(Term::Universe(level.clone().succ())),

            // -----------------------------------------------------------------
            // PI-TYPES
            // -----------------------------------------------------------------

            // Gamma |- A : U_i    Gamma, x:A |- B : U_j
            // ----------------------------------------- Pi-Form
            // Gamma |- Pi(x:A).B : U_max(i,j)
            Term::Pi { domain, codomain } => {
                let dom_level = self.infer_universe_level(ctx, domain)?;
                let ext_ctx = ctx.extend(domain.as_ref().clone());
                let cod_level = self.infer_universe_level(&ext_ctx, codomain)?;

                Ok(Term::Universe(dom_level.max(cod_level)))
            }

            // Gamma, x:A |- M : B
            // ------------------------- Lambda (with annotation)
            // Gamma |- lambda.M : Pi(x:A).B
            //
            // Note: Bare lambdas cannot be typed; they need annotation.
            Term::Lambda { .. } => Err(TypeError::CannotInferLambda { term: term.clone() }),

            // Gamma |- f : Pi(x:A).B    Gamma |- a : A
            // ----------------------------------------- App
            // Gamma |- f a : B[a/x]
            Term::App { func, arg } => {
                let func_ty = self.infer_type(ctx, func)?;
                let func_ty_nf = func_ty.normalize();

                match func_ty_nf {
                    Term::Pi { domain, codomain } => {
                        // Check argument has the domain type
                        self.check_inhabitation(ctx, arg, &domain)?;

                        // Result type: substitute argument into codomain
                        Ok(codomain.subst(arg.as_ref().clone()))
                    }
                    _ => Err(TypeError::NotAFunction {
                        term: func.as_ref().clone(),
                        inferred: func_ty,
                    }),
                }
            }

            // -----------------------------------------------------------------
            // SIGMA-TYPES
            // -----------------------------------------------------------------

            // Gamma |- A : U_i    Gamma, x:A |- B : U_j
            // ----------------------------------------- Sigma-Form
            // Gamma |- Sigma(x:A).B : U_max(i,j)
            Term::Sigma { fst_type, snd_type } => {
                let fst_level = self.infer_universe_level(ctx, fst_type)?;
                let ext_ctx = ctx.extend(fst_type.as_ref().clone());
                let snd_level = self.infer_universe_level(&ext_ctx, snd_type)?;

                Ok(Term::Universe(fst_level.max(snd_level)))
            }

            // Gamma |- a : A    Gamma |- b : B[a/x]
            // ------------------------------------- Pair (requires annotation)
            // Gamma |- (a, b) : Sigma(x:A).B
            Term::Pair { .. } => Err(TypeError::CannotInferPair { term: term.clone() }),

            // Gamma |- p : Sigma(x:A).B
            // -------------------------- Fst
            // Gamma |- fst(p) : A
            Term::Fst(pair) => {
                let pair_ty = self.infer_type(ctx, pair)?;
                let pair_ty_nf = pair_ty.normalize();

                match pair_ty_nf {
                    Term::Sigma { fst_type, .. } => Ok(fst_type.as_ref().clone()),
                    _ => Err(TypeError::NotAPair {
                        term: pair.as_ref().clone(),
                        inferred: pair_ty,
                    }),
                }
            }

            // Gamma |- p : Sigma(x:A).B
            // ---------------------------- Snd
            // Gamma |- snd(p) : B[fst(p)/x]
            Term::Snd(pair) => {
                let pair_ty = self.infer_type(ctx, pair)?;
                let pair_ty_nf = pair_ty.normalize();

                match pair_ty_nf {
                    Term::Sigma { snd_type, .. } => {
                        // Substitute fst(p) for the bound variable
                        let fst = Term::Fst(pair.clone());
                        Ok(snd_type.subst(fst))
                    }
                    _ => Err(TypeError::NotAPair {
                        term: pair.as_ref().clone(),
                        inferred: pair_ty,
                    }),
                }
            }

            // -----------------------------------------------------------------
            // IDENTITY TYPES
            // -----------------------------------------------------------------

            // Gamma |- A : U_i    Gamma |- a : A    Gamma |- b : A
            // ---------------------------------------------------- Id-Form
            // Gamma |- Id_A(a,b) : U_i
            Term::Id { ty, lhs, rhs } => {
                let level = self.infer_universe_level(ctx, ty)?;
                self.check_inhabitation(ctx, lhs, ty)?;
                self.check_inhabitation(ctx, rhs, ty)?;

                Ok(Term::Universe(level))
            }

            // Gamma |- a : A
            // ----------------------- Refl
            // Gamma |- refl(a) : Id_A(a,a)
            Term::Refl(a) => {
                let ty = self.infer_type(ctx, a)?;
                Ok(Term::Id {
                    ty: Box::new(ty),
                    lhs: a.clone(),
                    rhs: a.clone(),
                })
            }

            // J eliminator - complex rule, see specification
            Term::J {
                motive,
                base_case,
                target,
            } => self.infer_j_type(ctx, motive, base_case, target),

            // -----------------------------------------------------------------
            // FINITE TYPES
            // -----------------------------------------------------------------

            // ------------- Empty-Form
            // Gamma |- Empty : U_0
            Term::Empty => Ok(Term::universe(0)),

            // Gamma |- C : U_i    Gamma |- e : Empty
            // --------------------------------------- Empty-Elim
            // Gamma |- absurd(C, e) : C
            Term::EmptyElim { motive, scrutinee } => {
                let _ = self.infer_universe_level(ctx, motive)?;
                self.check_inhabitation(ctx, scrutinee, &Term::Empty)?;
                Ok(motive.as_ref().clone())
            }

            // ------------- Unit-Form
            // Gamma |- Unit : U_0
            Term::Unit => Ok(Term::universe(0)),

            // ------------- Star
            // Gamma |- * : Unit
            Term::Star => Ok(Term::Unit),

            // Unit elimination
            Term::UnitElim {
                motive,
                base_case,
                scrutinee,
            } => self.infer_unit_elim_type(ctx, motive, base_case, scrutinee),

            // ------------- Bool-Form
            // Gamma |- Bool : U_0
            Term::Bool => Ok(Term::universe(0)),

            // ------------- True
            // Gamma |- true : Bool
            Term::True => Ok(Term::Bool),

            // ------------- False
            // Gamma |- false : Bool
            Term::False => Ok(Term::Bool),

            // Bool elimination
            Term::BoolElim {
                motive,
                true_case,
                false_case,
                scrutinee,
            } => self.infer_bool_elim_type(ctx, motive, true_case, false_case, scrutinee),

            // -----------------------------------------------------------------
            // ANNOTATIONS
            // -----------------------------------------------------------------

            // Gamma |- t : A (checked)
            // ------------------------ Ann
            // Gamma |- (t : A) : A
            Term::Ann { term: inner, ty } => {
                // First verify ty is a type
                let _ = self.infer_universe_level(ctx, ty)?;

                // Then check the inner term against the annotation
                self.check_against(ctx, inner, ty)?;

                Ok(ty.as_ref().clone())
            }
        }
    }

    /// Check a term against a given type (checking mode).
    fn check_against(&self, ctx: &Context, term: &Term, ty: &Term) -> Result<(), TypeError> {
        // Normalize the expected type
        let ty_nf = ty.normalize();

        match (term, &ty_nf) {
            // Check lambda against Pi type
            (Term::Lambda { body }, Term::Pi { domain, codomain }) => {
                let ext_ctx = ctx.extend(domain.as_ref().clone());
                self.check_against(&ext_ctx, body, codomain)?;
                Ok(())
            }

            // Check pair against Sigma type
            (Term::Pair { fst, snd }, Term::Sigma { fst_type, snd_type }) => {
                self.check_against(ctx, fst, fst_type)?;
                let snd_ty_subst = snd_type.subst(fst.as_ref().clone());
                self.check_against(ctx, snd, &snd_ty_subst)?;
                Ok(())
            }

            // Fall back to inference and comparison
            _ => {
                let inferred = self.infer_type(ctx, term)?;
                if definitionally_equal(&inferred, ty) {
                    Ok(())
                } else {
                    Err(TypeError::TypeMismatch {
                        expected: ty.clone(),
                        found: inferred,
                        term: term.clone(),
                    })
                }
            }
        }
    }

    // =========================================================================
    // DEFINITIONAL EQUALITY
    // =========================================================================

    /// Check that two terms are definitionally equal.
    fn check_equality(
        &self,
        ctx: &Context,
        lhs: &Term,
        rhs: &Term,
        ty: &Term,
    ) -> Result<(), TypeError> {
        // Verify both terms have the given type
        self.check_inhabitation(ctx, lhs, ty)?;
        self.check_inhabitation(ctx, rhs, ty)?;

        // Check definitional equality
        if !definitionally_equal(lhs, rhs) {
            return Err(TypeError::NotEqual {
                lhs: lhs.clone(),
                rhs: rhs.clone(),
                ty: ty.clone(),
            });
        }

        Ok(())
    }

    // =========================================================================
    // HELPER METHODS FOR COMPLEX ELIMINATORS
    // =========================================================================

    /// Infer the type of a J eliminator.
    fn infer_j_type(
        &self,
        ctx: &Context,
        motive: &Term,
        base_case: &Term,
        target: &Term,
    ) -> Result<Term, TypeError> {
        // The J elimination rule:
        //
        // Gamma |- p : Id_A(a, b)
        // Gamma, x:A, y:A, q:Id_A(x,y) |- C : U_i
        // Gamma, z:A |- c : C[z/x, z/y, refl(z)/q]
        // ----------------------------------------
        // Gamma |- J(C, c, p) : C[a/x, b/y, p/q]

        // 1. Infer type of target, extract A, a, b
        let target_ty = self.infer_type(ctx, target)?;
        let target_ty_nf = target_ty.normalize();

        let (ty_a, term_a, term_b) = match target_ty_nf {
            Term::Id { ty, lhs, rhs } => (*ty, *lhs, *rhs),
            _ => {
                return Err(TypeError::JTargetNotId {
                    target: target.clone(),
                    inferred: target_ty,
                })
            }
        };

        // 2. Check motive in extended context
        // Context: Gamma, x:A, y:A, q:Id_A(x,y)
        let ctx_x = ctx.extend(ty_a.clone());
        let ctx_xy = ctx_x.extend(ty_a.clone().shift(1, 0));
        let id_xy = Term::Id {
            ty: Box::new(ty_a.clone().shift(2, 0)),
            lhs: Box::new(Term::var(1)), // x
            rhs: Box::new(Term::var(0)), // y
        };
        let ctx_xyq = ctx_xy.extend(id_xy);

        let _motive_level = self.infer_universe_level(&ctx_xyq, motive)?;

        // 3. Check base case
        // Type: C[z/x, z/y, refl(z)/q]
        let ctx_z = ctx.extend(ty_a.clone());
        let base_expected = motive
            .subst(Term::refl(Term::var(0))) // refl(z) for q
            .subst(Term::var(0)) // z for y
            .subst(Term::var(0)); // z for x

        self.check_against(&ctx_z, base_case, &base_expected)?;

        // 4. Result type: C[a/x, b/y, p/q]
        let result = motive
            .subst(target.clone()) // p for q
            .subst(term_b.shift(1, 0)) // b for y
            .subst(term_a); // a for x

        Ok(result)
    }

    /// Infer the type of a Unit eliminator.
    fn infer_unit_elim_type(
        &self,
        ctx: &Context,
        motive: &Term,
        base_case: &Term,
        scrutinee: &Term,
    ) -> Result<Term, TypeError> {
        // Gamma, x:Unit |- C : U_i
        let ctx_x = ctx.extend(Term::Unit);
        let _ = self.infer_universe_level(&ctx_x, motive)?;

        // Gamma |- c : C[*/x]
        let base_expected = motive.subst(Term::Star);
        self.check_against(ctx, base_case, &base_expected)?;

        // Gamma |- s : Unit
        self.check_inhabitation(ctx, scrutinee, &Term::Unit)?;

        // Result: C[s/x]
        Ok(motive.subst(scrutinee.clone()))
    }

    /// Infer the type of a Bool eliminator.
    fn infer_bool_elim_type(
        &self,
        ctx: &Context,
        motive: &Term,
        true_case: &Term,
        false_case: &Term,
        scrutinee: &Term,
    ) -> Result<Term, TypeError> {
        // Gamma, x:Bool |- C : U_i
        let ctx_x = ctx.extend(Term::Bool);
        let _ = self.infer_universe_level(&ctx_x, motive)?;

        // Gamma |- t : C[true/x]
        let true_expected = motive.subst(Term::True);
        self.check_against(ctx, true_case, &true_expected)?;

        // Gamma |- f : C[false/x]
        let false_expected = motive.subst(Term::False);
        self.check_against(ctx, false_case, &false_expected)?;

        // Gamma |- b : Bool
        self.check_inhabitation(ctx, scrutinee, &Term::Bool)?;

        // Result: C[b/x]
        Ok(motive.subst(scrutinee.clone()))
    }
}

impl Default for TrustKernel {
    fn default() -> Self {
        Self::new()
    }
}

/// Check if level `a` is less than or equal to level `b`.
fn universe_leq(a: &UniverseLevel, b: &UniverseLevel) -> bool {
    match (a.to_nat(), b.to_nat()) {
        (Some(na), Some(nb)) => na <= nb,
        _ => {
            // WIP: Handle universe variables
            // For now, be conservative
            a == b
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn kernel() -> TrustKernel {
        TrustKernel::new()
    }

    #[test]
    fn test_infer_star() {
        let k = kernel();
        let ctx = Context::empty();
        let ty = k.infer(&ctx, &Term::Star).unwrap();
        assert_eq!(ty, Term::Unit);
    }

    #[test]
    fn test_infer_true() {
        let k = kernel();
        let ctx = Context::empty();
        let ty = k.infer(&ctx, &Term::True).unwrap();
        assert_eq!(ty, Term::Bool);
    }

    #[test]
    fn test_infer_false() {
        let k = kernel();
        let ctx = Context::empty();
        let ty = k.infer(&ctx, &Term::False).unwrap();
        assert_eq!(ty, Term::Bool);
    }

    #[test]
    fn test_infer_universe() {
        let k = kernel();
        let ctx = Context::empty();

        // Bool : U_0
        let ty = k.infer(&ctx, &Term::Bool).unwrap();
        assert_eq!(ty, Term::universe(0));

        // U_0 : U_1
        let ty = k.infer(&ctx, &Term::universe(0)).unwrap();
        assert_eq!(ty, Term::universe(1));
    }

    #[test]
    fn test_infer_variable() {
        let k = kernel();
        let ctx = Context::empty().extend(Term::Bool);
        let ty = k.infer(&ctx, &Term::var(0)).unwrap();
        assert_eq!(ty, Term::Bool);
    }

    #[test]
    fn test_infer_variable_unbound() {
        let k = kernel();
        let ctx = Context::empty();
        let err = k.infer(&ctx, &Term::var(0)).unwrap_err();
        assert!(matches!(err, TypeError::UnboundVariable { index: 0 }));
    }

    #[test]
    fn test_infer_pi_type() {
        let k = kernel();
        let ctx = Context::empty();

        // Pi(Bool).Unit : U_0
        let pi = Term::Pi {
            domain: Box::new(Term::Bool),
            codomain: Box::new(Term::Unit),
        };
        let ty = k.infer(&ctx, &pi).unwrap();
        assert_eq!(ty, Term::universe(0));
    }

    #[test]
    fn test_check_lambda_against_pi() {
        let k = kernel();
        let ctx = Context::empty();

        // lambda.#0 : Pi(Bool).Bool
        let lam = Term::lam(Term::var(0));
        let pi = Term::Pi {
            domain: Box::new(Term::Bool),
            codomain: Box::new(Term::Bool),
        };

        let judgment = Judgment::inhabitation(ctx, lam, pi);
        assert!(k.check(judgment).is_ok());
    }

    #[test]
    fn test_infer_application() {
        let k = kernel();
        let ctx = Context::empty();

        // (lambda.#0 : Bool -> Bool) true : Bool
        let lam = Term::ann(
            Term::lam(Term::var(0)),
            Term::Pi {
                domain: Box::new(Term::Bool),
                codomain: Box::new(Term::Bool),
            },
        );
        let app = Term::app(lam, Term::True);
        let ty = k.infer(&ctx, &app).unwrap();
        assert_eq!(ty, Term::Bool);
    }

    #[test]
    fn test_check_pair_against_sigma() {
        let k = kernel();
        let ctx = Context::empty();

        // <true, *> : Sigma(Bool).Unit
        let pair = Term::pair(Term::True, Term::Star);
        let sigma = Term::Sigma {
            fst_type: Box::new(Term::Bool),
            snd_type: Box::new(Term::Unit),
        };

        let judgment = Judgment::inhabitation(ctx, pair, sigma);
        assert!(k.check(judgment).is_ok());
    }

    #[test]
    fn test_infer_fst() {
        let k = kernel();
        let ctx = Context::empty();

        // fst(<true, *> : Sigma(Bool).Unit) : Bool
        let pair = Term::ann(
            Term::pair(Term::True, Term::Star),
            Term::Sigma {
                fst_type: Box::new(Term::Bool),
                snd_type: Box::new(Term::Unit),
            },
        );
        let fst = Term::Fst(Box::new(pair));
        let ty = k.infer(&ctx, &fst).unwrap();
        assert_eq!(ty, Term::Bool);
    }

    #[test]
    fn test_infer_refl() {
        let k = kernel();
        let ctx = Context::empty();

        // refl(true) : Id[Bool](true, true)
        let refl = Term::refl(Term::True);
        let ty = k.infer(&ctx, &refl).unwrap();
        let expected = Term::id(Term::Bool, Term::True, Term::True);
        assert!(definitionally_equal(&ty, &expected));
    }

    #[test]
    fn test_check_equality() {
        let k = kernel();
        let ctx = Context::empty();

        // true = true : Bool
        let judgment = Judgment::equality(ctx.clone(), Term::True, Term::True, Term::Bool);
        assert!(k.check(judgment).is_ok());

        // true != false : Bool
        let judgment = Judgment::equality(ctx, Term::True, Term::False, Term::Bool);
        assert!(k.check(judgment).is_err());
    }

    #[test]
    fn test_type_formation() {
        let k = kernel();
        let ctx = Context::empty();

        // Bool : U_0
        let judgment = Judgment::type_formation(ctx.clone(), Term::Bool, UniverseLevel::zero());
        assert!(k.check(judgment).is_ok());

        // Bool : U_1 (also valid, level can be higher)
        let judgment = Judgment::type_formation(ctx.clone(), Term::Bool, UniverseLevel::from_nat(1));
        assert!(k.check(judgment).is_ok());

        // Pi(U_0).U_0 requires at least U_1
        let pi = Term::Pi {
            domain: Box::new(Term::universe(0)),
            codomain: Box::new(Term::universe(0)),
        };
        // This should be at level 1 (max(1, 1) = 1 since U_0 : U_1)
        let judgment = Judgment::type_formation(ctx, pi, UniverseLevel::from_nat(1));
        assert!(k.check(judgment).is_ok());
    }

    #[test]
    fn test_bool_elim() {
        let k = kernel();
        let ctx = Context::empty();

        // if true then * else * : Unit (with constant motive)
        let motive = Term::Unit; // Constant motive: lambda._. Unit simplified to just Unit
        let if_term = Term::BoolElim {
            motive: Box::new(motive),
            true_case: Box::new(Term::Star),
            false_case: Box::new(Term::Star),
            scrutinee: Box::new(Term::True),
        };

        let ty = k.infer(&ctx, &if_term).unwrap();
        assert!(definitionally_equal(&ty, &Term::Unit));
    }
}
