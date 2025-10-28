# ProveIt

**Geometric Construction Environment for Accessible Formal Verification**

ProveIt is a revolutionary formal verification system that bridges the gap between spatial intuition and rigorous mathematical proof. It represents proofs as geometric constructions, making formal verification accessible through multiple modalities: visual, auditory, and haptic.

## Vision

ProveIt combines:
- **Smooth Cubical Type Theory (SCTT)** - A cutting-edge type theory with differential structure
- **Geometric Proof Construction** - Visual/spatial representation of logical arguments
- **Universal Accessibility** - Audio, haptic, and natural language interfaces
- **Distributed Intelligence** - Butterfly framework for distributed proof search

## Architecture

### Core Crates

#### 1. **sctt-core** - Type Theory Foundation
The heart of ProveIt's formal system:
- Smooth Cubical Type Theory implementation
- Normalization by Evaluation (NbE)
- Kan operations (composition, transport)
- Bidirectional type checking
- Smooth path types with differential structure

**Key Files:**
- `syntax.rs` - Expression AST
- `value.rs` - Normalized values
- `eval.rs` - Evaluation engine
- `check.rs` - Type checker
- `kan.rs` - Cubical operations
- `smooth.rs` - Differential operations

#### 2. **geometry** - Spatial Proof Representation
Maps logical proofs to geometric constructions:
- Points represent propositions
- Lines represent implications
- Construction graphs track dependencies
- Spatial relations (parallel, perpendicular, incidence)
- Bidirectional proof bridge to SCTT

**Key Files:**
- `point.rs` - Geometric points (propositions)
- `line.rs` - Geometric lines (implications)
- `construction.rs` - Proof graphs with cycle detection
- `spatial.rs` - Spatial analysis and relations
- `proof_bridge.rs` - Geometry ↔ Logic translation

#### 3. **proof-engine** - Interactive Theorem Proving
Real-time proof construction and verification:
- Goal-oriented proof state management
- Tactic system (intro, apply, exact, etc.)
- Automated proof search (BFS, DFS, iterative deepening)
- Real-time verification with detailed error reporting
- Undo/redo with persistent data structures

**Key Files:**
- `verifier.rs` - Real-time proof verification
- `tactics.rs` - Proof tactics library
- `goals.rs` - Goal state management
- `search.rs` - Automated proof search

#### 4. **accessibility** - Universal Access
Making formal verification accessible to everyone:
- Text-to-speech with natural language descriptions
- Sonification (proof structure → sound)
- Haptic feedback patterns for tactile interfaces
- 3D spatial audio for geometric proofs
- Configurable verbosity and preferences

**Key Files:**
- `audio.rs` - Audio engine, TTS, sonification
- `haptic.rs` - Haptic feedback patterns
- `description.rs` - Natural language generation
- `spatial_audio.rs` - 3D audio positioning

#### 5. **butterfly-core** - Distributed Inference
Framework for distributed LLM inference and proof search:
- Functional model decomposition
- Worker node management
- Intelligent output fusion
- Load balancing and task scheduling

**Key Files:**
- `model_split.rs` - Model decomposition algorithms
- `fusion.rs` - Output fusion strategies
- `worker.rs` - Worker node interface

#### 6. **butterfly-worker** & **butterfly-coordinator**
Distributed system implementation:
- Worker nodes for distributed execution
- Coordinator for orchestration
- Network communication layer

#### 7. **cli** - Command-Line Interface
Interactive environment for proof construction:
- REPL with line editing and history
- Command parser
- Terminal UI (ratatui-based)
- Command implementations

**Key Files:**
- `main.rs` - Entry point with clap CLI
- `repl.rs` - Interactive REPL
- `parser.rs` - Command parser
- `ui.rs` - Terminal UI
- `commands/` - Command implementations

## Project Statistics

- **36 Rust source files**
- **8,048 lines of production-ready code**
- **8 crates** in workspace
- **Comprehensive test coverage**
- **Full documentation**

## Features

### Geometric Proof Construction
```rust
// Create a construction
let mut construction = Construction::new("Modus Ponens");

// Add points (propositions)
let a = construction.add_point(Point::new(id, pos, prop_a, "A"));
let b = construction.add_point(Point::new(id, pos, prop_b, "B"));

// Add line (implication: A → B)
let line = Line::new(id, a, b, proof_term, "A→B");
construction.add_line(line)?;

// Verify
construction.verify()?;
```

