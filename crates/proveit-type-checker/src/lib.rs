//! O(1) Neural Type Checker
//!
//! Provides constant-time type checking using pre-computed type signatures
//! and hash-based lookup. Designed for real-time verification in interactive
//! geometric proof construction.

use proveit_core::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Type category in the type system
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TypeCategory {
    /// Base types (geometric primitives)
    Base,
    /// Function types (transformations)
    Function,
    /// Product types (tuples/records)
    Product,
    /// Sum types (variants)
    Sum,
    /// Universe types (type of types)
    Universe(usize),
    /// Path types (for homotopy type theory)
    Path,
}

/// Type signature for O(1) lookup
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TypeSignature {
    pub category: TypeCategory,
    pub hash: u64,
    pub description: String,
}

impl TypeSignature {
    pub fn new(category: TypeCategory, description: String) -> Self {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        category.hash(&mut hasher);
        description.hash(&mut hasher);
        let hash = hasher.finish();
        
        Self {
            category,
            hash,
            description,
        }
    }
}

/// O(1) Neural Type Checker using hash-based lookups
pub struct NeuralTypeChecker {
    /// Pre-computed type signatures for O(1) lookup
    type_cache: HashMap<u64, TypeSignature>,
    /// Type compatibility matrix (pre-computed)
    compatibility: HashMap<(TypeCategory, TypeCategory), bool>,
}

impl NeuralTypeChecker {
    /// Create a new type checker with pre-computed signatures
    pub fn new() -> Self {
        let mut checker = Self {
            type_cache: HashMap::new(),
            compatibility: HashMap::new(),
        };
        
        // Pre-compute compatibility rules for O(1) checking
        checker.initialize_compatibility();
        
        checker
    }
    
    /// Initialize type compatibility rules
    fn initialize_compatibility(&mut self) {
        use TypeCategory::*;
        
        // Base types are compatible with themselves
        self.compatibility.insert((Base, Base), true);
        
        // Functions are compatible with functions
        self.compatibility.insert((Function, Function), true);
        
        // Products are compatible with products
        self.compatibility.insert((Product, Product), true);
        
        // Sums are compatible with sums
        self.compatibility.insert((Sum, Sum), true);
        
        // Path types for homotopy
        self.compatibility.insert((Path, Path), true);
        
        // Universe hierarchy
        for i in 0..10 {
            for j in 0..10 {
                if i <= j {
                    self.compatibility.insert((Universe(i), Universe(j)), true);
                }
            }
        }
    }
    
    /// Register a type signature for O(1) lookup
    pub fn register_type(&mut self, signature: TypeSignature) {
        self.type_cache.insert(signature.hash, signature);
    }
    
    /// Check type compatibility in O(1) time
    pub fn check_compatible(&self, sig1: &TypeSignature, sig2: &TypeSignature) -> Result<bool> {
        // O(1) hash lookup for exact match
        if sig1.hash == sig2.hash {
            return Ok(true);
        }
        
        // O(1) lookup in compatibility matrix
        let key = (sig1.category, sig2.category);
        Ok(self.compatibility.get(&key).copied().unwrap_or(false))
    }
    
    /// Verify a type signature exists in O(1) time
    pub fn verify_type(&self, hash: u64) -> Result<bool> {
        Ok(self.type_cache.contains_key(&hash))
    }
    
    /// Get type signature by hash in O(1) time
    pub fn get_type(&self, hash: u64) -> Option<&TypeSignature> {
        self.type_cache.get(&hash)
    }
    
    /// Get description for accessibility
    pub fn describe_type(&self, hash: u64) -> String {
        self.type_cache
            .get(&hash)
            .map(|sig| sig.description.clone())
            .unwrap_or_else(|| "Unknown type".to_string())
    }
}

impl Default for NeuralTypeChecker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_signature_creation() {
        let sig = TypeSignature::new(TypeCategory::Base, "Point".to_string());
        assert_eq!(sig.category, TypeCategory::Base);
        assert_eq!(sig.description, "Point");
    }

    #[test]
    fn test_type_registration() {
        let mut checker = NeuralTypeChecker::new();
        let sig = TypeSignature::new(TypeCategory::Base, "Point".to_string());
        let hash = sig.hash;
        
        checker.register_type(sig);
        assert!(checker.verify_type(hash).unwrap());
    }

    #[test]
    fn test_type_compatibility() {
        let checker = NeuralTypeChecker::new();
        let sig1 = TypeSignature::new(TypeCategory::Base, "Point".to_string());
        let sig2 = TypeSignature::new(TypeCategory::Base, "Line".to_string());
        
        assert!(checker.check_compatible(&sig1, &sig2).unwrap());
    }

    #[test]
    fn test_universe_hierarchy() {
        let checker = NeuralTypeChecker::new();
        let sig1 = TypeSignature::new(TypeCategory::Universe(0), "Type0".to_string());
        let sig2 = TypeSignature::new(TypeCategory::Universe(1), "Type1".to_string());
        
        assert!(checker.check_compatible(&sig1, &sig2).unwrap());
    }
}
