# Claude Code Instructions for ProveIt

## Project Overview

ProveIt is a geometric construction environment for accessible formal verification. The project enables users to build proofs, verify systems, and compose neural networks through spatial reasoning.

### Core Principles

1. **Accessibility-First Design**: All features must be accessible to blind, neurodivergent, and post-injury users
2. **Multiple Formal Methods**: Support for type theory, category theory, and homotopy type theory
3. **Real-Time Verification**: Proofs should be validated as they are constructed
4. **Spatial Reasoning**: Use geometric constructions as the primary interaction paradigm

## Technology Stack

- **Language**: Rust
- **License**: MIT

## Code Style and Conventions

### Rust Guidelines

- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `rustfmt` for formatting (default configuration)
- Use `clippy` for linting with `#![warn(clippy::all)]`
- Prefer `Result<T, E>` over panicking for recoverable errors
- Document all public APIs with doc comments (`///`)
- Use meaningful variable and function names that reflect mathematical concepts

### Naming Conventions

```rust
// Types and traits: PascalCase
struct ProofStep { ... }
trait Verifiable { ... }

// Functions and variables: snake_case
fn verify_construction(proof: &Proof) -> Result<(), VerificationError> { ... }

// Constants: SCREAMING_SNAKE_CASE
const MAX_PROOF_DEPTH: usize = 1000;

// Modules: snake_case
mod geometric_primitives;
```

### Error Handling

- Define domain-specific error types using `thiserror` or similar
- Provide helpful error messages that guide users toward solutions
- Include context about what operation failed and why

### Documentation

- Every public item should have documentation
- Include examples in doc comments where appropriate
- Explain mathematical concepts for contributors unfamiliar with formal methods

## Accessibility Requirements

When implementing features or reviewing code, ensure:

1. **Screen Reader Compatibility**: All UI elements must have proper labels and descriptions
2. **Keyboard Navigation**: Full functionality must be accessible via keyboard
3. **Audio Feedback**: Provide audio cues for geometric operations
4. **Haptic Support**: Design for potential haptic feedback integration
5. **Cognitive Load**: Minimize complexity and provide clear, step-by-step guidance
6. **Color Independence**: Never rely solely on color to convey information

## Architecture Guidelines

### Module Organization

```
src/
  lib.rs              # Library root
  proof/              # Proof construction and verification
  geometry/           # Geometric primitives and operations
  formal_methods/     # Type theory, category theory, homotopy
  accessibility/      # Accessibility utilities and interfaces
  verification/       # Real-time verification engine
```

### Design Patterns

- Use the **Builder pattern** for complex proof construction
- Use **Traits** to abstract over different formal methods
- Prefer **Composition over inheritance**
- Use **Type states** to enforce valid proof construction sequences

## Testing Requirements

### Test Coverage

- Unit tests for all public functions
- Integration tests for proof verification workflows
- Property-based tests for mathematical invariants (using `proptest` or `quickcheck`)
- Accessibility tests for UI components

### Test Organization

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction_validity() {
        // Test implementation
    }
}
```

## Pull Request Review Criteria

When reviewing PRs, Claude should evaluate:

1. **Correctness**: Does the code correctly implement the intended functionality?
2. **Accessibility**: Does it maintain or improve accessibility standards?
3. **Mathematical Soundness**: Are formal verification aspects correctly implemented?
4. **Performance**: Are there any obvious performance issues?
5. **Test Coverage**: Are new features adequately tested?
6. **Documentation**: Are changes properly documented?
7. **Code Style**: Does it follow project conventions?

## Security Considerations

- Validate all external inputs to the verification engine
- Avoid unsafe Rust unless absolutely necessary (and document why)
- Be cautious with serialization/deserialization of proofs
- Do not execute arbitrary code from proof files

## Common Tasks

### Adding a New Geometric Primitive

1. Define the type in `src/geometry/primitives.rs`
2. Implement the `Constructible` trait
3. Add accessibility descriptions
4. Write unit tests
5. Document with examples

### Implementing a New Formal Method

1. Create a new module under `src/formal_methods/`
2. Implement the `FormalSystem` trait
3. Add translation layers to/from other supported methods
4. Include comprehensive tests
5. Document the mathematical foundations

### Debugging Verification Failures

1. Enable verbose logging with `RUST_LOG=debug`
2. Use the proof trace functionality to identify the failing step
3. Check type constraints and construction validity
4. Verify accessibility metadata is preserved

## Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Homotopy Type Theory Book](https://homotopytypetheory.org/book/)
- [WCAG 2.1 Guidelines](https://www.w3.org/TR/WCAG21/)

## Getting Help

- Check existing issues for similar problems
- Tag issues with appropriate labels (accessibility, verification, geometry, etc.)
- Provide minimal reproducible examples when reporting bugs
