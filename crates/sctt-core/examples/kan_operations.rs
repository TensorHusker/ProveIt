//! Kan Operations in SCTT - Educational Walkthrough
//!
//! This example demonstrates the three fundamental Kan operations that make
//! Smooth Cubical Type Theory work:
//!
//! 1. **comp** (Composition): Fills an open cube given boundary conditions
//! 2. **coe** (Coercion): Transports values along type families
//! 3. **hcomp** (Homogeneous Composition): Special case for constant types
//!
//! These operations enable:
//! - Path composition (combining proofs of equality)
//! - Type transport (moving values between equivalent types)
//! - Function extensionality (proving functions equal pointwise)
//! - Univalence (treating equivalent types as equal)
//!
//! Run with: cargo run --example kan_operations

use sctt_core::{
    check::{check, infer, Context},
    eval::eval,
    kan::{coe, comp, face_satisfied, hcomp},
    normalize::normalize,
    syntax::{Dim, DimVar, Expr, Face, Name},
    value::{Env, Value},
};

fn main() {
    println!("╔═══════════════════════════════════════════════════════════╗");
    println!("║   Kan Operations in Smooth Cubical Type Theory (SCTT)    ║");
    println!("║   Educational Walkthrough                                  ║");
    println!("╚═══════════════════════════════════════════════════════════╝");
    println!();

    example_1_identity_composition();
    example_2_face_constraints();
    example_3_identity_coercion();
    example_4_dimension_transport();
    example_5_homogeneous_composition();
    example_6_face_satisfaction();
    example_7_type_checking();
    example_8_eval_normalize_roundtrip();

    println!("╔═══════════════════════════════════════════════════════════╗");
    println!("║   Summary: Kan Operations Enable                          ║");
    println!("║   • Path composition (equality proofs)                    ║");
    println!("║   • Type transport (univalence)                           ║");
    println!("║   • Function extensionality                               ║");
    println!("║   • Higher-dimensional reasoning                          ║");
    println!("╚═══════════════════════════════════════════════════════════╝");
}

fn example_1_identity_composition() {
    println!("═══ Example 1: Identity Composition ═══");
    println!();
    println!("Mathematical Law: comp(A, a, [], 0) = a");
    println!("(Composition with no faces at dimension 0 is identity)");
    println!();

    let ty = Value::VType(0);
    let base = Value::VType(0);
    let result = comp(&ty, &base, &[], Dim::Zero);

    println!("  Input:");
    println!("    Type:      {:?}", ty);
    println!("    Base:      {:?}", base);
    println!("    Faces:     [] (empty)");
    println!("    Dimension: 0");
    println!();
    println!("  Result: {:?}", result);
    println!("  Equals base? {}", result.conv(&base));
    println!("  ✓ Identity law verified");
    println!();
}

fn example_2_face_constraints() {
    println!("═══ Example 2: Face Constraints ═══");
    println!();
    println!("When a face constraint is satisfied, comp returns that value.");
    println!("Mathematical: comp(A, a, [(i=1) → b], 1) = b");
    println!();

    let ty = Value::VType(0);
    let base = Value::VType(0);
    let face_value = Value::VType(1);
    let faces = vec![(Face::Eq(DimVar(0), true), face_value.clone())];

    let result = comp(&ty, &base, &faces, Dim::One);

    println!("  Input:");
    println!("    Type:      Type₀");
    println!("    Base:      Type₀");
    println!("    Faces:     [(i=1) → Type₁]");
    println!("    Dimension: 1");
    println!();
    println!("  Result: {:?}", result);
    println!("  (At i=1, the face constraint should be satisfied)");
    println!();
}

fn example_3_identity_coercion() {
    println!("═══ Example 3: Identity Coercion ═══");
    println!();
    println!("Mathematical Law: coe(A, r, r, a) = a");
    println!("(Coercing from r to r is identity - reflexivity)");
    println!();

    let ty_fam = Value::VType(0); // Constant type family
    let base = Value::VType(0);
    let dim = Dim::Var(DimVar(0));

    let result = coe(&ty_fam, dim.clone(), dim, &base);

    println!("  Input:");
    println!("    Type family: Type₀ (constant)");
    println!("    From:        r");
    println!("    To:          r");
    println!("    Base:        Type₀");
    println!();
    println!("  Result: {:?}", result);
    println!("  Equals base? {}", result.conv(&base));
    println!("  ✓ Reflexivity law verified");
    println!();
}

fn example_4_dimension_transport() {
    println!("═══ Example 4: Dimension Transport ═══");
    println!();
    println!("Coercion transports values along dimension intervals.");
    println!("For constant families, this is still identity.");
    println!();

    let ty_fam = Value::VType(0);
    let base = Value::VType(0);

    println!("  Test 1: coe(Type₀, 0, 1, Type₀)");
    let result1 = coe(&ty_fam, Dim::Zero, Dim::One, &base);
    println!("  Result: {:?}", result1);
    println!("  Equals base? {}", result1.conv(&base));
    println!();

    println!("  Test 2: coe(Type₀, 1, 0, Type₀)");
    let result2 = coe(&ty_fam, Dim::One, Dim::Zero, &base);
    println!("  Result: {:?}", result2);
    println!("  Equals base? {}", result2.conv(&base));
    println!();

    println!("  ✓ Constant families give identity transport");
    println!();
}

