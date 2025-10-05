# ProveIt Architecture

## Overview

ProveIt is a geometric construction environment for accessible formal verification, built with a modular architecture that separates concerns while maintaining tight integration.

## System Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                        Frontend Layer                        │
├──────────────────┬──────────────────┬──────────────────────┤
│   SwiftUI        │    Web (WASM)    │    CLI               │
│  (macOS/iOS)     │                  │                      │
└────────┬─────────┴────────┬─────────┴──────────┬───────────┘
         │                  │                    │
         └──────────────────┼────────────────────┘
                            │
         ┌──────────────────┴────────────────────┐
         │         FFI/WASM Bindings              │
         └──────────────────┬────────────────────┘
                            │
┌─────────────────────────────────────────────────────────────┐
│                      Rust Core Layer                         │
├──────────────────┬──────────────────┬──────────────────────┤
│  Type Checker    │   Spatial Graph  │  Formal Methods      │
│  (O(1) lookup)   │   Engine         │  (TTT/SCTT/HoTT)     │
├──────────────────┼──────────────────┼──────────────────────┤
│  Accessibility   │   GPU Accel.     │  Core Types          │
│  (Audio/SR)      │   (WebGPU)       │  (Position/Proof)    │
└──────────────────┴──────────────────┴──────────────────────┘
```

## Core Components

### 1. proveit-core

**Purpose**: Foundation types and traits for the entire system

**Key Types**:
- `Position`: 3D spatial position
- `ProofId`: Unique identifier for proof objects
- `Verifiable`: Trait for objects that can be verified
- `ProveItError`: Error types for the system
- `SpatialObject`: Geometric objects with verification

**Design Philosophy**:
- Minimal dependencies
- Type-safe by design
- Serializable for cross-platform use

### 2. proveit-type-checker

**Purpose**: O(1) neural type checker for real-time verification

**Key Features**:
- **Hash-based lookup**: Pre-computed type signatures enable O(1) type checking
- **Type categories**: Base, Function, Product, Sum, Universe, Path types
- **Compatibility matrix**: Pre-computed rules for instant compatibility checks

**Algorithm**:
```rust
// Type checking is O(1) due to hash table lookup
fn check_compatible(sig1, sig2) -> bool {
    if sig1.hash == sig2.hash { return true; }  // O(1)
    compatibility_matrix[(sig1.category, sig2.category)]  // O(1)
}
```

**Use Cases**:
- Interactive proof construction (real-time feedback)
- Type inference during geometric transformations
- Verification of neural network compositions

### 3. proveit-spatial

**Purpose**: Spatial graph engine with geometric transformations

**Key Components**:
- **SpatialGraph**: Directed graph of geometric objects
- **Transformations**: Translation, Rotation, Scaling, Reflection, Composite
- **Matrix operations**: 4x4 homogeneous transformation matrices

**Transformation Pipeline**:
```
Input Position → Transformation Matrix → Matrix Multiply → Output Position
```

**Graph Operations**:
- Add/remove nodes (O(1))
- Add edges with transformations (O(1))
- Query neighbors (O(degree))
- Apply transformations (O(1) per node)

### 4. proveit-formal

**Purpose**: Formal methods foundation (TTT, SCTT, HoTT)

**Type Theory Support**:

1. **Homotopy Type Theory**:
   - Paths (identity types)
   - Homotopies (2-paths)
   - Higher-dimensional structures

2. **Cubical Type Theory**:
   - n-dimensional cubes
   - Face structures
   - Composition operations

3. **Universe Hierarchy**:
   - Proper stratification (Type₀ : Type₁ : Type₂ : ...)
   - Cumulative universes
   - Level polymorphism

**Type System**:
```
FormalType:
  - Base types (geometric primitives)
  - Arrow types (A → B)
  - Dependent functions (Π(x:A).B)
  - Dependent sums (Σ(x:A).B)
  - Path types (a =_A b)
  - Universe types (Type_i)
