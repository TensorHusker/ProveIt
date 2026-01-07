//! Output fusion strategies for combining distributed inference results

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{ComponentId, Error, Result};

/// Strategy for fusing outputs from multiple workers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FusionStrategy {
    /// Simple concatenation
    Concatenate,
    /// Weighted average
    WeightedAverage { weights: Vec<f32> },
    /// Maximum probability
    MaxProbability,
    /// Ensemble voting
    Voting { threshold: f32 },
    /// Custom fusion function
    Custom(String),
}

/// Output fusion engine
pub struct OutputFusion {
    strategy: FusionStrategy,
}

/// Intermediate output from a component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentOutput {
    pub component_id: ComponentId,
    pub data: Vec<f32>,
    pub metadata: HashMap<String, String>,
}

impl OutputFusion {
    pub fn new(strategy: FusionStrategy) -> Self {
        Self { strategy }
    }

    /// Fuse multiple outputs into a single result
    pub fn fuse(&self, outputs: Vec<ComponentOutput>) -> Result<Vec<f32>> {
        if outputs.is_empty() {
            return Err(Error::FusionError("No outputs to fuse".to_string()));
        }

        match &self.strategy {
            FusionStrategy::Concatenate => Ok(self.concatenate(outputs)),
            FusionStrategy::WeightedAverage { weights } => self.weighted_average(outputs, weights),
            FusionStrategy::MaxProbability => self.max_probability(outputs),
            FusionStrategy::Voting { threshold } => self.voting(outputs, *threshold),
            FusionStrategy::Custom(name) => Err(Error::FusionError(format!(
                "Custom fusion '{}' not implemented",
                name
            ))),
        }
    }

    fn concatenate(&self, outputs: Vec<ComponentOutput>) -> Vec<f32> {
        outputs.into_iter().flat_map(|o| o.data).collect()
    }

    fn weighted_average(&self, outputs: Vec<ComponentOutput>, weights: &[f32]) -> Result<Vec<f32>> {
        if outputs.len() != weights.len() {
            return Err(Error::FusionError("Weights length mismatch".to_string()));
        }

        let size = outputs[0].data.len();
        let mut result = vec![0.0; size];

        for (output, &weight) in outputs.iter().zip(weights.iter()) {
            for (i, &val) in output.data.iter().enumerate() {
                result[i] += val * weight;
            }
        }

        Ok(result)
    }

    fn max_probability(&self, outputs: Vec<ComponentOutput>) -> Result<Vec<f32>> {
        let size = outputs[0].data.len();
        let mut result = vec![0.0f32; size];

        for output in outputs {
            for (i, &val) in output.data.iter().enumerate() {
                result[i] = result[i].max(val);
            }
        }

        Ok(result)
    }

    fn voting(&self, outputs: Vec<ComponentOutput>, threshold: f32) -> Result<Vec<f32>> {
        let size = outputs[0].data.len();
        let mut votes = vec![0; size];

        for output in outputs {
            for (i, &val) in output.data.iter().enumerate() {
                if val > threshold {
                    votes[i] += 1;
                }
            }
        }

        Ok(votes.into_iter().map(|v| v as f32).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_concatenate() {
        let fusion = OutputFusion::new(FusionStrategy::Concatenate);
        let outputs = vec![
            ComponentOutput {
                component_id: ComponentId(0),
                data: vec![1.0, 2.0],
                metadata: HashMap::new(),
            },
            ComponentOutput {
                component_id: ComponentId(1),
                data: vec![3.0, 4.0],
                metadata: HashMap::new(),
            },
        ];

        let result = fusion.fuse(outputs).unwrap();
        assert_eq!(result, vec![1.0, 2.0, 3.0, 4.0]);
    }
}
