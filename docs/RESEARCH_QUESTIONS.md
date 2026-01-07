# TTT Research Questions

**Tracking specific technical questions that need answering**

---

## Overview

This document tracks the specific research questions that must be answered before committing to Tiny Type Theory (TTT) as ProveIt's foundation. Each question includes current status, how to answer it, and decision criteria.

---

## Critical Questions

### Q1: Can discrete combinatorics be efficiently encoded in smooth types?

**Why It Matters**:
- Current ProveIt uses SCTT (smooth cubical types)
- Graph theory, discrete math are important use cases
- If smooth → discrete encoding is inefficient, need separate foundation

**Current Status**: ⬜ Not tested

**How to Answer**:
1. Implement graph theory problems in current SCTT
   - Simple: Graph connectivity, shortest path
   - Complex: Graph coloring, max flow
2. Measure overhead vs. specialized discrete representation
3. Compare proof size, type checking time, conceptual clarity

**Success Criteria**:
- Overhead < 20% for typical discrete problems
- Proofs remain human-readable
- No fundamental expressiveness limitations

**Timeline**: 1 month

**Resources Needed**:
- 1 developer
- Access to discrete mathematics test suite
- Benchmark harness

**Decision Impact**:
- If YES: SCTT may be sufficient, TTT optional
- If NO: Need separate discrete foundation or TTT

---

### Q2: What would TTT specification actually look like?

**Why It Matters**:
- Can't build what you can't specify
- Need to know if "minimal" is actually achievable
- Spec informs all other decisions

**Current Status**: ⬜ No specification exists

**How to Answer**:
1. Survey existing minimal type theories:
   - Coquand's calculus of constructions (minimal)
   - Pure Type Systems
   - System F variants
   - Martin-Löf's minimal type theory
2. Design TTT as "just enough" for:
   - Dependent types (for correctness)
   - Tensor primitives (for neural hardware)
   - Minimal operations (for thermodynamics)
3. Formalize in proof assistant (Agda/Coq)
4. Prove basic metatheory (soundness, normalization)

**Success Criteria**:
- Specification fits on 2-3 pages
- Metatheory provable (not just conjectured)
- Can express: basic math, tensor operations, proofs

**Timeline**: 3 months

**Resources Needed**:
- 1 type theorist
- Access to proof assistant
- Literature review

**Decision Impact**:
- If achievable: TTT is feasible
- If too complex: Abandon or simplify further
- If too weak: Reconsider minimalism

---

### Q3: Can SCTT → TTT compilation be efficient?

**Why It Matters**:
- Current ProveIt built on SCTT
- Migration cost depends on compilation efficiency
- Translation overhead might negate thermodynamic gains

**Current Status**: ⬜ No prototype

**How to Answer**:
1. Design compilation strategy:
   - How to encode smooth paths in discrete TTT?
   - How to handle Kan operations?
   - What gets lost in translation?
2. Build prototype compiler:
   - Input: Simple SCTT proofs (identity, modus ponens, Pythagorean theorem)
   - Output: TTT code
3. Measure:
   - Code size blowup
   - Compilation time
   - Runtime overhead (if interpretable)
   - Conceptual clarity (human judgment)

**Success Criteria**:
- Code size < 5x blowup
- Compilation time < 10 seconds for typical proofs
- Preserves correctness guarantees
- Human can understand compiled output (for debugging)

**Timeline**: 2 months (after Q2)

**Resources Needed**:
- 1 compiler developer
- TTT specification from Q2
- SCTT implementation (exists)
- Benchmark proofs

**Decision Impact**:
- If efficient: SCTT → TTT migration feasible
- If inefficient: Abandon TTT or keep SCTT alongside

---

### Q4: Do neural accelerators benefit from type-theoretic verification?

**Why It Matters**:
- This is the killer app for TTT
- If no performance advantage, market demand weak
- Validates thermodynamic argument in practice

**Current Status**: ⬜ No prototype

**How to Answer**:
1. Choose representative workload:
   - Neural shader (DLSS-style)
   - AI inference (Apple Intelligence-style)
   - Tensor operations (basic)
