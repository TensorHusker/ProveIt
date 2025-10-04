//! Spatial Graph Engine with Geometric Transformations
//!
//! Provides graph-based spatial reasoning with geometric transformations.
//! Designed for interactive proof construction through spatial manipulation.

use proveit_core::{Position, ProofId, ProveItError, Result};
use nalgebra::{Matrix4, Vector3, Vector4};
use petgraph::graph::{Graph, NodeIndex};
use serde::{Deserialize, Serialize};

/// Geometric transformation in 3D space
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Transformation {
    /// Translation by a vector
    Translation(Vector3<f64>),
    /// Rotation around an axis by angle (radians)
    Rotation { axis: Vector3<f64>, angle: f64 },
    /// Scaling by factors
    Scale(Vector3<f64>),
    /// Reflection across a plane
    Reflection(Vector3<f64>),
    /// Composite transformation
    Composite(Vec<Transformation>),
}

impl Transformation {
    /// Convert transformation to a 4x4 matrix
    pub fn to_matrix(&self) -> Matrix4<f64> {
        match self {
            Transformation::Translation(v) => {
                Matrix4::new_translation(v)
            }
            Transformation::Rotation { axis, angle } => {
                Matrix4::from_axis_angle(&nalgebra::Unit::new_normalize(*axis), *angle)
            }
            Transformation::Scale(s) => {
                Matrix4::new_nonuniform_scaling(s)
            }
            Transformation::Reflection(n) => {
                let n = n.normalize();
                let mut m = Matrix4::identity();
                m[(0, 0)] = 1.0 - 2.0 * n.x * n.x;
                m[(0, 1)] = -2.0 * n.x * n.y;
                m[(0, 2)] = -2.0 * n.x * n.z;
                m[(1, 0)] = -2.0 * n.y * n.x;
                m[(1, 1)] = 1.0 - 2.0 * n.y * n.y;
                m[(1, 2)] = -2.0 * n.y * n.z;
                m[(2, 0)] = -2.0 * n.z * n.x;
                m[(2, 1)] = -2.0 * n.z * n.y;
                m[(2, 2)] = 1.0 - 2.0 * n.z * n.z;
                m
            }
            Transformation::Composite(transforms) => {
                transforms.iter()
                    .map(|t| t.to_matrix())
                    .fold(Matrix4::identity(), |acc, m| acc * m)
            }
        }
    }
    
    /// Apply transformation to a position
    pub fn apply(&self, pos: &Position) -> Position {
        let matrix = self.to_matrix();
        let vec = Vector4::new(pos.x, pos.y, pos.z, 1.0);
        let result = matrix * vec;
        
        Position::new(
            result.x / result.w,
            result.y / result.w,
            result.z / result.w,
        )
    }
    
    /// Get human-readable description for accessibility
    pub fn describe(&self) -> String {
        match self {
            Transformation::Translation(v) => {
                format!("Translation by ({:.2}, {:.2}, {:.2})", v.x, v.y, v.z)
            }
            Transformation::Rotation { axis, angle } => {
                format!("Rotation by {:.2} radians around ({:.2}, {:.2}, {:.2})", 
                    angle, axis.x, axis.y, axis.z)
            }
            Transformation::Scale(s) => {
                format!("Scale by ({:.2}, {:.2}, {:.2})", s.x, s.y, s.z)
            }
            Transformation::Reflection(n) => {
                format!("Reflection across plane with normal ({:.2}, {:.2}, {:.2})", 
                    n.x, n.y, n.z)
            }
            Transformation::Composite(transforms) => {
                format!("Composite of {} transformations", transforms.len())
            }
        }
    }
}

/// Node in the spatial graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpatialNode {
    pub id: ProofId,
    pub position: Position,
    pub label: String,
}

/// Edge representing a spatial relationship
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpatialEdge {
    pub transformation: Transformation,
    pub label: String,
}

/// Spatial graph for geometric reasoning
pub struct SpatialGraph {
    graph: Graph<SpatialNode, SpatialEdge>,
    node_map: std::collections::HashMap<ProofId, NodeIndex>,
}

