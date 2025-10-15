//! Model splitting and functional decomposition

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{Error, ModelId, Result, WorkerId};

/// Represents a split model across multiple workers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelSplit {
    /// Model identifier
    pub model_id: ModelId,
    /// Decomposition strategy used
    pub decomposition: FunctionalDecomposition,
    /// Assignment of components to workers
    pub worker_assignment: HashMap<ComponentId, WorkerId>,
}

/// Unique identifier for model components
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ComponentId(pub usize);

/// Functional decomposition of a model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionalDecomposition {
    /// Decomposition strategy
    pub strategy: DecompositionStrategy,
    /// Model components
    pub components: Vec<ModelComponent>,
    /// Dependencies between components
    pub dependencies: Vec<(ComponentId, ComponentId)>,
}

/// Strategy for decomposing a model
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DecompositionStrategy {
    /// Split by layers (e.g., transformer layers)
    LayerWise,
    /// Split by functional roles (e.g., encoder/decoder)
    Functional,
    /// Split by attention heads
    AttentionHeads,
    /// Hybrid approach
    Hybrid,
}

/// A component of a decomposed model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelComponent {
    /// Component identifier
    pub id: ComponentId,
    /// Human-readable name
    pub name: String,
    /// Component type
    pub component_type: ComponentType,
    /// Estimated computation cost (arbitrary units)
    pub compute_cost: f64,
    /// Estimated memory requirement (bytes)
    pub memory_requirement: u64,
    /// Input tensor shapes
    pub input_shapes: Vec<TensorShape>,
    /// Output tensor shapes
    pub output_shapes: Vec<TensorShape>,
}

/// Type of model component
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComponentType {
    Embedding,
    AttentionLayer { layer_num: usize },
    FeedForward { layer_num: usize },
    OutputHead,
    Normalization,
    Custom(String),
}

/// Shape of a tensor
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TensorShape {
    pub dimensions: Vec<usize>,
}

impl TensorShape {
    pub fn new(dimensions: Vec<usize>) -> Self {
        Self { dimensions }
    }

    pub fn total_elements(&self) -> usize {
        self.dimensions.iter().product()
    }
}

impl FunctionalDecomposition {
    /// Create a new decomposition
    pub fn new(strategy: DecompositionStrategy) -> Self {
        Self {
            strategy,
            components: Vec::new(),
            dependencies: Vec::new(),
        }
    }

    /// Add a component
    pub fn add_component(&mut self, component: ModelComponent) -> ComponentId {
        let id = component.id;
        self.components.push(component);
        id
    }

    /// Add a dependency between components
    pub fn add_dependency(&mut self, from: ComponentId, to: ComponentId) {
        self.dependencies.push((from, to));
    }

    /// Get execution order (topological sort)
    pub fn execution_order(&self) -> Result<Vec<ComponentId>> {
        let mut in_degree: HashMap<ComponentId, usize> = HashMap::new();
        let mut adj_list: HashMap<ComponentId, Vec<ComponentId>> = HashMap::new();

        // Initialize
        for component in &self.components {
            in_degree.insert(component.id, 0);
            adj_list.insert(component.id, Vec::new());
        }

        // Build graph
        for (from, to) in &self.dependencies {
            adj_list.get_mut(from).unwrap().push(*to);
            *in_degree.get_mut(to).unwrap() += 1;
        }

        // Topological sort (Kahn's algorithm)
        let mut queue: Vec<ComponentId> = in_degree
            .iter()
            .filter(|(_, &degree)| degree == 0)
            .map(|(&id, _)| id)
            .collect();

        let mut result = Vec::new();

        while let Some(node) = queue.pop() {
            result.push(node);

            if let Some(neighbors) = adj_list.get(&node) {
                for &neighbor in neighbors {
                    let degree = in_degree.get_mut(&neighbor).unwrap();
                    *degree -= 1;
                    if *degree == 0 {
                        queue.push(neighbor);
                    }
                }
            }
        }

        if result.len() != self.components.len() {
            return Err(Error::ModelSplitError(
                "Cycle detected in dependencies".to_string(),
            ));
        }

        Ok(result)
    }

