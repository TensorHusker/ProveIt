# ProveIt Architecture

This document describes the high-level architecture of ProveIt, including major components, their interactions, and the design rationale.

For mathematical foundations, see [docs/tex/proveit-manual.tex](docs/tex/proveit-manual.tex).
For development conventions, see [CONTRIBUTING.md](CONTRIBUTING.md).

## Table of Contents

- [System Overview](#system-overview)
- [Architectural Principles](#architectural-principles)
- [Module Structure](#module-structure)
- [Core Abstractions](#core-abstractions)
- [Data Flow](#data-flow)
- [Verification Pipeline](#verification-pipeline)
- [Accessibility Layer](#accessibility-layer)
- [Persistence](#persistence)
- [Extensibility](#extensibility)
- [Performance Considerations](#performance-considerations)
- [Open Questions](#open-questions)

## System Overview

ProveIt is structured as a layered system with clear separation between:

```
┌─────────────────────────────────────────────────┐
│              User Interface Layer               │
│  (geometric construction UI, screen reader API) │
└──────────────┬──────────────────────────────────┘
               │
┌──────────────▼──────────────────────────────────┐
│            Accessibility Adapter                │
│   (multi-modal: visual, auditory, haptic)       │
└──────────────┬──────────────────────────────────┘
               │
┌──────────────▼──────────────────────────────────┐
│           Proof Construction Engine             │
│  (proof state, step application, history)      │
└──────────────┬──────────────────────────────────┘
               │
┌──────────────▼──────────────────────────────────┐
│         Real-Time Verification Engine           │
│   (incremental verification, error reporting)   │
└──────────────┬──────────────────────────────────┘
               │
┌──────────────▼──────────────────────────────────┐
│            Formal Methods Backend               │
│  (type theory, category theory, HoTT)           │
└─────────────────────────────────────────────────┘
```

Each layer depends only on the layers below it. Cross-layer communication happens through well-defined trait boundaries.

## Architectural Principles

### 1. Accessibility as Architecture

Accessibility is not implemented as a layer on top of a "standard" application. Instead, the core abstractions include accessibility as a first-class concern. Every constructible object knows how to describe itself, emit audio cues, and support haptic feedback.

**Implication**: A backend without a graphical UI must still produce equivalent audio/haptic output.

### 2. Soundness Above All

The verification engine is the trust boundary. Code outside this engine cannot make verification claims. The verifier is:

- **Small**: Easier to audit and verify
- **Pure**: No side effects in core verification logic
- **Deterministic**: Same input always produces same result
- **Testable**: Property-based tests for soundness

### 3. Multiple Formal Methods, One Interface

The system supports multiple formal methods through a unified `FormalSystem` trait. Translations between systems are explicit and well-typed.

```
   Type Theory          Category Theory          HoTT
        │                      │                   │
        └──────────────────────┼───────────────────┘
                               │
                          FormalSystem
                          (common trait)
```

### 4. Geometric Reasoning as Primary

Geometric constructions are not a visualization of abstract proofs—they are the proofs. The geometric structure carries logical content, not just presentation.

### 5. Type-Driven Design

The Rust type system enforces invariants at compile time:

- **Type states** for proof construction phases
- **Phantom types** for formal system parameters
- **Sealed traits** for trusted implementations
- **Newtypes** for semantic distinction

## Module Structure

```
src/
├── lib.rs                      # Library root, re-exports
│
├── proof/                      # Proof construction
│   ├── mod.rs                  # Public API
│   ├── builder.rs              # ProofBuilder with type states
│   ├── step.rs                 # ProofStep enumeration
│   ├── context.rs              # Proof context (Γ)
│   ├── history.rs              # Undo/redo, serialization
│   └── error.rs                # ProofError types
│
├── geometry/                   # Geometric primitives
│   ├── mod.rs                  # Public API
│   ├── primitives.rs           # Point, Line, Circle, etc.
│   ├── operations.rs           # Construction operations
│   ├── transforms.rs           # Affine, projective transforms
│   ├── intersections.rs        # Intersection computations
│   └── invariants.rs           # Geometric invariants
│
├── formal_methods/             # Formal system implementations
│   ├── mod.rs                  # FormalSystem trait
│   ├── type_theory/            # Martin-Löf type theory
│   │   ├── mod.rs
│   │   ├── judgment.rs         # Typing judgments
│   │   ├── inference.rs        # Inference rules
│   │   └── normalization.rs    # NbE evaluator
│   ├── category_theory/        # Category theory
│   │   ├── mod.rs
│   │   ├── category.rs         # Category trait
│   │   ├── functor.rs          # Functors
│   │   └── natural.rs          # Natural transformations
│   ├── hott/                   # Homotopy type theory
│   │   ├── mod.rs
│   │   ├── identity.rs         # Identity types
│   │   ├── univalence.rs       # Univalence axiom
│   │   └── hit.rs              # Higher inductive types
│   └── translation/            # Inter-system translation
│       ├── mod.rs
│       ├── tt_to_ct.rs
│       └── ct_to_hott.rs
│
├── verification/               # Verification engine
│   ├── mod.rs                  # Public API
│   ├── engine.rs               # Core verification loop
│   ├── incremental.rs          # Incremental verification
│   ├── trace.rs                # Verification traces
│   ├── error.rs                # VerificationError types
│   └── soundness.rs            # Soundness checks
│
├── accessibility/              # Accessibility infrastructure
│   ├── mod.rs                  # Accessible trait
│   ├── audio.rs                # Audio cue generation
│   ├── haptic.rs               # Haptic patterns
│   ├── description.rs          # Screen reader descriptions
│   ├── keyboard.rs             # Keyboard navigation
│   └── focus.rs                # Focus management
│
└── ui/                         # Optional UI layer
    ├── mod.rs
    ├── canvas.rs               # Geometric canvas
    ├── inspector.rs            # Proof inspector
    └── shortcuts.rs            # Keyboard shortcuts
```

## Core Abstractions

### The `Verifiable` Trait

```rust
/// A type whose validity can be formally verified.
pub trait Verifiable {
    /// Witness produced by successful verification.
    type Witness: Clone + Send + Sync;

    /// Errors produced by verification failure.
    type Error: std::error::Error;

    /// Verify this object, producing a witness on success.
    fn verify(&self) -> Result<Self::Witness, Self::Error>;

    /// Quick validity check (may be conservative).
    fn is_valid(&self) -> bool {
        self.verify().is_ok()
    }
}
```

### The `Constructible` Trait

```rust
/// A type built up through a sequence of construction steps.
pub trait Constructible: Verifiable {
    /// The kind of step that builds this object.
    type Step: Clone + Send + Sync;

    /// Apply a construction step.
    fn apply_step(&mut self, step: Self::Step) -> Result<(), Self::Error>;

    /// History of construction steps.
    fn history(&self) -> &[Self::Step];

    /// Replay history from scratch (useful for verification).
    fn replay(history: &[Self::Step]) -> Result<Self, Self::Error>
    where
        Self: Sized;
}
```

### The `FormalSystem` Trait

```rust
/// A formal system providing types, terms, and inference rules.
pub trait FormalSystem {
    type Type: Clone + Eq + std::fmt::Debug;
    type Term: Clone + Eq + std::fmt::Debug;
    type Context: Clone + Default;
    type Rule: Clone + std::fmt::Debug;

    /// Check that a term has the given type in context.
    fn type_check(
        ctx: &Self::Context,
        term: &Self::Term,
        ty: &Self::Type,
    ) -> Result<(), TypeError>;

    /// Apply an inference rule.
    fn apply_rule(
        ctx: &Self::Context,
        rule: &Self::Rule,
        premises: &[Self::Term],
    ) -> Result<Self::Term, RuleError>;
}
```

### The `Accessible` Trait

```rust
/// A type with accessibility metadata.
pub trait Accessible {
    /// Screen-reader description.
    fn describe(&self) -> AccessibleDescription;

    /// Optional audio cue for this element.
    fn audio_cue(&self) -> Option<AudioCue> {
        None
    }

    /// Optional haptic pattern.
    fn haptic_pattern(&self) -> Option<HapticPattern> {
        None
    }

    /// Keyboard shortcut for primary action, if any.
    fn primary_shortcut(&self) -> Option<KeyBinding> {
        None
    }
}
```

## Data Flow

A typical proof construction flows through the system:

```
User Input
    │
    ▼
[Keyboard/Mouse/Touch Event]
    │
    ▼
Accessibility Adapter ──────► [Audio Cue Emitted]
    │                       └► [Screen Reader Update]
    ▼
Proof Builder
    │
    ▼
[ProofStep Created]
    │
    ▼
Verification Engine ────────► [Incremental Verify]
    │
    ▼
[Verification Result]
    │
    ▼
Proof State Update ─────────► [UI Update]
                            └► [Audio Feedback]
                            └► [Haptic Feedback]
```

### Event Sourcing

Proofs are stored as event logs (sequences of `ProofStep`s) rather than as final states. This provides:

- **Reproducibility**: Same steps always produce same proof
- **Auditability**: Full history of construction
- **Undo/Redo**: Trivial via event truncation/replay
- **Merging**: Conflict resolution at step granularity

## Verification Pipeline

The verification engine processes a proof through several stages:

```
   ProofStep
       │
       ▼
   ┌─────────────────┐
   │  Lex & Parse    │  (if from text)
   └────────┬────────┘
            │
            ▼
   ┌─────────────────┐
   │  Well-Formedness│  Syntactic checks
   └────────┬────────┘
            │
            ▼
   ┌─────────────────┐
   │  Type Checking  │  Type-theoretic validation
   └────────┬────────┘
            │
            ▼
   ┌─────────────────┐
   │  Rule Validation│  Verify inference rules apply
   └────────┬────────┘
            │
            ▼
   ┌─────────────────┐
   │  Soundness Chk  │  Final consistency check
   └────────┬────────┘
            │
            ▼
       Witness
```

### Incremental Verification

Rather than re-verifying entire proofs on each change, the engine:

1. **Identifies affected subgoals** based on the changed step
2. **Invalidates dependent witnesses** in the proof DAG
3. **Re-verifies only invalidated portions** in topological order
4. **Caches stable witnesses** for unchanged portions

This makes real-time verification feasible for large proofs.

### Error Recovery

When verification fails, the engine produces:

- **Precise error location** (proof step, term position)
- **Expected vs. actual** types or terms
- **Suggested fixes** when possible
- **Accessible error descriptions** for screen readers
- **Audio error cues** distinct from informational sounds

## Accessibility Layer

### Multi-Modal Output

Each verifiable object emits information through multiple channels:

| Channel | Content | Use Case |
|---------|---------|----------|
| Visual | Geometric rendering | Sighted users |
| Screen reader | Structured description | Blind users |
| Audio cue | Spatial/operation tone | Spatial awareness |
| Haptic | Vibration pattern | Confirmation, location |
| Status | Text/braille output | Multiple platforms |

### Spatial Audio

Geometric positions map to audio properties:

- **X coordinate** → Stereo pan (-1.0 to +1.0)
- **Y coordinate** → Frequency (200Hz to 2000Hz)
- **Z coordinate** (3D) → Reverb depth
- **Object type** → Timbre/instrument
- **Verification state** → Modulation (clean = valid, distorted = invalid)

### Cognitive Load Management

The system minimizes cognitive load through:

- **Progressive disclosure**: Simple operations are simple to access
- **Consistent shortcuts**: Standard keys for standard actions
- **Predictable behavior**: No surprising side effects
- **Clear feedback**: Every action produces immediate, distinct feedback
- **Configurable verbosity**: Users control information density

## Persistence

### Proof Files

Proofs are serialized as structured documents:

```
proof.pi               # Main proof file
├── metadata           # Title, author, date, dependencies
├── context            # Type universe, imported definitions
├── steps[]            # Sequence of ProofSteps
└── annotations        # Optional human-readable notes
```

Format characteristics:

- **Plain text**: Human-readable, version-control friendly
- **Deterministic**: Same proof always serializes identically
- **Self-contained**: All dependencies explicit
- **Streamable**: Can be parsed incrementally

### Configuration

User preferences are stored separately:

```
~/.config/proveit/
├── settings.toml      # General settings
├── shortcuts.toml     # Keyboard customization
├── audio.toml         # Audio cue configuration
└── themes/            # Visual themes
```

## Extensibility

### Adding a New Geometric Primitive

1. Define the type in `src/geometry/primitives.rs`
2. Implement `Constructible`
3. Add to the operations enumeration
4. Implement `Accessible`
5. Add audio cue and haptic patterns
6. Write unit and property tests
7. Document with examples

### Adding a New Formal System

1. Create module in `src/formal_methods/<name>/`
2. Implement `FormalSystem` trait
3. Implement translations to/from existing systems
4. Add inference rules
5. Provide property tests for soundness
6. Add to system selection UI
7. Document mathematical foundations

### Adding an Accessibility Modality

1. Define new trait in `src/accessibility/`
2. Add default implementation
3. Implement for core types
4. Add configuration options
5. Add testing utilities
6. Document the modality

## Performance Considerations

### Critical Paths

These paths are performance-critical:

1. **Real-time verification**: Must complete within ~16ms for UI responsiveness
2. **Audio cue generation**: Must complete within audio frame latency (~10ms)
3. **Proof rendering**: Must support proofs with 10,000+ steps
4. **Incremental verification**: Must scale sublinearly with proof size

### Optimization Strategies

- **Caching**: Verified subterms cached by hash
- **Parallelism**: Independent subgoals verified concurrently
- **Lazy evaluation**: Only compute what's displayed/queried
- **Memoization**: Pure verification functions memoized
- **Specialization**: Hot paths use specialized data structures

### Memory Management

- **Arena allocation** for short-lived proof state
- **Reference counting** for shared subterms
- **Persistent data structures** for undo/redo
- **Streaming** for large proof files

## Open Questions

These architectural decisions are not yet settled:

### 1. Distributed Verification

Should ProveIt support distributing verification across multiple machines? This would help with very large proofs but complicates the trust model.

### 2. WebAssembly Target

A browser-based version would expand reach but requires careful consideration of:
- Performance vs. native
- Accessibility API access
- File system integration
- Cryptographic operations

### 3. AI-Assisted Proving

Integration with theorem-proving AI assistants raises questions about:
- Trust boundary (AI suggestions are not verifications)
- Reproducibility of AI-generated proofs
- Accessibility of AI suggestions

### 4. Real-Time Collaboration

Multi-user editing of proofs requires:
- Conflict-free replicated data types (CRDTs)?
- Server-mediated synchronization?
- Verification consistency across users?

### 5. Mobile Support

Touch and gesture-based construction is appealing but:
- How does it work with screen readers?
- What's the haptic feedback model?
- How do we handle small screens for complex proofs?

## References

- [Mathematical foundations](docs/tex/proveit-manual.tex) — Comprehensive theoretical background
- [API design rationale](docs/api-design.md) — Why specific traits look the way they do
- [Performance benchmarks](docs/benchmarks.md) — Empirical measurements

## Changelog

This document evolves with the project. See [CHANGELOG.md](CHANGELOG.md) for revisions.
