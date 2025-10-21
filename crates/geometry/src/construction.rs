//! Construction graphs: Managing dependencies and proof structure

use sctt_core::Expr;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};

use crate::line::{Line, LineId, ProofPath};
use crate::point::{Point, PointId};
use crate::{Error, Result};

/// A complete geometric construction representing a proof
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Construction {
    /// Human-readable name
    pub name: String,
    /// The underlying graph structure
    pub graph: ConstructionGraph,
    /// Target theorem (if any)
    pub target: Option<Expr>,
    /// Metadata
    pub metadata: ConstructionMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstructionMetadata {
    /// Creation timestamp
    pub created_at: Option<String>,
    /// Last modified
    pub modified_at: Option<String>,
    /// Author
    pub author: Option<String>,
    /// Tags for organization
    pub tags: Vec<String>,
    /// Difficulty level (1-10)
    pub difficulty: Option<u8>,
}

impl Construction {
    pub fn new(name: String) -> Self {
        Self {
            name,
            graph: ConstructionGraph::new(),
            target: None,
            metadata: ConstructionMetadata {
                created_at: None,
                modified_at: None,
                author: None,
                tags: Vec::new(),
                difficulty: None,
            },
        }
    }

    /// Add a point (proposition) to the construction
    pub fn add_point(&mut self, point: Point) -> PointId {
        self.graph.add_point(point)
    }

    /// Add a line (implication) to the construction
    pub fn add_line(&mut self, line: Line) -> Result<LineId> {
        self.graph.add_line(line)
    }

    /// Find a proof path from hypothesis to conclusion
    pub fn find_proof_path(&self, from: PointId, to: PointId) -> Option<ProofPath> {
        self.graph.find_path(from, to)
    }

    /// Verify that the construction is valid
    pub fn verify(&self) -> Result<()> {
        self.graph.verify()
    }

    /// Check if the target theorem is proven
    pub fn is_complete(&self) -> bool {
        if let Some(target) = &self.target {
            // Check if there's a point with the target proposition
            self.graph.points.values().any(|p| &p.proposition == target)
        } else {
            false
        }
    }

    /// Get all axioms (points with no incoming edges)
    pub fn axioms(&self) -> Vec<PointId> {
        self.graph.axioms()
    }

    /// Get all theorems (points with no outgoing edges)
    pub fn theorems(&self) -> Vec<PointId> {
        self.graph.theorems()
    }

    /// Compute the depth of the proof (longest path from axiom to theorem)
    pub fn depth(&self) -> usize {
        self.graph.compute_depth()
    }

    /// Export to DOT format for visualization
    pub fn to_dot(&self) -> String {
        self.graph.to_dot(&self.name)
    }
}

/// The dependency graph of geometric constructions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstructionGraph {
    /// All points in the construction
    pub points: HashMap<PointId, Point>,
    /// All lines in the construction
    pub lines: HashMap<LineId, Line>,
    /// Adjacency list: point -> outgoing lines
    pub outgoing: HashMap<PointId, Vec<LineId>>,
    /// Adjacency list: point -> incoming lines
    pub incoming: HashMap<PointId, Vec<LineId>>,
    /// Counter for generating unique IDs
    next_point_id: u64,
    next_line_id: u64,
}

impl ConstructionGraph {
    pub fn new() -> Self {
        Self {
            points: HashMap::new(),
            lines: HashMap::new(),
            outgoing: HashMap::new(),
            incoming: HashMap::new(),
            next_point_id: 0,
            next_line_id: 0,
        }
    }

    /// Add a point to the graph
    pub fn add_point(&mut self, mut point: Point) -> PointId {
        let id = PointId(self.next_point_id);
        self.next_point_id += 1;
        point.id = id;
        self.points.insert(id, point);
        self.outgoing.insert(id, Vec::new());
        self.incoming.insert(id, Vec::new());
        id
    }

