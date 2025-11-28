//! Evaluation: converting syntax to semantic values

use crate::syntax::{Dim, Expr};
use crate::value::{Closure, DimClosure, DimEnv, Env, Value};
use std::sync::Arc;

/// Evaluate an expression in an environment to produce a value
pub fn eval(expr: &Expr, env: &Env) -> Value {
    eval_with_dims(expr, env, &DimEnv::new())
}

/// Evaluate with both term and dimension environments
pub fn eval_with_dims(expr: &Expr, env: &Env, dim_env: &DimEnv) -> Value {
    match expr {
        Expr::Type(level) => Value::VType(*level),

        Expr::Var(_name, idx) => env
            .get(*idx as usize)
            .cloned()
            .unwrap_or_else(|| panic!("Unbound variable at index {}", idx)),

        Expr::Pi {
            name,
            domain,
            codomain,
        } => {
            let domain_val = eval_with_dims(domain, env, dim_env);
            let closure = Closure::new(env.clone(), codomain.as_ref().clone());
            Value::VPi {
                name: name.clone(),
                domain: Arc::new(domain_val),
                closure,
            }
        }

        Expr::Lambda { name, body } => {
            let closure = Closure::new(env.clone(), body.as_ref().clone());
            Value::VLam {
                name: name.clone(),
                closure,
            }
        }

        Expr::App { func, arg } => {
            let func_val = eval_with_dims(func, env, dim_env);
            let arg_val = eval_with_dims(arg, env, dim_env);
            func_val.apply(arg_val)
        }

        Expr::Path { ty, left, right } => {
            let ty_val = eval_with_dims(ty, env, dim_env);
            let left_val = eval_with_dims(left, env, dim_env);
            let right_val = eval_with_dims(right, env, dim_env);
            Value::VPath {
                ty: Arc::new(ty_val),
                left: Arc::new(left_val),
                right: Arc::new(right_val),
            }
        }

        Expr::PathLam { dim, body } => {
            let dim_closure = DimClosure::new(env.clone(), dim_env.clone(), body.as_ref().clone());
            Value::VPathLam {
                dim: *dim,
                dim_closure,
            }
        }

        Expr::PathApp { path, dim } => {
            let path_val = eval_with_dims(path, env, dim_env);
            let resolved_dim = resolve_dim(dim, dim_env);
            path_val.apply_path(resolved_dim)
        }

        Expr::SmoothPath {
            order,
            ty,
            left,
            right,
        } => {
            let ty_val = eval_with_dims(ty, env, dim_env);
            let left_val = eval_with_dims(left, env, dim_env);
            let right_val = eval_with_dims(right, env, dim_env);
            Value::VSmoothPath {
                order: *order,
                ty: Arc::new(ty_val),
                left: Arc::new(left_val),
                right: Arc::new(right_val),
            }
        }

        Expr::Comp { ty, base, faces } => {
            // Full Kan composition implementation
            let ty_val = eval_with_dims(ty, env, dim_env);
            let base_val = eval_with_dims(base, env, dim_env);
            let faces_val: Vec<(crate::syntax::Face, Value)> = faces
                .iter()
                .map(|(face, expr)| {
                    // Resolve dimension variables in the face against dim_env
                    let resolved_face = resolve_face(face, dim_env);
                    (resolved_face, eval_with_dims(expr, env, dim_env))
                })
                .collect();

            // Call actual Kan composition from kan.rs
            crate::kan::comp(&ty_val, &base_val, &faces_val, Dim::One)
        }

        Expr::Coe {
            ty_fam,
            from,
            to,
            base,
        } => {
            // Full coercion implementation
            let ty_fam_val = eval_with_dims(ty_fam, env, dim_env);
            let base_val = eval_with_dims(base, env, dim_env);
            let from_resolved = resolve_dim(from, dim_env);
            let to_resolved = resolve_dim(to, dim_env);

            // Call actual Kan coercion from kan.rs
            crate::kan::coe(&ty_fam_val, from_resolved, to_resolved, &base_val)
        }

        Expr::HComp { ty, base, faces } => {
            // Full homogeneous composition implementation
            let ty_val = eval_with_dims(ty, env, dim_env);
            let base_val = eval_with_dims(base, env, dim_env);
            let faces_val: Vec<(crate::syntax::Face, Value)> = faces
                .iter()
                .map(|(face, expr)| {
                    // Resolve dimension variables in the face against dim_env
                    let resolved_face = resolve_face(face, dim_env);
                    (resolved_face, eval_with_dims(expr, env, dim_env))
                })
                .collect();

            // Call homogeneous composition from kan.rs
            crate::kan::hcomp(&ty_val, &base_val, &faces_val)
        }

        Expr::Glue {
            base,
            equivalences,
        } => {
            // Evaluate Glue type: Glue [φ ↦ (T, e)] A
            let base_val = eval_with_dims(base, env, dim_env);

            // Evaluate each equivalence in the system
            let system: Vec<(crate::syntax::Face, std::sync::Arc<Value>, std::sync::Arc<Value>)> = equivalences
                .iter()
                .map(|(face, ty_expr, equiv_expr)| {
                    let resolved_face = resolve_face(face, dim_env);
                    let ty_val = std::sync::Arc::new(eval_with_dims(ty_expr, env, dim_env));
                    let equiv_val = std::sync::Arc::new(eval_with_dims(equiv_expr, env, dim_env));
                    (resolved_face, ty_val, equiv_val)
                })
                .collect();

            // If face is true (satisfied), Glue type reduces to the fiber type T
            for (face, ty_val, _equiv) in &system {
                if matches!(face, crate::syntax::Face::True) {
                    return (**ty_val).clone();
                }
            }

            Value::VGlue {
                base: std::sync::Arc::new(base_val),
                system,
            }
        }

        Expr::GlueTm { base, fibers } => {
            // Evaluate glue term: glue [φ ↦ t] a
            let base_val = eval_with_dims(base, env, dim_env);

            // Evaluate fiber values
            let fibers_val: Vec<(crate::syntax::Face, std::sync::Arc<Value>)> = fibers
                .iter()
                .map(|(face, fiber_expr)| {
                    let resolved_face = resolve_face(face, dim_env);
                    let fiber_val = std::sync::Arc::new(eval_with_dims(fiber_expr, env, dim_env));
                    (resolved_face, fiber_val)
                })
                .collect();

            // If any face is true, return the fiber value
            for (face, fiber_val) in &fibers_val {
                if matches!(face, crate::syntax::Face::True) {
                    return (**fiber_val).clone();
                }
            }

            // For now, return base (simplified - full impl would track Glue type)
            Value::VGlueTm {
                base: std::sync::Arc::new(base_val),
                fibers: fibers_val,
                ty: std::sync::Arc::new(Value::VType(0)), // Placeholder, would be inferred
            }
        }

        Expr::Unglue { glue } => {
            // Evaluate unglue: extracts base value from glued value
            let glue_val = eval_with_dims(glue, env, dim_env);

            match &glue_val {
                // If we have a glue term, return its base
                Value::VGlueTm { base, .. } => (**base).clone(),

                // If glue is neutral, create neutral unglue
                Value::VNeutral { ty, neutral } => {
                    Value::VNeutral {
                        ty: match ty.as_ref() {
                            Value::VGlue { base, .. } => base.clone(),
                            _ => ty.clone(),
                        },
                        neutral: crate::value::Neutral::NUnglue {
                            glue_val: Box::new(neutral.clone()),
                            glue_ty: ty.clone(),
                        },
                    }
                }

                // For other values, return as-is (shouldn't happen with well-typed terms)
                _ => glue_val,
            }
        }

        Expr::Diff { order: _, dim: _, expr } => {
            // Differential operator - simplified
            let _expr_val = eval_with_dims(expr, env, dim_env);
            // Placeholder: actual implementation would compute derivative
            Value::VType(0) // Temporary
        }

        Expr::Integral {
            dim: _,
            from: _,
            to: _,
            expr,
        } => {
            // Integral operator - simplified
            let _expr_val = eval_with_dims(expr, env, dim_env);
            // Placeholder: actual implementation would compute integral
            Value::VType(0) // Temporary
        }

        Expr::Taylor { order: _, point, expr } => {
            // Taylor expansion - simplified
            let _point_val = eval_with_dims(point, env, dim_env);
            let _expr_val = eval_with_dims(expr, env, dim_env);
            // Placeholder
            Value::VType(0) // Temporary
        }
    }
}

