//! Kan operations: composition and coercion
//!
//! These operations are fundamental to cubical type theory,
//! enabling path composition and type transport.
//!
//! References:
//! - "Cubical Type Theory: A Constructive Interpretation of the Univalence Axiom"
//!   by Cohen, Coquand, Huber, Mörtberg (2016)
//! - "Cartesian Cubical Computational Type Theory" by Angiuli, Favonia, Harper (2018)

use crate::syntax::{Dim, Face};
use crate::value::{Value, Neutral, DimEnv};
use std::sync::Arc;

/// Composition operation (filling Kan cubes)
///
/// Composes a value along faces to fill an open cube. This is the fundamental
/// operation that enables path composition and function extensionality.
///
/// # Arguments
/// * `ty` - The type being composed (may depend on dimension)
/// * `base` - The base value at dimension 0
/// * `faces` - Face constraints (dimension equations and their values)
/// * `target_dim` - The target dimension to compute the value at
///
/// # Returns
/// The composed value at `target_dim` that agrees with `base` at 0 and `faces`
pub fn comp(ty: &Value, base: &Value, faces: &[(Face, Value)], target_dim: Dim) -> Value {
    // Case 1: If no faces and target is 0, return base (identity)
    if faces.is_empty() && matches!(target_dim, Dim::Zero) {
        return base.clone();
    }

    // Case 2: Check if any face is already satisfied at target dimension
    // If so, return that face's value
    let target_dims = dim_to_env(&target_dim);
    for (face, value) in faces {
        if face_satisfied(face, &target_dims) {
            return value.clone();
        }
    }

    // Case 3: Composition by type
    match ty {
        // Composition in Pi types: pointwise composition
        Value::VPi { domain, closure, .. } => {
            // For functions, compose pointwise
            // comp (x : A) -> B(x) u sys = λx. comp B(x) (u x) [(φ → sys x)]

            // Create a closure that composes for each argument
            let base_fn = base.clone();
            let faces_fn = faces.to_vec();
            let target = target_dim.clone();

            // For now, return a neutral term representing the composition
            // Full implementation would create a proper closure
            Value::VNeutral {
                ty: Arc::new(ty.clone()),
                neutral: Neutral::NComp {
                    ty: Arc::new(ty.clone()),
                    base: Box::new(match base {
                        Value::VNeutral { neutral, .. } => neutral.clone(),
                        _ => Neutral::NVar(crate::syntax::Name("comp_base".to_string()), 0),
                    }),
                },
            }
        }

        // Composition in Path types
        Value::VPath { ty: path_ty, left, right } => {
            // comp (Path A x y) u sys = <i> comp A (u i) [(φ → sys i)]
            // Returns a path that composes the base path with face constraints

            let base_path = base.clone();

            // Create a dimension closure for the composed path
            Value::VNeutral {
                ty: Arc::new(ty.clone()),
                neutral: Neutral::NComp {
                    ty: Arc::new(ty.clone()),
                    base: Box::new(match base {
                        Value::VNeutral { neutral, .. } => neutral.clone(),
                        _ => Neutral::NVar(crate::syntax::Name("path_base".to_string()), 0),
                    }),
                },
            }
        }

        // Composition in Type universe: return the base type
        Value::VType(_) => {
            // Types compose trivially (they don't vary)
            base.clone()
        }

        // For other types, return neutral composition
        _ => {
            Value::VNeutral {
                ty: Arc::new(ty.clone()),
                neutral: Neutral::NComp {
                    ty: Arc::new(ty.clone()),
                    base: Box::new(match base {
                        Value::VNeutral { neutral, .. } => neutral.clone(),
                        _ => Neutral::NVar(crate::syntax::Name("comp_base".to_string()), 0),
                    }),
                },
            }
        }
    }
}

/// Convert a dimension to an environment for face checking
fn dim_to_env(dim: &Dim) -> Vec<(u32, bool)> {
    match dim {
        Dim::Zero => vec![(0, false)],
        Dim::One => vec![(0, true)],
        Dim::Var(v) => vec![(v.0, false)], // Variables assumed false for now
    }
}