    /// Add a line to the graph
    pub fn add_line(&mut self, mut line: Line) -> Result<LineId> {
        // Verify endpoints exist
        if !self.points.contains_key(&line.from) {
            return Err(Error::InvalidConstruction(format!(
                "Source point {:?} does not exist",
                line.from
            )));
        }
        if !self.points.contains_key(&line.to) {
            return Err(Error::InvalidConstruction(format!(
                "Target point {:?} does not exist",
                line.to
            )));
        }

        let id = LineId(self.next_line_id);
        self.next_line_id += 1;
        line.id = id;

        // Add to adjacency lists
        self.outgoing.entry(line.from).or_default().push(id);
        self.incoming.entry(line.to).or_default().push(id);

        // Check for cycles
        if self.has_cycle_after_adding(id, &line)? {
            // Rollback
            self.outgoing.get_mut(&line.from).unwrap().pop();
            self.incoming.get_mut(&line.to).unwrap().pop();
            return Err(Error::DependencyCycle);
        }

        self.lines.insert(id, line);
        Ok(id)
    }

    /// Check if adding a line would create a cycle
    fn has_cycle_after_adding(&self, line_id: LineId, line: &Line) -> Result<bool> {
        // Use DFS to detect cycles
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();

        self.has_cycle_dfs(line.to, &mut visited, &mut rec_stack, Some(line_id))
    }

    fn has_cycle_dfs(
        &self,
        point: PointId,
        visited: &mut HashSet<PointId>,
        rec_stack: &mut HashSet<PointId>,
        new_line: Option<LineId>,
    ) -> Result<bool> {
        if rec_stack.contains(&point) {
            return Ok(true);
        }
        if visited.contains(&point) {
            return Ok(false);
        }

        visited.insert(point);
        rec_stack.insert(point);

        // Check all outgoing edges
        if let Some(edges) = self.outgoing.get(&point) {
            for &line_id in edges {
                if let Some(line) = self.lines.get(&line_id) {
                    if self.has_cycle_dfs(line.to, visited, rec_stack, new_line)? {
                        return Ok(true);
                    }
                }
            }
        }

        // Also check the new line if we're at its source
        if let Some(new_id) = new_line {
            if let Some(line) = self.lines.get(&new_id) {
                if line.from == point
                    && self.has_cycle_dfs(line.to, visited, rec_stack, new_line)? {
                        return Ok(true);
                    }
            }
        }

        rec_stack.remove(&point);
        Ok(false)
    }

    /// Find a path from one point to another using BFS
    pub fn find_path(&self, from: PointId, to: PointId) -> Option<ProofPath> {
        if from == to {
            return Some(ProofPath::new(from, to));
        }

        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let mut parent: HashMap<PointId, (PointId, LineId)> = HashMap::new();

        queue.push_back(from);
        visited.insert(from);

        while let Some(current) = queue.pop_front() {
            if current == to {
                // Reconstruct path
                let mut path = ProofPath::new(from, to);
                let mut node = to;
                let mut line_path = Vec::new();

                while node != from {
                    if let Some((prev, line_id)) = parent.get(&node) {
                        line_path.push(*line_id);
                        node = *prev;
                    } else {
                        break;
                    }
                }

                line_path.reverse();
                for line_id in line_path {
                    path.add_line(line_id);
                }

                return Some(path);
            }

            // Explore neighbors
            if let Some(edges) = self.outgoing.get(&current) {
                for &line_id in edges {
                    if let Some(line) = self.lines.get(&line_id) {
                        if !visited.contains(&line.to) {
                            visited.insert(line.to);
                            parent.insert(line.to, (current, line_id));
                            queue.push_back(line.to);
                        }
                    }
                }
            }
        }

        None
    }

