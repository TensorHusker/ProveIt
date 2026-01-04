//! TTT Trust Kernel
//!
//! The minimal type checker that serves as the sole authority for
//! proof validity in ProveIt.
//!
//! # Design
//!
//! The trust kernel is deliberately minimal (~500 LOC) to enable
//! human auditing. All proof validity flows through `TrustKernel::check`.
//!
//! # Example
//!
//! ```
//! use ttt_kernel::prelude::*;
//! use ttt_core::prelude::*;
//!
//! let kernel = TrustKernel::new();
//!
//! // Check that Star has type Unit
//! let judgment = Judgment::inhabitation(
//!     Context::empty(),
//!     Term::Star,
//!     Term::Unit,
//! );
//!
//! let verified = kernel.check(judgment).expect("should type check");
//! ```
//!
//! # Trust Boundary
//!
//! The `TrustKernel` is the only component that can create `VerifiedJudgment`
//! and `Certificate` values. These types serve as unforgeable evidence that
//! the kernel validated a proof.

pub mod certificate;
pub mod error;
pub mod judgment;
pub mod kernel;

pub mod prelude {
    //! Convenient re-exports for common usage.
    pub use crate::certificate::Certificate;
    pub use crate::error::TypeError;
    pub use crate::judgment::{Judgment, VerifiedJudgment};
    pub use crate::kernel::TrustKernel;
}

pub use error::TypeError;
pub use kernel::TrustKernel;
