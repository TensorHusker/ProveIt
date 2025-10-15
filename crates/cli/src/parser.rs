//! Command language parser

use crate::{Error, Result};
use sctt_core::{Expr, Name};

/// Parser for ProveIt command language
pub struct CommandParser {
    // Parser state would go here
}

impl CommandParser {
    pub fn new() -> Self {
        Self {}
    }

    /// Parse a command string
    pub fn parse_command(&self, input: &str) -> Result<Command> {
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        if parts.is_empty() {
            return Err(Error::ParseError("Empty command".to_string()));
        }

        match parts[0] {
            "intro" => Ok(Command::Intro { name: None }),
            "exact" => {
                if parts.len() < 2 {
                    return Err(Error::ParseError("exact requires an argument".to_string()));
                }
                Ok(Command::Exact {
                    term: parts[1].to_string(),
                })
            }
            "apply" => {
                if parts.len() < 2 {
                    return Err(Error::ParseError("apply requires an argument".to_string()));
                }
                Ok(Command::Apply {
                    func: parts[1].to_string(),
                })
            }
            _ => Err(Error::ParseError(format!("Unknown command: {}", parts[0]))),
        }
    }

    /// Parse an expression
    pub fn parse_expr(&self, input: &str) -> Result<Expr> {
        // Simplified parser - would use pest in production
        match input.trim() {
            "Type" | "Type0" => Ok(Expr::Type(0)),
            "Type1" => Ok(Expr::Type(1)),
            var if !var.is_empty() => Ok(Expr::Var(Name(var.to_string()), 0)),
            _ => Err(Error::ParseError(format!("Cannot parse: {}", input))),
        }
    }
}

impl Default for CommandParser {
    fn default() -> Self {
        Self::new()
    }
}

/// Parsed command
#[derive(Debug, Clone)]
pub enum Command {
    Intro { name: Option<String> },
    Exact { term: String },
    Apply { func: String },
    Assumption,
    Refl,
    Undo,
    Redo,
    Show,
    Help,
    Quit,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_command() {
        let parser = CommandParser::new();
        let cmd = parser.parse_command("intro").unwrap();
        matches!(cmd, Command::Intro { .. });
    }

    #[test]
    fn test_parse_expr() {
        let parser = CommandParser::new();
        let expr = parser.parse_expr("Type").unwrap();
        matches!(expr, Expr::Type(0));
    }
}