    /// Verify the graph is valid (no dangling references, no cycles)
    pub fn verify(&self) -> Result<()> {
        // Check all line endpoints exist
        for (line_id, line) in &self.lines {
            if !self.points.contains_key(&line.from) {
                return Err(Error::InvalidConstruction(format!(
                    "Line {:?} references non-existent source point {:?}",
                    line_id, line.from
                )));
            }
            if !self.points.contains_key(&line.to) {
                return Err(Error::InvalidConstruction(format!(
                    "Line {:?} references non-existent target point {:?}",
                    line_id, line.to
                )));
            }
        }

        // Check for cycles
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();

        for &point_id in self.points.keys() {
            if !visited.contains(&point_id)
                && self.has_cycle_dfs(point_id, &mut visited, &mut rec_stack, None)? {
                    return Err(Error::DependencyCycle);
                }
        }

        Ok(())
    }

    /// Get all axioms (points with no incoming edges)
    pub fn axioms(&self) -> Vec<PointId> {
        self.points
            .keys()
            .filter(|&id| {
                self.incoming
                    .get(id)
                    .map(|edges| edges.is_empty())
                    .unwrap_or(true)
            })
            .copied()
            .collect()
    }

    /// Get all theorems (points with no outgoing edges)
    pub fn theorems(&self) -> Vec<PointId> {
        self.points
            .keys()
            .filter(|&id| {
                self.outgoing
                    .get(id)
                    .map(|edges| edges.is_empty())
                    .unwrap_or(true)
            })
            .copied()
            .collect()
    }

    /// Compute the maximum depth of the graph
    pub fn compute_depth(&self) -> usize {
        let axioms = self.axioms();
        let mut max_depth = 0;

        for axiom in axioms {
            let depth = self.compute_depth_from(axiom);
            max_depth = max_depth.max(depth);
        }

        max_depth
    }

    fn compute_depth_from(&self, start: PointId) -> usize {
        let mut max_depth = 0;
        let mut visited = HashSet::new();
        let mut stack = vec![(start, 0)];

        while let Some((point, depth)) = stack.pop() {
            if visited.contains(&point) {
                continue;
            }
            visited.insert(point);
            max_depth = max_depth.max(depth);

            if let Some(edges) = self.outgoing.get(&point) {
                for &line_id in edges {
                    if let Some(line) = self.lines.get(&line_id) {
                        stack.push((line.to, depth + 1));
                    }
                }
            }
        }

        max_depth
    }

    /// Export to DOT format for Graphviz
    pub fn to_dot(&self, graph_name: &str) -> String {
        let mut dot = format!("digraph {} {{\n", graph_name.replace(' ', "_"));
        dot.push_str("  rankdir=TB;\n");
        dot.push_str("  node [shape=circle, style=filled, fillcolor=lightblue];\n");

        // Add nodes
        for (id, point) in &self.points {
            dot.push_str(&format!("  p{} [label=\"{}\"];\n", id.0, point.label));
        }

        // Add edges
        for (_id, line) in &self.lines {
            dot.push_str(&format!(
                "  p{} -> p{} [label=\"{}\"];\n",
                line.from.0, line.to.0, line.label
            ));
        }

        dot.push_str("}\n");
        dot
    }
}

impl Default for ConstructionGraph {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::Point2;
    use sctt_core::Name;

    #[test]
    fn test_construction_creation() {
        let construction = Construction::new("Test Proof".to_string());
        assert_eq!(construction.name, "Test Proof");
        assert_eq!(construction.graph.points.len(), 0);
    }

