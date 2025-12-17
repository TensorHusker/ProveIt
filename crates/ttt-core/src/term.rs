//! Core term representation for Tiny Type Theory.
//!
//! This module defines the abstract syntax of TTT terms using de Bruijn
//! indices for bound variables. All binding forms (Pi, Sigma, Lambda) bind exactly
//! one variable, which is referenced by index 0 in the body.
//!
//! # Representation Invariants
//!
//! - All `DeBruijnIndex` values must be less than the number of enclosing binders
//! - `UniverseLevel` must be finite (no infinite descending chains)
//! - Terms should be in "locally nameless" style: bound vars use indices,
//!   free vars use indices into an external context

use std::fmt;

/// A de Bruijn index representing a bound variable.
///
/// Index 0 refers to the innermost binder, 1 to the next enclosing, etc.
///
/// # Example
///
/// In `lambda. lambda. Var(0)`, the `Var(0)` refers to the inner lambda.
/// In `lambda. lambda. Var(1)`, the `Var(1)` refers to the outer lambda.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DeBruijnIndex(pub u32);

impl DeBruijnIndex {
    /// Create a new de Bruijn index.
    pub fn new(idx: u32) -> Self {
        DeBruijnIndex(idx)
    }

    /// Increment the index (shift by 1).
    pub fn succ(self) -> Self {
        DeBruijnIndex(self.0 + 1)
    }

    /// Decrement the index, returning None if already 0.
    pub fn pred(self) -> Option<Self> {
        self.0.checked_sub(1).map(DeBruijnIndex)
    }
}

/// Universe levels for the type hierarchy.
///
/// We use an explicit inductive representation rather than raw integers
/// to support universe polymorphism in future extensions.
///
/// # Universe Rules
///
/// - `U_i : U_{i+1}` (universes form a hierarchy)
/// - `Pi(x:A).B : U_max(i,j)` where `A : U_i` and `B : U_j`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum UniverseLevel {
    /// The base universe level (U_0).
    Zero,
    /// Successor of a level.
    Succ(Box<UniverseLevel>),
    /// Maximum of two levels (for Pi-types spanning universes).
    Max(Box<UniverseLevel>, Box<UniverseLevel>),
    /// Universe level variable (for future universe polymorphism).
    /// WIP: Not implemented in Phase 1.
    Var(UniverseLevelVar),
}

/// A universe level variable identifier.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct UniverseLevelVar(pub u32);

impl UniverseLevel {
    /// Create level 0.
    pub fn zero() -> Self {
        UniverseLevel::Zero
    }

    /// Create level n.
    pub fn from_nat(n: u32) -> Self {
        let mut level = UniverseLevel::Zero;
        for _ in 0..n {
            level = UniverseLevel::Succ(Box::new(level));
        }
        level
    }

    /// Compute the successor level.
    pub fn succ(self) -> Self {
        UniverseLevel::Succ(Box::new(self))
    }

    /// Compute the maximum of two levels.
    pub fn max(self, other: Self) -> Self {
        // Simplification: if both are concrete, compute directly
        match (&self, &other) {
            (UniverseLevel::Zero, _) => other,
            (_, UniverseLevel::Zero) => self,
            _ => UniverseLevel::Max(Box::new(self), Box::new(other)),
        }
    }

    /// Convert to a concrete natural number, if possible.
    /// Returns None if the level contains variables or unsimplified Max.
    pub fn to_nat(&self) -> Option<u32> {
        match self {
            UniverseLevel::Zero => Some(0),
            UniverseLevel::Succ(l) => l.to_nat().map(|n| n + 1),
            UniverseLevel::Max(l1, l2) => {
                let n1 = l1.to_nat()?;
                let n2 = l2.to_nat()?;
                Some(n1.max(n2))
            }
            UniverseLevel::Var(_) => None,
        }
    }
}

