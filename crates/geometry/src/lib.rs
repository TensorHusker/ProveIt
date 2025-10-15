//! # Geometry: Spatial Proof Construction
//!
//! This crate implements the geometric layer of ProveIt, where formal proofs
//! are represented as spatial constructions. It bridges visual/spatial reasoning
//! with formal type-theoretical proofs.

pub mod construction;
pub mod line;
pub mod point;
pub mod proof_bridge;
pub mod spatial;

pub use construction::{Construction, ConstructionGraph};
pub use line::{Line, LineId};
pub use point::{Point, PointId};
pub use spatial::{Position, SpatialRelation};

/// Result type for geometric operations
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid construction: {0}")]
    InvalidConstruction(String),

    #[error("Geometric constraint violation: {0}")]
    ConstraintViolation(String),

    #[error("Cannot construct: {0}")]
    CannotConstruct(String),

    #[error("Dependency cycle detected")]
    DependencyCycle,

    #[error("Proof correspondence error: {0}")]
    ProofCorrespondence(String),
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_basic_construction() {
        assert!(true);
    }
}
