//! # Butterfly Core: Distributed Inference Framework
//!
//! Butterfly enables distributed execution of large language models and proof search
//! across multiple devices, using functional decomposition and intelligent output fusion.

pub mod fusion;
pub mod model_split;
pub mod worker;

pub use fusion::{FusionStrategy, OutputFusion};
pub use model_split::{ComponentId, FunctionalDecomposition, ModelSplit};
pub use worker::{WorkTask, WorkerCapabilities, WorkerNode};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Result type for Butterfly operations
pub type Result<T> = std::result::Result<T, Error>;

/// Errors in the distributed system
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Worker unavailable: {0}")]
    WorkerUnavailable(Uuid),

    #[error("Model split failed: {0}")]
    ModelSplitError(String),

    #[error("Fusion failed: {0}")]
    FusionError(String),

    #[error("Task execution failed: {0}")]
    TaskExecutionError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Timeout")]
    Timeout,
}

/// Unique identifier for model instances
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ModelId(pub Uuid);

impl ModelId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for ModelId {
    fn default() -> Self {
        Self::new()
    }
}

/// Unique identifier for workers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WorkerId(pub Uuid);

impl WorkerId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for WorkerId {
    fn default() -> Self {
        Self::new()
    }
}

/// Unique identifier for tasks
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TaskId(pub Uuid);

impl TaskId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for TaskId {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id_generation() {
        let id1 = ModelId::new();
        let id2 = ModelId::new();
        assert_ne!(id1, id2);
    }
}
