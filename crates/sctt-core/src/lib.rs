//! # SCTT Core: Smooth Cubical Type Theory
//!
//! This crate implements the foundational type theory for ProveIt's formal verification system.
//! It provides a smooth extension of cubical type theory, enabling differential structure
//! within dependent types.
//!
//! ## Key Concepts
//!
//! - **Types**: Universe hierarchy, dependent products, paths, smooth paths
//! - **Terms**: Variables, lambdas, applications, path constructors
//! - **Values**: Normalized computational forms
//! - **Evaluation**: Normalization by Evaluation (NbE) for efficient type checking
//! - **Kan Operations**: Composition and coercion for cubical structure

pub mod check;
pub mod eval;
pub mod kan;
pub mod normalize;
pub mod smooth;
pub mod syntax;
pub mod value;

pub use check::{check as check_type, infer, Context};
pub use eval::eval;
pub use normalize::normalize;
pub use syntax::{Expr, Level, Name};
pub use value::{Closure, Env, Neutral, Value};

/// Result type for SCTT operations
pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can occur during type checking and evaluation
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Type mismatch: expected {expected}, got {got}")]
    TypeMismatch { expected: String, got: String },

    #[error("Variable not found: {0}")]
    UnboundVariable(String),

    #[error("Cannot infer type for: {0}")]
    CannotInfer(String),

    #[error("Dimension variable not found: {0}")]
    UnboundDimension(String),

    #[error("Invalid Kan operation: {0}")]
    InvalidKan(String),

    #[error("Smoothness violation: expected C^{expected} but got C^{got}")]
    SmoothnessViolation { expected: u32, got: u32 },

    #[error("Universe level inconsistency")]
    UniverseLevel,
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_basic_type_checking() {
        // Simple identity function: λx. x : (A : Type) → A → A
        // This will be expanded with actual implementation
        assert!(true);
    }
}
