//! Interactive REPL for proof construction

use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result as RustylineResult};
use std::sync::Arc;

use accessibility::{AccessibilityPreferences, AudioEngine, SpeechSynthesizer};
use geometry::Construction;
use proof_engine::{Goal, ProofState, TacticLibrary, Verifier};
use sctt_core::{Context, Expr};

use crate::{CommandParser, Result};

/// Interactive REPL state
pub struct Repl {
    /// Current proof state
    proof_state: ProofState,
    /// Type checking context
    context: Context,
    /// Current construction
    construction: Option<Construction>,
    /// Command parser
    parser: CommandParser,
    /// Verifier
    verifier: Verifier,
    /// Tactic library
    tactics: TacticLibrary,
    /// Accessibility preferences
    accessibility: AccessibilityPreferences,
    /// Audio engine (optional)
    audio_engine: Option<Arc<AudioEngine>>,
    /// Line editor
    editor: DefaultEditor,
}

impl Repl {
    pub fn new() -> Self {
        // Try to initialize audio
        let audio_engine = AudioEngine::new().ok().map(Arc::new);

        Self {
            proof_state: ProofState::new(),
            context: Context::new(),
            construction: None,
            parser: CommandParser::new(),
            verifier: Verifier::new(),
            tactics: TacticLibrary::new(),
            accessibility: AccessibilityPreferences::default(),
            audio_engine,
            editor: DefaultEditor::new().unwrap(),
        }
    }

    /// Load a proof file
    pub fn load_file(&mut self, _path: &str) -> Result<()> {
        // File loading logic
        Ok(())
    }

    /// Main REPL loop
    pub async fn run(&mut self) -> Result<()> {
        self.print_welcome();

        loop {
            let readline = self.editor.readline("proveit> ");

            match readline {
                Ok(line) => {
                    if line.trim().is_empty() {
                        continue;
                    }

                    self.editor.add_history_entry(&line).ok();

                    match self.process_line(&line).await {
                        Ok(should_continue) => {
                            if !should_continue {
                                break;
                            }
                        }
                        Err(e) => {
                            eprintln!("Error: {}", e);
                        }
                    }
                }
                Err(ReadlineError::Interrupted) => {
                    println!("^C");
                    break;
                }
                Err(ReadlineError::Eof) => {
                    println!("^D");
                    break;
                }
                Err(err) => {
                    eprintln!("Error: {:?}", err);
                    break;
                }
            }
        }

        println!("Goodbye!");
        Ok(())
    }

    fn print_welcome(&self) {
        println!("ProveIt v{}", env!("CARGO_PKG_VERSION"));
        println!("Geometric construction environment for accessible formal verification");
        println!("Type 'help' for available commands, 'exit' to quit");
        println!();
    }

    async fn process_line(&mut self, line: &str) -> Result<bool> {
        // Parse command
        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        if parts.is_empty() {
            return Ok(true);
        }

        match parts[0] {
            "exit" | "quit" => Ok(false),

            "help" => {
                self.print_help();
                Ok(true)
            }

            "goal" => {
                if let Some(goal) = self.proof_state.current_goal() {
                    println!("Current goal:");
                    println!("  {:?}", goal.proposition);
                } else {
                    println!("No active goal");
                }
                Ok(true)
            }

            "goals" => {
                println!("Proof state: {} goal(s)", self.proof_state.num_goals());
                for (i, goal) in self.proof_state.goals().enumerate() {
                    println!("  {}: {:?}", i + 1, goal.proposition);
                }
                Ok(true)
            }

            "intro" => {
                println!("Applying intro tactic...");
                // Tactic application logic
                Ok(true)
            }

            "construct" => {
                if parts.len() < 2 {
                    println!("Usage: construct <name>");
                    return Ok(true);
                }
                let name = parts[1..].join(" ");
                self.construction = Some(Construction::new(name));
                println!("Created new construction");
                Ok(true)
            }

            "verify" => {
                if let Some(construction) = &self.construction {
                    println!("Verifying construction...");
                    match self.verifier.verify_construction(construction) {
                        Ok(result) => {
                            if result.valid {
                                println!("✓ Construction is valid");
                            } else {
                                println!("✗ Construction has errors:");
                                for msg in result.messages {
                                    println!("  {}", msg.message);
                                }
                            }
                        }
                        Err(e) => println!("Verification failed: {}", e),
                    }
                } else {
                    println!("No construction to verify");
                }
                Ok(true)
            }

            "undo" => {
                match self.proof_state.undo() {
                    Ok(()) => println!("Undone"),
                    Err(e) => println!("Cannot undo: {}", e),
                }
                Ok(true)
            }

            "redo" => {
                match self.proof_state.redo() {
                    Ok(()) => println!("Redone"),
                    Err(e) => println!("Cannot redo: {}", e),
                }
                Ok(true)
            }

            "tactics" => {
                println!("Available tactics:");
                for name in self.tactics.list() {
                    if let Some(tactic) = self.tactics.get(name) {
                        println!("  {} - {}", name, tactic.description());
                    }
                }
                Ok(true)
            }

            "audio" => {
                if parts.len() >= 2 {
                    match parts[1] {
                        "on" => {
                            self.accessibility.enable_tts = true;
                            println!("Audio enabled");
                        }
                        "off" => {
                            self.accessibility.enable_tts = false;
                            println!("Audio disabled");
                        }
                        _ => println!("Usage: audio [on|off]"),
                    }
                } else {
                    println!(
                        "Audio: {}",
                        if self.accessibility.enable_tts {
                            "on"
                        } else {
                            "off"
                        }
                    );
                }
                Ok(true)
            }

            _ => {
                println!(
                    "Unknown command: {}. Type 'help' for available commands.",
                    parts[0]
                );
                Ok(true)
            }
        }
    }

    fn print_help(&self) {
        println!("Available commands:");
        println!("  help                - Show this help message");
        println!("  exit, quit          - Exit the REPL");
        println!();
        println!("Proof commands:");
        println!("  goal                - Show current goal");
        println!("  goals               - Show all goals");
        println!("  intro               - Apply introduction tactic");
        println!("  tactics             - List available tactics");
        println!("  undo                - Undo last action");
        println!("  redo                - Redo last undone action");
        println!();
        println!("Construction commands:");
        println!("  construct <name>    - Create new geometric construction");
        println!("  verify              - Verify current construction");
        println!();
        println!("Accessibility:");
        println!("  audio [on|off]      - Toggle audio feedback");
        println!();
    }
}

impl Default for Repl {
    fn default() -> Self {
        Self::new()
    }
}
