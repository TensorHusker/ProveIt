//! Abstract syntax tree for SCTT expressions
//!
//! This module defines the surface syntax of Smooth Cubical Type Theory,
//! representing terms before evaluation.

use serde::{Deserialize, Serialize};

/// Universe levels for the type hierarchy
pub type Level = u32;

/// Names for variables and definitions
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Name(pub String);

impl Name {
    pub fn fresh(base: &str, avoid: &[Name]) -> Self {
        let mut i = 0;
        loop {
            let candidate = if i == 0 {
                Name(base.to_string())
            } else {
                Name(format!("{}{}", base, i))
            };
            if !avoid.contains(&candidate) {
                return candidate;
            }
            i += 1;
        }
    }
}

impl std::fmt::Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Dimension variables for cubical structure (i, j, k, ...)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DimVar(pub u32);

/// Dimension expressions: variables or endpoints (0 or 1)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Dim {
    Var(DimVar),
    Zero,
    One,
}

/// Face formulas for boundary conditions
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Face {
    /// Dimension equals 0 or 1
    Eq(DimVar, bool),
    /// Conjunction of faces
    And(Box<Face>, Box<Face>),
    /// True face (always satisfied)
    True,
}

/// Main expression type representing SCTT terms
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Expr {
    /// Universe at a given level: Type₀, Type₁, ...
    Type(Level),

    /// Variables (de Bruijn indices for implementation, names for display)
    Var(Name, u32),

    /// Dependent function types: (x : A) → B
    Pi {
        name: Name,
        domain: Box<Expr>,
        codomain: Box<Expr>,
    },

    /// Lambda abstraction: λx. body
    Lambda { name: Name, body: Box<Expr> },

    /// Function application: f a
    App { func: Box<Expr>, arg: Box<Expr> },

    /// Path types: Path A x y
    Path {
        ty: Box<Expr>,
        left: Box<Expr>,
        right: Box<Expr>,
    },

    /// Path abstraction: ⟨i⟩ body
    PathLam { dim: DimVar, body: Box<Expr> },

    /// Path application: path @ i
    PathApp { path: Box<Expr>, dim: Dim },

    /// Smooth paths with specified order of differentiability
    SmoothPath {
        order: u32, // C^k smoothness
        ty: Box<Expr>,
        left: Box<Expr>,
        right: Box<Expr>,
    },

    /// Composition operation (Kan filling)
    Comp {
        ty: Box<Expr>,
        base: Box<Expr>,
        faces: Vec<(Face, Expr)>,
    },

    /// Coercion along type family
    Coe {
        ty_fam: Box<Expr>,
        from: Dim,
        to: Dim,
        base: Box<Expr>,
    },

    /// Homogeneous composition
    HComp {
        ty: Box<Expr>,
        base: Box<Expr>,
        faces: Vec<(Face, Expr)>,
    },

    /// Glue types for univalence
    Glue {
        base: Box<Expr>,
        equivalences: Vec<(Face, Expr, Expr)>, // (face, type, equiv)
    },

    /// Differential operator: d/di (smooth_path)
    Diff {
        order: u32,
        dim: DimVar,
        expr: Box<Expr>,
    },

    /// Integral operator (inverse of differentiation)
    Integral {
        dim: DimVar,
        from: Dim,
        to: Dim,
        expr: Box<Expr>,
    },

    /// Taylor expansion around a point
    Taylor {
        order: u32,
        point: Box<Expr>,
        expr: Box<Expr>,
    },
}

impl Expr {
    /// Create a simple function type: A → B (non-dependent)
    pub fn arrow(domain: Expr, codomain: Expr) -> Self {
        Expr::Pi {
            name: Name("_".to_string()),
            domain: Box::new(domain),
            codomain: Box::new(codomain),
        }
    }

    /// Substitute a term for variable at given de Bruijn level
    pub fn subst(&self, level: u32, replacement: &Expr) -> Expr {
        match self {
            Expr::Type(l) => Expr::Type(*l),
            Expr::Var(name, idx) => {
                if *idx == level {
                    replacement.clone()
                } else {
                    Expr::Var(name.clone(), *idx)
                }
            }
            Expr::Pi {
                name,
                domain,
                codomain,
            } => Expr::Pi {
                name: name.clone(),
                domain: Box::new(domain.subst(level, replacement)),
                codomain: Box::new(codomain.subst(level + 1, replacement)),
            },
            Expr::Lambda { name, body } => Expr::Lambda {
                name: name.clone(),
                body: Box::new(body.subst(level + 1, replacement)),
            },
            Expr::App { func, arg } => Expr::App {
                func: Box::new(func.subst(level, replacement)),
                arg: Box::new(arg.subst(level, replacement)),
            },
            Expr::Path { ty, left, right } => Expr::Path {
                ty: Box::new(ty.subst(level, replacement)),
                left: Box::new(left.subst(level, replacement)),
                right: Box::new(right.subst(level, replacement)),
            },
            Expr::PathLam { dim, body } => Expr::PathLam {
                dim: *dim,
                body: Box::new(body.subst(level, replacement)),
            },
            Expr::PathApp { path, dim } => Expr::PathApp {
                path: Box::new(path.subst(level, replacement)),
                dim: dim.clone(),
            },
            Expr::SmoothPath {
                order,
                ty,
                left,
                right,
            } => Expr::SmoothPath {
                order: *order,
                ty: Box::new(ty.subst(level, replacement)),
                left: Box::new(left.subst(level, replacement)),
                right: Box::new(right.subst(level, replacement)),
            },
            // Add cases for other constructors...
            _ => self.clone(), // Placeholder
        }
    }
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Type(l) => write!(f, "Type{}", l),
            Expr::Var(name, _) => write!(f, "{}", name),
            Expr::Pi {
                name,
                domain,
                codomain,
            } => {
                write!(f, "({} : {}) → {}", name, domain, codomain)
            }
            Expr::Lambda { name, body } => write!(f, "λ{}. {}", name, body),
            Expr::App { func, arg } => write!(f, "({} {})", func, arg),
            Expr::Path { ty, left, right } => {
                write!(f, "Path {} {} {}", ty, left, right)
            }
            Expr::SmoothPath {
                order,
                ty,
                left,
                right,
            } => {
                write!(f, "SmoothPath^{} {} {} {}", order, ty, left, right)
            }
            _ => write!(f, "..."),
        }
    }
}
