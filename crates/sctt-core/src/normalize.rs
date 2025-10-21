//! Normalization: converting values back to expressions

use crate::syntax::Expr;
use crate::value::{Neutral, Value};

/// Convert a value back to an expression (read-back)
pub fn normalize(val: &Value) -> Expr {
    normalize_at_level(val, 0)
}

fn normalize_at_level(val: &Value, level: u32) -> Expr {
    match val {
        Value::VType(l) => Expr::Type(*l),

        Value::VPi {
            name,
            domain,
            closure,
        } => {
            let domain_expr = normalize_at_level(domain, level);

            // Create fresh variable
            let var = Value::VNeutral {
                ty: domain.clone(),
                neutral: Neutral::NVar(name.clone(), level),
            };
            let codomain_val = closure.apply(var);
            let codomain_expr = normalize_at_level(&codomain_val, level + 1);

            Expr::Pi {
                name: name.clone(),
                domain: Box::new(domain_expr),
                codomain: Box::new(codomain_expr),
            }
        }

        Value::VLam { name, closure } => {
            // Apply to fresh variable
            let var = Value::VNeutral {
                ty: std::sync::Arc::new(Value::VType(0)), // Simplified
                neutral: Neutral::NVar(name.clone(), level),
            };
            let body_val = closure.apply(var);
            let body_expr = normalize_at_level(&body_val, level + 1);

            Expr::Lambda {
                name: name.clone(),
                body: Box::new(body_expr),
            }
        }

        Value::VPath { ty, left, right } => {
            let ty_expr = normalize_at_level(ty, level);
            let left_expr = normalize_at_level(left, level);
            let right_expr = normalize_at_level(right, level);

            Expr::Path {
                ty: Box::new(ty_expr),
                left: Box::new(left_expr),
                right: Box::new(right_expr),
            }
        }

        Value::VSmoothPath {
            order,
            ty,
            left,
            right,
        } => {
            let ty_expr = normalize_at_level(ty, level);
            let left_expr = normalize_at_level(left, level);
            let right_expr = normalize_at_level(right, level);

            Expr::SmoothPath {
                order: *order,
                ty: Box::new(ty_expr),
                left: Box::new(left_expr),
                right: Box::new(right_expr),
            }
        }

        Value::VPathLam { dim, dim_closure } => {
            // Apply to dimension variable
            let dim_val = crate::syntax::Dim::Var(*dim);
            let body_val = dim_closure.apply(dim_val);
            let body_expr = normalize_at_level(&body_val, level);

            Expr::PathLam {
                dim: *dim,
                body: Box::new(body_expr),
            }
        }

        Value::VNeutral { neutral, .. } => normalize_neutral(neutral, level),
    }
}

fn normalize_neutral(neutral: &Neutral, level: u32) -> Expr {
    match neutral {
        Neutral::NVar(name, idx) => Expr::Var(name.clone(), *idx),

        Neutral::NApp { func, arg } => {
            let func_expr = normalize_neutral(func, level);
            let arg_expr = normalize_at_level(arg, level);

            Expr::App {
                func: Box::new(func_expr),
                arg: Box::new(arg_expr),
            }
        }

        Neutral::NPathApp { path, dim } => {
            let path_expr = normalize_neutral(path, level);

            Expr::PathApp {
                path: Box::new(path_expr),
                dim: dim.clone(),
            }
        }

        Neutral::NComp { ty, base } => {
            let ty_expr = normalize_at_level(ty, level);
            let base_expr = normalize_neutral(base, level);

            Expr::Comp {
                ty: Box::new(ty_expr),
                base: Box::new(base_expr),
                faces: Vec::new(), // Simplified
            }
        }

        Neutral::NCoe {
            ty_fam,
            from,
            to,
            base,
        } => {
            let ty_fam_expr = normalize_at_level(ty_fam, level);
            let base_expr = normalize_neutral(base, level);

            Expr::Coe {
                ty_fam: Box::new(ty_fam_expr),
                from: from.clone(),
                to: to.clone(),
                base: Box::new(base_expr),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::eval::eval;
    use crate::syntax::Name;
    use crate::value::Env;

    #[test]
    fn test_normalize_type() {
        let val = Value::VType(0);
        let expr = normalize(&val);
        assert!(matches!(expr, Expr::Type(0)));
    }

    #[test]
    fn test_eval_normalize_roundtrip() {
        // λx. x should normalize to itself
        let expr = Expr::Lambda {
            name: Name("x".to_string()),
            body: Box::new(Expr::Var(Name("x".to_string()), 0)),
        };

        let val = eval(&expr, &Env::new());
        let normalized = normalize(&val);

        // Check structural equality (simplified)
        match normalized {
            Expr::Lambda { .. } => {} // Success
            _ => panic!("Expected lambda"),
        }
    }
}
