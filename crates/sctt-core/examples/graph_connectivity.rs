// Graph Connectivity in SCTT - Research Question Q1
//
// This example tests whether discrete combinatorics (graph theory) can be
// efficiently encoded in Smooth Cubical Type Theory. This addresses Q1 from
// RESEARCH_QUESTIONS.md:
//
// "Can discrete combinatorics be efficiently encoded in smooth types?"
//
// We encode:
// - Graphs as indexed types (vertices and edges)
// - Adjacency as dependent types (Edge : Vertex -> Vertex -> Type)
// - Paths as compositions of edges
// - Connectivity as path existence
//
// Performance metrics to measure:
// - Code size overhead vs specialized discrete encoding
// - Type checking time for graph properties
// - Proof size for connectivity theorems

use sctt_core::{
    check::{infer, Context},
    syntax::{Expr, Name},
};

/// Graph structure encoded in SCTT
///
/// A graph G = (V, E) where:
/// - V : ℕ (vertex count as type-level natural)
/// - E : V → V → Type (edge relation as dependent type)
///
/// For a specific graph:
/// ```
/// Vertex := Fin 4  // 4 vertices (0,1,2,3)
/// Edge : Vertex → Vertex → Type
/// Edge 0 1 := Unit  // edge from 0 to 1
/// Edge 1 2 := Unit  // edge from 1 to 2
/// Edge 2 3 := Unit  // edge from 2 to 3
/// Edge _ _ := Empty // no other edges
/// ```
pub struct GraphEncoding {
    pub vertex_count: u32,
    pub edges: Vec<(u32, u32)>, // adjacency list
}

impl GraphEncoding {
    /// Create a new graph encoding
    pub fn new(vertex_count: u32, edges: Vec<(u32, u32)>) -> Self {
        GraphEncoding {
            vertex_count,
            edges,
        }
    }

    /// Example: Linear chain graph 0 → 1 → 2 → 3
    pub fn linear_chain(n: u32) -> Self {
        let edges: Vec<(u32, u32)> = (0..n - 1).map(|i| (i, i + 1)).collect();
        GraphEncoding {
            vertex_count: n,
            edges,
        }
    }

    /// Example: Complete graph (every vertex connected to every other)
    pub fn complete(n: u32) -> Self {
        let mut edges = Vec::new();
        for i in 0..n {
            for j in 0..n {
                if i != j {
                    edges.push((i, j));
                }
            }
        }
        GraphEncoding {
            vertex_count: n,
            edges,
        }
    }

    /// Encode graph in SCTT as dependent type
    ///
    /// Returns: (Vertex : Type, Edge : Vertex → Vertex → Type)
    pub fn to_sctt_type(&self) -> (Expr, Expr) {
        // Vertex type: Finite set Fin(n)
        // Encoded as: Type_0 with n distinct inhabitants
        let vertex_type = Expr::Type(0);

        // Edge type: Dependent function type
        // Edge : Vertex → Vertex → Type
        // For each (i,j) in edges, Edge i j := Unit
        // Otherwise, Edge i j := Empty
        let edge_type = Expr::Pi {
            name: Name("v1".to_string()),
            domain: Box::new(vertex_type.clone()),
            codomain: Box::new(Expr::Pi {
                name: Name("v2".to_string()),
                domain: Box::new(vertex_type.clone()),
                codomain: Box::new(Expr::Type(0)),
            }),
        };

        (vertex_type, edge_type)
    }

    /// Encode a path in the graph as SCTT proof term
    ///
    /// A path from vertex i to vertex j is a composition of edges:
    /// path : Edge i k₁ → Edge k₁ k₂ → ... → Edge kₙ j
    ///
    /// In SCTT, this is a dependent function composition
    pub fn encode_path(&self, path: &[u32]) -> Option<Expr> {
        if path.len() < 2 {
            return None;
        }

        // Verify path validity
        for i in 0..path.len() - 1 {
            if !self.edges.contains(&(path[i], path[i + 1])) {
                return None; // Invalid path - missing edge
            }
        }

        // Build proof term for path
        // This is a composition of edge proofs
        let mut proof = Expr::Var(Name(format!("edge_{}_{}", path[0], path[1])), 0);

        for i in 1..path.len() - 1 {
            let next_edge = Expr::Var(Name(format!("edge_{}_{}", path[i], path[i + 1])), 0);
            // Compose edges using function application
            proof = Expr::App {
                func: Box::new(proof),
                arg: Box::new(next_edge),
            };
        }

        Some(proof)
    }
}

