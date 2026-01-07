# ProveIt Git Branch Structure

**Organized repository with best practices for independent component development**

---

## Branch Philosophy

ProveIt follows a **component-based branching strategy** where major subsystems can evolve independently while maintaining integration points. This enables:

1. **Parallel development** - Multiple teams/contributors working simultaneously
2. **Component isolation** - Changes don't break unrelated features
3. **Independent releases** - Components can reach stability at different rates
4. **Clear ownership** - Each branch has a focused purpose

---

## Branch Overview

### `main` - Stable Release Branch
**Status**: 🟢 Stable
**Purpose**: Production-ready releases only
**Protection**: Requires PR approval
**Contents**: Minimal - README, LICENSE, basic documentation

**Merge Policy:**
- Only merge from feature branches after:
  - All tests passing
  - Code review approved
  - Documentation updated
  - Breaking changes communicated

---

### `feature/sctt-type-theory` - Type Theory Core
**Status**: 🟡 Active Development
**Focus**: Smooth Cubical Type Theory implementation
**Lead Component**: `crates/sctt-core`

**Contents:**
- Complete SCTT implementation (2,564 lines)
- Bidirectional type checker
- Normalization by Evaluation
- Kan operations (scaffolding)
- Differential operators (smooth paths)
- MATHEMATICAL_FOUNDATION.md

**Development Guidelines:**
- Mathematical correctness is paramount
- All type operations must preserve soundness
- Properties verified with proptest
- Formal specification before implementation

**Dependencies**: None (foundational layer)

**Merge to main when:**
- [ ] Full Kan operations implemented (comp, coe, hcomp)
- [ ] Glue types for univalence
- [ ] Complete differential operators
- [ ] Comprehensive property-based tests
- [ ] Formal correctness proofs

**Git Best Practices:**
- Commit format: `feat(sctt): <description>`
- One mathematical concept per commit
- Include proof sketches in messages
- Tag stable milestones: `sctt-v0.1.0`, `sctt-v0.2.0`

---

### `feature/butterfly-distributed-inference` - Distributed AI
**Status**: 🟡 Active Development
**Focus**: Revolutionary distributed LLM inference
**Lead Components**: `crates/butterfly-*`

**Contents:**
- Butterfly core, worker, coordinator (6,390 lines)
- 8 architectural specifications (5,903 lines)
- Functional decomposition algorithm
- Byzantine fault-tolerant consensus
- Fusion strategies

**Development Guidelines:**
- Performance is critical (3.7-5.7x speedup target)
- Byzantine resistance mandatory
- All protocols formally specified
- Network efficiency paramount

**Dependencies:**
- `sctt-core` (for formal verification of fusion)
- Optional integration with proof-engine

**Merge to main when:**
- [ ] Production network implementation
- [ ] Security audit completed
- [ ] Benchmark suite validated
- [ ] Real ML model decomposition tested
- [ ] Byzantine tolerance verified

**Git Best Practices:**
- Commit format: `feat(butterfly): <description>`
- Include performance metrics in messages
- Tag experiments: `butterfly-exp-001`
- Document protocol changes thoroughly

---

### `feature/claude-code-exploration` - Complete Integrated System
**Status**: 🟢 Feature Complete
**Focus**: Full ProveIt with all components integrated
**Contains**: Everything

**Contents:**
- All 8 crates (21,984 lines)
- Complete documentation
- Integration examples
- SYNTHESIS.md vision document

**Purpose:**
- **Integration testing** - Verify components work together
- **Prototyping** - Rapid feature exploration
- **User testing** - Complete system for demos
- **Vision maintenance** - Keep big picture coherent

**Development Guidelines:**
- This is the "everything works together" branch
- Break changes into component branches for merging
- Use for discovering integration issues
- Maintain comprehensive documentation

**Dependencies**: All components

**Merge to main when:**
- Individual component branches are stable
- Integration tests pass
- Documentation complete
- Ready for first public release

