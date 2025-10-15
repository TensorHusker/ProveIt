//! Kan operations: composition and coercion
//!
//! These operations are fundamental to cubical type theory,
//! enabling path composition and type transport.

use crate::syntax::{Dim, Face};
use crate::value::Value;

/// Composition operation (filling Kan cubes)
pub fn comp(ty: &Value, base: &Value, faces: &[(Face, Value)], target_dim: Dim) -> Value {
    // Simplified implementation: return base
    // Full implementation would compute Kan filling
    base.clone()
}

/// Coercion along a type family
pub fn coe(ty_family: &Value, from: Dim, to: Dim, base: &Value) -> Value {
    // If from == to, return base (identity coercion)
    if from == to {
        return base.clone();
    }

    // Simplified: return base
    // Full implementation would transport along type family
    base.clone()
}

/// Homogeneous composition (special case where type is constant)
pub fn hcomp(ty: &Value, base: &Value, faces: &[(Face, Value)]) -> Value {
    // Simplified implementation
    base.clone()
}

/// Check if a face formula is satisfied by dimension values
pub fn face_satisfied(face: &Face, dims: &[(u32, bool)]) -> bool {
    match face {
        Face::Eq(var, val) => dims
            .iter()
            .find(|(v, _)| *v == var.0)
            .map_or(false, |(_, v)| v == val),
        Face::And(f1, f2) => face_satisfied(f1, dims) && face_satisfied(f2, dims),
        Face::True => true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::syntax::DimVar;

    #[test]
    fn test_identity_coercion() {
        let ty = Value::VType(0);
        let base = Value::VType(0);
        let dim = Dim::Zero;

        let result = coe(&ty, dim.clone(), dim, &base);
        assert!(result.conv(&base));
    }

    #[test]
    fn test_face_satisfaction() {
        let face = Face::Eq(DimVar(0), true);
        let dims = vec![(0, true)];
        assert!(face_satisfied(&face, &dims));

        let dims2 = vec![(0, false)];
        assert!(!face_satisfied(&face, &dims2));
    }
}
