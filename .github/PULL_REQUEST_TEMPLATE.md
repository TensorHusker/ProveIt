<!-- Thank you for contributing to ProveIt! Please fill out this template to help reviewers. -->

## Summary

<!-- One or two sentences describing what this PR does and why -->

## Type of Change

- [ ] Bug fix (non-breaking change that fixes an issue)
- [ ] New feature (non-breaking change that adds functionality)
- [ ] Breaking change (fix or feature that would cause existing functionality to not work as expected)
- [ ] Documentation update
- [ ] Performance improvement
- [ ] Refactoring (no functional change)
- [ ] Test improvement
- [ ] Accessibility improvement
- [ ] Build/CI change

## Related Issues

<!-- Link related issues. Use "Closes #N" to auto-close on merge, "Refs #N" otherwise. -->

Closes #
Refs #

## Changes

<!-- Describe the changes in detail. Bullet points are fine. -->

-
-
-

## Test Plan

<!-- How did you test these changes? What should reviewers check? -->

### Automated Tests

- [ ] Existing tests pass: `cargo test`
- [ ] Added new tests for new functionality
- [ ] Added regression test for bug fix
- [ ] Property-based tests added/updated where applicable

### Manual Testing

<!-- Steps reviewers should take to verify the change works -->

1.
2.
3.

## Accessibility Impact

All changes should be assessed for accessibility impact.

- [ ] No accessibility impact (justify briefly)
- [ ] Improves accessibility (describe how)
- [ ] Maintains existing accessibility
- [ ] Required new accessibility features (which ones?)

### Accessibility Checklist

If this PR adds or modifies UI elements:

- [ ] All elements have accessible labels
- [ ] All actions are keyboard-accessible
- [ ] Focus indicators are visible
- [ ] No information conveyed by color alone
- [ ] Audio cues provided for spatial operations
- [ ] Tested with at least one screen reader
- [ ] Tested with keyboard-only navigation
- [ ] Color contrast meets WCAG AA (or AAA where applicable)

If this PR modifies documentation:

- [ ] Headings have proper hierarchy
- [ ] Images have alt text
- [ ] Code blocks include language
- [ ] Math notation has plain text alternative
- [ ] Tables have headers
- [ ] Links have descriptive text

## Mathematical Soundness

If this PR affects formal methods or verification:

- [ ] Not applicable
- [ ] I have considered soundness implications
- [ ] Soundness is unchanged
- [ ] Soundness is improved (describe)
- [ ] **This change requires soundness review** (mention maintainers)

## Documentation

- [ ] Code comments updated
- [ ] Public API documented
- [ ] CHANGELOG.md updated under `[Unreleased]`
- [ ] User-facing docs updated (if applicable)
- [ ] Examples added/updated (if applicable)

## Breaking Changes

<!-- If this is a breaking change, describe the migration path -->

- [ ] Not a breaking change
- [ ] Breaking change details:

### Migration

```rust
// Before
old_api_call();

// After
new_api_call();
```

## Performance

- [ ] No performance impact
- [ ] Improves performance (provide benchmarks)
- [ ] Has acceptable performance cost (justify)
- [ ] Needs performance review

## Screenshots / Recordings

<!-- For UI changes, include before/after screenshots with descriptions -->
<!-- Remember to include text descriptions for accessibility -->

## Checklist

Before requesting review:

- [ ] I have read the [Contributing Guidelines](../CONTRIBUTING.md)
- [ ] My code follows the project style (`cargo fmt` passes)
- [ ] My code passes lints (`cargo clippy` with no warnings)
- [ ] My code compiles without warnings
- [ ] All tests pass (`cargo test`)
- [ ] I have updated documentation as needed
- [ ] I have added tests covering my changes
- [ ] I have considered accessibility impact
- [ ] I have updated CHANGELOG.md
- [ ] My commits follow the [conventional commit format](../CONTRIBUTING.md#commit-messages)

## Reviewer Focus

<!-- Optional: highlight specific areas where you'd like extra scrutiny -->

- Areas needing careful review:
- Decisions you're uncertain about:
- Assumptions that should be validated:

## Additional Notes

<!-- Anything else reviewers should know -->
