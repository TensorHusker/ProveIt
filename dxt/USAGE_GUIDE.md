# ProveIt Desktop Extension: Usage Guide

**Welcome to formal proof assistance with ProveIt and Claude!**

This guide will teach you how to build rigorous mathematical proofs using natural conversation with Claude Desktop. Whether you're new to formal verification or an experienced type theorist, you'll find your path here.

---

## Table of Contents

1. [Getting Started](#getting-started)
2. [Your First Proof](#your-first-proof)
3. [Basic Workflows](#basic-workflows)
4. [Interactive Proof Building](#interactive-proof-building)
5. [Geometric Proof Construction](#geometric-proof-construction)
6. [Advanced Features](#advanced-features)
7. [Complete Example Sessions](#complete-example-sessions)
8. [Best Practices](#best-practices)
9. [Prompt Patterns](#prompt-patterns)
10. [Troubleshooting](#troubleshooting)

---

## Getting Started

### Prerequisites

✅ ProveIt DXT installed and configured (see [README.md](README.md))
✅ Claude Desktop running
✅ Basic understanding of mathematical logic (we'll guide you!)

### Verify Installation

**Try this prompt** (copy and paste into Claude):

```
Can you check if ProveIt is available? List the available tactics.
```

**Expected response**: Claude should show you a list of proof tactics (intro, apply, refl, etc.)

If this works, you're ready! 🎉

---

## Your First Proof

Let's prove something simple: **every number equals itself**.

### The Simplest Proof

**Prompt:**
```
Using ProveIt, help me prove that for any natural number n, we have a path from n to n.
```

**What happens:**
1. Claude will start a proof session
2. Apply the `intro` tactic to introduce `n`
3. Apply the `refl` (reflexivity) tactic to complete the proof
4. Show you the completed proof

**Expected Claude response:**
```
Let me help you prove reflexivity for natural numbers using ProveIt.

[Claude uses start_proof with goal "∀(n : Nat). Path Nat n n"]
[Claude uses apply_tactic with "intro"]
[Claude uses apply_tactic with "refl"]

✓ Proof complete! We've shown that every natural number has a reflexive path to itself.
```

### Understanding What Happened

- **`∀(n : Nat)`**: "For all natural numbers n"
- **`Path Nat n n`**: A path from n to itself (equality in type theory)
- **`intro`**: Introduced the variable n
- **`refl`**: Applied reflexivity (things equal themselves)

**Congratulations! You just built your first formal proof.** 🎊

---

## Basic Workflows

### Workflow 1: Check If Something Type Checks

**Use Case**: Verify that an expression makes sense mathematically.

**Prompt (copy-ready):**
```
Check if this expression type checks in ProveIt: "λ(x : Nat). x"
```

**What you'll learn**: The type of the identity function on natural numbers.

**Expected result**: Type is `Nat → Nat`

---

### Workflow 2: Prove a Simple Implication

**Use Case**: Show that if A is true, then A is true (tautology).

**Prompt:**
```
Help me prove A → A using ProveIt, where A is any type.
```

**Steps Claude will take:**
1. Start proof with goal `A → A`
2. Use `intro` to assume A
3. Use `assumption` to conclude with A
4. Proof complete!

**What you'll learn**: How implications work in type theory.

---

### Workflow 3: Save Your Work

**Prompt:**
```
Save this proof to a file called "my_first_proof.json"
```

**What happens**: Your proof is saved and you can load it later.

**To load it later:**
```
Load the proof from "my_first_proof.json"
```

---

## Interactive Proof Building

Let's build a more complex proof step-by-step.

### Goal: Prove Symmetry of Paths

**Mathematical statement**: If there's a path from x to y, there's a path from y to x.

### Step-by-Step Tutorial

#### Step 1: Start the Proof

**Prompt:**
```
Start a new proof in ProveIt. I want to prove that paths are symmetric:
if we have a path from x to y, then we have a path from y to x.
```

**What Claude does:**
```
start_proof({
  "goal": "∀(A : Type). ∀(x y : A). Path A x y → Path A y x",
  "name": "path_symmetry"
})
```

**Current state**: We need to prove the statement for all types A and elements x, y.

#### Step 2: Introduce Variables

**Prompt:**
```
Introduce the type A and elements x and y.
```

**What Claude does:**
```
apply_tactic({ "tactic": "intro", "arguments": ["A"] })
apply_tactic({ "tactic": "intro", "arguments": ["x"] })
apply_tactic({ "tactic": "intro", "arguments": ["y"] })
```

**Current state**: We now assume A is a type, x and y are elements of A.

#### Step 3: Assume the Path

**Prompt:**
```
Now assume we have a path p from x to y.
```

**What Claude does:**
```
apply_tactic({ "tactic": "intro", "arguments": ["p"] })
```

**Current state**: We have `p : Path A x y` and need to construct `Path A y x`.

#### Step 4: Apply Symmetry

**Prompt:**
```
Apply path symmetry to p.
```

**What Claude does:**
```
apply_tactic({ "tactic": "apply", "arguments": ["path_symm", "p"] })
```

**Result**: ✓ Proof complete!

### Check Your Work

**Prompt:**
```
Show me the current proof state with full details.
```

**Expected response**: Complete proof term showing how we constructed the symmetric path.

---

## Geometric Proof Construction

**This is where ProveIt shines!** Proofs are spatial constructions.

### Understanding the Geometric Model

In ProveIt:
- **Points** = Propositions (statements)
- **Lines** = Implications (if-then relationships)
- **Geometric configurations** = Logical arguments

### Example: Modus Ponens as a Triangle

**Mathematical statement**: If A → B and A is true, then B is true.

**Geometric interpretation**:
```
     A (point)
    / \
   /   \
  /     \
 B       A→B
  \     /
   \   /
    \ /
     B (conclusion)
```

**Three points form a triangle**:
1. Point A (assumption)
2. Point B (conclusion)
3. Implication A → B

### Building Geometric Proofs

**Prompt (copy-ready):**
```
Using ProveIt's geometric construction, help me build a proof that:
if P implies Q, and Q implies R, then P implies R.

Use points for propositions and lines for implications.
```

**What Claude will do:**

1. Create three points: P, Q, R
2. Create two lines: P → Q and Q → R
3. Detect the transitive chain
4. Extract the formal proof: P → R

**The geometric construction:**
```
construct_geometric_proof({
  "points": [
    { "id": "P", "proposition": "P" },
    { "id": "Q", "proposition": "Q" },
    { "id": "R", "proposition": "R" }
  ],
  "lines": [
    { "from": "P", "to": "Q", "implication": "P → Q" },
    { "from": "Q", "to": "R", "implication": "Q → R" }
  ]
})
```

**Result**: A formal proof that P → R, extracted from the geometric construction!

### Why This Matters

**For visual thinkers**: You can "see" the proof structure.
**For accessibility**: Geometric proofs can be described spatially with audio cues.
**For intuition**: Complex logical relationships become spatial patterns.

---

## Advanced Features

### Feature 1: Expression Normalization

**Use Case**: Simplify complex expressions to see what they really mean.

**Prompt:**
```
Normalize the expression "(λ(x : Nat). x + 1) 5" using ProveIt.
```

**Expected result**: `6` (the function applied to 5)

**Strategies available**:
- `nbe`: Normalization by Evaluation (full computation)
- `whnf`: Weak Head Normal Form (partial simplification)
- `full`: Complete reduction

**Advanced prompt:**
```
Normalize this expression using weak head normal form strategy:
"(λ(f : Nat → Nat). λ(x : Nat). f (f x)) (λ(y : Nat). y + 1) 3"
```

---

### Feature 2: Theorem Querying

**Use Case**: Find existing theorems and definitions.

**Prompt:**
```
Search ProveIt for theorems about path composition.
```

**What you get**: List of relevant theorems with their types and descriptions.

**Filtering options**:
- `"all"`: Everything
- `"theorems"`: Only proven theorems
- `"definitions"`: Only definitions
- `"tactics"`: Only tactics

**Example:**
```
Search for definitions related to "equality" in ProveIt, filter by definitions only.
```

---

### Feature 3: Type Checking with Context

**Use Case**: Check types in a specific mathematical context.

**Prompt:**
```
In a context where n has type Nat and P is a property of naturals,
check the type of the expression "P n".
```

**What Claude does:**
```
check_type({
  "expression": "P n",
  "context": ["n : Nat", "P : Nat → Type"]
})
```

**Result**: `Type` (it's a proposition about n)

---

## Complete Example Sessions

### Session 1: Prove Commutativity of And

**Complete conversation:**

**You:**
```
I want to prove that A ∧ B implies B ∧ A using ProveIt.
Walk me through it step by step.
```

**Claude:**
```
Let's prove that conjunction is commutative! I'll guide you through each step.

First, I'll start the proof:
[Starts proof with goal "∀(A B : Type). A ∧ B → B ∧ A"]

The proof state now shows we need to prove this for all types A and B.

Step 1: Introduce the types A and B
[Applies intro tactic twice]

Step 2: Assume we have a proof of A ∧ B (call it H)
[Applies intro for H]

Now we have:
- A : Type
- B : Type
- H : A ∧ B

And we need to construct: B ∧ A

Step 3: Extract the two components from H
[Applies destruct on H to get H1 : A and H2 : B]

Step 4: Construct B ∧ A from the pieces
[Applies constructor to build pair (H2, H1)]

✓ Proof complete!

We showed that if we have both A and B, we can rearrange them as B and A.
```

**You:**
```
Great! Save this as "and_commutative.json"
```

**Claude:**
```
[Saves the proof]
✓ Proof saved to "and_commutative.json"
```

---

### Session 2: Understanding Function Composition

**You:**
```
Help me understand function composition in ProveIt.
Can you prove that composing two functions works?
```

**Claude:**
```
Absolutely! Let's explore function composition.

Function composition takes two functions:
- f : B → C
- g : A → B

And creates a new function: f ∘ g : A → C

Let me prove this composition is well-typed:

[Checks type: "λ(A B C : Type). λ(f : B → C). λ(g : A → B). λ(x : A). f (g x)"]

✓ Type checks as:
  ∀(A B C : Type). (B → C) → (A → B) → (A → C)

This confirms that function composition is a valid operation in our type theory!

Let me normalize an example:
[Normalizes: "(λ(f : Nat → Nat). λ(g : Nat → Nat). λ(x : Nat). f (g x))
              (λ(n : Nat). n + 1)
              (λ(m : Nat). m * 2)
              5"]

Result: 11
(Because: 5 * 2 = 10, then 10 + 1 = 11)

Would you like to see more examples of composition?
```

---

### Session 3: Geometric Proof - Syllogism

**You:**
```
Show me a geometric proof of the classic syllogism:
"All humans are mortal. Socrates is human. Therefore Socrates is mortal."
```

**Claude:**
```
Excellent choice! Let's build this as a geometric proof.

The three propositions become points:
- Point H: "Socrates is human"
- Point M: "Socrates is mortal"
- Point All: "All humans are mortal"

The logical structure has two implications:
- Line 1: H → M (from the universal rule "all humans are mortal")
- We need to prove: H ⊢ M (from H we can derive M)

[Constructs geometric proof with appropriate points and lines]

The geometric construction forms a simple path:
H ——→ M

By following the implication line, we can traverse from the assumption
"Socrates is human" to the conclusion "Socrates is mortal."

The extracted formal proof uses modus ponens:
Given H : Human(Socrates)
Given All : ∀x. Human(x) → Mortal(x)
Apply All to Socrates to get: Human(Socrates) → Mortal(Socrates)
Apply modus ponens with H to conclude: Mortal(Socrates)

✓ Geometric proof complete and verified!
```

---

## Best Practices

### 1. Start Simple, Build Up

❌ **Don't**: "Prove the Yoneda lemma in ProveIt"
✅ **Do**: "Let's prove identity first, then function composition, then work toward Yoneda"

**Why**: Complex proofs are built from simple pieces. Master the basics first.

---

### 2. Ask for Explanations

**Effective prompts:**
```
"Before we prove this, explain what this statement means in plain English."

"Why did you use the intro tactic here?"

"What does this proof term actually represent?"
```

**Why**: Understanding beats memorization. Ask Claude to teach, not just prove.

---

### 3. Save Intermediate Results

**Workflow:**
1. Prove a lemma
2. Save it: `"Save this as lemma_1.json"`
3. Use it later: `"Load lemma_1 and use it to prove the main theorem"`

**Why**: Build a personal library of proven results.

---

### 4. Use Geometric Thinking

**For spatial reasoners:**
```
"Show me this proof as a geometric construction first,
then extract the formal proof."
```

**Why**: Geometry makes abstract logic concrete.

---

### 5. Iterate and Refine

**If stuck:**
```
"That tactic didn't work. What are my other options?"

"Can you explain why this proof is failing?"

"Let's try a different approach to this proof."
```

**Why**: Proof discovery is iterative. Embrace exploration.

---

### 6. Check Your Assumptions

**Regular reality check:**
```
"Show me the current proof state. What do we still need to prove?"

"What assumptions are we currently working with?"
```

**Why**: Easy to lose track in complex proofs. Check frequently.

---

## Prompt Patterns

### Pattern 1: The Learning Request

**Structure**: "Help me understand [concept] by proving [simple example]"

**Examples:**
```
"Help me understand dependent types by proving a simple theorem using them."

"Help me understand path types by showing how equality works in SCTT."

"Help me understand induction by proving something about natural numbers."
```

**Why it works**: Combines theory with practice.

---

### Pattern 2: The Guided Proof

**Structure**: "I want to prove [goal]. Guide me step-by-step and explain each step."

**Examples:**
```
"I want to prove that addition is commutative. Guide me step-by-step."

"I want to prove function composition is associative. Explain each step."
```

**Why it works**: You learn the tactics while building the proof.

---

### Pattern 3: The Geometric Translation

**Structure**: "Show me [logical concept] as a geometric construction."

**Examples:**
```
"Show me how 'if-then-else' works as a geometric construction."

"Visualize proof by contradiction geometrically."

"Show me De Morgan's laws as geometric transformations."
```

**Why it works**: Leverages ProveIt's unique geometric proof system.

---

### Pattern 4: The Type Explorer

**Structure**: "What is the type of [expression]? What does that mean?"

**Examples:**
```
"What is the type of 'λ(x : A). λ(y : B). (x, y)'? What does that mean?"

"Check the type of 'Path' and explain what paths represent in SCTT."
```

**Why it works**: Type checking teaches you the type system.

---

### Pattern 5: The Proof Debugger

**Structure**: "I'm trying to prove [goal] but getting stuck at [step]. Help me debug."

**Examples:**
```
"I'm trying to prove A → B → A but the intro tactic isn't working. Help me debug."

"My proof of transitivity is failing at the composition step. What's wrong?"
```

**Why it works**: Claude can analyze the proof state and suggest fixes.

---

### Pattern 6: The Batch Prover

**Structure**: "Prove these three related facts: [list]. Show connections between them."

**Examples:**
```
"Prove these three facts about paths:
1. Reflexivity: Path A x x
2. Symmetry: Path A x y → Path A y x
3. Transitivity: Path A x y → Path A y z → Path A x z

Show how they relate."
```

**Why it works**: Builds a coherent understanding of related concepts.

---

### Pattern 7: The Real-World Connection

**Structure**: "Translate [real-world reasoning] into a formal proof."

**Examples:**
```
"Translate this argument into ProveIt: 'If it's raining, the ground is wet.
It's raining. Therefore the ground is wet.'"

"Formalize this reasoning: 'All prime numbers greater than 2 are odd.'"
```

**Why it works**: Connects formal methods to intuitive reasoning.

---

## Troubleshooting

### Problem 1: "ProveIt tools not available"

**Symptoms**: Claude says it can't access ProveIt tools.

**Solutions**:
1. Check Claude Desktop config file has correct path
2. Restart Claude Desktop
3. Verify DXT is built: `ls dxt/dist/index.js`
4. Check logs: Set `PROVEIT_LOG_LEVEL=debug` in config

---

### Problem 2: "Tactic application failed"

**Symptoms**: Claude reports a tactic didn't work.

**Solutions**:
1. Ask: "What's the current proof state?"
2. Ask: "Why did that tactic fail? What should I try instead?"
3. Ask: "List all available tactics for this goal type"

**Remember**: Failed tactics are learning opportunities!

---

### Problem 3: "Type checking error"

**Symptoms**: Expression doesn't type check.

**Solutions**:
1. Ask: "Explain this type error in simple terms"
2. Ask: "What type did you expect vs. what type did you get?"
3. Simplify: Break complex expression into parts and check each

**Example debugging session:**
```
You: "Check type of '(λ(x : Nat). x) true'"

Claude: "Type error: Expected Nat but got Bool"

You: "Ah, I see. What if I use 5 instead of true?"

Claude: "✓ Type checks! Result: Nat"
```

---

### Problem 4: "Proof is stuck"

**Symptoms**: Can't figure out next step.

**Solutions**:
1. **Get current state**: "Show me the proof state with all assumptions"
2. **Get suggestions**: "What tactics might work here?"
3. **Try simpler goal**: "Can we prove a simpler version first?"
4. **Use automation**: "Can auto-search help with this subgoal?"

---

### Problem 5: "Don't understand the proof term"

**Symptoms**: Proof complete but confused about what it means.

**Solutions**:
1. Ask: "Explain this proof term in plain English"
2. Ask: "Walk through the proof step-by-step and explain the reasoning"
3. Ask: "Show me this proof geometrically"

**Remember**: Understanding > completion. Always ask for explanations!

---

## Next Steps

### Beginner Track

1. ✅ Complete "Your First Proof"
2. ✅ Try all three basic workflows
3. ✅ Work through one complete example session
4. 📚 Read about type theory: See [MATHEMATICAL_FOUNDATION.md](../MATHEMATICAL_FOUNDATION.md)
5. 🎯 Prove 10 simple theorems on your own

### Intermediate Track

1. ✅ Master all proof tactics (see `list_tactics`)
2. ✅ Build your first geometric proof
3. ✅ Create a library of proven lemmas
4. 📚 Study SCTT: Smooth Cubical Type Theory concepts
5. 🎯 Prove theorems requiring induction

### Advanced Track

1. ✅ Explore advanced normalization strategies
2. ✅ Prove metatheoretical properties
3. ✅ Extend the theorem library
4. 📚 Read research papers on cubical type theory
5. 🎯 Contribute new tactics or proof techniques

---

## Resources

- **Main Documentation**: [README.md](../README.md)
- **Design Vision**: [DESIGN_UX.md](../DESIGN_UX.md)
- **Implementation**: [IMPLEMENTATION_PLAN.md](../IMPLEMENTATION_PLAN.md)
- **Mathematical Foundations**: [MATHEMATICAL_FOUNDATION.md](../MATHEMATICAL_FOUNDATION.md)
- **SCTT Overview**: [crates/sctt-core/](../crates/sctt-core/)

---

## Community & Support

### Getting Help

**If stuck**: Ask Claude! Use the debugging prompt patterns above.

**For bugs**: Open an issue on GitHub

**For discussions**: Join the ProveIt community discussions

### Contributing

**Found a useful prompt pattern?** Share it!

**Proved something interesting?** Add it to the examples!

**Improved the guide?** Submit a PR!

---

## Closing Thoughts

**You're learning formal verification - one of the most rigorous forms of mathematical reasoning.** It's challenging, but incredibly rewarding.

**Remember**:
- ✨ Every mathematician started as a beginner
- 🧠 Understanding takes time - be patient with yourself
- 🎯 Small proofs build to big theorems
- 🤝 Claude is here to help - ask questions freely
- 🎉 Celebrate each completed proof!

**Welcome to the ProveIt community. Happy proving!** 🚀

---

*Guide Version: 1.0*
*Last Updated: 2025-10-26*
*Feedback Welcome: Open an issue or PR*
