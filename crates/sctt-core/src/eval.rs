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
                    (face.clone(), eval_with_dims(expr, env, dim_env))
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
                    (face.clone(), eval_with_dims(expr, env, dim_env))
                })
                .collect();

            // Call homogeneous composition from kan.rs
            crate::kan::hcomp(&ty_val, &base_val, &faces_val)
        }

        Expr::Glue {
            base,
            equivalences: _,
        } => {
            // Simplified glue
            
            eval_with_dims(base, env, dim_env)
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