/// The core term type for Tiny Type Theory.
///
/// This is a fully explicit representation-no implicit arguments,
/// no inference markers, no holes. Every term is complete.
///
/// # Design Decisions
///
/// - **De Bruijn indices**: Avoid alpha-equivalence issues entirely
/// - **Explicit boxes**: Clear ownership, easy cloning
/// - **Flat enum**: Pattern matching is the primary decomposition method
/// - **No metadata**: Source locations, names are stored separately
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Term {
    // =========================================================================
    // VARIABLES
    // =========================================================================
    /// A bound variable, referenced by de Bruijn index.
    Var(DeBruijnIndex),

    // =========================================================================
    // UNIVERSES
    // =========================================================================
    /// A universe type U_i.
    Universe(UniverseLevel),

    // =========================================================================
    // PI-TYPES (Dependent Functions)
    // =========================================================================
    /// Dependent function type: Pi(x : A). B
    ///
    /// The `codomain` binds one variable (the function argument).
    Pi {
        domain: Box<Term>,
        codomain: Box<Term>, // binds variable 0
    },

    /// Lambda abstraction: lambda. body
    ///
    /// Note: No explicit domain annotation. The type is inferred or
    /// provided via `Ann`.
    Lambda { body: Box<Term> }, // binds variable 0

    /// Function application: f a
    App { func: Box<Term>, arg: Box<Term> },

    // =========================================================================
    // SIGMA-TYPES (Dependent Pairs)
    // =========================================================================
    /// Dependent pair type: Sigma(x : A). B
    ///
    /// The `snd_type` binds one variable (the first component).
    Sigma {
        fst_type: Box<Term>,
        snd_type: Box<Term>, // binds variable 0
    },

    /// Pair constructor: (a, b)
    Pair { fst: Box<Term>, snd: Box<Term> },

    /// First projection: pi_1(p)
    Fst(Box<Term>),

    /// Second projection: pi_2(p)
    Snd(Box<Term>),

    // =========================================================================
    // IDENTITY TYPES (Propositional Equality)
    // =========================================================================
    /// Identity type: Id_A(a, b)
    ///
    /// Represents propositional equality between `lhs` and `rhs` at type `ty`.
    Id {
        ty: Box<Term>,
        lhs: Box<Term>,
        rhs: Box<Term>,
    },

    /// Reflexivity proof: refl(a) : Id_A(a, a)
    Refl(Box<Term>),

    /// Path induction (J eliminator).
    ///
    /// Given:
    /// - `motive`: The type family we're proving (binds 3 variables: x, y, p)
    /// - `base_case`: Proof for the reflexivity case (binds 1 variable: z)
    /// - `target`: The equality proof we're eliminating
    ///
    /// J produces a proof of `motive[lhs, rhs, target]`.
    J {
        motive: Box<Term>,    // binds variables: 0=path, 1=rhs, 2=lhs
        base_case: Box<Term>, // binds variable 0 (the point)
        target: Box<Term>,    // the equality proof
    },

    // =========================================================================
    // FINITE TYPES
    // =========================================================================
    /// The empty type (a.k.a. Void, False).
    Empty,

    /// Elimination for Empty. Ex falso quodlibet.
    ///
    /// `EmptyElim { motive, scrutinee }` proves `motive` given a proof
    /// of `Empty`.
    EmptyElim {
        motive: Box<Term>,
        scrutinee: Box<Term>,
    },

    /// The unit type (a.k.a. (), True).
    Unit,

    /// The sole inhabitant of Unit.
    Star,

    /// Elimination for Unit.
    ///
    /// Since Unit has only one inhabitant, we can substitute any `t : Unit`
    /// with `Star` in a type-respecting way.
    UnitElim {
        motive: Box<Term>,    // binds variable 0
        base_case: Box<Term>, // proof for Star
        scrutinee: Box<Term>, // the unit value
    },

    /// The boolean type (a.k.a. Bool).
    Bool,

    /// Boolean true.
    True,

    /// Boolean false.
    False,

    /// Boolean elimination (if-then-else).
    BoolElim {
        motive: Box<Term>,     // binds variable 0
        true_case: Box<Term>,  // proof/value for true branch
        false_case: Box<Term>, // proof/value for false branch
        scrutinee: Box<Term>,  // the boolean
    },

    // =========================================================================
    // ANNOTATIONS
    // =========================================================================
    /// Type annotation: (t : T)
    ///
    /// Erased during normalization but used during type checking.
    Ann { term: Box<Term>, ty: Box<Term> },
}

impl Term {
    // =========================================================================
    // SMART CONSTRUCTORS
    // =========================================================================

    /// Create a variable term.
    pub fn var(idx: u32) -> Self {
        Term::Var(DeBruijnIndex::new(idx))
    }

    /// Create a universe term.
    pub fn universe(level: u32) -> Self {
        Term::Universe(UniverseLevel::from_nat(level))
    }

