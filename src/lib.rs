//! ProveIt - Geometric construction environment for accessible formal verification
//!
//! This is the main library that re-exports all ProveIt functionality.

pub use proveit_accessibility as accessibility;
pub use proveit_core as core;
pub use proveit_formal as formal;
pub use proveit_gpu as gpu;
pub use proveit_spatial as spatial;
pub use proveit_type_checker as type_checker;

// Re-export commonly used types
pub use proveit_core::{Position, ProofId, ProveItError, Result, Verifiable};
pub use proveit_type_checker::{NeuralTypeChecker, TypeCategory, TypeSignature};
pub use proveit_spatial::{SpatialGraph, SpatialNode, Transformation};
pub use proveit_formal::{FormalType, Path, UniverseLevel};
pub use proveit_accessibility::{AccessibilitySettings, SpatialAudio};
