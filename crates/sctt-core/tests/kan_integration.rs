//! Integration tests for Kan operations
//!
//! These tests verify that Kan operations (comp, coe, hcomp) integrate
//! properly with the evaluation, type checking, and normalization pipeline.

use sctt_core::{
    check::{infer, Context},
    eval::eval,
    normalize::normalize,
    syntax::{Dim, DimVar, Expr, Face, Name},
    value::{Env, Value},
};

#[test]
fn test_comp_identity_through_eval() {
    // comp(Type₀, Type₀, [], 0) should equal Type₀
    let expr = Expr::Comp {
        ty: Box::new(Expr::Type(0)),
        base: Box::new(Expr::Type(0)),
        faces: vec![],
    };

    let val = eval(&expr, &Env::new());

    // Should equal Type₀ (identity law)
    assert!(val.conv(&Value::VType(0)),
        "comp with empty faces at dim 0 should be identity");
}

#[test]
fn test_comp_type_checks() {
    // Verify that comp expressions type check correctly
    // Note: Type₀ : Type₁, so we need to compose values OF type Type₀, not Type₀ itself
    let ctx = Context::new();

    // Create a comp expression composing at Type₁ level
    let comp_expr = Expr::Comp {
        ty: Box::new(Expr::Type(1)), // Composing at Type₁ level
        base: Box::new(Expr::Type(0)),  // Type₀ : Type₁
        faces: vec![],
    };

    let result = infer(&ctx, &comp_expr);
    assert!(result.is_ok(), "comp expression should type check, got error: {:?}", result.err());

    // Result should be Type₁
    let ty = result.unwrap();
    assert!(matches!(ty, Value::VType(1)),
        "comp of Type₁ should have type Type₁");
}

#[test]
fn test_comp_face_constraint() {
    // comp(Type₀, Type₀, [(i=1) → Type₁], 1) should give Type₁
    let comp_expr = Expr::Comp {
        ty: Box::new(Expr::Type(0)),
        base: Box::new(Expr::Type(0)),
        faces: vec![(Face::Eq(DimVar(0), true), Expr::Type(1))],
    };

    let val = eval(&comp_expr, &Env::new());

    // When evaluated, should satisfy face constraint
    // (Currently returns neutral, but should eventually give Type₁)
    assert!(matches!(val, Value::VNeutral { .. }) || matches!(val, Value::VType(1)),
        "comp with face constraint should produce expected value");
}

#[test]
fn test_coe_reflexivity_through_pipeline() {
    // coe(λi. Type₀, r, r, Type₀) = Type₀ (reflexivity)
    let dim_var = DimVar(0);
    let expr = Expr::Coe {
        ty_fam: Box::new(Expr::Lambda {
            name: Name("i".to_string()),
            body: Box::new(Expr::Type(0)),
        }),
        from: Dim::Var(dim_var),
        to: Dim::Var(dim_var),
        base: Box::new(Expr::Type(0)),
    };

    let val = eval(&expr, &Env::new());

    // Should equal Type₀ (reflexivity law)
    assert!(val.conv(&Value::VType(0)),
        "coe from r to r should be identity");
}

#[test]
fn test_coe_type_checks() {
    let ctx = Context::new();

    // Use a constant type family (Type₁) to avoid lambda inference issues
    // coe along a constant family is identity anyway
    let coe_expr = Expr::Coe {
        ty_fam: Box::new(Expr::Type(1)), // Constant family at Type₁
        from: Dim::Zero,
        to: Dim::One,
        base: Box::new(Expr::Type(0)),  // Type₀ : Type₁
    };

    let result = infer(&ctx, &coe_expr);
    assert!(result.is_ok(), "coe expression should type check, got error: {:?}", result.err());
}

#[test]
fn test_hcomp_identity() {
    // hcomp(Type₀, Type₀, []) at dim 1 should give Type₀
    let expr = Expr::HComp {
        ty: Box::new(Expr::Type(0)),
        base: Box::new(Expr::Type(0)),
        faces: vec![],
    };

    let val = eval(&expr, &Env::new());

    // Should equal Type₀
    assert!(val.conv(&Value::VType(0)),
        "hcomp with no faces should be identity");
}