    /// Decompose a transformer-style model
    pub fn decompose_transformer(num_layers: usize, embed_dim: usize) -> Self {
        let mut decomp = Self::new(DecompositionStrategy::LayerWise);

        // Embedding layer
        let embed_id = ComponentId(0);
        decomp.add_component(ModelComponent {
            id: embed_id,
            name: "embedding".to_string(),
            component_type: ComponentType::Embedding,
            compute_cost: 1.0,
            memory_requirement: (embed_dim * 50000 * 4) as u64, // Rough estimate
            input_shapes: vec![TensorShape::new(vec![1, 512])],
            output_shapes: vec![TensorShape::new(vec![1, 512, embed_dim])],
        });

        let mut prev_id = embed_id;

        // Attention and FFN layers
        for i in 0..num_layers {
            // Attention
            let attn_id = ComponentId(2 * i + 1);
            decomp.add_component(ModelComponent {
                id: attn_id,
                name: format!("attention_{}", i),
                component_type: ComponentType::AttentionLayer { layer_num: i },
                compute_cost: 10.0,
                memory_requirement: (embed_dim * embed_dim * 16) as u64,
                input_shapes: vec![TensorShape::new(vec![1, 512, embed_dim])],
                output_shapes: vec![TensorShape::new(vec![1, 512, embed_dim])],
            });
            decomp.add_dependency(prev_id, attn_id);

            // FFN
            let ffn_id = ComponentId(2 * i + 2);
            decomp.add_component(ModelComponent {
                id: ffn_id,
                name: format!("ffn_{}", i),
                component_type: ComponentType::FeedForward { layer_num: i },
                compute_cost: 8.0,
                memory_requirement: (embed_dim * embed_dim * 32) as u64,
                input_shapes: vec![TensorShape::new(vec![1, 512, embed_dim])],
                output_shapes: vec![TensorShape::new(vec![1, 512, embed_dim])],
            });
            decomp.add_dependency(attn_id, ffn_id);

            prev_id = ffn_id;
        }

        // Output head
        let out_id = ComponentId(2 * num_layers + 1);
        decomp.add_component(ModelComponent {
            id: out_id,
            name: "output_head".to_string(),
            component_type: ComponentType::OutputHead,
            compute_cost: 5.0,
            memory_requirement: (embed_dim * 50000 * 4) as u64,
            input_shapes: vec![TensorShape::new(vec![1, 512, embed_dim])],
            output_shapes: vec![TensorShape::new(vec![1, 512, 50000])],
        });
        decomp.add_dependency(prev_id, out_id);

        decomp
    }
}

impl ModelSplit {
    /// Create a new model split
    pub fn new(model_id: ModelId, decomposition: FunctionalDecomposition) -> Self {
        Self {
            model_id,
            decomposition,
            worker_assignment: HashMap::new(),
        }
    }

    /// Assign a component to a worker
    pub fn assign_component(&mut self, component: ComponentId, worker: WorkerId) {
        self.worker_assignment.insert(component, worker);
    }

    /// Get worker for a component
    pub fn worker_for_component(&self, component: ComponentId) -> Option<WorkerId> {
        self.worker_assignment.get(&component).copied()
    }

    /// Validate that all components are assigned
    pub fn is_complete(&self) -> bool {
        self.decomposition.components.len() == self.worker_assignment.len()
    }

    /// Get total compute cost
    pub fn total_compute_cost(&self) -> f64 {
        self.decomposition
            .components
            .iter()
            .map(|c| c.compute_cost)
            .sum()
    }

    /// Get total memory requirement
    pub fn total_memory_requirement(&self) -> u64 {
        self.decomposition
            .components
            .iter()
            .map(|c| c.memory_requirement)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_functional_decomposition() {
        let decomp = FunctionalDecomposition::new(DecompositionStrategy::LayerWise);
        assert_eq!(decomp.components.len(), 0);
    }

    #[test]
    fn test_transformer_decomposition() {
        let decomp = FunctionalDecomposition::decompose_transformer(4, 768);
        assert!(decomp.components.len() > 4);
    }

    #[test]
    fn test_execution_order() {
        let mut decomp = FunctionalDecomposition::new(DecompositionStrategy::LayerWise);

        let c0 = ComponentId(0);
        let c1 = ComponentId(1);
        let c2 = ComponentId(2);

        decomp.add_component(ModelComponent {
            id: c0,
            name: "c0".to_string(),
            component_type: ComponentType::Embedding,
            compute_cost: 1.0,
            memory_requirement: 1000,
            input_shapes: vec![],
            output_shapes: vec![],
        });

        decomp.add_component(ModelComponent {
            id: c1,
            name: "c1".to_string(),
            component_type: ComponentType::OutputHead,
            compute_cost: 1.0,
            memory_requirement: 1000,
            input_shapes: vec![],
            output_shapes: vec![],
        });

        decomp.add_component(ModelComponent {
            id: c2,
            name: "c2".to_string(),
            component_type: ComponentType::OutputHead,
            compute_cost: 1.0,
            memory_requirement: 1000,
            input_shapes: vec![],
            output_shapes: vec![],
        });

        decomp.add_dependency(c0, c1);
        decomp.add_dependency(c1, c2);

        let order = decomp.execution_order().unwrap();
        assert_eq!(order, vec![c0, c1, c2]);
    }

    #[test]
    fn test_model_split() {
        let decomp = FunctionalDecomposition::decompose_transformer(2, 512);
        let split = ModelSplit::new(ModelId::new(), decomp);

        assert!(!split.is_complete());
        assert!(split.total_compute_cost() > 0.0);
    }
}
