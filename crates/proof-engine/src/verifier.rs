//! Real-time proof verification

use geometry::{Construction, Line, Point};
use sctt_core::{check_type, eval, infer, normalize, Context, Expr};
use serde::{Deserialize, Serialize};

use crate::{Error, Result};

/// Proof verifier that checks correctness in real-time
pub struct Verifier {
    /// Type checking context
    context: Context,
    /// Verification options
    options: VerificationOptions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationOptions {
    /// Check proof normalization
    pub normalize_proofs: bool,
    /// Verify geometric correspondence
    pub verify_geometry: bool,
    /// Maximum verification time (milliseconds)
    pub timeout_ms: Option<u64>,
}

impl Default for VerificationOptions {
    fn default() -> Self {
        Self {
            normalize_proofs: true,
            verify_geometry: true,
            timeout_ms: Some(5000),
        }
    }
}

/// Result of verification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationResult {
    /// Whether the proof is valid
    pub valid: bool,
    /// Detailed messages
    pub messages: Vec<VerificationMessage>,
    /// Time taken (milliseconds)
    pub time_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationMessage {
    /// Severity level
    pub level: MessageLevel,
    /// Message text
    pub message: String,
    /// Location in proof (optional)
    pub location: Option<ProofLocation>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessageLevel {
    Info,
    Warning,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofLocation {
    /// Line number in proof script
    pub line: Option<usize>,
    /// Column number
    pub column: Option<usize>,
    /// Point ID in geometric construction
    pub point_id: Option<geometry::PointId>,
    /// Line ID in geometric construction
    pub line_id: Option<geometry::LineId>,
}

impl Verifier {
    pub fn new() -> Self {
        Self {
            context: Context::new(),
            options: VerificationOptions::default(),
        }
    }

    pub fn with_options(options: VerificationOptions) -> Self {
        Self {
            context: Context::new(),
            options,
        }
    }

    /// Verify a single proof term
    pub fn verify_term(&self, term: &Expr, expected_type: &Expr) -> Result<VerificationResult> {
        let start = std::time::Instant::now();
        let mut messages = Vec::new();

        // Evaluate expected type to Value
        let expected_type_val = eval(expected_type, self.context.env());

        // Type check
        match check_type(&self.context, term, &expected_type_val) {
            Ok(()) => {
                messages.push(VerificationMessage {
                    level: MessageLevel::Info,
                    message: "Type checking succeeded".to_string(),
                    location: None,
                });
            }
            Err(e) => {
                messages.push(VerificationMessage {
                    level: MessageLevel::Error,
                    message: format!("Type checking failed: {}", e),
                    location: None,
                });

                return Ok(VerificationResult {
                    valid: false,
                    messages,
                    time_ms: start.elapsed().as_millis() as u64,
                });
            }
        }

        // Optionally normalize
        if self.options.normalize_proofs {
            let term_val = eval(term, self.context.env());
            let normalized = normalize(&term_val);
            messages.push(VerificationMessage {
                level: MessageLevel::Info,
                message: format!("Normalized to: {:?}", normalized),
                location: None,
            });
        }

        Ok(VerificationResult {
            valid: true,
            messages,
            time_ms: start.elapsed().as_millis() as u64,
        })
    }

    /// Verify an entire geometric construction
    pub fn verify_construction(&self, construction: &Construction) -> Result<VerificationResult> {
        let start = std::time::Instant::now();
        let mut messages = Vec::new();
        let mut valid = true;

        // Verify graph structure
        match construction.verify() {
            Ok(()) => {
                messages.push(VerificationMessage {
                    level: MessageLevel::Info,
                    message: "Graph structure is valid".to_string(),
                    location: None,
                });
            }
            Err(e) => {
                messages.push(VerificationMessage {
                    level: MessageLevel::Error,
                    message: format!("Graph structure invalid: {}", e),
                    location: None,
                });
                valid = false;
            }
        }

        // Verify each point (proposition)
        for point in construction.graph.points.values() {
            match self.verify_point(point) {
                Ok(result) => {
                    messages.extend(result.messages);
                    if !result.valid {
                        valid = false;
                    }
                }
                Err(e) => {
                    messages.push(VerificationMessage {
                        level: MessageLevel::Error,
                        message: format!("Point {:?} verification failed: {}", point.id, e),
                        location: Some(ProofLocation {
                            line: None,
                            column: None,
                            point_id: Some(point.id),
                            line_id: None,
                        }),
                    });
                    valid = false;
                }
            }
        }

        // Verify each line (implication)
        for line in construction.graph.lines.values() {
            let from_point = construction.graph.points.get(&line.from);
            let to_point = construction.graph.points.get(&line.to);

            if let (Some(from), Some(to)) = (from_point, to_point) {
                match self.verify_line(line, from, to) {
                    Ok(result) => {
                        messages.extend(result.messages);
                        if !result.valid {
                            valid = false;
                        }
                    }
                    Err(e) => {
                        messages.push(VerificationMessage {
                            level: MessageLevel::Error,
                            message: format!("Line {:?} verification failed: {}", line.id, e),
                            location: Some(ProofLocation {
                                line: None,
                                column: None,
                                point_id: None,
                                line_id: Some(line.id),
                            }),
                        });
                        valid = false;
                    }
                }
            } else {
                messages.push(VerificationMessage {
                    level: MessageLevel::Error,
                    message: format!("Line {:?} has invalid endpoints", line.id),
                    location: Some(ProofLocation {
                        line: None,
                        column: None,
                        point_id: None,
                        line_id: Some(line.id),
                    }),
                });
                valid = false;
            }
        }

        Ok(VerificationResult {
            valid,
            messages,
            time_ms: start.elapsed().as_millis() as u64,
        })
    }

    /// Verify a single point
    fn verify_point(&self, point: &Point) -> Result<VerificationResult> {
        let mut messages = Vec::new();

        // Infer the type
        match infer(&self.context, &point.proposition) {
            Ok(_ty) => {
                messages.push(VerificationMessage {
                    level: MessageLevel::Info,
                    message: format!("Point {} type checks", point.label),
                    location: Some(ProofLocation {
                        line: None,
                        column: None,
                        point_id: Some(point.id),
                        line_id: None,
                    }),
                });
            }
            Err(e) => {
                messages.push(VerificationMessage {
                    level: MessageLevel::Error,
                    message: format!("Point {} type error: {}", point.label, e),
                    location: Some(ProofLocation {
                        line: None,
                        column: None,
                        point_id: Some(point.id),
                        line_id: None,
                    }),
                });

                return Ok(VerificationResult {
                    valid: false,
                    messages,
                    time_ms: 0,
                });
            }
        }

        Ok(VerificationResult {
            valid: true,
            messages,
            time_ms: 0,
        })
    }

    /// Verify a single line (implication)
    fn verify_line(&self, line: &Line, from: &Point, to: &Point) -> Result<VerificationResult> {
        let mut messages = Vec::new();

        // The line should represent a function from -> to
        let expected_type_expr = Expr::Pi {
            name: sctt_core::Name("_".to_string()),
            domain: Box::new(from.proposition.clone()),
            codomain: Box::new(to.proposition.clone()),
        };

        let expected_type = eval(&expected_type_expr, self.context.env());

        match check_type(&self.context, &line.proof_term, &expected_type) {
            Ok(()) => {
                messages.push(VerificationMessage {
                    level: MessageLevel::Info,
                    message: format!("Line {} is valid", line.label),
                    location: Some(ProofLocation {
                        line: None,
                        column: None,
                        point_id: None,
                        line_id: Some(line.id),
                    }),
                });
            }
            Err(e) => {
                messages.push(VerificationMessage {
                    level: MessageLevel::Error,
                    message: format!("Line {} type error: {}", line.label, e),
                    location: Some(ProofLocation {
                        line: None,
                        column: None,
                        point_id: None,
                        line_id: Some(line.id),
                    }),
                });

                return Ok(VerificationResult {
                    valid: false,
                    messages,
                    time_ms: 0,
                });
            }
        }

        Ok(VerificationResult {
            valid: true,
            messages,
            time_ms: 0,
        })
    }

    /// Add a type binding to the context
    pub fn add_axiom(&mut self, name: String, ty: Expr) -> Result<()> {
        let ty_value = sctt_core::eval(&ty, &sctt_core::Env::new());
        self.context = self.context.extend(sctt_core::Name(name), ty_value);
        Ok(())
    }

    /// Get the current context
    pub fn context(&self) -> &Context {
        &self.context
    }
}

impl Default for Verifier {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sctt_core::Name;

    #[test]
    fn test_verifier_creation() {
        let verifier = Verifier::new();
        assert!(verifier.options.normalize_proofs);
    }

    #[test]
    fn test_verify_simple_term() {
        let verifier = Verifier::new();
        let term = Expr::Type(0);
        let ty = Expr::Type(1);

        let result = verifier.verify_term(&term, &ty).unwrap();
        assert!(result.valid);
    }

    #[test]
    fn test_verification_message() {
        let msg = VerificationMessage {
            level: MessageLevel::Info,
            message: "Test message".to_string(),
            location: None,
        };

        assert_eq!(msg.level, MessageLevel::Info);
    }
}
