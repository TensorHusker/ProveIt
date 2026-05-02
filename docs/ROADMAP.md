# ProveIt Roadmap

This document outlines the planned evolution of ProveIt. It is a living document that reflects current priorities and may change based on community feedback, contributions, and discoveries.

## Vision

ProveIt aims to become the most accessible formal verification environment, making rigorous mathematical reasoning available to people who have historically been excluded from formal methods due to:

- Vision differences
- Motor limitations
- Cognitive differences
- Lack of advanced mathematical training

We measure success not by feature count, but by who can use ProveIt productively.

## Current Status

**Phase**: Pre-Alpha (0.0.x)

We are in the design and foundational implementation phase. Core abstractions are being established, and infrastructure is being built.

## Roadmap Overview

```
0.0.x  Pre-Alpha   ──── Design & Foundation
   │
0.1.x  Alpha       ──── Core Functionality
   │
0.5.x  Beta        ──── Feature Complete
   │
1.0.0  Stable      ──── Production Ready
   │
1.x    Mature      ──── Ecosystem Growth
   │
2.0    Next Gen    ──── Major Architectural Evolution
```

## Phase 1: Foundation (0.0.x → 0.1.0)

**Status**: In Progress
**Target**: Q3 2026

### Goals

Establish the core architecture and prove the design works for a small set of features.

### Milestones

#### M1.1: Core Abstractions
- [x] Project setup and tooling
- [x] Documentation infrastructure
- [ ] Core trait definitions: `Verifiable`, `Constructible`, `Accessible`, `FormalSystem`
- [ ] Type-state proof builder skeleton
- [ ] Error type hierarchy

#### M1.2: First Geometric Primitives
- [ ] `Point` with full trait implementations
- [ ] `Line` with construction operations
- [ ] `Circle` with center and radius
- [ ] Basic intersection computations
- [ ] Property-based tests

#### M1.3: Minimal Type Theory
- [ ] Simply-typed lambda calculus implementation
- [ ] Type checking algorithm
- [ ] Normalization
- [ ] Basic inference rules
- [ ] Test suite for soundness

#### M1.4: Verification Engine v0
- [ ] Synchronous verification
- [ ] Error reporting with location
- [ ] Verification trace
- [ ] Initial benchmarks

#### M1.5: Accessibility v0
- [ ] Screen reader announcements for primitives
- [ ] Basic audio cue framework
- [ ] Keyboard navigation skeleton
- [ ] ARIA labels and live regions

### Definition of Done for 0.1.0

- Can construct and verify a simple geometric proof
- Screen reader can describe the proof structure
- Audio cues confirm operations
- All public APIs documented
- 80% test coverage on core engine
- Tutorial for first proof completes successfully

## Phase 2: Core Features (0.1.x → 0.5.0)

**Status**: Planned
**Target**: 2026-2027

### Goals

Expand functionality to support meaningful formal verification work, with full multi-modal accessibility.

### Major Features

#### Formal Methods Expansion

- [ ] Dependent type theory (full Martin-Löf)
- [ ] Inductive type definitions
- [ ] Category theory backend
- [ ] Translation: type theory ↔ category theory
- [ ] HoTT prototype with identity types

#### Geometric Operations

- [ ] Polygon and polyhedron primitives
- [ ] Affine transformations
- [ ] Curves (Bezier, splines)
- [ ] Constraint solving
- [ ] Geometric algebra operations

#### Verification

- [ ] Incremental verification
- [ ] Parallel verification of independent goals
- [ ] Caching of verified subterms
- [ ] Performance: 10,000-step proofs in real-time

#### Accessibility

- [ ] Spatial audio for 3D geometric awareness
- [ ] Haptic feedback API
- [ ] Switch access support
- [ ] Voice control integration
- [ ] Multiple color schemes including color-blind support
- [ ] Screen reader optimization for math content

#### Tooling

- [ ] Command-line interface
- [ ] Library API for embedding
- [ ] LSP server for IDE integration
- [ ] Proof file format specification
- [ ] Import/export with other proof assistants

### Definition of Done for 0.5.0

- Real-world proofs (e.g., basic theorems of geometry, algebra) verifiable
- All accessibility modalities functional
- Documentation covers common workflows
- Performance acceptable for typical use
- Test coverage > 85%

## Phase 3: Production Readiness (0.5.x → 1.0.0)

**Status**: Future
**Target**: 2027-2028

### Goals

Polish, stability, and the API guarantees needed for production use.

### Focus Areas

#### Stability

- [ ] API freeze
- [ ] Comprehensive integration tests
- [ ] Soundness audit (external review)
- [ ] Security audit
- [ ] Performance benchmarks meeting targets
- [ ] No known critical bugs

#### Documentation

- [ ] Complete API documentation
- [ ] User manual (XeTeX) — finished
- [ ] Tutorial series — comprehensive
- [ ] Video tutorials with captions
- [ ] Audio descriptions of visual content
- [ ] Translations to additional languages

#### Tooling Maturity

- [ ] Stable file format with migration tools
- [ ] Cross-platform installers
- [ ] Package manager distribution
- [ ] Continuous deployment pipeline
- [ ] Community proof library

#### Accessibility Certification

