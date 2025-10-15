//! Semantic values resulting from evaluation
//!
//! Values represent fully evaluated terms in normal form.

use crate::syntax::{Dim, DimVar, Level, Name};
use std::sync::Arc;

/// Semantic values (results of evaluation)
#[derive(Debug, Clone)]
pub enum Value {
    /// Universe type
    VType(Level),

    /// Dependent function types
    VPi {
        name: Name,
        domain: Arc<Value>,
        closure: Closure,
    },

    /// Lambda values (closures)
    VLam {
        name: Name,
        closure: Closure,
    },

    /// Path type values
    VPath {
        ty: Arc<Value>,
        left: Arc<Value>,
        right: Arc<Value>,
    },

    /// Path lambda values
    VPathLam {
        dim: DimVar,
        dim_closure: DimClosure,
    },

    /// Smooth path values with differentiability order
    VSmoothPath {
        order: u32,
        ty: Arc<Value>,
        left: Arc<Value>,
        right: Arc<Value>,
    },

    /// Neutral values (stuck computations)
    VNeutral { ty: Arc<Value>, neutral: Neutral },
}

/// Closures capture environment for lazy evaluation
#[derive(Clone)]
pub struct Closure {
    env: Env,
    body: crate::syntax::Expr,
}

impl Closure {
    pub fn new(env: Env, body: crate::syntax::Expr) -> Self {
        Self { env, body }
    }

    pub fn apply(&self, arg: Value) -> Value {
        let mut new_env = self.env.clone();
        new_env.push_back(arg);
        crate::eval::eval(&self.body, &new_env)
    }
}

impl std::fmt::Debug for Closure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<closure>")
    }
}

/// Dimension closures for path types
#[derive(Clone)]
pub struct DimClosure {
    env: Env,
    dim_env: DimEnv,
    body: crate::syntax::Expr,
}

impl DimClosure {
    pub fn new(env: Env, dim_env: DimEnv, body: crate::syntax::Expr) -> Self {
        Self { env, dim_env, body }
    }

    pub fn apply(&self, dim: Dim) -> Value {
        let mut new_dim_env = self.dim_env.clone();
        new_dim_env.push_back(dim);
        crate::eval::eval(&self.body, &self.env)
    }
}

impl std::fmt::Debug for DimClosure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<dim-closure>")
    }
}

/// Neutral terms (blocked computations that cannot reduce further)
#[derive(Debug, Clone)]
pub enum Neutral {
    /// Variables
    NVar(Name, u32),

    /// Application to neutral
    NApp { func: Box<Neutral>, arg: Arc<Value> },

    /// Path application to neutral
    NPathApp { path: Box<Neutral>, dim: Dim },

    /// Composition (Kan operation)
    NComp {
        ty: Arc<Value>,
        base: Box<Neutral>,
        // Simplified for now
    },

    /// Coercion
    NCoe {
        ty_fam: Arc<Value>,
        from: Dim,
        to: Dim,
        base: Box<Neutral>,
    },
}

/// Environment for term evaluation
pub type Env = im::Vector<Value>;

/// Environment for dimension variables
pub type DimEnv = im::Vector<Dim>;

impl Value {
    /// Check if two values are definitionally equal
    pub fn conv(&self, other: &Value) -> bool {
        match (self, other) {
            (Value::VType(l1), Value::VType(l2)) => l1 == l2,
            (Value::VPi { domain: d1, .. }, Value::VPi { domain: d2, .. }) => {
                d1.conv(d2) // Simplified: should also check codomain
            }
            (
                Value::VPath {
                    ty: t1,
                    left: l1,
                    right: r1,
                },
                Value::VPath {
                    ty: t2,
                    left: l2,
                    right: r2,
                },
            ) => t1.conv(t2) && l1.conv(l2) && r1.conv(r2),
            (Value::VNeutral { neutral: n1, .. }, Value::VNeutral { neutral: n2, .. }) => {
                n1.conv_neutral(n2)
            }
            _ => false,
        }
    }

    /// Apply a function value to an argument
    pub fn apply(&self, arg: Value) -> Value {
        match self {
            Value::VLam { closure, .. } => closure.apply(arg),
            Value::VNeutral { ty, neutral } => {
                // Extract codomain from Pi type
                let codomain_ty = match ty.as_ref() {
                    Value::VPi { closure, .. } => Arc::new(closure.apply(arg.clone())),
                    _ => ty.clone(),
                };
                Value::VNeutral {
                    ty: codomain_ty,
                    neutral: Neutral::NApp {
                        func: Box::new(neutral.clone()),
                        arg: Arc::new(arg),
                    },
                }
            }
            _ => panic!("Cannot apply non-function value"),
        }
    }

    /// Apply a path value to a dimension
    pub fn apply_path(&self, dim: Dim) -> Value {
        match self {
            Value::VPathLam { dim_closure, .. } => dim_closure.apply(dim.clone()),
            Value::VNeutral { ty, neutral } => {
                // Extract endpoint type from Path type
                let endpoint_ty = match ty.as_ref() {
                    Value::VPath { ty, .. } => ty.clone(),
                    _ => ty.clone(),
                };
                Value::VNeutral {
                    ty: endpoint_ty,
                    neutral: Neutral::NPathApp {
                        path: Box::new(neutral.clone()),
                        dim,
                    },
                }
            }
            _ => panic!("Cannot apply path to non-path value"),
        }
    }
}

impl Neutral {
    fn conv_neutral(&self, other: &Neutral) -> bool {
        match (self, other) {
            (Neutral::NVar(n1, i1), Neutral::NVar(n2, i2)) => n1 == n2 && i1 == i2,
            (Neutral::NApp { func: f1, arg: a1 }, Neutral::NApp { func: f2, arg: a2 }) => {
                f1.conv_neutral(f2) && a1.conv(a2)
            }
            (Neutral::NPathApp { path: p1, dim: d1 }, Neutral::NPathApp { path: p2, dim: d2 }) => {
                p1.conv_neutral(p2) && d1 == d2
            }
            _ => false,
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::VType(l) => write!(f, "Type{}", l),
            Value::VPi { name, domain, .. } => write!(f, "({} : {}) → ...", name, domain),
            Value::VLam { name, .. } => write!(f, "λ{}. ...", name),
            Value::VPath { ty, left, right } => {
                write!(f, "Path {} {} {}", ty, left, right)
            }
            Value::VSmoothPath {
                order,
                ty,
                left,
                right,
            } => {
                write!(f, "SmoothPath^{} {} {} {}", order, ty, left, right)
            }
            Value::VNeutral { neutral, .. } => write!(f, "{}", neutral),
            _ => write!(f, "..."),
        }
    }
}

impl std::fmt::Display for Neutral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Neutral::NVar(name, _) => write!(f, "{}", name),
            Neutral::NApp { func, arg } => write!(f, "({} {})", func, arg),
            Neutral::NPathApp { path, dim } => write!(f, "({} @ {:?})", path, dim),
            _ => write!(f, "..."),
        }
    }
}