2. Implement unverified version:
   - Metal 4 Tensor API (for M5)
   - CUDA Tensor Core (for RTX 50)
3. Implement verified version:
   - TTT source code
   - Compile to Metal/CUDA
   - Add verification annotations
4. Benchmark:
   - Compilation time
   - Runtime performance
   - Energy consumption (if measurable)
   - Bug detection (intentional bugs)

**Success Criteria**:
- Runtime performance >= 90% of unverified
- Compilation overhead < 2x
- Catches real bugs (not just toy examples)
- Developer time comparable or better

**Timeline**: 3-4 months (after Q2, Q3)

**Resources Needed**:
- 1 GPU programmer
- Apple M5 device (MacBook Pro)
- NVIDIA RTX 50 device
- Energy measurement tools
- TTT → Metal/CUDA compiler from Q3

**Decision Impact**:
- If strong benefits: Neural hardware is killer app
- If weak benefits: Rethink value proposition
- If negative: Abandon neural hardware angle

---

### Q5: Is there market demand for verified neural compute?

**Why It Matters**:
- Technical success doesn't ensure market success
- Need users willing to pay for verification
- Informs prioritization and business model

**Current Status**: ⬜ No market research

**How to Answer**:
1. Identify potential user segments:
   - Game developers (RTX users)
   - Apple Intelligence developers
   - Enterprise AI (safety-critical)
   - Research labs
2. Conduct interviews (20-30):
   - What bugs do you encounter with neural shaders/AI?
   - How much time spent debugging?
   - What would you pay for guaranteed correctness?
   - Would performance trade-off be acceptable?
3. Run pilot program:
   - 3-5 early adopters
   - Give them TTT prototype
   - Measure actual usage and value
4. Economic analysis:
   - Market size
   - Willingness to pay
   - Competitive landscape
   - Business model options

**Success Criteria**:
- 10+ interviewees say "we need this"
- 3+ willing to pay beta test
- Willingness to pay > $1K/year/developer
- Clear pain point verified neural compute solves

**Timeline**: 2-3 months (parallel with Q4)

**Resources Needed**:
- 1 product manager / researcher
- Access to developer communities
- Budget for interviews ($50-100/interview)
- Prototype from Q4 for pilot

**Decision Impact**:
- If strong demand: Prioritize neural hardware
- If weak demand: Pivot to other TTT applications
- If no demand: Reconsider entire TTT approach

---

## Supporting Questions

### Q6: Can TTT's metatheory be proven correct?

**Why It Matters**: Verification bedrock must itself be verified

**Status**: ⬜ Unknown until spec exists (Q2)

**How to Answer**:
- Formalize TTT in Agda/Coq
- Prove soundness, normalization, decidability
- Estimate person-months required

**Success Criteria**:
- Proof completes in < 1 person-year
- No fundamental obstacles
- Proof is maintainable

---

### Q7: What is translation overhead for different theories?

**Why It Matters**: Common substrate only valuable if translations are efficient

**Status**: ⬜ Depends on Q2, Q3

**How to Answer**:
- CIC → TTT compiler
- HoTT → TTT compiler
- Measure overhead for real proofs from mathlib4

**Success Criteria**:
- Average overhead < 30%
- No exponential blowups
- Preserves readability

---

### Q8: Can we build verified compilers?

**Why It Matters**: Compilation must preserve correctness

**Status**: ⬜ Ambitious but possible

**How to Answer**:
- Use CompCert methodology
- Build verified compiler in proof assistant
- Extract to runnable code

**Success Criteria**:
- Proof of correctness exists
- Performance acceptable
- Maintainable

---

### Q9: What is attack surface comparison?

**Why It Matters**: Security is claimed advantage of TTT

**Status**: ⬜ Need security audit

**How to Answer**:
- Security audit of TTT implementation
- Security audit of theory-agnostic architecture
- Compare: lines of trusted code, attack vectors

