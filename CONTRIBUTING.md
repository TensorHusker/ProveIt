# Contributing to ProveIt

Thank you for your interest in contributing to ProveIt! This document outlines the process for contributing to this geometric formal verification environment.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [How Can I Contribute?](#how-can-i-contribute)
- [Development Setup](#development-setup)
- [Development Workflow](#development-workflow)
- [Coding Standards](#coding-standards)
- [Testing Requirements](#testing-requirements)
- [Accessibility Requirements](#accessibility-requirements)
- [Documentation Standards](#documentation-standards)
- [Pull Request Process](#pull-request-process)
- [Mathematical Contributions](#mathematical-contributions)
- [Community](#community)

## Code of Conduct

This project adheres to a [Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code. Please report unacceptable behavior to the project maintainers.

## How Can I Contribute?

### Reporting Bugs

Before creating bug reports, please check existing issues. When creating a bug report, include:

- **Clear title and description**
- **Steps to reproduce** with minimal example
- **Expected vs actual behavior**
- **Environment details** (OS, Rust version, terminal)
- **Accessibility context** if relevant (screen reader, input method)
- **Screenshots or recordings** if applicable

Use the [bug report template](.github/ISSUE_TEMPLATE/bug_report.md) when filing an issue.

### Suggesting Enhancements

Enhancement suggestions are welcome! Please include:

- **Use case** and motivation
- **Proposed solution** with examples
- **Alternatives considered**
- **Impact on accessibility**
- **Mathematical implications** if relevant

### Contributing Code

Areas where contributions are particularly welcome:

1. **Formal methods** — implementations of new type theories
2. **Accessibility** — screen reader support, audio cues, haptic feedback
3. **Geometric primitives** — new construction operations
4. **Verification engine** — performance improvements, soundness proofs
5. **Documentation** — tutorials, examples, mathematical exposition
6. **Testing** — property-based tests, accessibility audits

## Development Setup

### Prerequisites

| Tool | Version | Purpose |
|------|---------|---------|
| Rust | 1.70+ | Core language |
| Cargo | Latest | Build system |
| Git | 2.20+ | Version control |
| XeLaTeX | TeXLive 2022+ | Documentation (optional) |

### Initial Setup

```bash
# Clone the repository
git clone https://github.com/TensorHusker/ProveIt.git
cd ProveIt

# Install development tools
rustup component add rustfmt clippy
cargo install cargo-watch cargo-tarpaulin cargo-mutants

# Build the project
cargo build

# Run the test suite
cargo test

# Build documentation
cargo doc --open
```

### Recommended Tools

- **rust-analyzer** — IDE integration with semantic understanding
- **cargo-watch** — auto-rebuild on file changes
- **cargo-tarpaulin** — code coverage analysis
- **cargo-mutants** — mutation testing
- **bacon** — background code checker

## Development Workflow

### Branching Strategy

We use a feature-branch workflow:

```
main                          # Stable, production-ready code
  ├── feature/audio-cues      # New feature branches
  ├── fix/proof-tree-display  # Bug fix branches
  ├── docs/api-reference      # Documentation branches
  └── refactor/verifier       # Refactoring branches
```

### Branch Naming

- `feature/` — new functionality
- `fix/` — bug fixes
- `docs/` — documentation changes
- `refactor/` — code restructuring without behavior change
- `perf/` — performance improvements
- `test/` — test additions or improvements
- `a11y/` — accessibility improvements

### Commit Messages

Follow conventional commit format:

```
<type>(<scope>): <subject>

<body>

<footer>
```

**Types**: `feat`, `fix`, `docs`, `style`, `refactor`, `perf`, `test`, `chore`, `a11y`

**Examples**:

```
feat(geometry): add Bezier curve primitive

Implement cubic Bezier curves as a new geometric primitive,
including the Constructible trait implementation and
accessibility metadata.

Closes #42
```

```
a11y(proof): improve screen reader announcements for proof steps

Adds detailed ARIA labels and live region updates when proof
steps are added or modified. Tested with NVDA and JAWS.
```

## Coding Standards

### Rust Style

We follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/) with these additions:

#### Required Tools

```bash
cargo fmt          # Format code (must pass)
cargo clippy       # Lint code (must pass with no warnings)
cargo test         # All tests must pass
```

#### Specific Conventions

**Imports** — Group and order imports:

```rust
// 1. Standard library
use std::collections::HashMap;
use std::sync::Arc;

// 2. External crates
use serde::{Deserialize, Serialize};
use thiserror::Error;

// 3. Internal modules
use crate::geometry::Point;
use crate::proof::Proof;
```

**Error Types** — Use `thiserror` for libraries:

```rust
#[derive(Error, Debug)]
pub enum VerificationError {
    #[error("type mismatch at step {step}: expected {expected}, found {found}")]
    TypeMismatch {
        step: usize,
        expected: String,
        found: String,
    },
}
```

**Type States** — Encode invariants in types:

```rust
pub struct Proof<S: ProofState> {
    /* fields */
    _state: PhantomData<S>,
}

pub struct Draft;
pub struct Verified;
pub struct Published;
```

#### Code Organization

- One public type per file when the type is large
- Module-level documentation explaining purpose
- Public APIs at the top, private helpers below
- Tests at the bottom in a `tests` submodule

### Anti-Patterns to Avoid

```rust
// AVOID: Panicking in library code
fn process(input: &str) -> Output {
    input.parse().unwrap()  // panics on bad input
}

// PREFER: Return Result
fn process(input: &str) -> Result<Output, ParseError> {
    input.parse()
}
```

```rust
// AVOID: Stringly-typed APIs
fn add_step(&mut self, kind: &str, content: &str);

// PREFER: Strongly-typed APIs
fn add_step(&mut self, step: ProofStep);
```

```rust
// AVOID: Color-only information
println!("\x1b[31mError: invalid proof\x1b[0m");

// PREFER: Multiple modalities
log::error!("Invalid proof at step {}: {}", step, reason);
audio_cue::error_tone();
```

## Testing Requirements

### Test Coverage

- **Unit tests** for all public functions
- **Integration tests** for cross-module workflows
- **Property-based tests** for mathematical invariants
- **Accessibility tests** for UI components
- **Doc tests** for code examples in documentation

### Coverage Targets

| Component | Minimum Coverage |
|-----------|------------------|
| Core verification | 95% |
| Geometric primitives | 90% |
| Public API | 90% |
| Accessibility utilities | 85% |
| Overall | 80% |

### Property-Based Testing

Use `proptest` for mathematical invariants:

```rust
proptest! {
    #[test]
    fn verification_is_deterministic(
        proof in any::<Proof>(),
    ) {
        let result1 = proof.verify();
        let result2 = proof.verify();
        prop_assert_eq!(result1.is_ok(), result2.is_ok());
    }

    #[test]
    fn composition_associates(
        f in morphism_strategy(),
        g in morphism_strategy(),
        h in morphism_strategy(),
    ) {
        prop_assume!(f.target() == g.source());
        prop_assume!(g.target() == h.source());

        let left = h.compose(&g.compose(&f)?)?;
        let right = h.compose(&g)?.compose(&f)?;
        prop_assert_eq!(left, right);
    }
}
```

### Mutation Testing

Run mutation tests on critical paths:

```bash
cargo mutants --in-place --baseline=skip
```

Aim for 80%+ mutation kill rate on verification engine code.

## Accessibility Requirements

Every contribution must consider accessibility. Use this checklist:

### Code Checklist

- [ ] All UI elements have accessible labels
- [ ] All operations are keyboard-reachable
- [ ] Audio cues provided for spatial operations
- [ ] No information conveyed by color alone
- [ ] Focus indicators are visible
- [ ] Error messages are descriptive and actionable
- [ ] Screen reader announcements are tested
- [ ] Cognitive load is minimized

### Documentation Checklist

- [ ] Mathematical concepts explained in prose
- [ ] Diagrams have text descriptions
- [ ] Code examples include explanation
- [ ] Heading hierarchy is logical
- [ ] Links have descriptive text
- [ ] Tables have headers and captions

See [docs/ACCESSIBILITY.md](docs/ACCESSIBILITY.md) for detailed guidelines.

## Documentation Standards

### Doc Comments

Every public item requires documentation:

```rust
/// Brief one-line summary.
///
/// Detailed explanation of behavior, including any side effects,
/// performance characteristics, and accessibility considerations.
///
/// # Mathematical Background
///
/// Explanation of the mathematical concepts (when relevant).
///
/// # Examples
///
/// ```rust
/// use proveit::Module::Type;
///
/// let example = Type::new();
/// assert!(example.is_valid());
/// ```
///
/// # Errors
///
/// Returns `Err(...)` when... (for fallible functions)
///
/// # Panics
///
/// Panics if... (avoid panics; document if unavoidable)
///
/// # Accessibility
///
/// This function emits audio cues when... (when relevant)
pub fn well_documented_function() -> Result<(), Error> {
    /* implementation */
}
```

### Inline Comments

- Explain **why**, not **what** (the code shows what)
- Reference issues or RFCs for non-obvious decisions
- Mark `TODO`, `FIXME`, `HACK` consistently
- Include accessibility notes where relevant

### Mathematical Documentation

For modules implementing mathematical concepts:

```rust
//! # Type Theory Implementation
//!
//! This module implements Martin-Löf dependent type theory with
//! the following judgmental forms:
//!
//! - Γ ⊢ A : Type (type formation)
//! - Γ ⊢ a : A (term introduction)
//! - Γ ⊢ a ≡ b : A (definitional equality)
//!
//! See `docs/tex/proveit-manual.tex` for the full mathematical
//! exposition, or [GLOSSARY.md](../docs/GLOSSARY.md) for definitions
//! of terms used throughout the codebase.
```

## Pull Request Process

### Before Submitting

1. **Sync with main**: rebase your branch on the latest main
2. **Run all checks**:
   ```bash
   cargo fmt --check
   cargo clippy -- -D warnings
   cargo test
   cargo doc --no-deps
   ```
3. **Update documentation** for any API changes
4. **Add tests** for new functionality
5. **Update CHANGELOG.md** under `[Unreleased]`
6. **Verify accessibility** with relevant tools

### PR Description

Use the [PR template](.github/PULL_REQUEST_TEMPLATE.md). Include:

- **Summary** of changes (what and why)
- **Related issues** (use "Closes #N" or "Refs #N")
- **Type of change** (feature, fix, docs, etc.)
- **Testing** performed
- **Accessibility impact** assessment
- **Breaking changes** noted clearly
- **Screenshots/recordings** for UI changes

### Review Process

1. **Automated checks** must pass (CI, formatting, lints)
2. **At least one approval** from a maintainer
3. **Mathematical review** for formal methods changes
4. **Accessibility review** for UI changes
5. **Documentation review** for API changes

Reviewers will evaluate:

- Correctness and mathematical soundness
- Code quality and conventions
- Test coverage and quality
- Documentation completeness
- Accessibility compliance
- Performance implications
- Security considerations

### After Merge

- Delete your feature branch
- Verify the change works on main
- Update related issues
- Help review related PRs

## Mathematical Contributions

Contributing to formal methods requires special care:

### Soundness

Any change to verification logic must:

1. Maintain soundness of the underlying calculus
2. Be accompanied by a soundness argument
3. Include property-based tests for invariants
4. Be reviewed by someone familiar with the theory

### New Formal Systems

To add a new formal system:

1. Open an RFC issue describing the system
2. Discuss design with maintainers
3. Implement the `FormalSystem` trait
4. Provide translation to/from existing systems
5. Include comprehensive tests
6. Document mathematical foundations
7. Add examples to the manual

### Citations

When implementing published results:

```rust
/// Implementation of the cumulative hierarchy as described in
/// the HoTT Book, Section 10.5.
///
/// # References
///
/// - The Univalent Foundations Program. *Homotopy Type Theory:
///   Univalent Foundations of Mathematics*. 2013.
///   §10.5, pp. 367-378.
pub fn cumulative_hierarchy() -> Universe { /* ... */ }
```

## Community

### Communication Channels

- **GitHub Issues** — bugs, features, questions about specific code
- **GitHub Discussions** — design discussions, broad questions
- **Pull Requests** — code review and contribution discussion

### Getting Help

- Check existing [issues](https://github.com/TensorHusker/ProveIt/issues)
- Browse [discussions](https://github.com/TensorHusker/ProveIt/discussions)
- Read the [documentation](docs/README.md)
- Consult the [glossary](docs/GLOSSARY.md)

### Recognition

All contributors are acknowledged in:

- The project README
- Release notes for the release including their contribution
- The `AUTHORS` file (for substantial contributions)

## License

By contributing, you agree that your contributions will be licensed under the [MIT License](LICENSE).

---

Thank you for contributing to making formal verification more accessible!
