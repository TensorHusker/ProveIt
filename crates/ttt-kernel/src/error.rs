//! Type errors for the trust kernel.

use ttt_core::term::UniverseLevel;
use ttt_core::Term;

/// An error that occurred during type checking.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TypeError {
    /// A variable index was not found in the context.
    UnboundVariable { index: u32 },

    /// Expected a type (term of type U_i), got something else.
    NotAType { term: Term, inferred: Term },

    /// Expected a function type (Pi), got something else.
    NotAFunction { term: Term, inferred: Term },

    /// Expected a pair type (Sigma), got something else.
    NotAPair { term: Term, inferred: Term },

    /// Type mismatch between expected and inferred.
    TypeMismatch {
        expected: Term,
        found: Term,
        term: Term,
    },

    /// Universe level mismatch.
    UniverseLevelMismatch {
        expected: UniverseLevel,
        found: UniverseLevel,
    },

    /// Cannot infer type of bare lambda (needs annotation).
    CannotInferLambda { term: Term },

    /// Cannot infer type of bare pair (needs annotation).
    CannotInferPair { term: Term },

    /// J eliminator target is not an identity type.
    JTargetNotId { target: Term, inferred: Term },

    /// Two terms are not definitionally equal.
    NotEqual { lhs: Term, rhs: Term, ty: Term },
}

impl std::fmt::Display for TypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeError::UnboundVariable { index } => {
                write!(f, "unbound variable: index {} not in context", index)
            }
            TypeError::NotAType { term, inferred } => {
                write!(f, "expected a type, but {} has type {}", term, inferred)
            }
            TypeError::NotAFunction { term, inferred } => {
                write!(f, "expected a function, but {} has type {}", term, inferred)
            }
            TypeError::NotAPair { term, inferred } => {
                write!(f, "expected a pair, but {} has type {}", term, inferred)
            }
            TypeError::TypeMismatch {
                expected,
                found,
                term,
            } => {
                write!(
                    f,
                    "type mismatch for {}: expected {}, found {}",
                    term, expected, found
                )
            }
            TypeError::UniverseLevelMismatch { expected, found } => {
                write!(
                    f,
                    "universe level mismatch: expected {}, found {}",
                    expected, found
                )
            }
            TypeError::CannotInferLambda { term } => {
                write!(
                    f,
                    "cannot infer type of lambda {}: add type annotation",
                    term
                )
            }
            TypeError::CannotInferPair { term } => {
                write!(
                    f,
                    "cannot infer type of pair {}: add type annotation",
                    term
                )
            }
            TypeError::JTargetNotId { target, inferred } => {
                write!(
                    f,
                    "J eliminator target {} has type {}, expected identity type",
                    target, inferred
                )
            }
            TypeError::NotEqual { lhs, rhs, ty } => {
                write!(f, "{} and {} are not equal at type {}", lhs, rhs, ty)
            }
        }
    }
}

impl std::error::Error for TypeError {}