    /// Create a non-dependent function type: A -> B
    pub fn arrow(domain: Term, codomain: Term) -> Self {
        // Non-dependent: codomain doesn't use variable 0
        Term::Pi {
            domain: Box::new(domain),
            codomain: Box::new(codomain.shift(1, 0)), // shift to avoid capture
        }
    }

    /// Create a lambda with explicit body.
    pub fn lam(body: Term) -> Self {
        Term::Lambda {
            body: Box::new(body),
        }
    }

    /// Create an application.
    pub fn app(func: Term, arg: Term) -> Self {
        Term::App {
            func: Box::new(func),
            arg: Box::new(arg),
        }
    }

    /// Create multiple applications: f a_1 a_2 ... a_n
    pub fn apps(func: Term, args: impl IntoIterator<Item = Term>) -> Self {
        args.into_iter().fold(func, Term::app)
    }

    /// Create a non-dependent pair type: A * B
    pub fn product(fst_type: Term, snd_type: Term) -> Self {
        Term::Sigma {
            fst_type: Box::new(fst_type),
            snd_type: Box::new(snd_type.shift(1, 0)),
        }
    }

    /// Create a pair.
    pub fn pair(fst: Term, snd: Term) -> Self {
        Term::Pair {
            fst: Box::new(fst),
            snd: Box::new(snd),
        }
    }

