# ProveIt Tutorials

Learning materials for ProveIt, organized by experience level. All tutorials are designed to be accessible — every visual concept has audio and text equivalents.

## Learning Paths

### Path 1: First Steps with Formal Verification

For users new to formal methods.

1. [What is Formal Verification?](01-what-is-formal-verification.md)
2. [Your First Proof](02-your-first-proof.md)
3. [Understanding Types](03-understanding-types.md)
4. [Geometric Constructions as Proofs](04-geometric-as-proof.md)

### Path 2: Type Theory Foundations

For users wanting to understand the mathematical core.

1. [Types and Terms](type-theory/01-types-and-terms.md)
2. [Functions and Application](type-theory/02-functions.md)
3. [Dependent Types](type-theory/03-dependent-types.md)
4. [Identity Types](type-theory/04-identity-types.md)

### Path 3: Accessibility-First Development

For users learning to use ProveIt with assistive technology.

1. [Setting Up with a Screen Reader](accessibility/01-screen-reader-setup.md)
2. [Audio Navigation](accessibility/02-audio-navigation.md)
3. [Keyboard-Only Workflows](accessibility/03-keyboard-only.md)
4. [Customizing Your Environment](accessibility/04-customization.md)

### Path 4: Advanced Topics

For experienced users.

1. [Homotopy Type Theory](advanced/01-hott-introduction.md)
2. [Category Theory in ProveIt](advanced/02-category-theory.md)
3. [Custom Inference Rules](advanced/03-custom-rules.md)
4. [Performance Optimization](advanced/04-performance.md)

## Tutorial Format

Each tutorial follows a consistent structure:

- **Prerequisites**: What you should know first
- **Goal**: What you'll learn
- **Estimated time**: How long it takes
- **Content**: The tutorial itself
- **Exercises**: Practice problems
- **Solutions**: With explanation
- **Further reading**: Where to go next

## Format Conventions

### Code Blocks

```rust
// Annotated code with explanations
let point = Point::new(3.0, 4.0); // Creates a 2D point
```

### Mathematical Notation

Mathematical content appears in three forms:

- **LaTeX** for visual readers: `$\Pi_{x:A} B(x)$`
- **Plain text** for screen readers: "the dependent product over x of type A of B of x"
- **ASCII** for plaintext contexts: `Pi (x : A), B(x)`

### Audio Descriptions

Visual elements include audio descriptions:

> **Visual**: A point at coordinates (3, 4) is rendered as a small filled circle.
>
> **Audio cue**: A short tone, panned to the right (positive x), at medium pitch (positive y).
>
> **Description**: "Point A at three, four"

## Accessibility Notes

### Screen Reader Use

All tutorials are optimized for screen readers:

- Headings establish clear navigation structure
- Lists are properly formatted
- Tables have headers
- Images have descriptive alt text
- Code blocks are announced clearly

### Cognitive Accessibility

- Each tutorial has a clear, single learning objective
- Concepts are introduced one at a time
- Plenty of examples
- Frequent recap and review
- Optional advanced sections clearly marked

### Multiple Modalities

Where applicable, tutorials provide:

- Visual diagrams (with alt text)
- Audio explanations (with transcripts)
- Interactive examples (keyboard-accessible)
- Static text (always available)

## Contributing Tutorials

We welcome tutorial contributions! See [CONTRIBUTING.md](../../CONTRIBUTING.md) for general guidelines, and these tutorial-specific notes:

### Good Tutorials

- Have a clear, single learning objective
- Build on stated prerequisites
- Include working, tested examples
- Provide exercises with solutions
- Link to related material
- Use plain language alongside technical terms
- Are accessible by construction

### Tutorial Review

Tutorial PRs are reviewed for:

- Technical accuracy
- Pedagogical clarity
- Accessibility (especially for screen readers)
- Code examples actually work
- Exercises are appropriately difficult

## Status of Tutorial Content

This is the initial structure. Many tutorials are placeholders to be filled in as ProveIt develops:

| Tutorial | Status |
|----------|--------|
| What is Formal Verification? | Draft |
| Your First Proof | Pending implementation |
| Understanding Types | Draft |
| Other tutorials | Planned |

Want to write one? Open an issue to coordinate, then submit a PR.

## Quick Reference

### Common Questions

- **"What's the difference between `=` and `≡`?"** See [Glossary: Definitional vs. Propositional Equality](../GLOSSARY.md#definitional-equality)
- **"How do I navigate proofs with a screen reader?"** See [Audio Navigation](accessibility/02-audio-navigation.md)
- **"What formal system should I use?"** See [Choosing a Formal System](choosing-formal-system.md)

### Need Help?

- [Glossary](../GLOSSARY.md) for terminology
- [GitHub Discussions](https://github.com/TensorHusker/ProveIt/discussions) for questions
- [Issue tracker](https://github.com/TensorHusker/ProveIt/issues) for bugs in tutorials