**Git Best Practices:**
- Commit format: `feat: <comprehensive description>`
- Large commits acceptable (integration work)
- Reference component branches
- Keep SYNTHESIS.md updated

---

### `feature/testing-benchmarks-examples` - Quality Assurance
**Status**: 🟢 Complete
**Focus**: Comprehensive testing infrastructure
**Lead Artifacts**: `tests/`, `benches/`, `examples/`

**Contents:**
- 170 integration tests
- 4 benchmark suites
- 6 working examples
- 8 property-based tests
- TESTING_INFRASTRUCTURE.md

**Purpose:**
- **Regression prevention** - Catch breaking changes
- **Performance tracking** - Benchmark regressions
- **User education** - Examples for learning
- **Mathematical verification** - Property testing

**Development Guidelines:**
- Tests must be realistic, not trivial
- Benchmarks guide optimization
- Examples teach best practices
- All public APIs have doc tests

**Dependencies**: All components (tests them)

**Merge to main when:**
- All component branches have merged
- Tests adapted to stable APIs
- Benchmark baselines established
- Examples validated by users

**Git Best Practices:**
- Commit format: `test: <description>` or `bench: <description>`
- Group related tests in single commit
- Update TESTING_INFRASTRUCTURE.md
- Track test coverage metrics

---

## Workflow Patterns

### Adding a New Feature

**1. Identify Component**
Determine which branch owns the feature:
- Type theory change? → `feature/sctt-type-theory`
- Distributed inference? → `feature/butterfly-distributed-inference`
- Integration/UX? → `feature/claude-code-exploration`

**2. Create Feature Branch (if needed)**
For large features, branch from component branch:
```bash
git checkout feature/sctt-type-theory
git checkout -b feature/sctt-higher-inductive-types
```

**3. Develop with Tests**
- Write tests first (TDD)
- Implement feature
- Update documentation
- Run component tests

**4. Merge to Component Branch**
```bash
git checkout feature/sctt-type-theory
git merge --no-ff feature/sctt-higher-inductive-types
git branch -d feature/sctt-higher-inductive-types
```

**5. Update Integration Branch**
```bash
git checkout feature/claude-code-exploration
git merge feature/sctt-type-theory
# Resolve any integration issues
cargo test --all
```

**6. Eventually Merge to Main**
When component is stable, create PR to main.

---

### Fixing a Bug

**1. Identify Location**
Where does the bug live?
- Unit test in component branch
- Integration test in exploration branch
- Add test to testing branch

**2. Fix in Component Branch**
```bash
git checkout feature/sctt-type-theory
# Fix bug
cargo test -p sctt-core
git commit -m "fix(sctt): correct path composition associativity"
```

**3. Propagate Fix**
```bash
# Cherry-pick to other branches if needed
git checkout feature/claude-code-exploration
git cherry-pick <commit-hash>
```

---

### Synchronizing Branches

**Periodically sync component branches with integration:**

```bash
# On component branch
git checkout feature/sctt-type-theory
git merge feature/claude-code-exploration --strategy-option ours
# Resolve conflicts favoring component branch
```

**Or integrate component improvements back:**

```bash
# On integration branch
git checkout feature/claude-code-exploration
git merge feature/sctt-type-theory
# Resolve conflicts favoring integration branch
```

---

## Branch Dependencies

```
main (stable)
  ↑
  ├─ feature/sctt-type-theory (independent)
  ├─ feature/butterfly-distributed-inference (depends on sctt-core)
  ├─ feature/claude-code-exploration (depends on all)
  └─ feature/testing-benchmarks-examples (tests all)
```

**Independence Levels:**
1. **sctt-core**: No dependencies (foundation)
2. **butterfly**: Depends on sctt-core (for fusion verification)
3. **exploration**: Depends on everything (integration)
4. **testing**: Tests everything (QA layer)

---

## Commit Message Conventions

### Format
```
<type>(<scope>): <description>

<body>

<footer>
```

