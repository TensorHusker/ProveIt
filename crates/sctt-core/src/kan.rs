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
use crate::value::{Value, Neutral};
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
        // comp ((x : A) -> B(x)) u sys = λx. comp B(x) (u x) [(φ → sys x)]
        Value::VPi {
            name,
            domain,
            closure: codomain_closure,
        } => {
            // Build the result lambda using semantic computation
            // Strategy: Create a closure that captures everything needed and
            // builds the Expr body using normalized forms with proper indices

            use crate::syntax::{Expr, Name as SyntaxName};
            use crate::normalize::normalize;

            // Build environment: [codomain_ty_expr, base, face_val_0, face_val_1, ...]
            // After lambda binds x: x is at 0, others shift up by 1
            let mut closure_env = im::Vector::new();

            // Store the codomain type (we need it as a value to normalize)
            // We'll apply the codomain closure to a fresh var and normalize
            let fresh_var = Value::VNeutral {
                ty: domain.clone(),
                neutral: Neutral::NVar(name.clone(), 0),
            };
            let codomain_at_var = codomain_closure.apply(fresh_var);
            let codomain_expr = normalize(&codomain_at_var);

            // Store base in env at index 0
            closure_env.push_back(base.clone());

            // Store each face value in env
            let mut face_env_indices = Vec::new();
            for (_, face_val) in faces.iter() {
                face_env_indices.push(closure_env.len() as u32);
                closure_env.push_back(face_val.clone());
            }

            // Build the lambda body expression
            // Closure.apply pushes arg to the BACK of env, so:
            // - Values in closure_env keep their original indices
            // - x (lambda arg) is at the highest index (closure_env.len())
            let x_index = closure_env.len() as u32;
            let x_var = Expr::Var(name.clone(), x_index);

            // codomain_expr was normalized with x at index 0
            // We need to substitute index 0 with the actual x_var at x_index
            let codomain_adjusted = codomain_expr.subst(0, &x_var);

            // base(x) - base is at index 0 in env (unchanged after lambda)
            let base_applied = Expr::App {
                func: Box::new(Expr::Var(SyntaxName("base".to_string()), 0)),
                arg: Box::new(x_var.clone()),
            };

            // Build face applications: each face value applied to x
            let faces_applied: Vec<(Face, Expr)> = faces
                .iter()
                .enumerate()
                .map(|(i, (face, _))| {
                    // face_val is at face_env_indices[i] in env (unchanged)
                    let face_applied = Expr::App {
                        func: Box::new(Expr::Var(
                            SyntaxName(format!("face{}", i)),
                            face_env_indices[i],
                        )),
                        arg: Box::new(x_var.clone()),
                    };
                    (face.clone(), face_applied)
                })
                .collect();

            // Build: comp B(x) (base x) [(φ → face x)]
            let body = Expr::Comp {
                ty: Box::new(codomain_adjusted),
                base: Box::new(base_applied),
                faces: faces_applied,
            };

            // Create the closure and return VLam
            let result_closure = crate::value::Closure::new(closure_env, body);

            Value::VLam {
                name: name.clone(),
                closure: result_closure,
            }
        }

        // Composition in Path types
        // comp (Path A a b) u sys = <i> hcomp A (u @ i) [(i=0 -> a), (i=1 -> b), (φ -> sys @ i)]
        Value::VPath { ty: path_ty, left, right } => {
            use crate::syntax::{DimVar, Expr, Name as SyntaxName};
            use crate::value::DimClosure;

            // Create a fresh dimension variable for the path lambda
            let fresh_dim = DimVar(0); // Using 0 as fresh dim var

            // Build environment for the dimension closure
            // We need to capture: path_ty (A), base (u), left (a), right (b), and face values
            let mut closure_env = im::Vector::new();

            // Store A (the path element type) at index 0
            closure_env.push_back((**path_ty).clone());
            let ty_idx = 0u32;

            // Store base path (u) at index 1
            closure_env.push_back(base.clone());
            let base_idx = 1u32;

            // Store left endpoint (a) at index 2
            closure_env.push_back((**left).clone());
            let left_idx = 2u32;

            // Store right endpoint (b) at index 3
            closure_env.push_back((**right).clone());
            let right_idx = 3u32;

            // Store each face value
            let mut face_indices = Vec::new();
            for (_, face_val) in faces.iter() {
                face_indices.push(closure_env.len() as u32);
                closure_env.push_back(face_val.clone());
            }

            // Build the path lambda body: hcomp A (u @ i) [(i=0 -> a), (i=1 -> b), (φ -> face @ i)]
            // In the body, dimension variable i is used

            // A is at ty_idx in env
            let ty_var = Expr::Var(SyntaxName("A".to_string()), ty_idx);

            // u @ i - apply the base path to dimension i
            let base_at_i = Expr::PathApp {
                path: Box::new(Expr::Var(SyntaxName("u".to_string()), base_idx)),
                dim: Dim::Var(fresh_dim),
            };

            // Build face system: [(i=0 -> left), (i=1 -> right), (φ -> face @ i)]
            let mut hcomp_faces = vec![
                // i=0 -> left endpoint
                (Face::Eq(fresh_dim, false), Expr::Var(SyntaxName("left".to_string()), left_idx)),
                // i=1 -> right endpoint
                (Face::Eq(fresh_dim, true), Expr::Var(SyntaxName("right".to_string()), right_idx)),
            ];

            // Add original faces, each applied at dimension i
            for (idx, (face, _)) in faces.iter().enumerate() {
                let face_at_i = Expr::PathApp {
                    path: Box::new(Expr::Var(
                        SyntaxName(format!("face{}", idx)),
                        face_indices[idx],
                    )),
                    dim: Dim::Var(fresh_dim),
                };
                hcomp_faces.push((face.clone(), face_at_i));
            }

            // Build: hcomp A (u @ i) faces
            let body = Expr::HComp {
                ty: Box::new(ty_var),
                base: Box::new(base_at_i),
                faces: hcomp_faces,
            };

            // Create the dimension closure
            let dim_closure = DimClosure::new(closure_env, im::Vector::new(), body);

            Value::VPathLam {
                dim: fresh_dim,
                dim_closure,
            }
        }

        // Composition in Type universe: return the base type
        Value::VType(_) => {
            // Types compose trivially (they don't vary)
            base.clone()
        }

        // Composition in Glue types (CCHM Section 9.6)
        // comp (Glue [φ ↦ (T, e)] A) u sys =
        //   glue [φ ↦ comp T (e⁻¹ u) [ψ ↦ e⁻¹ sys]]
        //        (comp A (unglue u) [ψ ↦ unglue sys, φ ↦ e (comp T ...)])
        Value::VGlue { base: base_ty, system } => {
            // For now, simplified implementation:
            // - If no faces in the system are satisfied, compose in base type
            // - Otherwise, need full Glue composition

            // Check if any glue face is satisfied at target
            let target_dims = dim_to_env(&target_dim);
            for (face, fiber_ty, _equiv) in system {
                if face_satisfied(face, &target_dims) {
                    // Face is satisfied: compose in fiber type
                    // First extract fiber from base (would apply e⁻¹)
                    // For now, return comp in fiber type with same faces
                    return comp(fiber_ty, base, faces, target_dim.clone());
                }
            }

            // No glue face satisfied: compose in base type
            // Need to unglue the base and faces first
            let unglued_base = unglue_value(base);
            let unglued_faces: Vec<(Face, Value)> = faces
                .iter()
                .map(|(f, v)| (f.clone(), unglue_value(v)))
                .collect();

            // Compose in base type
            let base_result = comp(base_ty, &unglued_base, &unglued_faces, target_dim.clone());

            // Re-glue the result (would need to apply equivalence)
            // For now, return as VGlueTm
            Value::VGlueTm {
                base: Arc::new(base_result),
                fibers: vec![], // Would compute fiber values via equivalence
                ty: Arc::new(ty.clone()),
            }
        }

        // For other types, return neutral composition
        _ => {
            Value::VNeutral {
                ty: Arc::new(ty.clone()),
                neutral: Neutral::NComp {
                    ty: Arc::new(ty.clone()),
                    base: Arc::new(base.clone()),
                    faces: faces.to_vec(),
                    target_dim: target_dim.clone(),
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
        Value::VPi { .. } => {
            // coe (λi. (x : A(i)) → B(i,x)) r r' u =
            //   λx. coe (λi. B(i, coe (λj. A(j)) r' r x)) r r' (u (coe (λj. A(j)) r r' x))

            // This is complex - for now return neutral
            Value::VNeutral {
                ty: Arc::new(ty_family.clone()),
                neutral: Neutral::NCoe {
                    ty_fam: Arc::new(ty_family.clone()),
                    from: from.clone(),
                    to: to.clone(),
                    base: Arc::new(base.clone()),
                },
            }
        }

        // Coercion in Path types
        Value::VPath { .. } => {
            // coe (λi. Path (A i) (l i) (r i)) r r' u =
            //   <j> coe (λi. A i) r r' (u j)

            Value::VNeutral {
                ty: Arc::new(ty_family.clone()),
                neutral: Neutral::NCoe {
                    ty_fam: Arc::new(ty_family.clone()),
                    from: from.clone(),
                    to: to.clone(),
                    base: Arc::new(base.clone()),
                },
            }
        }

        // Coercion in Glue types (CCHM Section 9.5)
        // coe (λi. Glue [φ(i) ↦ (T(i), e(i))] A(i)) r r' u
        Value::VGlue { base: base_ty, system } => {
            // Simplified: if no faces active, coerce in base type
            // Full implementation would track face changes across dimension

            // Coerce the base value
            let unglued = unglue_value(base);
            let coerced_base = coe(base_ty, from.clone(), to.clone(), &unglued);

            // Re-glue with coerced fibers (would need to coerce each fiber)
            Value::VGlueTm {
                base: Arc::new(coerced_base),
                fibers: system.iter().map(|(face, fiber_ty, _equiv)| {
                    // Would coerce fiber and apply equivalence
                    let fiber = unglue_value(base); // Simplified
                    let coerced_fiber = coe(fiber_ty, from.clone(), to.clone(), &fiber);
                    (face.clone(), Arc::new(coerced_fiber))
                }).collect(),
                ty: Arc::new(ty_family.clone()),
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
                    base: Arc::new(base.clone()),
                },
            }
        }
    }
}

/// Helper function to unglue a value (extract base from glued value)
fn unglue_value(val: &Value) -> Value {
    match val {
        Value::VGlueTm { base, .. } => (**base).clone(),
        Value::VNeutral { ty, neutral } => {
            Value::VNeutral {
                ty: match ty.as_ref() {
                    Value::VGlue { base, .. } => base.clone(),
                    _ => ty.clone(),
                },
                neutral: Neutral::NUnglue {
                    glue_val: Box::new(neutral.clone()),
                    glue_ty: ty.clone(),
                },
            }
        }
        _ => val.clone(),
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
