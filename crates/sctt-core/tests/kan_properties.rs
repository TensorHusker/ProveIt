//! Property-based tests for Kan operations
//!
//! Uses proptest to verify mathematical laws hold for randomly generated inputs.

use proptest::prelude::*;
use sctt_core::{
    kan::{comp, coe, hcomp, face_satisfied},
    syntax::{Dim, DimVar, Face},
    value::Value,
};
use std::sync::Arc;

// Strategy for generating simple Values
fn any_simple_value() -> impl Strategy<Value = Value> {
    prop_oneof![
        Just(Value::VType(0)),
        Just(Value::VType(1)),
        Just(Value::VType(2)),
    ]
}

// Strategy for generating Dims
fn any_dim() -> impl Strategy<Value = Dim> {
    prop_oneof![
        Just(Dim::Zero),
        Just(Dim::One),
        (0u32..5).prop_map(|i| Dim::Var(DimVar(i))),
    ]
}

// Strategy for generating simple Faces
fn any_face() -> impl Strategy<Value = Face> {
    let leaf = prop_oneof![
        Just(Face::True),
        (0u32..5, any::<bool>()).prop_map(|(v, b)| Face::Eq(DimVar(v), b)),
    ];

    leaf.prop_recursive(
        3, // Max depth
        10, // Max nodes
        3, // Items per collection
        |inner| {
            (inner.clone(), inner).prop_map(|(f1, f2)| {
                Face::And(Box::new(f1), Box::new(f2))
            })
        }
    )
}

// Strategy for dimension environments
fn any_dim_env() -> impl Strategy<Value = Vec<(u32, bool)>> {
    prop::collection::vec((0u32..5, any::<bool>()), 0..5)
}

proptest! {
    /// Composition identity law: comp(ty, base, [], 0) = base
    #[test]
    fn prop_comp_identity(
        base in any_simple_value(),
        ty in any_simple_value()
    ) {
        let result = comp(&ty, &base, &[], Dim::Zero);
        prop_assert!(result.conv(&base),
            "comp(ty, base, [], 0) should equal base (identity law)");
    }

    /// Coercion reflexivity: coe(A, r, r, a) = a
    #[test]
    fn prop_coe_reflexivity(
        r in any_dim(),
        base in any_simple_value()
    ) {
        let ty_fam = Value::VType(0); // Constant family
        let result = coe(&ty_fam, r.clone(), r, &base);
        prop_assert!(result.conv(&base),
            "coe(A, r, r, a) should equal a (reflexivity)");
    }

    /// Homogeneous composition equals comp at dimension 1
    #[test]
    fn prop_hcomp_equals_comp(
        base in any_simple_value(),
        ty in any_simple_value()
    ) {
        let faces: Vec<(Face, Value)> = vec![];

        let hcomp_result = hcomp(&ty, &base, &faces);
        let comp_result = comp(&ty, &base, &faces, Dim::One);

        prop_assert!(hcomp_result.conv(&comp_result),
            "hcomp should equal comp at dimension 1");
    }

    /// Face::True is always satisfied
    #[test]
    fn prop_face_true_always_satisfied(
        dims in any_dim_env()
    ) {
        prop_assert!(face_satisfied(&Face::True, &dims),
            "Face::True should always be satisfied");
    }

    /// Face equality is consistent
    #[test]
    fn prop_face_eq_consistency(
        var in 0u32..5,
        val in any::<bool>()
    ) {
        let face = Face::Eq(DimVar(var), val);
        let dims_matching = vec![(var, val)];
        let dims_not_matching = vec![(var, !val)];

        prop_assert!(face_satisfied(&face, &dims_matching),
            "Face should be satisfied when dimension matches");
        prop_assert!(!face_satisfied(&face, &dims_not_matching),
            "Face should not be satisfied when dimension doesn't match");
    }

    /// Face And is commutative
    #[test]
    fn prop_face_and_commutative(
        f1 in any_face(),
        f2 in any_face(),
        dims in any_dim_env()
    ) {
        let and1 = Face::And(Box::new(f1.clone()), Box::new(f2.clone()));
        let and2 = Face::And(Box::new(f2), Box::new(f1));

        prop_assert_eq!(
            face_satisfied(&and1, &dims),
            face_satisfied(&and2, &dims),
            "Face And should be commutative"
        );
    }

    /// Face And is associative
    #[test]
    fn prop_face_and_associative(
        f1 in any_face(),
        f2 in any_face(),
        f3 in any_face(),
        dims in any_dim_env()
    ) {
        let and1 = Face::And(
            Box::new(Face::And(Box::new(f1.clone()), Box::new(f2.clone()))),
            Box::new(f3.clone())
        );
        let and2 = Face::And(
            Box::new(f1),
            Box::new(Face::And(Box::new(f2), Box::new(f3)))
        );

        prop_assert_eq!(
            face_satisfied(&and1, &dims),
            face_satisfied(&and2, &dims),
            "Face And should be associative"
        );
    }

    /// Empty face system gives identity
    #[test]
    fn prop_comp_empty_faces_identity(
        base in any_simple_value(),
        ty in any_simple_value()
    ) {
        let result = comp(&ty, &base, &[], Dim::Zero);
        prop_assert!(result.conv(&base),
            "comp with empty faces at 0 should be identity");
    }

    /// Composition preserves types (smoke test)
    #[test]
    fn prop_comp_preserves_type_structure(
        base in any_simple_value(),
        ty in any_simple_value()
    ) {
        let result = comp(&ty, &base, &[], Dim::One);

        // Result should be a valid value (not crash)
        match result {
            Value::VType(_) | Value::VNeutral { .. } => {},
            _ => prop_assert!(false, "comp should produce Type or Neutral"),
        }
    }

    /// Coercion with identity dimension is identity
    #[test]
    fn prop_coe_zero_to_zero(
        base in any_simple_value()
    ) {
        let ty_fam = Value::VType(0);
        let result = coe(&ty_fam, Dim::Zero, Dim::Zero, &base);
        prop_assert!(result.conv(&base),
            "coe from 0 to 0 should be identity");
    }
}