    #[test]
    fn test_add_points() {
        let mut graph = ConstructionGraph::new();

        let p1 = Point::new(
            PointId(0),
            Point2::new(0.0, 0.0),
            Expr::Type(0),
            "A".to_string(),
        );
        let p2 = Point::new(
            PointId(0),
            Point2::new(1.0, 1.0),
            Expr::Type(0),
            "B".to_string(),
        );

        let id1 = graph.add_point(p1);
        let id2 = graph.add_point(p2);

        assert_eq!(graph.points.len(), 2);
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_add_line() {
        let mut graph = ConstructionGraph::new();

        let p1 = Point::new(
            PointId(0),
            Point2::new(0.0, 0.0),
            Expr::Type(0),
            "A".to_string(),
        );
        let p2 = Point::new(
            PointId(0),
            Point2::new(1.0, 1.0),
            Expr::Type(0),
            "B".to_string(),
        );

        let id1 = graph.add_point(p1);
        let id2 = graph.add_point(p2);

        let line = Line::new(
            LineId(0),
            id1,
            id2,
            Expr::Var(Name("f".to_string()), 0),
            "A→B".to_string(),
        );

        let line_id = graph.add_line(line).unwrap();
        assert_eq!(graph.lines.len(), 1);
    }

    #[test]
    fn test_cycle_detection() {
        let mut graph = ConstructionGraph::new();

        let p1 = Point::new(
            PointId(0),
            Point2::new(0.0, 0.0),
            Expr::Type(0),
            "A".to_string(),
        );
        let p2 = Point::new(
            PointId(0),
            Point2::new(1.0, 1.0),
            Expr::Type(0),
            "B".to_string(),
        );

        let id1 = graph.add_point(p1);
        let id2 = graph.add_point(p2);

        // Add A -> B
        let line1 = Line::new(
            LineId(0),
            id1,
            id2,
            Expr::Var(Name("f".to_string()), 0),
            "A→B".to_string(),
        );
        graph.add_line(line1).unwrap();

        // Try to add B -> A (should fail due to cycle)
        let line2 = Line::new(
            LineId(0),
            id2,
            id1,
            Expr::Var(Name("g".to_string()), 0),
            "B→A".to_string(),
        );

        assert!(graph.add_line(line2).is_err());
    }

    #[test]
    fn test_find_path() {
        let mut graph = ConstructionGraph::new();

        let p1 = Point::new(
            PointId(0),
            Point2::new(0.0, 0.0),
            Expr::Type(0),
            "A".to_string(),
        );
        let p2 = Point::new(
            PointId(0),
            Point2::new(1.0, 0.0),
            Expr::Type(0),
            "B".to_string(),
        );
        let p3 = Point::new(
            PointId(0),
            Point2::new(2.0, 0.0),
            Expr::Type(0),
            "C".to_string(),
        );

        let id1 = graph.add_point(p1);
        let id2 = graph.add_point(p2);
        let id3 = graph.add_point(p3);

        // Add A -> B -> C
        let line1 = Line::new(
            LineId(0),
            id1,
            id2,
            Expr::Var(Name("f".to_string()), 0),
            "f".to_string(),
        );
        let line2 = Line::new(
            LineId(0),
            id2,
            id3,
            Expr::Var(Name("g".to_string()), 0),
            "g".to_string(),
        );

        graph.add_line(line1).unwrap();
        graph.add_line(line2).unwrap();

        // Find path from A to C
        let path = graph.find_path(id1, id3);
        assert!(path.is_some());
        let path = path.unwrap();
        assert_eq!(path.lines.len(), 2);
    }

    #[test]
    fn test_axioms_and_theorems() {
        let mut graph = ConstructionGraph::new();

        let p1 = Point::new(
            PointId(0),
            Point2::new(0.0, 0.0),
            Expr::Type(0),
            "A".to_string(),
        );
        let p2 = Point::new(
            PointId(0),
            Point2::new(1.0, 1.0),
            Expr::Type(0),
            "B".to_string(),
        );

        let id1 = graph.add_point(p1);
        let id2 = graph.add_point(p2);

        let line = Line::new(
            LineId(0),
            id1,
            id2,
            Expr::Var(Name("f".to_string()), 0),
            "A→B".to_string(),
        );
        graph.add_line(line).unwrap();

        let axioms = graph.axioms();
        let theorems = graph.theorems();

        assert_eq!(axioms.len(), 1);
        assert_eq!(theorems.len(), 1);
        assert_eq!(axioms[0], id1);
        assert_eq!(theorems[0], id2);
    }
}