impl SpatialGraph {
    /// Create a new empty spatial graph
    pub fn new() -> Self {
        Self {
            graph: Graph::new(),
            node_map: std::collections::HashMap::new(),
        }
    }
    
    /// Add a node to the graph
    pub fn add_node(&mut self, node: SpatialNode) -> NodeIndex {
        let id = node.id;
        let index = self.graph.add_node(node);
        self.node_map.insert(id, index);
        index
    }
    
    /// Add an edge between two nodes
    pub fn add_edge(&mut self, from: ProofId, to: ProofId, edge: SpatialEdge) -> Result<()> {
        let from_idx = self.node_map.get(&from)
            .ok_or_else(|| ProveItError::SpatialError("Source node not found".to_string()))?;
        let to_idx = self.node_map.get(&to)
            .ok_or_else(|| ProveItError::SpatialError("Target node not found".to_string()))?;
        
        self.graph.add_edge(*from_idx, *to_idx, edge);
        Ok(())
    }
    
    /// Get a node by its proof ID
    pub fn get_node(&self, id: ProofId) -> Option<&SpatialNode> {
        self.node_map.get(&id)
            .and_then(|idx| self.graph.node_weight(*idx))
    }
    
    /// Apply a transformation to a node
    pub fn transform_node(&mut self, id: ProofId, transformation: &Transformation) -> Result<()> {
        let idx = self.node_map.get(&id)
            .ok_or_else(|| ProveItError::SpatialError("Node not found".to_string()))?;
        
        if let Some(node) = self.graph.node_weight_mut(*idx) {
            node.position = transformation.apply(&node.position);
        }
        
        Ok(())
    }
    
    /// Get neighbors of a node
    pub fn neighbors(&self, id: ProofId) -> Vec<ProofId> {
        if let Some(idx) = self.node_map.get(&id) {
            self.graph.neighbors(*idx)
                .filter_map(|n| self.graph.node_weight(n))
                .map(|node| node.id)
                .collect()
        } else {
            Vec::new()
        }
    }
    
    /// Describe the graph for accessibility
    pub fn describe(&self) -> String {
        format!("Spatial graph with {} nodes and {} edges", 
            self.graph.node_count(), 
            self.graph.edge_count())
    }
}

impl Default for SpatialGraph {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translation() {
        let transform = Transformation::Translation(Vector3::new(1.0, 2.0, 3.0));
        let pos = Position::new(0.0, 0.0, 0.0);
        let result = transform.apply(&pos);
        
        assert!((result.x - 1.0).abs() < 1e-10);
        assert!((result.y - 2.0).abs() < 1e-10);
        assert!((result.z - 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_spatial_graph() {
        let mut graph = SpatialGraph::new();
        
        let node1 = SpatialNode {
            id: ProofId(1),
            position: Position::new(0.0, 0.0, 0.0),
            label: "Point A".to_string(),
        };
        
        let node2 = SpatialNode {
            id: ProofId(2),
            position: Position::new(1.0, 0.0, 0.0),
            label: "Point B".to_string(),
        };
        
        graph.add_node(node1);
        graph.add_node(node2);
        
        assert!(graph.get_node(ProofId(1)).is_some());
        assert!(graph.get_node(ProofId(2)).is_some());
    }

    #[test]
    fn test_graph_edges() {
        let mut graph = SpatialGraph::new();
        
        let node1 = SpatialNode {
            id: ProofId(1),
            position: Position::new(0.0, 0.0, 0.0),
            label: "Point A".to_string(),
        };
        
        let node2 = SpatialNode {
            id: ProofId(2),
            position: Position::new(1.0, 0.0, 0.0),
            label: "Point B".to_string(),
        };
        
        graph.add_node(node1);
        graph.add_node(node2);
        
        let edge = SpatialEdge {
            transformation: Transformation::Translation(Vector3::new(1.0, 0.0, 0.0)),
            label: "Move right".to_string(),
        };
        
        assert!(graph.add_edge(ProofId(1), ProofId(2), edge).is_ok());
    }
}