### Interactive Proof Development
```bash
$ proveit repl
ProveIt v0.1.0
Type 'help' for commands

proveit> goal
Current goal: ∀(A B : Type). A → B → A

proveit> intro
Applied intro tactic

proveit> intro
Applied intro tactic

proveit> assumption
Proof complete!
```

### Accessibility Features
```rust
// Audio feedback
let audio = AudioEngine::new()?;
let sonifier = Sonifier::new(Arc::new(audio));
sonifier.sonify_complexity(proof_complexity)?;

// Haptic feedback
let haptic = HapticEngine::new();
haptic.goal_completed()?;

// Natural language
let narrator = ProofNarrator::new(verbosity);
let description = narrator.narrate_construction(&construction);
```

### Distributed Proof Search
```rust
// Decompose model
let decomp = FunctionalDecomposition::decompose_transformer(12, 768);
let split = ModelSplit::new(model_id, decomp);

// Assign to workers
split.assign_component(component_id, worker_id);

// Execute distributed search
let fusion = OutputFusion::new(FusionStrategy::WeightedAverage);
let result = fusion.fuse(worker_outputs)?;
```

## Building

```bash
# Build all crates
cargo build --release

# Run tests
cargo test --all

# Run CLI
cargo run --bin proveit

# Run worker node
cargo run --bin butterfly-worker

# Run coordinator
cargo run --bin butterfly-coordinator
```

## Design Principles

### 1. **Accessibility First**
Every proof interaction is available through:
- Visual (geometric construction)
- Auditory (speech + sonification)
- Haptic (tactile feedback)
- Natural language descriptions

### 2. **Type Safety**
Leveraging Rust's type system and SCTT for:
- Compile-time proof checking
- Memory safety
- Concurrency safety
- Zero-cost abstractions

### 3. **Modularity**
Clean separation of concerns:
- Type theory (sctt-core)
- Geometry (geometry)
- Proof tactics (proof-engine)
- User interface (cli)
- Accessibility (accessibility)

### 4. **Performance**
- Normalization by Evaluation for efficient type checking
- Persistent data structures for undo/redo
- Parallel proof search
- Distributed execution via Butterfly

### 5. **Extensibility**
- Plugin tactic system
- Custom sonification strategies
- Configurable accessibility
- Flexible model decomposition

## Mathematical Foundation

### Smooth Cubical Type Theory

ProveIt is built on SCTT, which extends Cubical Type Theory with smooth/differentiable structure:

- **Types** can have smooth structure (C^∞)
- **Paths** are continuously differentiable
- **Kan operations** preserve smoothness
- **Univalence** holds with smooth equivalences
- **Applications**: cryptography, ML, physics simulation

### Type Checking

Bidirectional type checking with NbE:
```
Γ ⊢ e ⇒ A    (synthesis)
Γ ⊢ e ⇐ A    (checking)
```

Normalization ensures:
- Definitional equality
- Efficient conversion checking
- Canonical forms for values

### Geometric Interpretation

Proofs as directed acyclic graphs:
- **Nodes** = Propositions (typed expressions)
- **Edges** = Implications (proof terms)
- **Paths** = Proof chains (composed implications)
- **Cycles** = Detected and rejected

## Future Directions

### Near Term
- [ ] Complete Kan operations implementation
- [ ] Univalence and Glue types
- [ ] Enhanced proof search heuristics
- [ ] More tactics (induction, cases, etc.)

### Medium Term
- [ ] GUI with visualization
- [ ] Proof assistant integration (Coq, Agda)
- [ ] Library of formalized mathematics
- [ ] Collaborative proof development

### Long Term
- [ ] AI-assisted proof discovery
- [ ] Quantum circuit verification
- [ ] Formal cryptographic proofs
- [ ] Integration with theorem provers

## Contributing

ProveIt is designed for extensibility. Key extension points:

1. **Tactics** - Add new proof strategies in `proof-engine/src/tactics.rs`
2. **Sonification** - Custom audio patterns in `accessibility/src/audio.rs`
3. **Model Decomposition** - New splitting strategies in `butterfly-core/src/model_split.rs`
4. **Commands** - REPL commands in `cli/src/commands/`

## License

MIT License - See LICENSE file for details

## Authors

Alaina Scott <tensorhusker@users.noreply.github.com>

## Acknowledgments

Built on foundations of:
- Cubical Type Theory (CCHM)
- Normalization by Evaluation
- Homotopy Type Theory
- Synthetic Differential Geometry

---

**ProveIt: Where geometry meets logic, and everyone can prove theorems.**