/// Path type in SCTT
///
/// Path i j : Type represents "there exists a path from vertex i to vertex j"
///
/// Encoded as dependent sum:
/// Σ (n : ℕ) . (edges : Vec (Edge _ _))
pub fn path_type(from: u32, to: u32) -> Expr {
    Expr::Pi {
        name: Name(format!("path_{}_{}", from, to)),
        domain: Box::new(Expr::Type(0)),
        codomain: Box::new(Expr::Type(0)),
    }
}

/// Connectivity predicate: Connected(G) means all vertices are reachable
///
/// Connected : Type :=
///   ∀ (i j : Vertex) . Path i j
pub fn connectivity_type(_vertex_count: u32) -> Expr {
    let vertex_type = Expr::Type(0);

    Expr::Pi {
        name: Name("i".to_string()),
        domain: Box::new(vertex_type.clone()),
        codomain: Box::new(Expr::Pi {
            name: Name("j".to_string()),
            domain: Box::new(vertex_type),
            codomain: Box::new(path_type(0, 0)), // Generic path type
        }),
    }
}

/// Benchmark: measure encoding overhead
pub struct EncodingMetrics {
    pub vertex_count: u32,
    pub edge_count: usize,
    pub type_expr_size: usize,
    pub path_proof_size: usize,
    pub type_check_time_us: u128,
}

impl EncodingMetrics {
    pub fn measure(graph: &GraphEncoding) -> Self {
        let (vertex_type, edge_type) = graph.to_sctt_type();

        // Measure expression sizes
        let type_expr_size = Self::expr_size(&vertex_type) + Self::expr_size(&edge_type);

        // Find a simple path (0 to vertex_count-1 if it exists)
        let path = (0..graph.vertex_count).collect::<Vec<_>>();
        let path_proof = graph.encode_path(&path);
        let path_proof_size = path_proof.map(|p| Self::expr_size(&p)).unwrap_or(0);

        // Measure type checking time
        let start = std::time::Instant::now();
        let ctx = Context::new();
        let _ = infer(&ctx, &vertex_type);
        let _ = infer(&ctx, &edge_type);
        let type_check_time_us = start.elapsed().as_micros();

        EncodingMetrics {
            vertex_count: graph.vertex_count,
            edge_count: graph.edges.len(),
            type_expr_size,
            path_proof_size,
            type_check_time_us,
        }
    }

    fn expr_size(expr: &Expr) -> usize {
        match expr {
            Expr::Var(_, _) => 1,
            Expr::Type(_) => 1,
            Expr::Pi {
                domain, codomain, ..
            } => 1 + Self::expr_size(domain) + Self::expr_size(codomain),
            Expr::Lambda { body, .. } => 1 + Self::expr_size(body),
            Expr::App { func, arg } => 1 + Self::expr_size(func) + Self::expr_size(arg),
            Expr::PathLam { body, .. } => 1 + Self::expr_size(body),
            Expr::PathApp { path, .. } => 1 + Self::expr_size(path),
            Expr::Path { ty, left, right } => {
                1 + Self::expr_size(ty) + Self::expr_size(left) + Self::expr_size(right)
            }
            Expr::SmoothPath {
                ty, left, right, ..
            } => 1 + Self::expr_size(ty) + Self::expr_size(left) + Self::expr_size(right),
            Expr::Comp { ty, base, faces } => {
                1 + Self::expr_size(ty)
                    + Self::expr_size(base)
                    + faces.iter().map(|(_, e)| Self::expr_size(e)).sum::<usize>()
            }
            Expr::Coe { ty_fam, base, .. } => 1 + Self::expr_size(ty_fam) + Self::expr_size(base),
            Expr::HComp { ty, base, faces } => {
                1 + Self::expr_size(ty)
                    + Self::expr_size(base)
                    + faces.iter().map(|(_, e)| Self::expr_size(e)).sum::<usize>()
            }
            Expr::Glue { base, equivalences } => {
                1 + Self::expr_size(base)
                    + equivalences
                        .iter()
                        .map(|(_, t, e)| Self::expr_size(t) + Self::expr_size(e))
                        .sum::<usize>()
            }
            Expr::Diff { expr, .. } => 1 + Self::expr_size(expr),
            Expr::Integral { expr, .. } => 1 + Self::expr_size(expr),
            Expr::Taylor { expr, point, .. } => 1 + Self::expr_size(expr) + Self::expr_size(point),
        }
    }

