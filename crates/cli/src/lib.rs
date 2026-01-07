//! # CLI: Command-line interface for ProveIt
//!
//! Provides an interactive REPL, command parser, terminal UI, and command implementations.

pub mod commands;
pub mod parser;
pub mod repl;
pub mod ui;

pub use parser::CommandParser;
pub use repl::Repl;

/// Result type for CLI operations
pub type Result<T> = std::result::Result<T, Error>;

/// CLI errors
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Command error: {0}")]
    CommandError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Proof engine error: {0}")]
    ProofEngineError(#[from] proof_engine::Error),

    #[error("Geometry error: {0}")]
    GeometryError(#[from] geometry::Error),

    #[error("SCTT error: {0}")]
    ScttError(#[from] sctt_core::Error),
}
