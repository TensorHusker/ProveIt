# ProveIt

Geometric construction environment for accessible formal verification. Build proofs, verify systems, and compose neural networks through spatial reasoning.

## Vision

ProveIt reimagines formal verification through the lens of geometric construction. Instead of wrestling with abstract syntax trees and type annotations, users build proofs by constructing geometric objects that embody logical relationships.

## Key Features

- **Multiple Formal Methods**: Supports type theory, category theory, and homotopy type theory
- **Real-Time Verification**: Validates proofs as they are constructed
- **Spatial Reasoning**: Geometric constructions as the primary interaction paradigm
- **Neural Network Composition**: Verify and compose neural network architectures geometrically

## Accessibility

ProveIt is designed accessibility-first for:

- **Blind users**: Full screen reader support with audio feedback for geometric operations
- **Neurodivergent users**: Clear, step-by-step guidance with minimal cognitive load
- **Post-injury users**: Flexible input methods and adjustable interaction speeds

### Accessibility Principles

- Screen reader compatible with descriptive labels
- Complete keyboard navigation
- Audio cues for spatial relationships
- Designed for haptic feedback integration
- Color-independent information display

## Getting Started

### Prerequisites

- Rust 1.70 or later
- Cargo package manager

### Installation

```bash
git clone https://github.com/TensorHusker/ProveIt.git
cd ProveIt
cargo build
```

### Running Tests

```bash
cargo test
```

### Running with Debug Logging

```bash
RUST_LOG=debug cargo run
```

## Project Structure

```
ProveIt/
  src/
    lib.rs              # Library root
    proof/              # Proof construction and verification
    geometry/           # Geometric primitives and operations
    formal_methods/     # Type theory, category theory, homotopy
    accessibility/      # Accessibility utilities and interfaces
    verification/       # Real-time verification engine
  tests/                # Integration tests
  examples/             # Example proofs and constructions
```

## Contributing

We welcome contributions! Please read our contributing guidelines before submitting PRs.

### Development Workflow

1. Fork the repository
2. Create a feature branch
3. Make your changes with tests
4. Ensure `cargo fmt` and `cargo clippy` pass
5. Submit a pull request

### Code Review

All pull requests are reviewed for:
- Correctness and mathematical soundness
- Accessibility compliance
- Test coverage
- Documentation quality

See [CLAUDE.md](CLAUDE.md) for detailed coding conventions and review criteria.

## Documentation

- [CLAUDE.md](CLAUDE.md) - Development guidelines and coding conventions
- [API Documentation](https://docs.rs/proveit) (coming soon)

## License

MIT License - see [LICENSE](LICENSE) for details.

## Acknowledgments

ProveIt builds on ideas from:
- Homotopy Type Theory
- Interactive Theorem Provers (Coq, Lean, Agda)
- Accessible design research
- Geometric algebra

## Contact

- Issues: [GitHub Issues](https://github.com/TensorHusker/ProveIt/issues)
- Discussions: [GitHub Discussions](https://github.com/TensorHusker/ProveIt/discussions)
