//! Typing contexts for TTT.
//!
//! A context Gamma is a sequence of type declarations, representing the
//! assumptions in scope at any point in a derivation.
//!
//! Gamma ::= . | Gamma, x : A
//!
//! In de Bruijn representation, we don't need explicit names-position
//! in the context determines the variable index.

use crate::term::Term;

/// A typing context: a list of types for variables in scope.
///
/// Index 0 in a term refers to the *last* entry (most recently bound).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Context {
    /// Types in reverse order: `entries[0]` is the type of Var(entries.len()-1)
    /// This allows O(1) push/pop at the most-recent end.
    entries: Vec<ContextEntry>,
}

/// An entry in the typing context.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ContextEntry {
    /// The type of the variable at this position.
    pub ty: Term,
    /// Optional definition (for let-bound variables).
    /// WIP: Not fully implemented in Phase 1.
    pub def: Option<Term>,
    /// Optional name hint (for error messages and pretty printing).
    pub name: Option<String>,
}

impl Context {
    /// Create an empty context.
    pub fn empty() -> Self {
        Context {
            entries: Vec::new(),
        }
    }

    /// Create a context with a single entry.
    pub fn singleton(ty: Term) -> Self {
        Context {
            entries: vec![ContextEntry {
                ty,
                def: None,
                name: None,
            }],
        }
    }

    /// Number of variables in scope.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Check if context is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Extend the context with a new variable type.
    ///
    /// The new variable will have de Bruijn index 0.
    pub fn extend(&self, ty: Term) -> Self {
        let mut new_entries = self.entries.clone();
        new_entries.push(ContextEntry {
            ty,
            def: None,
            name: None,
        });
        Context {
            entries: new_entries,
        }
    }

    /// Extend with a named variable.
    pub fn extend_named(&self, name: impl Into<String>, ty: Term) -> Self {
        let mut new_entries = self.entries.clone();
        new_entries.push(ContextEntry {
            ty,
            def: None,
            name: Some(name.into()),
        });
        Context {
            entries: new_entries,
        }
    }

    /// Extend with a let-defined variable.
    pub fn extend_def(&self, ty: Term, def: Term) -> Self {
        let mut new_entries = self.entries.clone();
        new_entries.push(ContextEntry {
            ty,
            def: Some(def),
            name: None,
        });
        Context {
            entries: new_entries,
        }
    }

    /// Look up the type of a variable by de Bruijn index.
    ///
    /// Returns `None` if the index is out of bounds.
    pub fn lookup(&self, idx: u32) -> Option<&Term> {
        let position = self.entries.len().checked_sub(1 + idx as usize)?;
        Some(&self.entries[position].ty)
    }

    /// Look up the full entry for a variable.
    pub fn lookup_entry(&self, idx: u32) -> Option<&ContextEntry> {
        let position = self.entries.len().checked_sub(1 + idx as usize)?;
        Some(&self.entries[position])
    }

    /// Look up any definition for a variable.
    pub fn lookup_def(&self, idx: u32) -> Option<&Term> {
        self.lookup_entry(idx)?.def.as_ref()
    }

    /// Iterate over entries from oldest to newest (outer to inner binder).
    pub fn iter(&self) -> impl Iterator<Item = &ContextEntry> {
        self.entries.iter()
    }

    /// Iterate over entries from newest to oldest (inner to outer binder).
    pub fn iter_rev(&self) -> impl Iterator<Item = &ContextEntry> {
        self.entries.iter().rev()
    }

    /// Pop the most recently added entry.
    pub fn pop(&self) -> Option<(ContextEntry, Context)> {
        if self.entries.is_empty() {
            return None;
        }
        let mut new_entries = self.entries.clone();
        let entry = new_entries.pop().unwrap();
        Some((entry, Context { entries: new_entries }))
    }

    /// Get the types in the context as a slice.
    pub fn types(&self) -> Vec<&Term> {
        self.entries.iter().map(|e| &e.ty).collect()
    }
}

impl std::ops::Index<u32> for Context {
    type Output = Term;

    fn index(&self, idx: u32) -> &Self::Output {
        self.lookup(idx).expect("de Bruijn index out of bounds")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_context() {
        let ctx = Context::empty();
        assert!(ctx.is_empty());
        assert_eq!(ctx.len(), 0);
        assert!(ctx.lookup(0).is_none());
    }

    #[test]
    fn test_singleton_context() {
        let ctx = Context::singleton(Term::Bool);
        assert_eq!(ctx.len(), 1);
        assert_eq!(ctx.lookup(0), Some(&Term::Bool));
        assert!(ctx.lookup(1).is_none());
    }

    #[test]
    fn test_extend_context() {
        let ctx = Context::empty().extend(Term::Bool).extend(Term::Unit);

        assert_eq!(ctx.len(), 2);
        // Var(0) is the most recent: Unit
        assert_eq!(ctx.lookup(0), Some(&Term::Unit));
        // Var(1) is the older one: Bool
        assert_eq!(ctx.lookup(1), Some(&Term::Bool));
    }

    #[test]
    fn test_extend_named() {
        let ctx = Context::empty().extend_named("x", Term::Bool);
        let entry = ctx.lookup_entry(0).unwrap();
        assert_eq!(entry.ty, Term::Bool);
        assert_eq!(entry.name, Some("x".to_string()));
    }

    #[test]
    fn test_extend_def() {
        let ctx = Context::empty().extend_def(Term::Bool, Term::True);
        let entry = ctx.lookup_entry(0).unwrap();
        assert_eq!(entry.ty, Term::Bool);
        assert_eq!(entry.def, Some(Term::True));
    }

    #[test]
    fn test_pop_context() {
        let ctx = Context::empty().extend(Term::Bool).extend(Term::Unit);

        let (entry, popped) = ctx.pop().unwrap();
        assert_eq!(entry.ty, Term::Unit);
        assert_eq!(popped.len(), 1);
        assert_eq!(popped.lookup(0), Some(&Term::Bool));
    }

    #[test]
    fn test_pop_empty() {
        let ctx = Context::empty();
        assert!(ctx.pop().is_none());
    }

    #[test]
    fn test_index_operator() {
        let ctx = Context::empty().extend(Term::Bool).extend(Term::Unit);
        assert_eq!(ctx[0], Term::Unit);
        assert_eq!(ctx[1], Term::Bool);
    }

    #[test]
    #[should_panic(expected = "de Bruijn index out of bounds")]
    fn test_index_out_of_bounds() {
        let ctx = Context::empty().extend(Term::Bool);
        let _ = ctx[5]; // Should panic
    }

    #[test]
    fn test_iter() {
        let ctx = Context::empty()
            .extend(Term::Bool)
            .extend(Term::Unit)
            .extend(Term::Empty);

        let types: Vec<_> = ctx.iter().map(|e| &e.ty).collect();
        assert_eq!(types, vec![&Term::Bool, &Term::Unit, &Term::Empty]);
    }

    #[test]
    fn test_iter_rev() {
        let ctx = Context::empty()
            .extend(Term::Bool)
            .extend(Term::Unit)
            .extend(Term::Empty);

        let types: Vec<_> = ctx.iter_rev().map(|e| &e.ty).collect();
        assert_eq!(types, vec![&Term::Empty, &Term::Unit, &Term::Bool]);
    }
}