- [ ] WCAG 2.1 AA conformance verified
- [ ] User testing with diverse disabled users
- [ ] Published accessibility statement
- [ ] Third-party accessibility audit

### Definition of Done for 1.0.0

- Production-quality stability
- API stable with semver guarantees
- Accessibility independently verified
- Performance meets all targets
- Comprehensive documentation
- Active user community

## Phase 4: Ecosystem (1.x)

**Status**: Future
**Target**: 2028+

### Goals

Foster a thriving ecosystem of proofs, tools, and integrations.

### Possible Directions

#### Library Building

- [ ] Standard library of common proofs
- [ ] Mathematics curriculum proofs
- [ ] Formalization of computer science fundamentals
- [ ] Integration with mathlib (or similar)

#### Tool Integration

- [ ] LaTeX export with semantic markup
- [ ] Integration with Lean, Coq, Agda (import/export)
- [ ] Web-based collaborative editor
- [ ] Mobile applications
- [ ] Educational platform integration

#### Community

- [ ] Annual ProveIt conference
- [ ] Regular webinars and workshops
- [ ] Mentorship program for new contributors
- [ ] Academic partnerships
- [ ] Industry adoption stories

#### Research

- [ ] Novel accessibility research
- [ ] HCI research on geometric proof construction
- [ ] Performance research on verification algorithms
- [ ] Educational efficacy studies

## Phase 5: Future Directions (2.0+)

**Status**: Speculative
**Target**: Future

These are aspirational ideas, not commitments:

### Potential Major Features

#### AI Integration

- AI-assisted proof construction (suggestions only, never verification)
- Natural language interface for proof exploration
- Automated proof translation between formal systems
- Accessibility improvements via ML (e.g., better audio descriptions)

### Distributed Verification

- Sharing computational load across machines
- Verifiable distributed proofs
- Blockchain-based proof archives (if useful)

### Hardware Integration

- Custom haptic devices for geometric feedback
- Brain-computer interface support (when safe and useful)
- Specialized AT integration (e.g., refreshable braille for math)

### New Formalisms

- Cubical type theory
- Modal type theories
- Substructural type theories (linear, affine)
- Domain-specific formal systems

### Extended Application Areas

- Formal verification of accessibility itself
- Verifying neural network correctness
- Geometric ML model architectures
- Educational mathematics

## Non-Goals

To stay focused, we explicitly are not pursuing:

### Things We Won't Build

- **Closed-source components** — ProveIt remains MIT licensed
- **Anti-features for accessibility** — we never compromise on accessibility
- **Replacement for Coq/Lean/Agda** — we complement, not compete
- **Pure visual interface** — multi-modal is mandatory
- **Inaccessible "expert mode"** — features are accessible or not included
- **Telemetry by default** — privacy is paramount
- **Vendor lock-in** — proofs are portable

### Things We Won't Optimize For

- **Single-platform polish** at expense of cross-platform support
- **Sighted-user efficiency** at expense of accessibility
- **Performance** at expense of soundness
- **Feature breadth** at expense of feature quality

## How to Influence the Roadmap

The roadmap reflects current thinking but is open to change.

### To Suggest New Items

1. Open a [feature request](.github/ISSUE_TEMPLATE/feature_request.md)
2. Describe the use case and impact
3. Discuss in the issue
4. Items with strong support and clear plans may be added

### To Reprioritize Items

1. Open a discussion in [GitHub Discussions](https://github.com/TensorHusker/ProveIt/discussions)
2. Make the case for reordering
3. Listen to other community members
4. Maintainers will weigh input in roadmap updates

### Voting and Priority

We don't use simple voting because:

- It can disenfranchise minority needs (especially for accessibility)
- Loud voices are not always representative
- Implementation difficulty matters

Priority is determined by:

- Impact on the underserved (accessibility weight)
- Community input
- Implementation feasibility
- Alignment with vision
- Available contributor effort

## Metrics for Success

We track progress not by feature checkboxes, but by:

### User-Centric Metrics

- Number of disabled users actively using ProveIt
- User-reported productivity
- Accessibility audit scores
- Time to first successful proof for new users

### Technical Metrics

- Verification correctness (zero soundness bugs)
- Performance benchmarks
- Test coverage and mutation kill rate
- Documentation completeness

### Community Metrics

- Number of contributors
- Diversity of contributor backgrounds
- Issue response time
- Tutorial completion rates

## Contributing to the Roadmap

This document evolves. To contribute:

- **Comments**: open issues for discussion
- **Corrections**: PRs welcome for typos, broken links
- **Additions**: substantial changes go through discussion first
- **Translations**: localized versions welcome

## Acknowledgments

This roadmap is informed by:

- The accessibility community's needs and feedback
- Existing formal methods tools and their limitations
- Research on inclusive design
- Contributors' diverse perspectives

## Related Documents

- [Architecture](../ARCHITECTURE.md) — current architectural decisions
- [Contributing](../CONTRIBUTING.md) — how to participate
- [Changelog](../CHANGELOG.md) — what we've already done
- [Accessibility](ACCESSIBILITY.md) — accessibility commitments

---

*Last updated: 2026-05-02*
*Next review: 2026-08-01*