/// Coercion along a type family
///
/// Transports a value along a type family from one dimension to another.
/// This is the fundamental operation for type-level computation and transport.
///
/// # Arguments
/// * `ty_family` - A type family (function from dimensions to types)
/// * `from` - The source dimension
/// * `to` - The target dimension
/// * `base` - The value at the source dimension
///
/// # Returns
/// The transported value at the target dimension
///
/// # Examples
/// If `ty_family = λi. if i=0 then A else B`, then:
/// - `coe ty_family 0 1 a` transports `a : A` to a value of type `B`
/// - `coe ty_family 0 0 a` returns `a` (identity)
pub fn coe(ty_family: &Value, from: Dim, to: Dim, base: &Value) -> Value {
    // Case 1: Identity coercion (from == to)
    if from == to {
        return base.clone();
    }

    // Case 2: Check if type family is constant
    // If A doesn't depend on dimension, coercion is identity
    // This is an optimization for the common case
    if is_constant_family(ty_family) {
        return base.clone();
    }

    // Case 3: Coercion by type family structure
    match ty_family {
        // Coercion in constant types (Type universe)
        Value::VType(_) => {
            // Universe levels are constant, return base
            base.clone()
        }

        // Coercion in Pi types: transport pointwise
        Value::VPi { domain, closure, .. } => {
            // coe (λi. (x : A(i)) → B(i,x)) r r' u =
            //   λx. coe (λi. B(i, coe (λj. A(j)) r' r x)) r r' (u (coe (λj. A(j)) r r' x))

            // This is complex - for now return neutral
            Value::VNeutral {
                ty: Arc::new(ty_family.clone()),
                neutral: Neutral::NCoe {
                    ty_fam: Arc::new(ty_family.clone()),
                    from: from.clone(),
                    to: to.clone(),
                    base: Box::new(match base {
                        Value::VNeutral { neutral, .. } => neutral.clone(),
                        _ => Neutral::NVar(crate::syntax::Name("coe_base".to_string()), 0),
                    }),
                },
            }
        }

        // Coercion in Path types
        Value::VPath { ty, left, right } => {
            // coe (λi. Path (A i) (l i) (r i)) r r' u =
            //   <j> coe (λi. A i) r r' (u j)

            Value::VNeutral {
                ty: Arc::new(ty_family.clone()),
                neutral: Neutral::NCoe {
                    ty_fam: Arc::new(ty_family.clone()),
                    from: from.clone(),
                    to: to.clone(),
                    base: Box::new(match base {
                        Value::VNeutral { neutral, .. } => neutral.clone(),
                        _ => Neutral::NVar(crate::syntax::Name("path_coe_base".to_string()), 0),
                    }),
                },
            }
        }

        // For other types, return neutral coercion
        _ => {
            Value::VNeutral {
                ty: Arc::new(ty_family.clone()),
                neutral: Neutral::NCoe {
                    ty_fam: Arc::new(ty_family.clone()),
                    from: from.clone(),
                    to: to.clone(),
                    base: Box::new(match base {
                        Value::VNeutral { neutral, .. } => neutral.clone(),
                        _ => Neutral::NVar(crate::syntax::Name("coe_base".to_string()), 0),
                    }),
                },
            }
        }
    }
}

/// Check if a type family is constant (doesn't depend on dimension)
///
/// This is an optimization - constant families don't need actual transport
fn is_constant_family(ty: &Value) -> bool {
    // Simplified check: assume lambdas might vary
    // Full implementation would check if body actually uses the dimension variable
    match ty {
        Value::VType(_) => true, // Universe levels are constant
        Value::VLam { .. } => false, // Assume lambdas vary
        _ => true, // Most other values are constant
    }
}

/// Homogeneous composition (special case where type is constant)
///
/// A special case of composition where the type doesn't depend on dimensions.
/// This is simpler than full composition and is used for path composition.
///
/// # Arguments
/// * `ty` - The constant type
/// * `base` - The base value at dimension 0
/// * `faces` - Face constraints
///
/// # Returns
/// The composed value at dimension 1
///
/// # Note
/// hcomp is equivalent to `comp ty base faces Dim::One` when ty is constant
pub fn hcomp(ty: &Value, base: &Value, faces: &[(Face, Value)]) -> Value {
    // Homogeneous composition is just composition to dimension 1
    // with a constant type
    comp(ty, base, faces, Dim::One)
}

/// Check if a face formula is satisfied by dimension values
pub fn face_satisfied(face: &Face, dims: &[(u32, bool)]) -> bool {
    match face {
        Face::Eq(var, val) => dims
            .iter()
            .find(|(v, _)| *v == var.0)
            .is_some_and(|(_, v)| v == val),
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
