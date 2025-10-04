//! Basic ProveIt Example
//!
//! Demonstrates:
//! - Creating a spatial graph with geometric objects
//! - Type checking with O(1) lookup
//! - Applying geometric transformations
//! - Using accessibility features

use proveit_accessibility::{AccessibilitySettings, SpatialAudio};
use proveit_core::{Position, ProofId, Verifiable};
use proveit_formal::{FormalType, Path, UniverseLevel};
use proveit_spatial::{SpatialEdge, SpatialGraph, SpatialNode, Transformation};
use proveit_type_checker::{NeuralTypeChecker, TypeCategory, TypeSignature};
use nalgebra::Vector3;

fn main() {
    println!("=== ProveIt Basic Example ===\n");

    // 1. Create a type checker with O(1) lookup
    println!("1. Setting up O(1) Neural Type Checker");
    let mut type_checker = NeuralTypeChecker::new();

    // Register some basic types
    let point_sig = TypeSignature::new(TypeCategory::Base, "Point".to_string());
    let line_sig = TypeSignature::new(TypeCategory::Base, "Line".to_string());

    type_checker.register_type(point_sig.clone());
    type_checker.register_type(line_sig.clone());

    println!("   Registered types: Point, Line");
    println!(
        "   Point type verified: {}",
        type_checker.verify_type(point_sig.hash).unwrap()
    );

    // 2. Create a spatial graph
    println!("\n2. Creating Spatial Graph");
    let mut graph = SpatialGraph::new();

    // Add nodes
    let origin = SpatialNode {
        id: ProofId(1),
        position: Position::origin(),
        label: "Origin Point".to_string(),
    };

    let point_a = SpatialNode {
        id: ProofId(2),
        position: Position::new(1.0, 0.0, 0.0),
        label: "Point A".to_string(),
    };

    let point_b = SpatialNode {
        id: ProofId(3),
        position: Position::new(0.0, 1.0, 0.0),
        label: "Point B".to_string(),
    };

    graph.add_node(origin);
    graph.add_node(point_a);
    graph.add_node(point_b);

    println!("   {}", graph.describe());

    // Add edges with transformations
    let edge_to_a = SpatialEdge {
        transformation: Transformation::Translation(Vector3::new(1.0, 0.0, 0.0)),
        label: "Move to A".to_string(),
    };

    let edge_to_b = SpatialEdge {
        transformation: Transformation::Translation(Vector3::new(0.0, 1.0, 0.0)),
        label: "Move to B".to_string(),
    };

    graph.add_edge(ProofId(1), ProofId(2), edge_to_a).unwrap();
    graph.add_edge(ProofId(1), ProofId(3), edge_to_b).unwrap();

    println!("   Added edges with transformations");

    // 3. Apply geometric transformation
    println!("\n3. Applying Geometric Transformation");
    let rotation = Transformation::Rotation {
        axis: Vector3::new(0.0, 0.0, 1.0),
        angle: std::f64::consts::PI / 4.0, // 45 degrees
    };

    println!("   {}", rotation.describe());
    graph.transform_node(ProofId(2), &rotation).unwrap();
    
    if let Some(node) = graph.get_node(ProofId(2)) {
        println!("   Point A after rotation: {}", node.position);
    }

    // 4. Formal methods - create a path
    println!("\n4. Formal Methods - Homotopy Type Theory");
    let path = Path::new(ProofId(1), ProofId(2));
    println!("   Created: {}", path.describe());
    println!("   Path verified: {}", path.verify().unwrap());

    // 5. Demonstrate accessibility features
    println!("\n5. Accessibility Features");
    
    // Configure for blind users
    let settings = AccessibilitySettings::blind_preset();
    println!("   {}", settings.describe());

    // Set up spatial audio
    let mut audio = SpatialAudio::new();
    audio.set_listener(Position::origin(), (0.0, 0.0, -1.0));

    // Calculate audio parameters for Point A
    if let Some(node) = graph.get_node(ProofId(2)) {
        let params = audio.calculate_audio_params(&node.position);
        println!(
            "   Audio for {}: volume={:.2}, pan={:.2}, distance={:.2}",
            node.label, params.volume, params.pan, params.distance
        );
    }

    // 6. Type compatibility check
    println!("\n6. Type Compatibility Check");
    let compatible = type_checker
        .check_compatible(&point_sig, &line_sig)
        .unwrap();
    println!("   Point and Line compatible: {}", compatible);

    // 7. Universe hierarchy
    println!("\n7. Universe Hierarchy");
    let u0 = UniverseLevel::zero();
    let u1 = u0.succ();
    println!("   Universe 0: {:?}", u0);
    println!("   Universe 1: {:?}", u1);

    let type_type = FormalType::Universe(u0);
    println!("   Type of types: {}", type_type.describe());
    println!("   Universe level: {:?}", type_type.universe_level());

    println!("\n=== Example Complete ===");
}
