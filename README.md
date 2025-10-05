# ProveIt

Geometric construction environment for accessible formal verification. Build proofs, verify systems, and compose neural networks through spatial reasoning. Supports multiple formal methods (type theory, category theory, homotopy) with real-time verification. Designed accessibility-first for blind, neurodivergent, and post-injury users.

## 🎯 Goal

Solve ARC-AGI-2 challenges through geometric reasoning and formal verification, making advanced mathematics accessible to everyone.

## ✨ Features

### Core Technology
- **O(1) Neural Type Checker**: Constant-time type verification using hash-based lookups and pre-computed type signatures
- **Spatial Graph Engine**: Graph-based geometric reasoning with transformations (translation, rotation, scaling, reflection)
- **Geometric Transformations**: Full 3D transformation pipeline with matrix operations
- **GPU Acceleration**: WebGPU-based compute for parallel verification and transformations
- **WASM Support**: Run ProveIt in the browser with full feature parity

### Formal Methods
- **Topological Type Theory (TTT)**: Spatial reasoning about types
- **Spatial/Cubical Type Theory (SCTT)**: Cubical structures for higher-dimensional reasoning
- **Homotopy Type Theory**: Path types, homotopies, and higher-dimensional proofs
- **Universe Hierarchy**: Proper stratification of types across universe levels

### Accessibility First
- **Spatial Audio**: 3D audio positioning for non-visual navigation
- **Screen Reader Support**: Full ARIA compatibility and semantic descriptions
- **Customizable Layouts**: List, grid, 2D, and 3D spatial layouts
- **High Contrast Modes**: Visual accessibility for low vision users
- **Reduce Motion**: Vestibular-friendly interface options
- **Audio Cues**: Sonification of mathematical operations
- **Haptic Feedback**: Tactile response for touchscreen users

### Cross-Platform
- **Rust Core**: Fast, safe, and memory-efficient implementation
- **SwiftUI (macOS/iOS)**: Native Apple platform support
- **WASM (Web)**: Browser-based access without installation

## 🏗️ Architecture

```
ProveIt/
├── crates/
│   ├── proveit-core/           # Core types and traits
│   ├── proveit-type-checker/   # O(1) neural type checker
│   ├── proveit-spatial/        # Spatial graph engine
│   ├── proveit-formal/         # Formal methods (TTT, SCTT, HoTT)
│   ├── proveit-accessibility/  # Accessibility features
│   └── proveit-gpu/            # GPU acceleration
├── swift/
│   └── ProveItSwift/           # SwiftUI bindings
└── wasm/                       # WASM build configuration
```

## 🚀 Getting Started

### Prerequisites

- Rust 1.70+ (for core development)
- Swift 5.9+ (for macOS/iOS development)
- wasm-pack (for WASM builds)

### Building from Source

```bash
# Clone the repository
git clone https://github.com/TensorHusker/ProveIt.git
cd ProveIt

# Build Rust core
cargo build --release

# Run tests
cargo test

# Build for WASM
cd wasm
wasm-pack build --target web ../crates/proveit-core

# Build Swift bindings (macOS/iOS only)
cd swift/ProveItSwift
swift build
```

## 📚 Usage Examples

### Rust Core

```rust
use proveit_core::{Position, SpatialObject, ProofId, Verifiable};
use proveit_type_checker::{NeuralTypeChecker, TypeSignature, TypeCategory};
use proveit_spatial::{SpatialGraph, SpatialNode, Transformation};
use nalgebra::Vector3;

// Create a type checker
let mut checker = NeuralTypeChecker::new();

// Register a type
let sig = TypeSignature::new(TypeCategory::Base, "Point".to_string());
checker.register_type(sig);

// Create a spatial graph
let mut graph = SpatialGraph::new();

let node = SpatialNode {
    id: ProofId(1),
    position: Position::new(0.0, 0.0, 0.0),
    label: "Origin".to_string(),
};

graph.add_node(node);

// Apply a transformation
let transform = Transformation::Translation(Vector3::new(1.0, 0.0, 0.0));
graph.transform_node(ProofId(1), &transform).unwrap();
```

### Swift/SwiftUI

```swift
import SwiftUI
import ProveItSwift

struct ContentView: View {
    @StateObject var core = ProveItCore(
        accessibilitySettings: .blindPreset
    )
    
    var body: some View {
        ProveItView(core: core)
    }
}
```

### Accessibility

```rust
use proveit_accessibility::{AccessibilitySettings, SpatialAudio};
use proveit_core::Position;

// Configure for blind users
let settings = AccessibilitySettings::blind_preset();

// Set up spatial audio
let mut audio = SpatialAudio::new();
audio.set_listener(Position::origin(), (0.0, 0.0, -1.0));

// Calculate audio parameters for a sound source
let sound_pos = Position::new(10.0, 0.0, 0.0);
let params = audio.calculate_audio_params(&sound_pos);

println!("Volume: {}, Pan: {}", params.volume, params.pan);
```

### Formal Methods

```rust
use proveit_formal::{Path, Homotopy, FormalType, UniverseLevel};
use proveit_core::ProofId;

// Create a path between two points
let path1 = Path::new(ProofId(1), ProofId(2));
let path2 = Path::new(ProofId(1), ProofId(2));

// Create a homotopy (2-path) between paths
let homotopy = Homotopy::new(path1, path2).unwrap();

// Define a function type
let point_type = FormalType::Base("Point".to_string());
let arrow_type = FormalType::Arrow(
    Box::new(point_type.clone()),
    Box::new(point_type.clone())
);
```

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run tests for a specific crate
cargo test -p proveit-type-checker
```

## 🤝 Contributing

Contributions are welcome! This project aims to make formal verification accessible to everyone. Please see CONTRIBUTING.md for guidelines.

## 📄 License

MIT License - See [LICENSE](LICENSE) for details.

## 🙏 Acknowledgments

Built with accessibility in mind for:
- Blind and visually impaired users
- Neurodivergent individuals
- People recovering from brain injuries
- Anyone who learns better through spatial and auditory reasoning

## 🔗 Resources

- [Homotopy Type Theory Book](https://homotopytypetheory.org/book/)
- [Cubical Type Theory](https://arxiv.org/abs/1611.02108)
- [ARC-AGI Challenge](https://arcprize.org/)
- [WebGPU Specification](https://www.w3.org/TR/webgpu/)
- [Web Accessibility Guidelines](https://www.w3.org/WAI/WCAG21/quickref/)

## 🗺️ Roadmap

- [x] Core Rust implementation
- [x] O(1) type checker
- [x] Spatial graph engine
- [x] Formal methods foundation
- [x] Accessibility module
- [x] GPU acceleration structure
- [x] SwiftUI bindings skeleton
- [ ] Full FFI between Rust and Swift
- [ ] Complete WASM implementation
- [ ] Interactive proof construction UI
- [ ] ARC-AGI challenge solver
- [ ] Neural network composition
- [ ] Real-time collaborative proofs
