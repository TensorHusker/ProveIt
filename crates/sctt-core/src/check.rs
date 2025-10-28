//! Bidirectional type checking

use crate::eval::eval;
use crate::syntax::{Expr, Level, Name};
use crate::value::{Env, Value};
use crate::{Error, Result};
use std::sync::Arc;

/// Type checking context
#[derive(Debug, Clone)]
pub struct Context {
    /// Type bindings for variables
    types: Vec<Value>,
    /// Value environment for evaluation
    env: Env,
    /// Names for error messages
    names: Vec<Name>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            types: Vec::new(),
            env: Env::new(),
            names: Vec::new(),
        }
    }

    /// Extend context with a new binding
    pub fn extend(&self, name: Name, ty: Value) -> Self {
        let mut new_ctx = self.clone();
        new_ctx.names.push(name.clone());
        new_ctx.types.push(ty.clone());
        // Add neutral variable to environment
        let idx = new_ctx.types.len() - 1;
        new_ctx.env.push_back(Value::VNeutral {
            ty: Arc::new(ty),
            neutral: crate::value::Neutral::NVar(name, idx as u32),
        });
        new_ctx
    }

    /// Look up variable type by de Bruijn index
    pub fn lookup(&self, idx: u32) -> Result<Value> {
        self.types
            .get(idx as usize)
            .cloned()
            .ok_or_else(|| Error::UnboundVariable(format!("index {}", idx)))
    }

    /// Get the current environment
    pub fn env(&self) -> &Env {
        &self.env
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}

/// Infer the type of an expression
pub fn infer(ctx: &Context, expr: &Expr) -> Result<Value> {
    match expr {
        Expr::Type(level) => {
            // Type_i : Type_{i+1}
            Ok(Value::VType(level + 1))
        }

        Expr::Var(_name, idx) => ctx.lookup(*idx),

        Expr::Pi {
            name,
            domain,
            codomain,
        } => {
            // Check domain is a type
            let domain_ty = infer(ctx, domain)?;
            let domain_level = expect_type(&domain_ty)?;

            // Check codomain in extended context
            let domain_val = eval(domain, ctx.env());
            let ctx2 = ctx.extend(name.clone(), domain_val);
            let codomain_ty = infer(&ctx2, codomain)?;
            let codomain_level = expect_type(&codomain_ty)?;

            // Result is a type at max of the two levels
            Ok(Value::VType(domain_level.max(codomain_level)))
        }

        Expr::App { func, arg } => {
            let func_ty = infer(ctx, func)?;

            // Extract domain and codomain from Pi type
            let (domain, closure) = expect_pi(&func_ty)?;

            // Check argument against domain
            check(ctx, arg, &domain)?;

            // Apply codomain closure to argument value
            let arg_val = eval(arg, ctx.env());
            Ok(closure.apply(arg_val))
        }

        Expr::Path { ty, left, right } => {
            // Path A x y : Type_i if A : Type_i
            let ty_ty = infer(ctx, ty)?;
            let level = expect_type(&ty_ty)?;

            // Check endpoints have type A
            let a_val = eval(ty, ctx.env());
            check(ctx, left, &a_val)?;
            check(ctx, right, &a_val)?;

            Ok(Value::VType(level))
        }

        Expr::SmoothPath {
            order,
            ty,
            left,
            right,
        } => {
            // Similar to Path but with smoothness constraint
            let ty_ty = infer(ctx, ty)?;
            let level = expect_type(&ty_ty)?;

            let a_val = eval(ty, ctx.env());
            check(ctx, left, &a_val)?;
            check(ctx, right, &a_val)?;

            // Check smoothness compatibility (simplified)
            if *order > 1000 {
                return Err(Error::SmoothnessViolation {
                    expected: 1000,
                    got: *order,
                });
            }

            Ok(Value::VType(level))
        }

        Expr::PathApp { path, dim: _ } => {
            let path_ty = infer(ctx, path)?;

            // Extract base type and endpoints from path type
            let (base_ty, _left, _right) = expect_path(&path_ty)?;

            // Application returns base type
            Ok((*base_ty).clone())
        }

        Expr::Lambda { .. } => Err(Error::CannotInfer(
            "lambda requires type annotation".to_string(),
        )),

        Expr::PathLam { .. } => Err(Error::CannotInfer(
            "path lambda requires type annotation".to_string(),
        )),

        Expr::Comp { ty, base, faces } => {
            // Check type is a valid type
            let ty_ty = infer(ctx, ty)?;
            expect_type(&ty_ty)?;

            // Check base has type ty @ 0
            let ty_val = eval(ty, ctx.env());
            check(ctx, base, &ty_val)?;

            // Check each face expression has type ty
            for (face, face_expr) in faces {
                validate_face(face)?;
                check(ctx, face_expr, &ty_val)?;
            }

            // Result has type ty @ 1 (same as ty for constant types)
            Ok(ty_val)
        }

        Expr::Coe { ty_fam, from, to, base } => {
            // Type family should be a function from dimension to types
            // For now, simplified: just check it's a lambda returning a type
            let ty_fam_ty = infer(ctx, ty_fam)?;

            // Evaluate type family at 'from' dimension
            let ty_fam_val = eval(ty_fam, ctx.env());
            let from_ty = apply_to_dim(&ty_fam_val, from);

            // Check base has type (ty_fam from)
            check(ctx, base, &from_ty)?;

            // Result has type (ty_fam to)
            let to_ty = apply_to_dim(&ty_fam_val, to);
            Ok(to_ty)
        }

        Expr::HComp { ty, base, faces } => {
            // Similar to Comp but for constant types
            let ty_ty = infer(ctx, ty)?;
            expect_type(&ty_ty)?;

            let ty_val = eval(ty, ctx.env());
            check(ctx, base, &ty_val)?;

            for (face, face_expr) in faces {
                validate_face(face)?;
                check(ctx, face_expr, &ty_val)?;
            }

            Ok(ty_val)
        }

        _ => Err(Error::CannotInfer(format!("{}", expr))),
    }
}

