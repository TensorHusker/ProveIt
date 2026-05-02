# ProveIt Documentation

Welcome to the ProveIt documentation. This index helps you navigate to the right resource for your needs.

## Quick Links by Audience

### I'm new to ProveIt

Start here:

1. **[Project README](../README.md)** — what ProveIt is and why it exists
2. **[What is Formal Verification?](tutorials/01-what-is-formal-verification.md)** — gentle introduction
3. **[Glossary](GLOSSARY.md)** — terminology reference

### I want to use ProveIt

Practical guides:

1. **[README: Getting Started](../README.md#getting-started)** — installation and first steps
2. **[Tutorials](tutorials/)** — step-by-step learning
3. **[Accessibility Guide](ACCESSIBILITY.md)** — using ProveIt with assistive technology

### I want to contribute to ProveIt

Contributor resources:

1. **[Contributing Guide](../CONTRIBUTING.md)** — process, standards, expectations
2. **[Code of Conduct](../CODE_OF_CONDUCT.md)** — community standards
3. **[Architecture](../ARCHITECTURE.md)** — system design
4. **[CLAUDE.md](../CLAUDE.md)** — for AI-assisted contributions

### I'm a maintainer

Maintenance resources:

1. **[Architecture](../ARCHITECTURE.md)** — design decisions and rationale
2. **[Roadmap](ROADMAP.md)** — planning and priorities
3. **[Changelog](../CHANGELOG.md)** — release history
4. **[Security](../SECURITY.md)** — security policies

### I'm researching formal methods

Mathematical resources:

1. **[Technical Manual](tex/proveit-manual.tex)** — comprehensive mathematical reference
2. **[Glossary: Type Theory](GLOSSARY.md#type-theory)** — terminology
3. **[Architecture: Formal Methods](../ARCHITECTURE.md#core-abstractions)** — implementation notes

## Documentation Map

```
ProveIt Documentation
│
├── Top-Level (project root)
│   ├── README.md                    Quick overview, getting started
│   ├── CLAUDE.md                    Instructions for AI assistants
│   ├── CONTRIBUTING.md              How to contribute
│   ├── CODE_OF_CONDUCT.md           Community standards
│   ├── SECURITY.md                  Security policy
│   ├── ARCHITECTURE.md              System architecture
│   ├── CHANGELOG.md                 Version history
│   └── LICENSE                      MIT license
│
├── docs/ (this directory)
│   ├── README.md                    This file
│   ├── ACCESSIBILITY.md             Accessibility guide
│   ├── GLOSSARY.md                  Terminology reference
│   ├── ROADMAP.md                   Project roadmap
│   ├── DOCUMENTATION_COMPARISON.md  Markdown vs TeX analysis
│   │
│   ├── tutorials/                   Learning materials
│   │   ├── README.md                Tutorial index
│   │   ├── 01-what-is-formal...md   First steps
│   │   └── ...                      More tutorials
│   │
│   └── tex/                         Advanced TeX documentation
│       ├── proveit-manual.tex       Comprehensive manual source
│       ├── Makefile                 Build automation
│       └── proveit-manual.pdf       Compiled output (gitignored)
│
└── .github/
    ├── ISSUE_TEMPLATE/              Issue templates
    │   ├── bug_report.md
    │   ├── feature_request.md
    │   ├── accessibility_issue.md
    │   └── config.yml
    ├── PULL_REQUEST_TEMPLATE.md     PR template
    └── workflows/                   CI/CD
```

## Documentation by Topic

### Mathematics and Formal Methods

- **[Technical Manual](tex/proveit-manual.tex)** — full mathematical exposition with theorems
- **[Glossary](GLOSSARY.md)** — definitions of mathematical terms
- **[Tutorials: Type Theory](tutorials/)** — practical introductions
- **[Architecture: Formal Methods Backend](../ARCHITECTURE.md#module-structure)** — implementation

### Accessibility

- **[Accessibility Guide](ACCESSIBILITY.md)** — comprehensive guide
- **[Architecture: Accessibility Layer](../ARCHITECTURE.md#accessibility-layer)** — implementation
- **[Tutorials: Accessibility Path](tutorials/)** — using ProveIt with AT
- **[Issue Template: Accessibility](../.github/ISSUE_TEMPLATE/accessibility_issue.md)** — reporting issues

### Development

- **[Contributing](../CONTRIBUTING.md)** — workflow and standards
- **[Architecture](../ARCHITECTURE.md)** — design overview
- **[CLAUDE.md](../CLAUDE.md)** — coding conventions
- **[Documentation Comparison](DOCUMENTATION_COMPARISON.md)** — format choices

### Project Management

- **[Roadmap](ROADMAP.md)** — what's planned
- **[Changelog](../CHANGELOG.md)** — what's been done
- **[Security](../SECURITY.md)** — security practices
- **[Code of Conduct](../CODE_OF_CONDUCT.md)** — community standards

## Documentation Formats

ProveIt uses multiple documentation formats, each suited to different needs:

| Format | Use Case | Examples |
|--------|----------|----------|
| **Markdown** | Quick reference, GitHub display | All `*.md` files |
| **XeTeX** | Mathematical exposition, print | `docs/tex/proveit-manual.tex` |
| **rustdoc** | API reference (auto-generated) | Run `cargo doc --open` |
| **Inline comments** | Implementation details | In source files |

For a detailed comparison, see [DOCUMENTATION_COMPARISON.md](DOCUMENTATION_COMPARISON.md).

## Building Documentation

### API Documentation (rustdoc)

```bash
cargo doc --open
```

### Technical Manual (XeTeX)

```bash
cd docs/tex
make           # Standard build
make full      # Full build with index
make watch     # Auto-rebuild on changes
```

Requirements:
- XeLaTeX (TeX Live or MiKTeX)
- Latin Modern fonts
- TikZ packages

### Markdown Documentation

GitHub renders Markdown automatically. For local preview:

```bash
# Using grip (pip install grip)
grip docs/README.md

# Using vscode (built-in)
code docs/README.md  # then Ctrl+Shift+V
```

## Accessibility of This Documentation

All ProveIt documentation aims to be accessible:

- **Heading hierarchy** is logical and consistent
- **Lists** use proper Markdown formatting
- **Tables** include headers
- **Links** use descriptive text (not "click here")
- **Code blocks** specify language
- **Images** include alt text
- **Math notation** has plain-text alternatives

If you find documentation accessibility issues, please [report them](../.github/ISSUE_TEMPLATE/accessibility_issue.md).

## Documentation Conventions

### Style

- **Active voice** preferred over passive
- **Plain language** alongside technical terms
- **Examples** for every concept
- **Cross-references** to related material
- **Consistent terminology** (see [Glossary](GLOSSARY.md))

### Code Examples

```rust
// Good: complete, working example with context
use proveit::geometry::Point;

let origin = Point::new(0.0, 0.0);
let p = Point::new(3.0, 4.0);
assert_eq!(origin.distance_to(&p), 5.0);
```

```rust
// Avoid: snippets without context that don't compile
distance_to(&p)  // What's `distance_to`? What's `p`?
```

### Cross-References

Within the docs directory:
```markdown
See [Glossary](GLOSSARY.md) for terminology.
```

To project root:
```markdown
See [Architecture](../ARCHITECTURE.md) for design details.
```

To external resources:
```markdown
Following [WCAG 2.1](https://www.w3.org/TR/WCAG21/) guidelines.
```

### Mathematical Notation

Use multiple representations:

```markdown
The dependent product type is written:

- **LaTeX**: $\Pi_{x:A} B(x)$
- **Plain text**: "Pi over x of type A of B of x"
- **ASCII**: `Pi (x : A), B(x)`
```

## Contributing to Documentation

We welcome documentation contributions! See [CONTRIBUTING.md](../CONTRIBUTING.md) for general guidelines.

Documentation-specific notes:

### Good Documentation Contributions

- Fix typos and broken links
- Clarify confusing explanations
- Add missing examples
- Update outdated information
- Improve accessibility
- Translate to other languages

### Documentation Style Guide

- Match existing tone and structure
- Verify all code examples compile and run
- Test all links
- Run a spell-checker
- Read aloud to check flow

### Review Process

Documentation PRs are reviewed for:

- Technical accuracy
- Clarity and accessibility
- Style consistency
- Working examples
- Proper cross-referencing

## Getting Help

If documentation doesn't answer your question:

1. **Search** existing [issues](https://github.com/TensorHusker/ProveIt/issues) and [discussions](https://github.com/TensorHusker/ProveIt/discussions)
2. **Ask** in [GitHub Discussions](https://github.com/TensorHusker/ProveIt/discussions)
3. **Open an issue** if documentation is missing or unclear
4. **Join the community** for real-time help

## Reading Order Suggestions

### For First-Time Users

1. [Project README](../README.md)
2. [What is Formal Verification?](tutorials/01-what-is-formal-verification.md)
3. [Glossary](GLOSSARY.md) — bookmark for reference
4. [Tutorials](tutorials/) — work through learning paths

### For Contributors

1. [Project README](../README.md)
2. [Contributing](../CONTRIBUTING.md)
3. [Architecture](../ARCHITECTURE.md)
4. [Code of Conduct](../CODE_OF_CONDUCT.md)
5. [CLAUDE.md](../CLAUDE.md) (if using AI assistance)

### For Maintainers

1. [Architecture](../ARCHITECTURE.md)
2. [Roadmap](ROADMAP.md)
3. [Security](../SECURITY.md)
4. [Changelog](../CHANGELOG.md)

### For Researchers

1. [Project README](../README.md)
2. [Technical Manual](tex/proveit-manual.tex)
3. [Architecture](../ARCHITECTURE.md)
4. [Roadmap: Research Directions](ROADMAP.md#phase-5-future-directions-20)

## Document Status

| Document | Status | Last Updated |
|----------|--------|--------------|
| README.md | Stable | 2026-05-02 |
| CLAUDE.md | Stable | 2026-05-02 |
| CONTRIBUTING.md | Initial | 2026-05-02 |
| CODE_OF_CONDUCT.md | Initial | 2026-05-02 |
| SECURITY.md | Initial | 2026-05-02 |
| ARCHITECTURE.md | Initial — design phase | 2026-05-02 |
| CHANGELOG.md | Initial | 2026-05-02 |
| ACCESSIBILITY.md | Initial | 2026-05-02 |
| GLOSSARY.md | Initial | 2026-05-02 |
| ROADMAP.md | Initial | 2026-05-02 |
| Technical Manual | Draft | 2026-05-02 |
| Tutorials | Skeleton | 2026-05-02 |

As the project evolves, these documents will be updated. Check timestamps for recency.
