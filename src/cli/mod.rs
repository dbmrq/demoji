//! CLI interface module
//!
//! Handles command-line argument parsing and output formatting.

pub mod args;
pub mod init;
pub mod output;

pub use args::Args;
pub use init::run_init;
pub use output::Reporter;
