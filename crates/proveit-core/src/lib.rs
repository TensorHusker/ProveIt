//! ProveIt Core Library
//! 
//! Foundational types and traits for the ProveIt geometric verification system.
//! Provides core abstractions for proofs, verification, and spatial reasoning.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Error types for ProveIt operations
#[derive(Debug, thiserror::Error)]
pub enum ProveItError {
    #[error("Type error: {0}")]
    TypeError(String),
    
    #[error("Verification failed: {0}")]
    VerificationError(String),
    
    #[error("Spatial error: {0}")]
    SpatialError(String),
    
    #[error("Accessibility error: {0}")]
    AccessibilityError(String),
}

/// Result type for ProveIt operations
pub type Result<T> = std::result::Result<T, ProveItError>;

/// Unique identifier for proof objects
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ProofId(pub u64);

/// Core trait for verifiable objects
pub trait Verifiable {
    /// Verify the correctness of this object
    fn verify(&self) -> Result<bool>;
    
    /// Get a human-readable description for accessibility
    fn describe(&self) -> String;
}

/// Represents a position in 3D space for geometric reasoning
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Position {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
    
    pub fn origin() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:.2}, {:.2}, {:.2})", self.x, self.y, self.z)
    }
}

/// Represents a spatial object with geometric properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpatialObject {
    pub id: ProofId,
    pub position: Position,
    pub label: String,
}

impl Verifiable for SpatialObject {
    fn verify(&self) -> Result<bool> {
        // Basic spatial verification - ensure position is finite
        if !self.position.x.is_finite() 
            || !self.position.y.is_finite() 
            || !self.position.z.is_finite() {
            return Err(ProveItError::SpatialError(
                "Position contains non-finite coordinates".to_string()
            ));
        }
        Ok(true)
    }
    
    fn describe(&self) -> String {
        format!("{} at position {}", self.label, self.position)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_creation() {
        let pos = Position::new(1.0, 2.0, 3.0);
        assert_eq!(pos.x, 1.0);
        assert_eq!(pos.y, 2.0);
        assert_eq!(pos.z, 3.0);
    }

    #[test]
    fn test_spatial_object_verification() {
        let obj = SpatialObject {
            id: ProofId(1),
            position: Position::new(1.0, 2.0, 3.0),
            label: "Test Point".to_string(),
        };
        assert!(obj.verify().unwrap());
    }

    #[test]
    fn test_invalid_position() {
        let obj = SpatialObject {
            id: ProofId(1),
            position: Position::new(f64::INFINITY, 2.0, 3.0),
            label: "Invalid Point".to_string(),
        };
        assert!(obj.verify().is_err());
    }
}
