# Tutorial 1: What is Formal Verification?

**Path**: First Steps
**Level**: Beginner
**Prerequisites**: None
**Estimated time**: 20 minutes

## Goal

By the end of this tutorial, you will understand:

- What formal verification means
- Why it differs from testing
- How ProveIt approaches verification differently
- When formal verification is useful

## What is a Proof?

When mathematicians say something is "proven," they don't mean "we tested it and it seems to work." They mean: starting from agreed-upon facts (axioms) and using agreed-upon rules of reasoning (inference rules), they have shown that the statement *must* be true.

> **Example**: Proving that `2 + 2 = 4`
>
> A proof might proceed:
> 1. Start with the axioms of arithmetic
> 2. Apply the definition of addition
> 3. Show that following these rules leads to `4`
>
> This isn't checking many examples — it's showing the result holds *necessarily*.

## What is Formal Verification?

**Formal verification** is the use of mathematical proof to establish properties of systems, especially computer programs.

Instead of running tests and seeing if anything breaks, we *prove* that the program meets its specification.

### Verification vs. Testing

| Aspect | Testing | Formal Verification |
|--------|---------|---------------------|
| Scope | Specific cases | All cases |
| Confidence | Probabilistic | Mathematical certainty |
| Effort | Often less | Often more |
| Catches | Bugs you thought of | Bugs in the entire space |
| Tools | Test runners | Proof assistants |

### A Concrete Example

Suppose we have a function `divide(a, b)`. We might want to verify:

> "For all integers `a` and `b`, if `b ≠ 0`, then `divide(a, b) * b + remainder(a, b) = a`."

**Testing approach**:
- Try `divide(10, 3)`: returns `3`, remainder is `1`. Check: `3*3 + 1 = 10`. ✓
- Try `divide(7, 2)`: returns `3`, remainder is `1`. Check: `3*2 + 1 = 7`. ✓
- Try `divide(0, 5)`: returns `0`, remainder is `0`. Check: `0*5 + 0 = 0`. ✓
- And so on for many cases.

This gives us confidence, but doesn't *prove* the property holds for *every* possible input.

**Formal verification approach**:
- State the property mathematically
- Use the rules of arithmetic and the definition of `divide` to derive a proof
- The proof checker verifies our reasoning is valid
- We now know the property holds for *all* inputs

## Why Bother?

Formal verification is more work. Why use it?

### When Bugs Are Catastrophic

Some bugs are not just inconvenient:

- Aerospace systems can kill people
- Medical devices can harm patients
- Cryptographic protocols can leak secrets
- Financial systems can lose money

For these, the extra effort of formal verification can be justified.

### When Testing is Inadequate

Some properties are hard to test:

- "This algorithm terminates for all inputs"
- "This protocol prevents all unauthorized access"
- "This data structure maintains its invariants under any sequence of operations"

Testing can find counterexamples but rarely prove absence of bugs.

### When Specifications Need Clarity

The act of writing a formal specification often clarifies what the system should do. Many bugs come from unclear requirements; formalization helps.

## Why is Formal Verification Hard?

If proofs are so powerful, why isn't everything formally verified?

### 1. Steep Learning Curve

Formal methods involve:
- Mathematical logic
- Type theory
- Category theory (sometimes)
- Specialized tools with their own languages

Most developers haven't studied these.

### 2. Verbose

Proving simple properties can require many proof steps. Programs that take 100 lines to write might take 1000 lines to prove correct.

### 3. Specifications Are Hard

Often, the hardest part is figuring out what to prove. "The program is correct" is not a precise specification.

### 4. Tool Friction

Traditional proof assistants are powerful but unforgiving. Small mistakes produce intimidating error messages. The interaction is often text-only with limited modalities.

## How is ProveIt Different?

ProveIt approaches these problems by:

### 1. Geometric Reasoning

Instead of typing abstract syntax, you construct geometric objects. The geometry encodes the logic.

**Why this helps**:
- Spatial reasoning is intuitive for many people
- Visualization (or audio-spatial representation) makes structure clear
- Mistakes are often visible (or audible)

### 2. Multiple Modalities

Every concept is presented through multiple senses:
- Visual: traditional rendering
- Audio: spatial cues, distinct tones
- Haptic: vibration patterns
- Text: descriptive labels

**Why this helps**:
- Users with different abilities can all participate
- Multiple modalities reinforce understanding
- No information is lost when one channel is unavailable

### 3. Real-Time Feedback

As you build a proof, ProveIt verifies each step immediately.

**Why this helps**:
- Mistakes are caught early
- Successful steps build confidence
- The proof is always in a known-good state

### 4. Multiple Formal Methods

You can use the formalism best suited to your problem:
- Type theory for general reasoning
- Category theory for structural arguments
- Homotopy type theory for higher-dimensional reasoning

**Why this helps**:
- Different problems benefit from different formalisms
- Translations between systems make work portable
- You learn formalisms by using them, not before using them

## What Can You Verify with ProveIt?

ProveIt is designed for:

### Mathematical Theorems

Classical results, modern theorems, your own original work.

### Algorithm Correctness

Proving that algorithms do what they claim.

### Type-Level Properties

Properties expressible in dependent types.

### Geometric Theorems

Where the geometric construction *is* the proof.

### Educational Material

Building understanding through interactive construction.

## What Can't You Verify (Yet)?

Honest limitations:

- **Concurrent program correctness**: requires specialized formalisms
- **Real-time properties**: timing is hard to formalize
- **Statistical properties**: probabilistic reasoning is a different beast
- **Hardware verification**: there are specialized tools for this

## A Mental Model

Think of formal verification as **mathematical pair programming**:

- You propose a step
- The verifier (your "pair") checks it
- If the step is valid, you continue
- If not, the verifier explains the issue
- Together, you build a complete, verified proof

ProveIt's geometric interface makes this pairing more natural — you're constructing something visible, not just typing code at an interpreter.

## Exercise

You don't need to do anything technical for this exercise — just think about it.

**Question**: What's something in your life or work where you'd want mathematical certainty rather than probabilistic confidence?

Examples to consider:

- Code that handles your bank account
- Software in your car
- Medical devices you depend on
- Cryptographic protocols protecting your data

Write down (mentally or on paper):

1. What system?
2. What property would you want to verify?
3. What would the consequences be if the property failed?

This exercise is to ground formal verification in real motivations. Throughout these tutorials, when concepts feel abstract, return to concrete cases like these.

## Summary

You learned:

- **Formal verification** uses mathematical proof to establish system properties
- It differs from **testing** by covering all cases, not just some
- It's **harder than testing** but provides certainty when you need it
- **ProveIt** aims to make formal verification more accessible through geometric construction and multi-modal interfaces

## What's Next?

Continue your journey:

- **Next tutorial**: [Your First Proof](02-your-first-proof.md) — actually build something
- **Background reading**: [What is a Type?](03-understanding-types.md)
- **Glossary**: [Common Terms](../GLOSSARY.md)
- **Architecture**: [How ProveIt Works](../../ARCHITECTURE.md)

## Further Reading

- *Software Foundations* by Pierce et al. — accessible online textbook
- *Type-Driven Development with Idris* by Edwin Brady — practical book
- *The Little Typer* by Friedman & Christiansen — gentle introduction
- *Theorem Proving in Lean* — modern proof assistant tutorial

## Questions?

- Check the [Glossary](../GLOSSARY.md)
- Ask in [GitHub Discussions](https://github.com/TensorHusker/ProveIt/discussions)
- Open an issue if something is unclear
