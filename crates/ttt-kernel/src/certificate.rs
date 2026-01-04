//! Verification certificates.
//!
//! A certificate is cryptographic proof that the kernel validated a judgment.
//! Certificates are unforgeable: they can only be created by the kernel.

use std::time::{SystemTime, UNIX_EPOCH};

use crate::judgment::{Judgment, VerifiedJudgment};

/// A cryptographic certificate proving a judgment was verified.
#[derive(Clone, Debug)]
pub struct Certificate {
    /// The verified judgment.
    judgment: Judgment,
    /// Hash of the kernel version that produced this certificate.
    kernel_version: String,
    /// Unix timestamp of verification.
    timestamp: u64,
    /// Cryptographic signature.
    /// WIP: Using placeholder; will implement Ed25519 in later phase.
    signature: Vec<u8>,
}

impl Certificate {
    /// Create a new certificate for a verified judgment.
    ///
    /// This should only be called by the kernel after successful verification.
    pub fn new(verified: &VerifiedJudgment) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        // WIP: Proper signature implementation
        let signature = Self::compute_signature(verified.judgment(), timestamp);

        Certificate {
            judgment: verified.judgment().clone(),
            kernel_version: env!("CARGO_PKG_VERSION").to_string(),
            timestamp,
            signature,
        }
    }

    /// Get the certified judgment.
    pub fn judgment(&self) -> &Judgment {
        &self.judgment
    }

    /// Get the kernel version that issued this certificate.
    pub fn kernel_version(&self) -> &str {
        &self.kernel_version
    }

    /// Get the verification timestamp.
    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }

    /// Verify the certificate's integrity.
    ///
    /// Returns `true` if the signature is valid.
    pub fn verify(&self) -> bool {
        let expected = Self::compute_signature(&self.judgment, self.timestamp);
        self.signature == expected
    }

    /// Compute signature for a judgment.
    ///
    /// WIP: Replace with proper Ed25519 signature.
    fn compute_signature(judgment: &Judgment, timestamp: u64) -> Vec<u8> {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        // Hash the judgment structure
        format!("{:?}", judgment).hash(&mut hasher);
        timestamp.hash(&mut hasher);

        hasher.finish().to_le_bytes().to_vec()
    }

    /// Serialize to JSON.
    /// WIP: Implement proper serialization.
    pub fn to_json(&self) -> String {
        format!(
            r#"{{"kernel_version":"{}","timestamp":{},"signature":"{}"}}"#,
            self.kernel_version,
            self.timestamp,
            hex::encode(&self.signature)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kernel::TrustKernel;
    use ttt_core::{Context, Term};

    #[test]
    fn test_certificate_creation() {
        let kernel = TrustKernel::new();
        let judgment = Judgment::inhabitation(Context::empty(), Term::Star, Term::Unit);
        let verified = kernel.check(judgment).unwrap();

        let cert = Certificate::new(&verified);
        assert!(!cert.kernel_version().is_empty());
        assert!(cert.timestamp() > 0);
    }

    #[test]
    fn test_certificate_verification() {
        let kernel = TrustKernel::new();
        let judgment = Judgment::inhabitation(Context::empty(), Term::True, Term::Bool);
        let verified = kernel.check(judgment).unwrap();

        let cert = Certificate::new(&verified);
        assert!(cert.verify());
    }

    #[test]
    fn test_certificate_json() {
        let kernel = TrustKernel::new();
        let judgment = Judgment::inhabitation(Context::empty(), Term::Star, Term::Unit);
        let verified = kernel.check(judgment).unwrap();

        let cert = Certificate::new(&verified);
        let json = cert.to_json();

        assert!(json.contains("kernel_version"));
        assert!(json.contains("timestamp"));
        assert!(json.contains("signature"));
    }
}