```

### 5. proveit-accessibility

**Purpose**: Accessibility-first design for all users

**Features**:

1. **Spatial Audio**:
   - 3D audio positioning
   - Distance-based volume
   - Directional panning
   - Rolloff calculation

2. **Screen Reader Support**:
   - Semantic descriptions
   - Announcement priorities
   - Navigation aids

3. **Customizable Layouts**:
   - List (linear navigation)
   - Grid (2D grid)
   - Spatial 2D/3D (free positioning)
   - Custom layouts

4. **Accessibility Presets**:
   - Blind users (spatial audio + screen reader)
   - Visually impaired (high contrast + large fonts)
   - Neurodivergent (reduced motion + simplified UI)

**Audio Parameters**:
```
volume = 1 - (distance / max_distance)^rolloff
pan = clamp(atan2(dx, dz) / π, -1, 1)
```

### 6. proveit-gpu

**Purpose**: GPU acceleration for parallel computations

**Capabilities**:
- Batch geometric transformations
- Parallel type checking
- Verification pipeline acceleration

**Technology Stack**:
- WebGPU (wgpu) for cross-platform GPU access
- Compute shaders for parallel operations
- Buffer management for efficient data transfer

**Future Work**:
- Complete GPU pipeline implementation
- Shader optimization
- Multi-GPU support

## Cross-Platform Support

### Rust Core

- **Platform**: All platforms with Rust support
- **Build**: `cargo build`
- **Test**: `cargo test`

### Swift/SwiftUI

- **Platform**: macOS 13+, iOS 16+
- **Build**: `swift build`
- **Integration**: FFI bindings to Rust core

### WASM

- **Platform**: Modern web browsers
- **Build**: `wasm-pack build`
- **Features**: Full feature parity with native

## Data Flow

### Proof Construction Flow

```
User Input → Type Checker (O(1) validation)
          → Spatial Graph (add/transform nodes)
          → Formal Methods (construct proofs)
          → Accessibility (audio/visual feedback)
          → GPU (accelerated computation)
          → Verification Result
```

### Accessibility Flow

```
Geometric Object → Position
                → Spatial Audio (3D positioning)
                → Screen Reader (description)
                → Visual Rendering (with accessibility settings)
```

## Performance Characteristics

| Operation | Complexity | Notes |
|-----------|-----------|-------|
| Type checking | O(1) | Hash-based lookup |
| Add graph node | O(1) | HashMap insert |
| Transform node | O(1) | Matrix multiplication |
| Type registration | O(1) | Hash table insert |
| Path creation | O(1) | Structure allocation |
| Audio calculation | O(1) | Mathematical formula |

## Memory Layout

### Position (24 bytes)
```
struct Position {
    x: f64,  // 8 bytes
    y: f64,  // 8 bytes
    z: f64,  // 8 bytes
}
```

### TypeSignature (variable)
```
struct TypeSignature {
    category: TypeCategory,  // 16 bytes
    hash: u64,              // 8 bytes
    description: String,     // 24 bytes + heap
}
```

## Testing Strategy

1. **Unit Tests**: Each crate has comprehensive unit tests
2. **Integration Tests**: Cross-crate functionality
3. **Property Tests**: Formal verification properties
4. **Accessibility Tests**: Screen reader compatibility

## Security Considerations

- No unsafe code in core logic
- Type safety guaranteed by Rust
- Bounds checking on all array access
- No dynamic memory allocation in hot paths

## Future Directions

1. **Complete WASM Integration**: Full browser support
2. **Neural Network Composition**: Geometric neural architecture
3. **ARC-AGI Solver**: Challenge-specific implementations
4. **Collaborative Proofs**: Real-time multi-user
5. **Advanced GPU Pipelines**: Complete acceleration
6. **Mobile UI**: Touch-first interface
7. **Voice Control**: Hands-free operation

## Dependencies

### Core Dependencies
- `serde`: Serialization/deserialization
- `thiserror`: Error handling
- `nalgebra`: Linear algebra
- `petgraph`: Graph data structures

### Platform Dependencies
- `wgpu`: GPU acceleration
- `bytemuck`: Safe byte casting

### Optional Dependencies
- `cpal`: Audio output (future)
- `symphonia`: Audio decoding (future)
- `wasm-bindgen`: WASM support

## Build Targets

- `x86_64-unknown-linux-gnu`
- `x86_64-apple-darwin`
- `aarch64-apple-darwin`
- `x86_64-pc-windows-msvc`
- `wasm32-unknown-unknown`

## Accessibility Standards Compliance

- WCAG 2.1 Level AA
- ARIA 1.2
- Section 508
- EN 301 549

## License

MIT License - See LICENSE file