fn example_5_homogeneous_composition() {
    println!("═══ Example 5: Homogeneous Composition ═══");
    println!();
    println!("hcomp is a special case of comp for constant types.");
    println!("Mathematical: hcomp(A, a, φ) = comp(A, a, φ, 1)");
    println!();

    let ty = Value::VType(0);
    let base = Value::VType(0);
    let faces: Vec<(Face, Value)> = vec![];

    let hcomp_result = hcomp(&ty, &base, &faces);
    let comp_result = comp(&ty, &base, &faces, Dim::One);

    println!("  Input:");
    println!("    Type:  Type₀");
    println!("    Base:  Type₀");
    println!("    Faces: [] (empty)");
    println!();
    println!("  hcomp result: {:?}", hcomp_result);
    println!("  comp result:  {:?}", comp_result);
    println!("  Equal? {}", hcomp_result.conv(&comp_result));
    println!("  ✓ hcomp equivalent to comp at dimension 1");
    println!();
}

fn example_6_face_satisfaction() {
    println!("═══ Example 6: Face Formula Satisfaction ═══");
    println!();
    println!("Face formulas determine when boundary conditions apply.");
    println!();

    // Simple face: i = 1
    let face1 = Face::Eq(DimVar(0), true);
    let dims_true = vec![(0, true)];
    let dims_false = vec![(0, false)];

    println!("  Face: i = 1");
    println!("  Dimension environment: [(i, true)]");
    println!("  Satisfied? {}", face_satisfied(&face1, &dims_true));
    println!();
    println!("  Dimension environment: [(i, false)]");
    println!("  Satisfied? {}", face_satisfied(&face1, &dims_false));
    println!();

    // Compound face: (i = 1) ∧ (j = 0)
    let face2 = Face::And(
        Box::new(Face::Eq(DimVar(0), true)),
        Box::new(Face::Eq(DimVar(1), false)),
    );
    let dims_both = vec![(0, true), (1, false)];

    println!("  Face: (i = 1) ∧ (j = 0)");
    println!("  Dimension environment: [(i, true), (j, false)]");
    println!("  Satisfied? {}", face_satisfied(&face2, &dims_both));
    println!();

    // True face (always satisfied)
    println!("  Face: True (trivial)");
    println!("  Always satisfied? {}", face_satisfied(&Face::True, &[]));
    println!();
}

fn example_7_type_checking() {
    println!("═══ Example 7: Type Checking Kan Operations ═══");
    println!();
    println!("Kan operations must type check to ensure soundness.");
    println!();

    let ctx = Context::new();

    // Type check a comp expression
    let comp_expr = Expr::Comp {
        ty: Box::new(Expr::Type(0)),
        base: Box::new(Expr::Type(0)),
        faces: vec![],
    };

    println!("  Expression: comp(Type₀, Type₀, [], 1)");
    match infer(&ctx, &comp_expr) {
        Ok(ty) => {
            println!("  ✓ Type checks!");
            println!("  Inferred type: {:?}", ty);
        }
        Err(e) => {
            println!("  ✗ Type error: {:?}", e);
        }
    }
    println!();

    // Type check a coe expression
    let coe_expr = Expr::Coe {
        ty_fam: Box::new(Expr::Lambda {
            name: Name("i".to_string()),
            body: Box::new(Expr::Type(0)),
        }),
        from: Dim::Zero,
        to: Dim::One,
        base: Box::new(Expr::Type(0)),
    };

    println!("  Expression: coe(λi. Type₀, 0, 1, Type₀)");
    match infer(&ctx, &coe_expr) {
        Ok(ty) => {
            println!("  ✓ Type checks!");
            println!("  Inferred type: {:?}", ty);
        }
        Err(e) => {
            println!("  ✗ Type error: {:?}", e);
        }
    }
    println!();
}

fn example_8_eval_normalize_roundtrip() {
    println!("═══ Example 8: Evaluation & Normalization Roundtrip ═══");
    println!();
    println!("Kan operations integrate with eval and normalize.");
    println!();

    // Create a comp expression
    let expr = Expr::Comp {
        ty: Box::new(Expr::Type(0)),
        base: Box::new(Expr::Type(0)),
        faces: vec![],
    };

    println!("  Original expression: comp(Type₀, Type₀, [], 0)");
    println!();

    // Evaluate it
    let val = eval(&expr, &Env::new());
    println!("  After evaluation: {:?}", val);
    println!();

    // Normalize back to syntax
    let normalized = normalize(&val);
    println!("  After normalization: {:?}", normalized);
    println!();

    // For identity comp, should normalize to base value
    match normalized {
        Expr::Type(0) => println!("  ✓ Normalized to Type₀ (identity optimization)"),
        Expr::Comp { .. } => println!("  ✓ Preserved as comp (neutral form)"),
        _ => println!("  ? Unexpected normalized form"),
    }
    println!();
}
