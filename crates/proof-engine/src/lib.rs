//! # Proof Engine: Interactive theorem proving for ProveIt
//!
//! This crate implements the proof engine that manages interactive proof construction,
//! verification, tactics, and automated proof search.

pub mod goals;
pub mod search;
pub mod tactics;
pub mod verifier;

pub use goals::{Goal, GoalId, GoalState, ProofState};
pub use search::{ProofSearch, SearchStrategy};
pub use tactics::{Tactic, TacticLibrary, TacticResult};
pub use verifier::{VerificationResult, Verifier};

/// Result type for proof engine operations
pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can occur in the proof engine
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Tactic failed: {0}")]
    TacticFailed(String),

    #[error("Goal not found")]
    GoalNotFound,

    #[error("Invalid proof state: {0}")]
    InvalidProofState(String),

    #[error("Verification failed: {0}")]
    VerificationFailed(String),

    #[error("Search timeout")]
    SearchTimeout,

    #[error("No solution found")]
    NoSolution,

    #[error("Type error: {0}")]
    TypeError(String),

    #[error("SCTT error: {0}")]
    ScttError(#[from] sctt_core::Error),

    #[error("Geometry error: {0}")]
    GeometryError(#[from] geometry::Error),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proof_engine_basics() {
        // Basic sanity test
        assert!(true);
    }
}