/// Resolve a dimension expression to a concrete dimension
fn resolve_dim(dim: &Dim, dim_env: &DimEnv) -> Dim {
    match dim {
        Dim::Var(var) => dim_env
            .get(var.0 as usize)
            .cloned()
            .unwrap_or(Dim::Var(*var)),
        d => d.clone(),
    }
}

/// Resolve dimension variables in a face formula
/// Converts Face::Eq(DimVar(i), b) to Face::True or keeps as-is based on dim_env
fn resolve_face(face: &crate::syntax::Face, dim_env: &DimEnv) -> crate::syntax::Face {
    use crate::syntax::Face;

    match face {
        Face::Eq(var, val) => {
            // Look up the dimension variable in dim_env
            match dim_env.get(var.0 as usize) {
                Some(Dim::Zero) => {
                    // Dimension is 0, check if val is false (representing 0)
                    if !*val {
                        Face::True // i=0 is satisfied when i IS 0
                    } else {
                        // i=1 is NOT satisfied when i IS 0 - return impossible face
                        Face::Eq(*var, *val) // Keep as-is, won't be satisfied
                    }
                }
                Some(Dim::One) => {
                    // Dimension is 1, check if val is true (representing 1)
                    if *val {
                        Face::True // i=1 is satisfied when i IS 1
                    } else {
                        // i=0 is NOT satisfied when i IS 1 - return impossible face
                        Face::Eq(*var, *val) // Keep as-is, won't be satisfied
                    }
                }
                _ => Face::Eq(*var, *val), // Keep as-is if not resolved
            }
        }
        Face::And(f1, f2) => {
            let r1 = resolve_face(f1, dim_env);
            let r2 = resolve_face(f2, dim_env);
            match (&r1, &r2) {
                (Face::True, f) | (f, Face::True) => f.clone(),
                _ => Face::And(Box::new(r1), Box::new(r2)),
            }
        }
        Face::True => Face::True,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::syntax::Name;

    #[test]
    fn test_eval_type() {
        let expr = Expr::Type(0);
        let env = Env::new();
        let val = eval(&expr, &env);
        assert!(matches!(val, Value::VType(0)));
    }

    #[test]
    fn test_eval_identity() {
        // λx. x
        let expr = Expr::Lambda {
            name: Name("x".to_string()),
            body: Box::new(Expr::Var(Name("x".to_string()), 0)),
        };
        let env = Env::new();
        let val = eval(&expr, &env);
        assert!(matches!(val, Value::VLam { .. }));
    }
}