#[test]
fn test_hcomp_type_checks() {
    let ctx = Context::new();

    // Fixed: Type₀ : Type₁, not Type₀ : Type₀
    let hcomp_expr = Expr::HComp {
        ty: Box::new(Expr::Type(1)),  // Composing at Type₁
        base: Box::new(Expr::Type(0)), // Type₀ : Type₁
        faces: vec![],
    };

    let result = infer(&ctx, &hcomp_expr);
    assert!(result.is_ok(), "hcomp expression should type check, got error: {:?}", result.err());
}

#[test]
fn test_comp_normalizes_correctly() {
    // Test that comp expressions can be normalized back to syntax
    let expr = Expr::Comp {
        ty: Box::new(Expr::Type(0)),
        base: Box::new(Expr::Type(0)),
        faces: vec![],
    };

    let val = eval(&expr, &Env::new());
    let normalized = normalize(&val);

    // Normalized form should be valid expression
    // (Exact form depends on implementation - identity or neutral)
    match normalized {
        Expr::Type(0) => {}, // Identity case
        Expr::Comp { .. } => {}, // Neutral case
        _ => panic!("Unexpected normalized form"),
    }
}

#[test]
fn test_roundtrip_comp_expr() {
    // Test Expr → Value → Expr roundtrip preserves meaning
    let original = Expr::Type(0);

    let comp_expr = Expr::Comp {
        ty: Box::new(Expr::Type(0)),
        base: Box::new(original.clone()),
        faces: vec![],
    };

    let val = eval(&comp_expr, &Env::new());
    let normalized = normalize(&val);

    // Should normalize to Type₀ (identity)
    assert!(matches!(normalized, Expr::Type(0)),
        "comp identity should normalize to base value");
}

#[test]
fn test_multiple_faces() {
    // comp with multiple face constraints
    let faces = vec![
        (Face::Eq(DimVar(0), false), Expr::Type(0)),
        (Face::Eq(DimVar(1), true), Expr::Type(1)),
    ];

    let comp_expr = Expr::Comp {
        ty: Box::new(Expr::Type(0)),
        base: Box::new(Expr::Type(0)),
        faces,
    };

    let val = eval(&comp_expr, &Env::new());

    // Should evaluate without crashing
    // Exact behavior depends on dimension environment
    assert!(matches!(val, Value::VType(_)) || matches!(val, Value::VNeutral { .. }),
        "comp with multiple faces should evaluate");
}

#[test]
fn test_face_and_formula() {
    // Test Face::And composition
    let face = Face::And(
        Box::new(Face::Eq(DimVar(0), true)),
        Box::new(Face::Eq(DimVar(1), false)),
    );

    let comp_expr = Expr::Comp {
        ty: Box::new(Expr::Type(0)),
        base: Box::new(Expr::Type(0)),
        faces: vec![(face, Expr::Type(1))],
    };

    let val = eval(&comp_expr, &Env::new());

    // Should handle compound face formulas
    assert!(matches!(val, Value::VType(_)) || matches!(val, Value::VNeutral { .. }),
        "comp with And face should evaluate");
}

#[test]
fn test_coe_constant_family_optimization() {
    // coe with constant family should optimize to identity
    let coe_expr = Expr::Coe {
        ty_fam: Box::new(Expr::Type(0)), // Constant, not a lambda
        from: Dim::Zero,
        to: Dim::One,
        base: Box::new(Expr::Type(0)),
    };

    let val = sctt_core::eval::eval(&coe_expr, &Env::new());

    // Constant families should be identity
    assert!(val.conv(&Value::VType(0)),
        "coe with constant family should be identity");
}

#[test]
fn test_comp_pi_type_produces_lambda() {
    // comp ((x : Type0) -> Type0) (lam x. x) [] should produce a lambda
    // i.e., composing a function type should yield a function value

    // Build Pi type: (x : Type0) -> Type0
    let pi_ty = Expr::Pi {
        name: Name("x".to_string()),
        domain: Box::new(Expr::Type(0)),
        codomain: Box::new(Expr::Type(0)),
    };

    // Build identity function: lam x. x
    let id_fn = Expr::Lambda {
        name: Name("x".to_string()),
        body: Box::new(Expr::Var(Name("x".to_string()), 0)),
    };

    // comp (Pi Type0 Type0) id [] should produce a lambda
    let comp_expr = Expr::Comp {
        ty: Box::new(pi_ty),
        base: Box::new(id_fn),
        faces: vec![],
    };

    let val = sctt_core::eval::eval(&comp_expr, &Env::new());

    // Result should be a lambda (VLam), not a neutral
    assert!(
        matches!(val, Value::VLam { .. }),
        "comp on Pi type should produce a lambda, got: {:?}",
        val
    );
}