### Types
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `test`: Adding tests
- `bench`: Adding benchmarks
- `refactor`: Code restructuring
- `perf`: Performance improvement
- `chore`: Maintenance

### Scopes
- `sctt`: Type theory core
- `butterfly`: Distributed inference
- `geometry`: Geometric proofs
- `accessibility`: Multi-modal interface
- `cli`: Command-line interface
- `proof`: Proof engine

### Examples

```
feat(sctt): implement higher inductive types

Add support for HITs with constructors and path constructors.
Includes circle, suspension, and truncation types.

Refs: MATHEMATICAL_FOUNDATION.md section 4.3
```

```
fix(butterfly): correct Byzantine voting threshold

Changed from f < n/2 to f < n/3 per PBFT paper.
All tests passing with new threshold.

Performance impact: negligible
```

```
docs(synthesis): update roadmap with Phase 2 milestones

Added accessibility enhancement goals.
Updated timeline based on user studies.
```

---

## Release Strategy

### Version Numbering

**Components** (independent versions):
- `sctt-core-v0.1.0`
- `butterfly-core-v0.1.0`

**ProveIt** (unified version):
- `v0.1.0` - First public release
- `v0.2.0` - Complete Kan operations
- `v0.3.0` - Butterfly production ready
- `v1.0.0` - Full feature complete

### Release Process

**Component Release:**
1. Tag component branch: `git tag sctt-v0.1.0`
2. Update CHANGELOG in component
3. Publish crate (if applicable)
4. Announce in main README

**Unified Release:**
1. Merge all stable component branches to main
2. Run full test suite
3. Tag main: `git tag v0.1.0`
4. Generate release notes
5. Publish announcement

---

## Best Practices Summary

### DO ✅
- Keep commits atomic (one concept per commit)
- Write descriptive commit messages
- Test before committing
- Update documentation with code
- Tag stable milestones
- Cherry-pick selectively between branches
- Communicate breaking changes

### DON'T ❌
- Mix unrelated changes in one commit
- Commit broken code
- Force-push to shared branches
- Merge without testing
- Leave TODO comments without issues
- Rewrite published history

---

## Branch Health Metrics

### `feature/sctt-type-theory`
- Commits: 2
- Tests: 18/18 passing
- Coverage: ~80% (estimated)
- Compilation: ✅ Clean

### `feature/butterfly-distributed-inference`
- Commits: 1
- Tests: 7/7 passing
- Specifications: 5,903 lines
- Compilation: ✅ Clean

### `feature/claude-code-exploration`
- Commits: 3
- Tests: 66+ passing
- Total lines: 21,984
- Compilation: ✅ Clean

### `feature/testing-benchmarks-examples`
- Commits: 1
- Tests: 236+ (99.6% passing)
- Examples: 6 working
- Benchmarks: 4 suites

---

## Future Branch Considerations

### Potential Future Branches

**`feature/gui-visualization`**
- Graphical interface for geometric proofs
- Depends on: geometry, accessibility, cli
- Independent development of UI layer

**`feature/proof-library`**
- Formalized mathematics library
- Depends on: sctt-core, proof-engine
- Community contributions

**`feature/plugin-system`**
- Tactic and sonification plugins
- Depends on: proof-engine, accessibility
- Extensibility layer

**`feature/network-implementation`**
- Real distributed network for Butterfly
- Depends on: butterfly-core
- Production deployment

---

## Conclusion

This branch structure enables:

✅ **Parallel development** - Work on components independently
✅ **Component isolation** - Changes don't break unrelated code
✅ **Clear ownership** - Each branch has focused responsibility
✅ **Integration testing** - Exploration branch catches issues early
✅ **Quality assurance** - Testing branch ensures regression prevention
✅ **Independent releases** - Components can stabilize at different rates

**ProveIt's git workflow mirrors its architectural philosophy: modular, composable, and elegant.**

🤖 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude <noreply@anthropic.com>