/// Check that an expression has a given type
pub fn check(ctx: &Context, expr: &Expr, expected_ty: &Value) -> Result<()> {
    match (expr, expected_ty) {
        (
            Expr::Lambda { name, body },
            Value::VPi {
                name: _,
                domain,
                closure,
            },
        ) => {
            // Check body in extended context
            let ctx2 = ctx.extend(name.clone(), (**domain).clone());
            let codomain = closure.apply(Value::VNeutral {
                ty: domain.clone(),
                neutral: crate::value::Neutral::NVar(name.clone(), ctx.types.len() as u32),
            });
            check(&ctx2, body, &codomain)?;
            Ok(())
        }

        (
            Expr::PathLam { dim: _, body },
            Value::VPath {
                ty,
                left: _,
                right: _,
            },
        ) => {
            // Check body with dimension variable in scope
            // Simplified: would need dimension context
            check(ctx, body, ty)?;
            Ok(())
        }

        _ => {
            // Fall back to inference and conversion
            let inferred_ty = infer(ctx, expr)?;
            if inferred_ty.conv(expected_ty) {
                Ok(())
            } else {
                Err(Error::TypeMismatch {
                    expected: format!("{}", expected_ty),
                    got: format!("{}", inferred_ty),
                })
            }
        }
    }
}

/// Extract level from Type value
fn expect_type(val: &Value) -> Result<Level> {
    match val {
        Value::VType(l) => Ok(*l),
        _ => Err(Error::TypeMismatch {
            expected: "Type".to_string(),
            got: format!("{}", val),
        }),
    }
}

/// Extract domain and codomain from Pi value
fn expect_pi(val: &Value) -> Result<(Value, crate::value::Closure)> {
    match val {
        Value::VPi {
            domain, closure, ..
        } => Ok(((**domain).clone(), closure.clone())),
        _ => Err(Error::TypeMismatch {
            expected: "Pi type".to_string(),
            got: format!("{}", val),
        }),
    }
}

/// Extract components from Path value
fn expect_path(val: &Value) -> Result<(Arc<Value>, Arc<Value>, Arc<Value>)> {
    match val {
        Value::VPath { ty, left, right } => Ok((ty.clone(), left.clone(), right.clone())),
        Value::VSmoothPath {
            ty, left, right, ..
        } => Ok((ty.clone(), left.clone(), right.clone())),
        _ => Err(Error::TypeMismatch {
            expected: "Path type".to_string(),
            got: format!("{}", val),
        }),
    }
}

/// Validate face formula is well-formed
fn validate_face(face: &crate::syntax::Face) -> Result<()> {
    use crate::syntax::Face;
    match face {
        Face::Eq(_var, _val) => Ok(()),
        Face::And(f1, f2) => {
            validate_face(f1)?;
            validate_face(f2)
        }
        Face::True => Ok(()),
    }
}

/// Apply a value to a dimension (for type families)
fn apply_to_dim(val: &Value, _dim: &crate::syntax::Dim) -> Value {
    // Simplified: for constant type families, just return the type
    // Full implementation would handle dimension-dependent types
    match val {
        Value::VLam { closure, .. } => {
            // Apply closure to a neutral dimension variable
            let dim_val = Value::VType(0); // Placeholder
            closure.apply(dim_val)
        }
        _ => val.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_checking() {
        let ctx = Context::new();

        // Type₀ : Type₁
        let expr = Expr::Type(0);
        let ty = infer(&ctx, &expr).unwrap();
        assert!(matches!(ty, Value::VType(1)));
    }

    #[test]
    fn test_pi_type() {
        let ctx = Context::new();

        // (A : Type₀) → Type₀  :  Type₁
        let expr = Expr::Pi {
            name: Name("A".to_string()),
            domain: Box::new(Expr::Type(0)),
            codomain: Box::new(Expr::Type(0)),
        };

        let ty = infer(&ctx, &expr).unwrap();
        assert!(matches!(ty, Value::VType(1)));
    }
}