**Success Criteria**:
- TTT trusted base < 10K lines
- Measurably smaller than alternatives
- No critical vulnerabilities

---

### Q10: Do hardware vendors care?

**Why It Matters**: Hardware co-evolution requires vendor interest

**Status**: ⬜ Unknown

**How to Answer**:
- Contact Apple, NVIDIA, AMD, Intel
- Present TTT vision
- Gauge interest in optimization

**Success Criteria**:
- 1+ vendor interested
- Willing to collaborate
- See business value

---

## Decision Matrix

| Question | Go TTT | No-Go TTT | Decision Weight |
|----------|--------|-----------|----------------|
| Q1: Discrete efficiency | ✅ Overhead < 20% | ❌ Overhead > 50% | High |
| Q2: TTT specification | ✅ Achievable | ❌ Too complex | Critical |
| Q3: SCTT → TTT | ✅ Efficient | ❌ Inefficient | High |
| Q4: Neural hardware | ✅ Perf >= 90% | ❌ Perf < 80% | Critical |
| Q5: Market demand | ✅ 10+ users need | ❌ No clear demand | Critical |
| Q6: Metatheory | ✅ Provable | ❌ Intractable | High |
| Q7: Translation | ✅ Avg < 30% | ❌ Avg > 50% | Medium |
| Q8: Verified compilers | ✅ Feasible | ❌ Too hard | Medium |
| Q9: Attack surface | ✅ Measurably smaller | ❌ No difference | Low |
| Q10: Vendor interest | ✅ 1+ interested | ❌ No interest | Low |

**Decision Rule**:
- If 3+ Critical questions NO-GO → Abandon TTT
- If 2 Critical questions NO-GO → Seriously reconsider
- If all Critical questions GO + most High questions GO → Proceed with TTT

---

## Research Timeline

### Month 1-2: Foundational
- Q1: Discrete encoding tests
- Q2: TTT specification (start)

### Month 3-4: Core Theory
- Q2: TTT specification (complete)
- Q6: Metatheory proofs (start)
- Q3: SCTT → TTT compiler (start)

### Month 5-6: Validation
- Q3: SCTT → TTT compiler (complete)
- Q4: Neural hardware prototype
- Q5: Market research

### Month 6: Decision Point
- Review all critical questions
- Go/No-Go decision
- If GO: Begin Phase 2 (implementation)
- If NO-GO: Focus on theory-agnostic architecture

---

## Tracking Updates

### Q1: Discrete Encoding
**Status**: ⬜ Not Started
**Last Updated**: 2025-10-17
**Next Milestone**: Begin testing
**Blockers**: None

### Q2: TTT Specification
**Status**: ⬜ Not Started
**Last Updated**: 2025-10-17
**Next Milestone**: Literature review
**Blockers**: None

### Q3: SCTT → TTT Compiler
**Status**: ⬜ Not Started
**Last Updated**: 2025-10-17
**Next Milestone**: Design compilation strategy
**Blockers**: Q2 (need TTT spec)

### Q4: Neural Hardware
**Status**: ⬜ Not Started
**Last Updated**: 2025-10-17
**Next Milestone**: Choose benchmark workload
**Blockers**: Q2, Q3 (need TTT and compiler)

### Q5: Market Demand
**Status**: ⬜ Not Started
**Last Updated**: 2025-10-17
**Next Milestone**: Identify interview candidates
**Blockers**: Q4 (need prototype to show)

---

## See Also

- [TTT_RESEARCH_PROPOSAL.md](TTT_RESEARCH_PROPOSAL.md) - Overall research strategy
- [ARCHITECTURAL_ALTERNATIVES.md](ARCHITECTURAL_ALTERNATIVES.md) - Architecture comparison
- [NEURAL_HARDWARE_INTEGRATION.md](NEURAL_HARDWARE_INTEGRATION.md) - M5/RTX integration details

---

**Document Status**: Living document - updated as research progresses
**Last Updated**: 2025-10-17
**Next Review**: Monthly or after major milestone

🤖 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude <noreply@anthropic.com>
