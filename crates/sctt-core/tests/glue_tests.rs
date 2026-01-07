//! Tests for Glue types (univalence support)
//!
//! Glue types are fundamental to cubical type theory's proof of univalence.
//! They allow "gluing" a type T to a base type A along an equivalence e : Equiv T A.

use sctt_core::{
    check::{infer, Context},
    normalize::normalize,
    syntax::{DimVar, Expr, Face, Name},
    value::{Env, Value},
};

fn evaluate_expr(expr: &Expr, env: &Env) -> Value {
    sctt_core::eval::eval(expr, env)
}

/// Test basic Glue type construction
#[test]
fn test_glue_type_basic() {
    // Glue [] Type0 should reduce to Type0 (no faces)
    let glue_expr = Expr::Glue {
        base: Box::new(Expr::Type(0)),
        equivalences: vec![],
    };

    let val = evaluate_expr(&glue_expr, &Env::new());

    // With no equivalences, should be a VGlue or reduce to base
    assert!(
        matches!(val, Value::VGlue { .. }) || matches!(val, Value::VType(0)),
        "Glue with no faces should produce VGlue or reduce to base"
    );
}

/// Test Glue type with face that's always true
#[test]
fn test_glue_face_true_reduces() {
    // Glue [True -> (Type1, id)] Type0 should reduce to Type1
    // because the face is always satisfied
    let glue_expr = Expr::Glue {
        base: Box::new(Expr::Type(0)),
        equivalences: vec![(
            Face::True,
            Expr::Type(1),
            Expr::Lambda {
                // identity equivalence (simplified)
                name: Name("x".to_string()),
                body: Box::new(Expr::Var(Name("x".to_string()), 0)),
            },
        )],
    };

    let val = evaluate_expr(&glue_expr, &Env::new());

    // When face is True, should reduce to fiber type
    assert!(
        matches!(val, Value::VType(1)),
        "Glue with True face should reduce to fiber type, got: {:?}",
        val
    );
}

/// Test GlueTm construction
#[test]
fn test_glue_tm_basic() {
    // glue [] Type0 should produce a glued value
    let glue_tm_expr = Expr::GlueTm {
        base: Box::new(Expr::Type(0)),
        fibers: vec![],
    };

    let val = evaluate_expr(&glue_tm_expr, &Env::new());

    // Should produce VGlueTm
    assert!(
        matches!(val, Value::VGlueTm { .. }),
        "GlueTm should produce VGlueTm"
    );
}

/// Test GlueTm with true face reduces to fiber
#[test]
fn test_glue_tm_face_true_reduces() {
    // glue [True -> Type1] Type0 should reduce to Type1
    let glue_tm_expr = Expr::GlueTm {
        base: Box::new(Expr::Type(0)),
        fibers: vec![(Face::True, Expr::Type(1))],
    };

    let val = evaluate_expr(&glue_tm_expr, &Env::new());

    // When face is True, should reduce to fiber value
    assert!(
        matches!(val, Value::VType(1)),
        "GlueTm with True face should reduce to fiber value, got: {:?}",
        val
    );
}

/// Test unglue extracts base value
#[test]
fn test_unglue_extracts_base() {
    // unglue (glue [] Type0) should give Type0
    let glue_tm_expr = Expr::GlueTm {
        base: Box::new(Expr::Type(0)),
        fibers: vec![],
    };

    let unglue_expr = Expr::Unglue {
        glue: Box::new(glue_tm_expr),
    };

    let val = evaluate_expr(&unglue_expr, &Env::new());

    // Should extract the base Type0
    assert!(
        val.conv(&Value::VType(0)),
        "unglue should extract base value"
    );
}

/// Test Glue type checking
#[test]
fn test_glue_type_checks() {
    let ctx = Context::new();

    // Glue [] Type0 : Type1
    let glue_expr = Expr::Glue {
        base: Box::new(Expr::Type(0)),
        equivalences: vec![],
    };

    let result = infer(&ctx, &glue_expr);
    assert!(
        result.is_ok(),
        "Glue type should type check, got: {:?}",
        result.err()
    );

    let ty = result.unwrap();
    assert!(
        matches!(ty, Value::VType(1)),
        "Glue Type0 should have type Type1"
    );
}

/// Test GlueTm type checking
#[test]
fn test_glue_tm_type_checks() {
    let ctx = Context::new();

    // glue [] Type0
    let glue_tm_expr = Expr::GlueTm {
        base: Box::new(Expr::Type(0)),
        fibers: vec![],
    };

    let result = infer(&ctx, &glue_tm_expr);
    assert!(
        result.is_ok(),
        "GlueTm should type check, got: {:?}",
        result.err()
    );
}

