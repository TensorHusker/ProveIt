# Glossary

A reference for mathematical, technical, and domain-specific terms used throughout the ProveIt codebase and documentation.

Terms are organized by category. For comprehensive mathematical exposition, see the [technical manual](tex/proveit-manual.tex).

## Table of Contents

- [Type Theory](#type-theory)
- [Homotopy Type Theory](#homotopy-type-theory)
- [Category Theory](#category-theory)
- [Logic and Proofs](#logic-and-proofs)
- [Geometry](#geometry)
- [Verification](#verification)
- [Accessibility](#accessibility)
- [ProveIt-Specific Terms](#proveit-specific-terms)

## Type Theory

### Type

A classification of values. In dependent type theory, types themselves are values of a higher type called a universe (`Type` or `𝒰`).

> Example: `Bool` is a type with two values, `true` and `false`.

### Term

A value that has a type. The judgment `a : A` means "term `a` has type `A`".

### Universe

A type whose elements are themselves types. To avoid Russell's paradox, universes are stratified: `Type₀ : Type₁ : Type₂ : ...`.

### Dependent Function Type (Π-type)

A function type where the return type may depend on the input value. Written `Π(x:A) B(x)` or `(x:A) → B(x)`.

> Example: `Π(n:ℕ) Vec(n)` — a function that takes a natural number `n` and returns a vector of length `n`.

### Dependent Pair Type (Σ-type)

A pair type where the second component's type may depend on the first component. Written `Σ(x:A) B(x)` or `(x:A) × B(x)`.

> Example: `Σ(n:ℕ) Vec(n)` — a pair of a length and a vector of that length.

### Identity Type

The type of equality proofs. `Id_A(a, b)` is the type of proofs that `a` and `b` are equal at type `A`.

### Refl

The canonical proof of equality of a term to itself. `refl_a : Id_A(a, a)`.

### Inductive Type

A type defined by listing its constructors. The constructors fully determine the type, and induction principles allow reasoning about all terms.

> Example: Natural numbers `ℕ` are inductively defined by `zero : ℕ` and `succ : ℕ → ℕ`.

### Eliminator

A function that defines what to do for each constructor of an inductive type. The induction principle is a dependent eliminator.

### Judgment

A formal statement we can derive in the type theory. The four basic judgments are:

- `A : Type` — `A` is a type
- `a : A` — `a` is a term of type `A`
- `A ≡ B` — `A` and `B` are definitionally equal types
- `a ≡ b : A` — `a` and `b` are definitionally equal at type `A`

### Context

A list of typed variables, written `Γ = x₁:A₁, x₂:A₂(x₁), ..., xₙ:Aₙ(x₁,...,xₙ₋₁)`. Used in judgments like `Γ ⊢ a : A`.

### Definitional Equality

Equality that holds by computation/normalization. Written `≡` or `=ᵈᵉᶠ`. Distinct from propositional equality.

### Propositional Equality

Equality witnessed by a term of an identity type. Written `a = b`.

## Homotopy Type Theory

### HoTT

Abbreviation for Homotopy Type Theory.

### Univalence Axiom

The axiom that equivalent types are equal: `(A ≃ B) ≃ (A = B)`. Allows treating equivalent types as interchangeable.

### Equivalence

A function with both a left and right inverse, satisfying coherence conditions. Written `A ≃ B`.

### Path

A term of an identity type. Geometrically, a path between two points.

### Higher Inductive Type (HIT)

An inductive type with both point constructors and path constructors. Allows defining quotient types and topological spaces.

> Example: The circle `S¹` has a point `base` and a path `loop : base = base`.

### h-Level / n-Type

A measure of the complexity of equality in a type:

- **(-2)-type / Contractible**: A unique element up to path
- **(-1)-type / Proposition**: At most one element up to path
- **0-type / Set**: Equality is a proposition
- **1-type / Groupoid**: Equality is a set
- And so on

### Propositional Truncation

The operation `‖A‖` that turns a type into a proposition by identifying all elements.

### Transport

The function `transport : (A = B) → A → B` that turns a path between types into a function.

### Function Extensionality

The principle that pointwise equal functions are equal: `(∀x, f(x) = g(x)) → f = g`. Provable from univalence.

## Category Theory

### Category

A collection of objects with morphisms between them, satisfying associativity and identity laws.

### Object

A "thing" in a category. Notation varies; we use uppercase letters like `A`, `B`, `C`.

### Morphism (Arrow)

A "transformation" between objects. Written `f : A → B`. Also called an "arrow".

### Hom-set

The collection of morphisms between two objects. Written `Hom(A, B)` or `C(A, B)`.

### Identity Morphism

The morphism that maps an object to itself trivially. `id_A : A → A`.

### Composition

The operation that combines `f : A → B` and `g : B → C` into `g ∘ f : A → C`.

### Functor

A "morphism between categories" that preserves structure. A functor `F : C → D` maps objects to objects and morphisms to morphisms.

### Natural Transformation

A "morphism between functors". Given functors `F, G : C → D`, a natural transformation `α : F ⇒ G` consists of morphisms `α_X : F(X) → G(X)` satisfying naturality.

### Monoidal Category

A category equipped with a tensor product `⊗` and a unit object, satisfying associativity and identity up to isomorphism.

### Adjunction

A relationship between two functors `F : C → D` and `G : D → C` such that `Hom_D(F(A), B) ≃ Hom_C(A, G(B))`. Written `F ⊣ G`.

## Logic and Proofs

### Proposition

A statement that can be true or false. In type theory, propositions correspond to types where all elements are equal.

### Proof

A term of a propositional type. Constructing the term proves the proposition.

### Curry-Howard Correspondence

The deep connection between programs and proofs:

- Propositions ↔ Types
- Proofs ↔ Programs
- Implication ↔ Function type
- Conjunction ↔ Product type
- Disjunction ↔ Sum type
- ⊥ (false) ↔ Empty type
- ⊤ (true) ↔ Unit type

### Inference Rule

A formal rule for deriving new judgments from existing ones. Written:

```
Premise₁    Premise₂    ...    Premiseₙ
─────────────────────────────────────── (Rule Name)
              Conclusion
```

### Sequent

A statement of the form `Γ ⊢ A`, meaning "in context `Γ`, we can derive `A`".

### Soundness

A formal system is sound if every provable statement is true. The verification engine must be sound.

### Completeness

A formal system is complete if every true statement is provable. We don't require completeness for our verification engine.

### Decidability

A property is decidable if there's an algorithm to determine whether it holds. Type checking in dependent type theory is decidable; equality may not be.

### Normalization

The process of reducing a term to a canonical form. Strong normalization means every reduction sequence terminates.

## Geometry

### Geometric Primitive

A basic geometric object: point, line, circle, polygon, etc.

### Construction

A sequence of geometric operations producing a geometric object. In ProveIt, constructions encode proofs.

### Construction Step

A single operation in a construction. Each step has a precise meaning in the underlying formal system.

### Compass-and-Straightedge

A traditional model of geometric construction allowing only:

1. Drawing lines through two points
2. Drawing circles with a center and radius
3. Marking intersections

### Affine Transformation

A geometric transformation preserving parallelism: translation, rotation, scaling, shearing.

### Projective Transformation

A more general transformation preserving collinearity but not parallelism. Used for perspective and projective geometry.

### Locus

The set of points satisfying a geometric condition. Loci often have type-theoretic interpretations.

## Verification

### Verifier

A program that checks the validity of a proof. The ProveIt verifier is the trust boundary.

### Trusted Computing Base (TCB)

The minimal code that must be correct for the system's claims to hold. We aim for a small TCB.

### Witness

Evidence that something is true. A successful verification produces a witness.

### Counterexample

A specific case demonstrating that a proposed property is false.

### Incremental Verification

Verifying only the parts of a proof that have changed, rather than the entire proof.

### Trace

A record of the verification process, useful for debugging failures.

### Soundness Bug

A bug allowing the verifier to accept invalid proofs. The most serious kind of bug in this domain.

## Accessibility

### Assistive Technology (AT)

Hardware or software that helps people with disabilities use computers. Examples: screen readers, alternative keyboards, eye tracking.

### Screen Reader

Software that reads screen content aloud. Examples: NVDA, JAWS, VoiceOver, Orca.

### ARIA

Accessible Rich Internet Applications — a set of attributes for making web content accessible.

### Live Region

An area of a page that updates dynamically. ARIA `role="status"` or `role="alert"` makes screen readers announce updates.

### Focus Indicator

The visual indication of which element currently has keyboard focus.

### Focus Trap

A bug where keyboard focus cannot leave a particular area. Generally undesirable.

### WCAG

Web Content Accessibility Guidelines — the primary international standard for web accessibility.

### A11y

Numeronym for "accessibility" (a-eleven-letters-y).

### Multi-modal

Using multiple sensory channels (vision, hearing, touch). Multi-modal output makes information accessible regardless of which senses a user can use.

### Cognitive Load

The mental effort required to use a system. Reducing cognitive load makes systems more accessible to users with cognitive impairments and benefits all users.

## ProveIt-Specific Terms

### Constructible

A type implementing the `Constructible` trait — buildable through construction steps.

### Verifiable

A type implementing the `Verifiable` trait — having a notion of verification.

### Accessible

A type implementing the `Accessible` trait — having multi-modal output methods.

### Formal System

A type implementing the `FormalSystem` trait — providing a mathematical foundation for verification.

### Proof Builder

A typestate-based builder for constructing proofs incrementally.

### Audio Cue

A short sound conveying information about a geometric or logical operation.

### Haptic Pattern

A vibration pattern conveying information through touch.

### Spatial Audio

Audio output where the position of sounds in stereo/3D space corresponds to geometric positions.

### Geometric Proof Construction

A proof presented as a sequence of geometric construction steps, where the geometry encodes the logical structure.

### Translation (Between Formal Systems)

An explicit mapping between formalizations, allowing a proof in one system to be expressed in another.

## Symbol Index

| Symbol | Reading | Meaning |
|--------|---------|---------|
| `⊢` | "proves" or "yields" | Derivability in a context |
| `≡` | "is definitionally equal to" | Computational equality |
| `=` | "equals" | Propositional equality (identity type) |
| `≃` | "is equivalent to" | Type equivalence |
| `≅` | "is isomorphic to" | Categorical isomorphism |
| `Π` | "Pi" | Dependent function type / product |
| `Σ` | "Sigma" | Dependent pair type / sum |
| `λ` | "lambda" | Function abstraction |
| `∘` | "after" | Composition |
| `⇒` | "implies" or "transforms to" | Natural transformation |
| `⊗` | "tensor" | Monoidal product |
| `⊥` | "bottom" | False / empty type |
| `⊤` | "top" | True / unit type |
| `∀` | "for all" | Universal quantification |
| `∃` | "there exists" | Existential quantification |
| `∎` | "QED" | End of proof |
| `↦` | "maps to" | Function definition |

## Conventions Used in This Codebase

### Type Naming

- Mathematical types use Latin uppercase: `A`, `B`, `C`
- Specific types use descriptive PascalCase: `ProofStep`, `GeometricPrimitive`
- Type variables in Rust use single uppercase letters: `T`, `U`, `S`

### Variable Naming

- Mathematical terms use lowercase: `a`, `b`, `f`, `g`
- Code variables use snake_case: `proof_step`, `current_context`

### Mathematical Notation in Code

When mathematical notation appears in code:

- Use ASCII alternatives in identifiers: `Pi`, `Sigma`, `Lambda` (not `Π`, `Σ`, `λ`)
- Use Unicode in doc comments and string literals where it aids understanding
- Provide ASCII alternatives for input methods

## Further Reading

For deeper understanding:

- **The HoTT Book** — *Homotopy Type Theory: Univalent Foundations of Mathematics*
- **Programming in Martin-Löf's Type Theory** — Bengt Nordström, Kent Petersson, Jan M. Smith
- **Categories for the Working Mathematician** — Saunders Mac Lane
- **Practical Foundations of Mathematics** — Paul Taylor
- **Type-Driven Development with Idris** — Edwin Brady

For ProveIt-specific information:

- [Architecture](../ARCHITECTURE.md)
- [Technical Manual](tex/proveit-manual.tex)
- [Tutorials](tutorials/)

## Contributing to This Glossary

This glossary grows with the project. To suggest additions or corrections:

1. Open a PR modifying this file
2. Use clear, accessible definitions
3. Provide examples where helpful
4. Cross-reference related terms
5. Cite authoritative sources for technical definitions

We aim for definitions that are:

- **Accurate** without being overly formal
- **Accessible** to readers with varying backgrounds
- **Useful** for understanding the codebase
- **Concise** without sacrificing clarity
