---
name: Bug Report
about: Report a defect to help us improve ProveIt
title: '[BUG] '
labels: bug, needs-triage
assignees: ''
---

## Bug Description

A clear and concise description of the bug.

## Steps to Reproduce

1.
2.
3.

## Expected Behavior

What you expected to happen.

## Actual Behavior

What actually happened. Include error messages, stack traces, or unexpected output.

## Minimal Reproducible Example

```rust
// Smallest code sample that reproduces the issue
```

If applicable, attach a proof file or configuration that demonstrates the bug.

## Environment

- **OS**: <!-- e.g., Ubuntu 22.04, macOS 14.1, Windows 11 -->
- **Rust version**: <!-- output of `rustc --version` -->
- **ProveIt version**: <!-- output of `proveit --version` or git commit -->
- **Terminal/IDE**: <!-- if relevant -->

## Accessibility Context

If you encountered this while using assistive technology:

- **Screen reader**: <!-- e.g., NVDA 2024.1, JAWS 2024, VoiceOver -->
- **Input method**: <!-- e.g., keyboard only, switch access, voice control -->
- **Magnifier/zoom**: <!-- if relevant -->
- **Did the bug affect accessibility?**: Yes/No, explain

## Severity

How does this bug affect you?

- [ ] **Critical** — Soundness bug (verifier accepts invalid proof or rejects valid proof)
- [ ] **High** — Blocks normal usage
- [ ] **Medium** — Workaround available but inconvenient
- [ ] **Low** — Minor issue or cosmetic

## Soundness Implications

If this might affect verification soundness, please indicate:

- [ ] This bug might cause invalid proofs to be accepted
- [ ] This bug might cause valid proofs to be rejected
- [ ] This bug doesn't affect verification correctness
- [ ] Unknown — let the maintainers assess

## Logs and Diagnostics

```
Paste relevant log output here, with RUST_LOG=debug if possible
```

## Screenshots/Recordings

If applicable, add screenshots or screen recordings. **Please include text descriptions** of what they show, for accessibility.

## Additional Context

Any other context about the problem here. Have you found related issues? Tried any workarounds?

## Checklist

- [ ] I have searched existing issues to ensure this isn't a duplicate
- [ ] I have included a minimal reproducible example
- [ ] I have included environment information
- [ ] I have indicated the severity
