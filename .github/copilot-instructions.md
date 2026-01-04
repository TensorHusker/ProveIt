# Copilot Instructions for ProveIt

## Project Overview

ProveIt is a geometric construction environment for accessible formal verification. It enables users to build proofs, verify systems, and compose neural networks through spatial reasoning. The project supports multiple formal methods including type theory, category theory, and homotopy type theory with real-time verification.

**Key Mission:** Designed accessibility-first for blind, neurodivergent, and post-injury users.

## Language and Framework

- **Primary Language:** Rust
- **License:** MIT

## Code Style Guidelines

### Rust Conventions

- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `rustfmt` for code formatting
- Use `clippy` for linting with all warnings enabled
- Prefer descriptive variable and function names
- Use `snake_case` for functions and variables, `PascalCase` for types and traits
- Document public APIs with rustdoc comments (`///`)

### Error Handling

- Use `Result` and `Option` types appropriately
- Provide meaningful error messages
- Consider using `thiserror` or `anyhow` for error handling

### Testing

- Write unit tests in the same file as the code being tested (using `#[cfg(test)]` modules)
- Write integration tests in the `tests/` directory
- Use descriptive test names that explain what is being tested
- Include both positive and negative test cases

## Accessibility-First Design Principles

This project prioritizes accessibility. When contributing:

- Ensure all user-facing features are accessible to screen readers
- Provide text-based alternatives for visual representations
- Support keyboard navigation for all interactive elements
- Use clear, descriptive labels and error messages
- Consider cognitive load and provide simple, clear interfaces
- Test with assistive technologies when possible
- Document accessibility features and limitations

## Build and Development

```bash
# Build the project
cargo build

# Run tests
cargo test

# Run with optimizations
cargo build --release

# Check formatting
cargo fmt --check

# Run linter
cargo clippy
```

## Contribution Guidelines

- Keep commits small and focused
- Write clear commit messages
- Add tests for new functionality
- Update documentation for API changes
- Ensure all tests pass before submitting
- Follow the existing code patterns and conventions
