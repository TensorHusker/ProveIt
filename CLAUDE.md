# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

ProveIt is a geometric construction environment for accessible formal verification. The project enables users to build proofs, verify systems, and compose neural networks through spatial reasoning. It supports multiple formal methods including type theory, category theory, and homotopy theory with real-time verification.

**Core Mission**: Accessibility-first design for blind, neurodivergent, and post-injury users.

## Architecture Principles

### Accessibility-First Design
- All visual interfaces must have complete non-visual equivalents
- Spatial reasoning should be expressible through multiple modalities (audio, tactile, textual)
- Command-line interfaces are primary, not secondary
- Screen reader compatibility is mandatory, not optional

### Formal Methods Integration
When implementing verification systems, support:
- **Type Theory**: Dependent types, proof terms, type checking
- **Category Theory**: Functors, natural transformations, commutative diagrams
- **Homotopy Theory**: Path types, higher inductive types, cubical structures

### Geometric Reasoning
Proofs are represented as geometric constructions:
- Points, lines, and shapes map to logical propositions
- Spatial relationships encode logical dependencies
- Construction sequences represent proof steps
- Verification happens in real-time as constructions are built

## Integration with Runetika Ecosystem

This project is part of the larger Runetika mathematical computing ecosystem. Consider:
- **SCTT (Smooth Cubical Type Theory)**: Type-theoretical foundation for formal verification
- **Pattern Recognition**: ARC-style reasoning patterns may inform proof construction UI
- **Data Generation**: Proof construction traces could generate training data for AI reasoning

## Development Philosophy

### When Adding Features
1. Ask: "Can a blind user accomplish this task?"
2. Design the non-visual interface first
3. Add visual enhancements second
4. Test with screen readers before considering the feature complete

### Mathematical Correctness
- All proof verification must be formally sound
- No "approximate" verification - proofs are either valid or invalid
- Include explicit error messages that explain WHY a proof step fails
- Provide constructive feedback suggesting how to fix invalid steps

## Butterfly: Distributed LLM Inference

### Strategic Model Splitting
Butterfly is ProveIt's distributed inference system that strategically splits LLMs across worker nodes for parallel processing and intelligent combination. Unlike traditional layer-wise parallelism, Butterfly uses **functional decomposition** - splitting models by reasoning capability rather than just by layers.

### Key Principles
- **Parallel Inference**: Workers process in parallel, not sequentially
- **Ensemble Intelligence**: Multiple diverse outputs combined intelligently
- **Accessibility-First**: All operations controllable via terminal interface
- **Formal Verification**: Combination methods verified using SCTT

### Implementation Guidelines
When working on Butterfly:
- Follow ML/LLM git conventions (experiment tracking, model versioning)
- Include performance metrics in commit messages
- Tag model checkpoints with accuracy/latency data
- Document experiments in `butterfly/experiments/`
- Ensure terminal interface remains primary control method

See [butterfly/ARCHITECTURE.md](butterfly/ARCHITECTURE.md) for complete specification.

## Future Directions

As this codebase develops, expect to build:
- Terminal-based geometric construction tools
- Real-time proof verification engines
- Multiple backend verifiers (type checkers, SMT solvers, proof assistants)
- Neural network composition through categorical structures
- Accessible visualization systems for mathematical objects
- Distributed LLM inference with strategic model splitting

## License

MIT License - prioritize openness and accessibility in all development decisions.
