# Changelog

All notable changes to ProveIt will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Comprehensive documentation suite
  - `CONTRIBUTING.md` — contributor guidelines, coding standards, PR process
  - `CODE_OF_CONDUCT.md` — Contributor Covenant 2.1 with accessibility additions
  - `SECURITY.md` — security policy and vulnerability reporting
  - `ARCHITECTURE.md` — high-level architecture documentation
  - `CHANGELOG.md` — this file
- Documentation directory structure
  - `docs/README.md` — documentation index
  - `docs/ACCESSIBILITY.md` — detailed accessibility guidelines
  - `docs/GLOSSARY.md` — mathematical and technical terminology
  - `docs/ROADMAP.md` — project roadmap
  - `docs/tutorials/` — learning materials
- XeTeX technical manual
  - `docs/tex/proveit-manual.tex` — comprehensive mathematical reference
  - `docs/tex/Makefile` — build automation
- GitHub templates
  - Issue templates (bug report, feature request, accessibility issue)
  - Pull request template
- `CLAUDE.md` — instructions for Claude Code AI assistant

### Changed

- Expanded `README.md` with vision, features, and getting started
- Updated `.gitignore` for LaTeX auxiliary files

### Infrastructure

- Claude Code GitHub Actions for issue/PR automation
- Claude Code review workflow for pull requests

## [0.1.0] — TBD

Initial pre-release. The project is in the design and early implementation phase.

### Planned for 0.1.0

- Basic geometric primitives (point, line, circle, polygon)
- Simple type theory backend
- Proof construction with undo/redo
- Screen reader integration prototype
- Audio cue framework
- Test infrastructure with property-based tests
- Continuous integration

---

## Versioning Policy

ProveIt follows [Semantic Versioning 2.0.0](https://semver.org/spec/v2.0.0.html):

- **MAJOR** version increments for incompatible API changes
- **MINOR** version increments for backwards-compatible feature additions
- **PATCH** version increments for backwards-compatible bug fixes

### Pre-1.0 Caveats

While in 0.x versions:

- Breaking changes may occur in minor version bumps
- The API is considered experimental
- We will document breaking changes prominently
- Migration guides will be provided when feasible

### Release Cadence

Once stable:

- **Patch releases**: As needed for bug fixes and security
- **Minor releases**: Approximately quarterly
- **Major releases**: When significant breaking changes warrant

## Categories

Each version may include changes in these categories:

- **Added** — new features
- **Changed** — changes in existing functionality
- **Deprecated** — soon-to-be removed features
- **Removed** — features removed in this release
- **Fixed** — bug fixes
- **Security** — security-related changes
- **Accessibility** — accessibility improvements
- **Documentation** — documentation changes
- **Infrastructure** — build, CI, or tooling changes

## Migration Guides

When breaking changes occur, migration guides will be provided in `docs/migrations/`.

[Unreleased]: https://github.com/TensorHusker/ProveIt/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/TensorHusker/ProveIt/releases/tag/v0.1.0