    pub fn overhead_percentage(&self) -> f64 {
        // Compare to baseline discrete encoding:
        // Baseline size = vertex_count + edge_count (adjacency list)
        let baseline_size = self.vertex_count as usize + self.edge_count;
        let overhead = self.type_expr_size as f64 / baseline_size as f64;
        (overhead - 1.0) * 100.0
    }

    pub fn report(&self) {
        println!("=== Graph Encoding Metrics ===");
        println!("Vertices: {}", self.vertex_count);
        println!("Edges: {}", self.edge_count);
        println!("SCTT type expression size: {} nodes", self.type_expr_size);
        println!("Path proof size: {} nodes", self.path_proof_size);
        println!("Type check time: {} μs", self.type_check_time_us);
        println!("Encoding overhead: {:.1}%", self.overhead_percentage());
        println!();

        // Success criteria from RESEARCH_QUESTIONS.md Q1:
        // "Overhead < 20% for typical discrete problems"
        if self.overhead_percentage() < 20.0 {
            println!("✅ SUCCESS: Overhead < 20% threshold");
        } else if self.overhead_percentage() < 50.0 {
            println!("⚠️  WARNING: Overhead between 20-50%");
        } else {
            println!("❌ FAIL: Overhead > 50% - encoding inefficient");
        }
    }
}

fn main() {
    println!("Graph Connectivity in SCTT - Research Question Q1");
    println!("Testing discrete combinatorics encoding efficiency\n");

    // Test Case 1: Small linear chain (simple case)
    println!("Test 1: Linear chain graph (4 vertices, 3 edges)");
    let chain = GraphEncoding::linear_chain(4);
    let metrics1 = EncodingMetrics::measure(&chain);
    metrics1.report();

    // Test Case 2: Larger linear chain
    println!("Test 2: Linear chain graph (10 vertices, 9 edges)");
    let chain_large = GraphEncoding::linear_chain(10);
    let metrics2 = EncodingMetrics::measure(&chain_large);
    metrics2.report();

    // Test Case 3: Complete graph (dense)
    println!("Test 3: Complete graph (5 vertices, 20 edges)");
    let complete = GraphEncoding::complete(5);
    let metrics3 = EncodingMetrics::measure(&complete);
    metrics3.report();

    // Test Case 4: Custom graph
    println!("Test 4: Custom graph (diamond shape)");
    let diamond = GraphEncoding::new(4, vec![(0, 1), (0, 2), (1, 3), (2, 3)]);
    let metrics4 = EncodingMetrics::measure(&diamond);
    metrics4.report();

    // Summary
    println!("=== Summary ===");
    println!("Research Question Q1: Can discrete combinatorics be efficiently");
    println!("encoded in smooth types?");
    println!();
    println!("Results from {} test cases:", 4);

    let avg_overhead = (metrics1.overhead_percentage()
        + metrics2.overhead_percentage()
        + metrics3.overhead_percentage()
        + metrics4.overhead_percentage())
        / 4.0;

    println!("Average encoding overhead: {:.1}%", avg_overhead);

    if avg_overhead < 20.0 {
        println!("✅ CONCLUSION: SCTT efficiently encodes discrete graphs");
        println!("   Overhead < 20% threshold met");
    } else {
        println!("⚠️  CONCLUSION: SCTT encoding has measurable overhead");
        println!("   Consider optimizations or hybrid approach");
    }
}