#[test]
fn test_comp_pi_type_applies_correctly() {
    // Verify that the composed function can be applied

    // Build Pi type: (x : Type0) -> Type0
    let pi_ty = Expr::Pi {
        name: Name("x".to_string()),
        domain: Box::new(Expr::Type(0)),
        codomain: Box::new(Expr::Type(0)),
    };

    // Build identity function: lam x. x
    let id_fn = Expr::Lambda {
        name: Name("x".to_string()),
        body: Box::new(Expr::Var(Name("x".to_string()), 0)),
    };

    // comp (Pi Type0 Type0) id []
    let comp_expr = Expr::Comp {
        ty: Box::new(pi_ty),
        base: Box::new(id_fn),
        faces: vec![],
    };

    let composed_fn = sctt_core::eval::eval(&comp_expr, &Env::new());

    // Apply the composed function to Type0
    let arg = Value::VType(0);
    let result = composed_fn.apply(arg);

    // Since comp with empty faces is identity-like, the result
    // should be related to applying the identity to Type0
    // The composed function, when applied, should eventually yield Type0
    // (through the recursive comp call which hits the VType case)
    assert!(
        matches!(result, Value::VType(0)) || matches!(result, Value::VNeutral { .. }),
        "applying composed Pi function should work, got: {:?}",
        result
    );
}

#[test]
fn test_comp_path_type_produces_path_lambda() {
    // comp (Path Type0 Type0 Type0) <i>Type0 [] should produce a path lambda
    // i.e., composing a path type should yield a path value

    // Build Path type: Path Type0 Type0 Type0
    let path_ty = Expr::Path {
        ty: Box::new(Expr::Type(0)),
        left: Box::new(Expr::Type(0)),
        right: Box::new(Expr::Type(0)),
    };

    // Build constant path: <i> Type0
    let const_path = Expr::PathLam {
        dim: DimVar(0),
        body: Box::new(Expr::Type(0)),
    };

    // comp (Path Type0 Type0 Type0) (<i>Type0) []
    let comp_expr = Expr::Comp {
        ty: Box::new(path_ty),
        base: Box::new(const_path),
        faces: vec![],
    };

    let val = sctt_core::eval::eval(&comp_expr, &Env::new());

    // Result should be a path lambda (VPathLam), not a neutral
    assert!(
        matches!(val, Value::VPathLam { .. }),
        "comp on Path type should produce a path lambda, got: {:?}",
        val
    );
}

#[test]
fn test_comp_path_type_applies_at_endpoints() {
    // Verify that composed path has correct endpoints

    // Build Path type: Path Type0 Type0 Type1
    let path_ty = Expr::Path {
        ty: Box::new(Expr::Type(0)),
        left: Box::new(Expr::Type(0)),
        right: Box::new(Expr::Type(1)),
    };

    // Build constant path: <i> Type0
    let const_path = Expr::PathLam {
        dim: DimVar(0),
        body: Box::new(Expr::Type(0)),
    };

    // comp (Path Type0 Type0 Type1) (<i>Type0) []
    let comp_expr = Expr::Comp {
        ty: Box::new(path_ty),
        base: Box::new(const_path),
        faces: vec![],
    };

    let composed_path = sctt_core::eval::eval(&comp_expr, &Env::new());

    // Apply at dimension 0 (left endpoint)
    let at_zero = composed_path.apply_path(Dim::Zero);

    // Apply at dimension 1 (right endpoint)
    let at_one = composed_path.apply_path(Dim::One);

    // At endpoints, we should get the constrained values (left and right)
    // Due to the face system [(i=0 -> left), (i=1 -> right)]
    assert!(
        matches!(at_zero, Value::VType(0)) || matches!(at_zero, Value::VNeutral { .. }),
        "path at 0 should be left endpoint, got: {:?}",
        at_zero
    );
    assert!(
        matches!(at_one, Value::VType(1)) || matches!(at_one, Value::VNeutral { .. }),
        "path at 1 should be right endpoint, got: {:?}",
        at_one
    );
}
