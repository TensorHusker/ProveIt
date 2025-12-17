//! Tiny Type Theory Core
//!
//! This crate provides the fundamental building blocks for TTT:
//! - Term representation with de Bruijn indices
//! - Capture-avoiding substitution
//! - Normalization (beta-reduction)
//! - Definitional equality
//! - Typing contexts
//!
//! # Example
//!
//! ```
//! use ttt_core::prelude::*;
//!
//! // Construct the identity function: lambda x. x
//! let id = Term::lam(Term::var(0));
//!
//! // Apply it to Star
//! let app = Term::app(id, Term::Star);
//!
//! // Normalize: (lambda. #0) * --> *
//! assert_eq!(app.normalize(), Term::Star);
//! ```
//!
//! # Design Philosophy
//!
//! TTT is designed for:
//! - **Simplicity**: The entire core fits in ~1500 lines
//! - **Correctness**: De Bruijn indices eliminate alpha-equivalence issues
//! - **Performance**: Designed for GPU acceleration in later phases
//! - **Auditability**: Clear, straightforward implementations

pub mod context;
pub mod equality;
pub mod normalization;
pub mod substitution;
pub mod term;

pub mod prelude {
    //! Convenient re-exports for common usage.
    pub use crate::context::{Context, ContextEntry};
    pub use crate::equality::definitionally_equal;
    pub use crate::term::{DeBruijnIndex, Term, UniverseLevel};
}

// Re-export main types at crate root
pub use context::Context;
pub use term::Term;
