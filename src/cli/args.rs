//! Command-line argument parsing using clap
//!
//! Defines the CLI interface with subcommands and flags.

use clap::Parser;

/// A fast CLI tool to remove or replace emoji characters from source code files
#[derive(Parser, Debug)]
#[command(name = "demoji")]
#[command(version, about, long_about = None)]
pub struct Args {
    // Will be implemented in Phase 5
    // Placeholder for now to allow compilation
}

