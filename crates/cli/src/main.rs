//! ProveIt main entry point

use clap::{Parser, Subcommand};
use cli::{Repl, Result};
use tracing_subscriber::EnvFilter;

#[derive(Parser)]
#[command(name = "proveit")]
#[command(about = "Geometric construction environment for accessible formal verification", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Enable debug output
    #[arg(short, long)]
    debug: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Start interactive REPL
    Repl {
        /// Load a proof file
        #[arg(short, long)]
        file: Option<String>,
    },

    /// Verify a proof file
    Verify {
        /// Path to proof file
        file: String,
    },

    /// Check a single expression
    Check {
        /// Expression to check
        expr: String,
    },

    /// Start TUI mode
    Tui,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Setup logging
    let filter = if cli.debug {
        EnvFilter::new("debug")
    } else if cli.verbose {
        EnvFilter::new("info")
    } else {
        EnvFilter::new("warn")
    };

    tracing_subscriber::fmt().with_env_filter(filter).init();

    match cli.command {
        Some(Commands::Repl { file }) => {
            let mut repl = Repl::new();
            if let Some(path) = file {
                repl.load_file(&path)?;
            }
            repl.run().await?;
        }
        Some(Commands::Verify { file }) => {
            println!("Verifying proof file: {}", file);
            // Verification logic would go here
        }
        Some(Commands::Check { expr }) => {
            println!("Checking expression: {}", expr);
            // Type checking logic would go here
        }
        Some(Commands::Tui) => {
            println!("Starting TUI mode...");
            // TUI would launch here
        }
        None => {
            // Default: start REPL
            let mut repl = Repl::new();
            repl.run().await?;
        }
    }

    Ok(())
}