    /// Create an identity type.
    pub fn id(ty: Term, lhs: Term, rhs: Term) -> Self {
        Term::Id {
            ty: Box::new(ty),
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }

    /// Create a reflexivity proof.
    pub fn refl(term: Term) -> Self {
        Term::Refl(Box::new(term))
    }

    /// Create a type annotation.
    pub fn ann(term: Term, ty: Term) -> Self {
        Term::Ann {
            term: Box::new(term),
            ty: Box::new(ty),
        }
    }

    // =========================================================================
    // PREDICATES
    // =========================================================================

    /// Check if this term is a type (Universe, Pi, Sigma, Id, or finite type).
    pub fn is_type_former(&self) -> bool {
        matches!(
            self,
            Term::Universe(_)
                | Term::Pi { .. }
                | Term::Sigma { .. }
                | Term::Id { .. }
                | Term::Empty
                | Term::Unit
                | Term::Bool
        )
    }

    /// Check if this term is a canonical/introduction form.
    pub fn is_intro_form(&self) -> bool {
        matches!(
            self,
            Term::Lambda { .. }
                | Term::Pair { .. }
                | Term::Refl(_)
                | Term::Star
                | Term::True
                | Term::False
        )
    }

    /// Check if this term is in weak head normal form.
    ///
    /// A term is in WHNF if it's not a redex at the top level.
    pub fn is_whnf(&self) -> bool {
        match self {
            // Variables are always in WHNF
            Term::Var(_) => true,

            // Type formers are in WHNF
            Term::Universe(_)
            | Term::Pi { .. }
            | Term::Sigma { .. }
            | Term::Id { .. }
            | Term::Empty
            | Term::Unit
            | Term::Bool => true,

            // Introduction forms are in WHNF
            Term::Lambda { .. }
            | Term::Pair { .. }
            | Term::Refl(_)
            | Term::Star
            | Term::True
            | Term::False => true,

            // Applications: WHNF if function is not a lambda
            Term::App { func, .. } => !matches!(func.as_ref(), Term::Lambda { .. }),

            // Projections: WHNF if scrutinee is not a pair
            Term::Fst(t) => !matches!(t.as_ref(), Term::Pair { .. }),
            Term::Snd(t) => !matches!(t.as_ref(), Term::Pair { .. }),

            // Eliminators: WHNF if scrutinee is not canonical
            Term::J { target, .. } => !matches!(target.as_ref(), Term::Refl(_)),
            Term::EmptyElim { .. } => true, // Can't reduce (no Empty intro form)
            Term::UnitElim { scrutinee, .. } => !matches!(scrutinee.as_ref(), Term::Star),
            Term::BoolElim { scrutinee, .. } => {
                !matches!(scrutinee.as_ref(), Term::True | Term::False)
            }

            // Annotations are not in WHNF (need to be erased)
            Term::Ann { .. } => false,
        }
    }

    // =========================================================================
    // SHIFTING (de Bruijn index manipulation)
    // =========================================================================

    /// Shift free variables by `delta`, with cutoff `cutoff`.
    ///
    /// Variables with index >= cutoff are considered free and shifted.
    /// Variables with index < cutoff are considered bound and unchanged.
    pub fn shift(&self, delta: i32, cutoff: u32) -> Term {
        self.shift_inner(delta, cutoff)
    }

    fn shift_inner(&self, delta: i32, cutoff: u32) -> Term {
        match self {
            Term::Var(DeBruijnIndex(idx)) => {
                if *idx >= cutoff {
                    let new_idx = if delta >= 0 {
                        idx + delta as u32
                    } else {
                        idx.checked_sub((-delta) as u32)
                            .expect("shift underflow: variable would become negative")
                    };
                    Term::Var(DeBruijnIndex(new_idx))
                } else {
                    Term::Var(DeBruijnIndex(*idx))
                }
            }

            Term::Universe(l) => Term::Universe(l.clone()),

            Term::Pi { domain, codomain } => Term::Pi {
                domain: Box::new(domain.shift_inner(delta, cutoff)),
                codomain: Box::new(codomain.shift_inner(delta, cutoff + 1)),
            },

            Term::Lambda { body } => Term::Lambda {
                body: Box::new(body.shift_inner(delta, cutoff + 1)),
            },

            Term::App { func, arg } => Term::App {
                func: Box::new(func.shift_inner(delta, cutoff)),
                arg: Box::new(arg.shift_inner(delta, cutoff)),
            },

            Term::Sigma { fst_type, snd_type } => Term::Sigma {
                fst_type: Box::new(fst_type.shift_inner(delta, cutoff)),
                snd_type: Box::new(snd_type.shift_inner(delta, cutoff + 1)),
            },

            Term::Pair { fst, snd } => Term::Pair {
                fst: Box::new(fst.shift_inner(delta, cutoff)),
                snd: Box::new(snd.shift_inner(delta, cutoff)),
            },

            Term::Fst(t) => Term::Fst(Box::new(t.shift_inner(delta, cutoff))),
            Term::Snd(t) => Term::Snd(Box::new(t.shift_inner(delta, cutoff))),

            Term::Id { ty, lhs, rhs } => Term::Id {
                ty: Box::new(ty.shift_inner(delta, cutoff)),
                lhs: Box::new(lhs.shift_inner(delta, cutoff)),
                rhs: Box::new(rhs.shift_inner(delta, cutoff)),
            },

            Term::Refl(t) => Term::Refl(Box::new(t.shift_inner(delta, cutoff))),

            Term::J {
                motive,
                base_case,
                target,
            } => Term::J {
                motive: Box::new(motive.shift_inner(delta, cutoff + 3)),
                base_case: Box::new(base_case.shift_inner(delta, cutoff + 1)),
                target: Box::new(target.shift_inner(delta, cutoff)),
            },

            Term::Empty => Term::Empty,
            Term::EmptyElim { motive, scrutinee } => Term::EmptyElim {
                motive: Box::new(motive.shift_inner(delta, cutoff)),
                scrutinee: Box::new(scrutinee.shift_inner(delta, cutoff)),
            },

            Term::Unit => Term::Unit,
            Term::Star => Term::Star,
            Term::UnitElim {
                motive,
                base_case,
                scrutinee,
            } => Term::UnitElim {
                motive: Box::new(motive.shift_inner(delta, cutoff + 1)),
                base_case: Box::new(base_case.shift_inner(delta, cutoff)),
                scrutinee: Box::new(scrutinee.shift_inner(delta, cutoff)),
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
                motive: Box::new(motive.shift_inner(delta, cutoff + 1)),
                true_case: Box::new(true_case.shift_inner(delta, cutoff)),
                false_case: Box::new(false_case.shift_inner(delta, cutoff)),
                scrutinee: Box::new(scrutinee.shift_inner(delta, cutoff)),
            },

            Term::Ann { term, ty } => Term::Ann {
                term: Box::new(term.shift_inner(delta, cutoff)),
                ty: Box::new(ty.shift_inner(delta, cutoff)),
            },
        }
    }
}

// =============================================================================
// DISPLAY IMPLEMENTATIONS
// =============================================================================

impl fmt::Display for DeBruijnIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{}", self.0)
    }
}