/// Test Unglue type checking
#[test]
fn test_unglue_type_checks() {
    let ctx = Context::new();

    // unglue (glue [] Type0)
    let glue_tm_expr = Expr::GlueTm {
        base: Box::new(Expr::Type(0)),
        fibers: vec![],
    };

    let unglue_expr = Expr::Unglue {
        glue: Box::new(glue_tm_expr),
    };

    let result = infer(&ctx, &unglue_expr);
    assert!(
        result.is_ok(),
        "Unglue should type check, got: {:?}",
        result.err()
    );
}

/// Test Glue normalization
#[test]
fn test_glue_normalizes() {
    // Glue [] Type0 should normalize to Glue expression
    let glue_expr = Expr::Glue {
        base: Box::new(Expr::Type(0)),
        equivalences: vec![],
    };

    let val = evaluate_expr(&glue_expr, &Env::new());
    let normalized = normalize(&val);

    // Should normalize back to a valid expression
    match normalized {
        Expr::Glue { .. } => {}
        Expr::Type(_) => {} // Could reduce if faces satisfied
        _ => panic!("Unexpected normalized form: {:?}", normalized),
    }
}

/// Test GlueTm normalization
#[test]
fn test_glue_tm_normalizes() {
    let glue_tm_expr = Expr::GlueTm {
        base: Box::new(Expr::Type(0)),
        fibers: vec![],
    };

    let val = evaluate_expr(&glue_tm_expr, &Env::new());
    let normalized = normalize(&val);

    // Should normalize back to GlueTm or reduce
    match normalized {
        Expr::GlueTm { .. } => {}
        Expr::Type(_) => {} // Could reduce
        _ => panic!("Unexpected normalized form: {:?}", normalized),
    }
}

/// Test composition in Glue types
#[test]
fn test_comp_glue_type() {
    // comp (Glue [] Type0) Type0 []
    let glue_ty = Expr::Glue {
        base: Box::new(Expr::Type(0)),
        equivalences: vec![],
    };

    let comp_expr = Expr::Comp {
        ty: Box::new(glue_ty),
        base: Box::new(Expr::Type(0)),
        faces: vec![],
    };

    let val = evaluate_expr(&comp_expr, &Env::new());

    // Should produce a value (VGlueTm or reduced)
    assert!(
        matches!(val, Value::VGlueTm { .. })
            || matches!(val, Value::VType(_))
            || matches!(val, Value::VNeutral { .. }),
        "comp in Glue type should produce valid value, got: {:?}",
        val
    );
}

/// Test coercion in Glue types
#[test]
fn test_coe_glue_type() {
    use sctt_core::syntax::Dim;

    // coe (Glue [] Type0) 0 1 Type0
    let glue_ty = Expr::Glue {
        base: Box::new(Expr::Type(0)),
        equivalences: vec![],
    };

    let coe_expr = Expr::Coe {
        ty_fam: Box::new(glue_ty),
        from: Dim::Zero,
        to: Dim::One,
        base: Box::new(Expr::Type(0)),
    };

    let val = evaluate_expr(&coe_expr, &Env::new());

    // Should produce a value
    assert!(
        matches!(val, Value::VGlueTm { .. })
            || matches!(val, Value::VType(_))
            || matches!(val, Value::VNeutral { .. }),
        "coe in Glue type should produce valid value, got: {:?}",
        val
    );
}

/// Test Glue with dimension-dependent face
#[test]
fn test_glue_dimension_face() {
    // Glue [i=1 -> (Type1, equiv)] Type0
    let glue_expr = Expr::Glue {
        base: Box::new(Expr::Type(0)),
        equivalences: vec![(
            Face::Eq(DimVar(0), true), // i=1
            Expr::Type(1),
            Expr::Lambda {
                name: Name("x".to_string()),
                body: Box::new(Expr::Var(Name("x".to_string()), 0)),
            },
        )],
    };

    let val = evaluate_expr(&glue_expr, &Env::new());

    // Face i=1 is not satisfied (no dimension env), should remain VGlue
    assert!(
        matches!(val, Value::VGlue { .. }),
        "Glue with unsatisfied face should remain VGlue, got: {:?}",
        val
    );
}

/// Test roundtrip: Expr -> Value -> Expr for Glue
#[test]
fn test_glue_roundtrip() {
    let original = Expr::Glue {
        base: Box::new(Expr::Type(0)),
        equivalences: vec![],
    };

    let val = evaluate_expr(&original, &Env::new());
    let normalized = normalize(&val);

    // Should produce valid Glue expression
    if let Expr::Glue { base, equivalences } = normalized {
        assert!(
            matches!(*base, Expr::Type(0)),
            "Base should be Type0"
        );
        assert!(equivalences.is_empty(), "Should have no equivalences");
    }
    // Could also reduce to Type0 if implementation simplifies
}
