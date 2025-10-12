# Contributing to ProveIt

Thank you for your interest in contributing to ProveIt! This project aims to make formal verification accessible to everyone, especially blind, visually impaired, neurodivergent, and brain injury users.

## Code of Conduct

- Be respectful and inclusive
- Consider accessibility in all contributions
- Test with screen readers when possible
- Document accessibility features clearly

## Getting Started

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Run tests (`cargo test`)
5. Commit your changes (`git commit -m 'Add amazing feature'`)
6. Push to the branch (`git push origin feature/amazing-feature`)
7. Open a Pull Request

## Development Setup

### Rust Development

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build project
cargo build

# Run tests
cargo test

# Format code
cargo fmt

# Run linter
cargo clippy
```

### Swift Development (macOS/iOS)

```bash
cd swift/ProveItSwift
swift build
swift test
```

## Project Structure

- `crates/proveit-core` - Core types and traits
- `crates/proveit-type-checker` - O(1) neural type checker
- `crates/proveit-spatial` - Spatial graph engine
- `crates/proveit-formal` - Formal methods (TTT, SCTT, HoTT)
- `crates/proveit-accessibility` - Accessibility features
- `crates/proveit-gpu` - GPU acceleration
- `swift/ProveItSwift` - SwiftUI bindings

## Accessibility Guidelines

### When adding new features:

1. **Provide text descriptions** - Every visual element should have a screen reader description
2. **Support keyboard navigation** - All interactions should work without a mouse
3. **Consider spatial audio** - Audio cues help blind users navigate 3D spaces
4. **Test with accessibility tools** - Use VoiceOver, NVDA, or JAWS
5. **Support high contrast** - Ensure UI works in high contrast mode
6. **Reduce motion option** - Animations should be optional
7. **Font scaling** - UI should work at 2x font size

### Accessibility Testing Checklist

- [ ] Works with screen readers (VoiceOver/NVDA/JAWS)
- [ ] Keyboard navigation functional
- [ ] High contrast mode supported
- [ ] Reduced motion option works
- [ ] Font scaling up to 2x tested
- [ ] Spatial audio descriptions provided
- [ ] Documentation includes accessibility notes

## Code Style

- Follow Rust conventions (use `cargo fmt` and `cargo clippy`)
- Write comprehensive tests for new features
- Document public APIs with examples
- Keep functions focused and small
- Use meaningful variable names

## Testing

```bash
# Run all tests
cargo test

# Run specific crate tests
cargo test -p proveit-type-checker

# Run with verbose output
cargo test -- --nocapture
```

## Documentation

- Add doc comments to all public items
- Include usage examples in documentation
- Update README.md when adding major features
- Document accessibility features explicitly

## Pull Request Process

1. Update README.md with details of major changes
2. Update tests to cover new functionality
3. Ensure all tests pass (`cargo test`)
4. Run formatter (`cargo fmt`)
5. Run linter (`cargo clippy`)
6. Add accessibility documentation
7. Request review from maintainers

## Areas for Contribution

### High Priority
- FFI bindings between Rust and Swift
- WASM implementation completion
- Interactive proof construction UI
- Spatial audio implementation
- GPU compute shader implementation

### Medium Priority
- More formal method examples
- Performance optimizations
- Additional accessibility presets
- Documentation improvements
- Tutorial content

### Good First Issues
- Add more tests
- Improve documentation
- Fix typos
- Add usage examples
- Accessibility testing

## Questions?

Open an issue or start a discussion! We're happy to help.

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