impl fmt::Display for UniverseLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.to_nat() {
            Some(n) => write!(f, "{}", n),
            None => write!(f, "{:?}", self), // Fallback for complex levels
        }
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // WIP: Implement pretty-printing with name recovery
        // For now, use a simple structural representation
        match self {
            Term::Var(idx) => write!(f, "{}", idx),
            Term::Universe(l) => write!(f, "U{}", l),
            Term::Pi { domain, codomain } => write!(f, "Pi({}).{}", domain, codomain),
            Term::Lambda { body } => write!(f, "lam.{}", body),
            Term::App { func, arg } => write!(f, "({} {})", func, arg),
            Term::Sigma { fst_type, snd_type } => write!(f, "Sigma({}).{}", fst_type, snd_type),
            Term::Pair { fst, snd } => write!(f, "<{}, {}>", fst, snd),
            Term::Fst(t) => write!(f, "fst({})", t),
            Term::Snd(t) => write!(f, "snd({})", t),
            Term::Id { ty, lhs, rhs } => write!(f, "Id[{}]({}, {})", ty, lhs, rhs),
            Term::Refl(t) => write!(f, "refl({})", t),
            Term::J { .. } => write!(f, "J(...)"),
            Term::Empty => write!(f, "Empty"),
            Term::EmptyElim { .. } => write!(f, "absurd(...)"),
            Term::Unit => write!(f, "Unit"),
            Term::Star => write!(f, "*"),
            Term::UnitElim { .. } => write!(f, "unit-elim(...)"),
            Term::Bool => write!(f, "Bool"),
            Term::True => write!(f, "true"),
            Term::False => write!(f, "false"),
            Term::BoolElim { .. } => write!(f, "if-then-else(...)"),
            Term::Ann { term, ty } => write!(f, "({} : {})", term, ty),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_de_bruijn_index() {
        let idx = DeBruijnIndex::new(0);
        assert_eq!(idx.succ(), DeBruijnIndex::new(1));
        assert_eq!(idx.pred(), None);
        assert_eq!(DeBruijnIndex::new(1).pred(), Some(DeBruijnIndex::new(0)));
    }

    #[test]
    fn test_universe_level() {
        assert_eq!(UniverseLevel::zero().to_nat(), Some(0));
        assert_eq!(UniverseLevel::from_nat(3).to_nat(), Some(3));
        assert_eq!(
            UniverseLevel::from_nat(2).max(UniverseLevel::from_nat(5)).to_nat(),
            Some(5)
        );
    }

    #[test]
    fn test_smart_constructors() {
        let x = Term::var(0);
        assert!(matches!(x, Term::Var(DeBruijnIndex(0))));

        let u = Term::universe(1);
        assert!(matches!(u, Term::Universe(_)));

        let lam = Term::lam(Term::var(0));
        assert!(matches!(lam, Term::Lambda { .. }));
    }

    #[test]
    fn test_shift_free_variable() {
        // #0 shifted by 1 with cutoff 0 = #1
        let var = Term::var(0);
        assert_eq!(var.shift(1, 0), Term::var(1));
    }

    #[test]
    fn test_shift_bound_variable() {
        // #0 under a lambda, shifted with cutoff 0, stays #0 inside
        let lam = Term::lam(Term::var(0));
        // Shifting the lambda itself - var(0) is bound, so unchanged
        assert_eq!(lam.shift(1, 0), Term::lam(Term::var(0)));
    }

    #[test]
    fn test_shift_free_under_lambda() {
        // lambda. #1 shifted by 1 with cutoff 0 = lambda. #2
        // The #1 refers to something outside the lambda, so it gets shifted
        let lam = Term::lam(Term::var(1));
        assert_eq!(lam.shift(1, 0), Term::lam(Term::var(2)));
    }

    #[test]
    fn test_is_whnf() {
        assert!(Term::Star.is_whnf());
        assert!(Term::var(0).is_whnf());
        assert!(Term::lam(Term::var(0)).is_whnf());

        // Application of lambda is not WHNF
        let app = Term::app(Term::lam(Term::var(0)), Term::Star);
        assert!(!app.is_whnf());

        // Application of variable is WHNF
        let app_var = Term::app(Term::var(0), Term::Star);
        assert!(app_var.is_whnf());
    }

    #[test]
    fn test_predicates() {
        assert!(Term::universe(0).is_type_former());
        assert!(Term::Bool.is_type_former());
        assert!(!Term::True.is_type_former());

        assert!(Term::True.is_intro_form());
        assert!(Term::Star.is_intro_form());
        assert!(!Term::Bool.is_intro_form());
    }
}