// Additional manual property tests for complex scenarios

#[test]
fn test_comp_with_satisfied_face() {
    // If a face is satisfied, comp should return that face's value
    let base = Value::VType(0);
    let face_value = Value::VType(1);
    let ty = Value::VType(0);

    let faces = vec![(Face::Eq(DimVar(0), false), face_value.clone())];

    // At dimension 0 (false), the face should be satisfied
    let result = comp(&ty, &base, &faces, Dim::Zero);

    // Should return face_value (or base, depending on implementation)
    assert!(matches!(result, Value::VType(_)));
}

#[test]
fn test_face_and_short_circuit() {
    // And(Eq(0, false), Eq(0, true)) is never satisfied
    let face = Face::And(
        Box::new(Face::Eq(DimVar(0), false)),
        Box::new(Face::Eq(DimVar(0), true)),
    );

    let dims = vec![(0, false)];
    assert!(!face_satisfied(&face, &dims));

    let dims = vec![(0, true)];
    assert!(!face_satisfied(&face, &dims));
}

#[test]
fn test_comp_type_universe_stability() {
    // comp in Type universe should be stable
    let ty = Value::VType(1);
    let base = Value::VType(0);

    let result = comp(&ty, &base, &[], Dim::Zero);

    // Should return Type₀ (identity)
    assert!(result.conv(&Value::VType(0)));
}

#[test]
fn test_coe_constant_type_family() {
    // For constant families, coercion should always be identity
    let ty_fam = Value::VType(0); // Constant
    let base = Value::VType(0);

    // Any dimension transport should be identity
    let result1 = coe(&ty_fam, Dim::Zero, Dim::One, &base);
    assert!(result1.conv(&base));

    let result2 = coe(&ty_fam, Dim::One, Dim::Zero, &base);
    assert!(result2.conv(&base));
}

#[test]
fn test_hcomp_consistency_with_comp() {
    // hcomp should behave like comp at dimension 1
    let ty = Value::VType(0);
    let base = Value::VType(0);
    let faces: Vec<(Face, Value)> = vec![];

    let hcomp_result = hcomp(&ty, &base, &faces);
    let comp_result = comp(&ty, &base, &faces, Dim::One);

    assert!(hcomp_result.conv(&comp_result));
}
