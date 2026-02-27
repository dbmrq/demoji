//! Command-line argument parsing using clap
//!
//! Defines the CLI interface with subcommands and flags.

use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// A fast CLI tool to remove or replace emoji characters from source code files
#[derive(Parser, Debug)]
#[command(name = "demoji")]
#[command(
    version,
    about = "Remove or replace emoji characters from source code files"
)]
#[command(long_about = None)]
pub struct Args {
    /// Subcommand to execute (defaults to 'run')
    #[command(subcommand)]
    pub command: Option<Command>,

    /// Paths to process (defaults to current directory)
    #[arg(value_name = "PATH")]
    pub paths: Vec<PathBuf>,

    /// Dry run mode - preview changes without writing
    #[arg(long, global = true, conflicts_with = "write")]
    pub dry_run: bool,

    /// Write changes to files (default behavior, use for explicitness in scripts)
    #[arg(long, global = true, conflicts_with = "dry_run")]
    pub write: bool,

    /// Check mode - report emojis without modifying (exit 1 if found, ideal for CI)
    #[arg(long, global = true, conflicts_with = "write")]
    pub check: bool,

    /// Create backup files before modifying
    #[arg(long, global = true)]
    pub backup: bool,

    /// Replacement mode: smart, remove, replace, or placeholder
    #[arg(long, global = true, value_name = "MODE")]
    pub mode: Option<String>,

    /// File extensions to process (e.g., rs,py,js)
    #[arg(long, global = true, value_name = "EXTENSIONS")]
    pub extensions: Option<String>,

    /// Patterns to exclude from processing
    #[arg(long, global = true, value_name = "PATTERNS")]
    pub exclude: Option<String>,

    /// Verbose output
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// Quiet output
    #[arg(short, long, global = true)]
    pub quiet: bool,

    /// Custom placeholder text for replacement
    #[arg(long, global = true, value_name = "TEXT")]
    pub placeholder: Option<String>,

    /// Output format: human (default), gcc (compiler-style for IDEs), json, github (GitHub Actions)
    #[arg(long, global = true, value_name = "FORMAT", default_value = "human")]
    pub format: String,
}

/// Available subcommands
#[derive(Subcommand, Debug)]
pub enum Command {
    /// Process files and remove/replace emojis (default)
    Run {
        /// Paths to process
        #[arg(value_name = "PATH")]
        paths: Vec<PathBuf>,

        /// Dry run mode - preview changes without writing
        #[arg(long, conflicts_with = "write")]
        dry_run: bool,

        /// Write changes to files (default behavior, use for explicitness in scripts)
        #[arg(long, conflicts_with = "dry_run")]
        write: bool,

        /// Check mode - report emojis without modifying (exit 1 if found, ideal for CI)
        #[arg(long, conflicts_with = "write")]
        check: bool,

        /// Create backup files before modifying
        #[arg(long)]
        backup: bool,

        /// Replacement mode: smart, remove, replace, or placeholder
        #[arg(long, value_name = "MODE")]
        mode: Option<String>,

        /// File extensions to process (e.g., rs,py,js)
        #[arg(long, value_name = "EXTENSIONS")]
        extensions: Option<String>,

        /// Patterns to exclude from processing
        #[arg(long, value_name = "PATTERNS")]
        exclude: Option<String>,

        /// Verbose output
        #[arg(short, long)]
        verbose: bool,

        /// Quiet output
        #[arg(short, long)]
        quiet: bool,

        /// Custom placeholder text for replacement
        #[arg(long, value_name = "TEXT")]
        placeholder: Option<String>,

        /// Output format: human (default), gcc (compiler-style for IDEs), json, github
        #[arg(long, value_name = "FORMAT")]
        format: Option<String>,
    },

    /// Watch files for changes and process them automatically
    Watch {
        /// Paths to watch
        #[arg(value_name = "PATH")]
        paths: Vec<PathBuf>,

        /// Dry run mode - preview changes without writing
        #[arg(long, conflicts_with = "write")]
        dry_run: bool,

        /// Write changes to files (default behavior, use for explicitness in scripts)
        #[arg(long, conflicts_with = "dry_run")]
        write: bool,

        /// Check mode - report emojis without modifying (exit 1 if found, ideal for CI)
        #[arg(long, conflicts_with = "write")]
        check: bool,

        /// Create backup files before modifying
        #[arg(long)]
        backup: bool,

        /// Replacement mode: smart, remove, replace, or placeholder
        #[arg(long, value_name = "MODE")]
        mode: Option<String>,

        /// File extensions to process (e.g., rs,py,js)
        #[arg(long, value_name = "EXTENSIONS")]
        extensions: Option<String>,

        /// Patterns to exclude from processing
        #[arg(long, value_name = "PATTERNS")]
        exclude: Option<String>,

        /// Verbose output
        #[arg(short, long)]
        verbose: bool,

        /// Quiet output
        #[arg(short, long)]
        quiet: bool,

        /// Custom placeholder text for replacement
        #[arg(long, value_name = "TEXT")]
        placeholder: Option<String>,

        /// Output format: human (default), gcc (compiler-style for IDEs), json, github
        #[arg(long, value_name = "FORMAT")]
        format: Option<String>,
    },

    /// Initialize a .demoji.toml configuration file
    Init {
        /// Path where to create the config file (defaults to current directory)
        #[arg(value_name = "PATH")]
        path: Option<PathBuf>,

        /// Verbose output
        #[arg(short, long)]
        verbose: bool,

        /// Quiet output
        #[arg(short, long)]
        quiet: bool,
    },
}

#[cfg(test)]
#[allow(
    clippy::unwrap_used,
    clippy::needless_borrows_for_generic_args,
    clippy::str_to_string
)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_default_run_command() {
        let args = Args::try_parse_from(&["demoji", "src/"]).unwrap();
        assert_eq!(args.paths.len(), 1);
        assert_eq!(args.paths[0], PathBuf::from("src/"));
        assert!(args.command.is_none());
    }

    #[test]
    fn test_parse_explicit_run_command() {
        let args = Args::try_parse_from(&["demoji", "run", "src/"]).unwrap();
        assert!(matches!(args.command, Some(Command::Run { .. })));
    }

    #[test]
    fn test_parse_watch_command() {
        let args = Args::try_parse_from(&["demoji", "watch", "src/"]).unwrap();
        assert!(matches!(args.command, Some(Command::Watch { .. })));
    }

    #[test]
    fn test_parse_init_command() {
        let args = Args::try_parse_from(&["demoji", "init"]).unwrap();
        assert!(matches!(args.command, Some(Command::Init { .. })));
    }

    #[test]
    fn test_parse_dry_run_flag() {
        let args = Args::try_parse_from(&["demoji", "--dry-run", "src/"]).unwrap();
        assert!(args.dry_run);
    }

    #[test]
    fn test_parse_backup_flag() {
        let args = Args::try_parse_from(&["demoji", "--backup", "src/"]).unwrap();
        assert!(args.backup);
    }

    #[test]
    fn test_parse_mode_flag() {
        let args = Args::try_parse_from(&["demoji", "--mode", "replace", "src/"]).unwrap();
        assert_eq!(args.mode, Some("replace".to_string()));
    }

    #[test]
    fn test_parse_extensions_flag() {
        let args = Args::try_parse_from(&["demoji", "--extensions", "rs,py,js", "src/"]).unwrap();
        assert_eq!(args.extensions, Some("rs,py,js".to_string()));
    }

    #[test]
    fn test_parse_exclude_flag() {
        let args = Args::try_parse_from(&["demoji", "--exclude", "test,vendor", "src/"]).unwrap();
        assert_eq!(args.exclude, Some("test,vendor".to_string()));
    }

    #[test]
    fn test_parse_verbose_flag() {
        let args = Args::try_parse_from(&["demoji", "-v", "src/"]).unwrap();
        assert!(args.verbose);
    }

    #[test]
    fn test_parse_verbose_long_flag() {
        let args = Args::try_parse_from(&["demoji", "--verbose", "src/"]).unwrap();
        assert!(args.verbose);
    }

    #[test]
    fn test_parse_quiet_flag() {
        let args = Args::try_parse_from(&["demoji", "-q", "src/"]).unwrap();
        assert!(args.quiet);
    }

    #[test]
    fn test_parse_quiet_long_flag() {
        let args = Args::try_parse_from(&["demoji", "--quiet", "src/"]).unwrap();
        assert!(args.quiet);
    }

    #[test]
    fn test_parse_placeholder_flag() {
        let args = Args::try_parse_from(&["demoji", "--placeholder", "[EMOJI]", "src/"]).unwrap();
        assert_eq!(args.placeholder, Some("[EMOJI]".to_string()));
    }

    #[test]
    fn test_parse_multiple_paths() {
        let args = Args::try_parse_from(&["demoji", "src/", "tests/", "lib/"]).unwrap();
        assert_eq!(args.paths.len(), 3);
        assert_eq!(args.paths[0], PathBuf::from("src/"));
        assert_eq!(args.paths[1], PathBuf::from("tests/"));
        assert_eq!(args.paths[2], PathBuf::from("lib/"));
    }

    #[test]
    fn test_parse_combined_flags() {
        let args = Args::try_parse_from(&[
            "demoji",
            "--dry-run",
            "--backup",
            "--mode",
            "placeholder",
            "--extensions",
            "rs,py",
            "--exclude",
            "test",
            "--verbose",
            "--placeholder",
            "[EMOJI]",
            "src/",
        ])
        .unwrap();
        assert!(args.dry_run);
        assert!(args.backup);
        assert_eq!(args.mode, Some("placeholder".to_string()));
        assert_eq!(args.extensions, Some("rs,py".to_string()));
        assert_eq!(args.exclude, Some("test".to_string()));
        assert!(args.verbose);
        assert_eq!(args.placeholder, Some("[EMOJI]".to_string()));
    }

    #[test]
    fn test_parse_run_with_flags() {
        let args =
            Args::try_parse_from(&["demoji", "run", "--dry-run", "--mode", "remove", "src/"])
                .unwrap();
        assert!(matches!(args.command, Some(Command::Run { .. })));
        if let Some(Command::Run {
            dry_run,
            mode,
            paths,
            ..
        }) = args.command
        {
            assert!(dry_run);
            assert_eq!(mode, Some("remove".to_string()));
            assert_eq!(paths.len(), 1);
        }
    }

    #[test]
    fn test_parse_watch_with_flags() {
        let args =
            Args::try_parse_from(&["demoji", "watch", "--verbose", "--extensions", "rs", "src/"])
                .unwrap();
        assert!(matches!(args.command, Some(Command::Watch { .. })));
        if let Some(Command::Watch {
            verbose,
            extensions,
            paths,
            ..
        }) = args.command
        {
            assert!(verbose);
            assert_eq!(extensions, Some("rs".to_string()));
            assert_eq!(paths.len(), 1);
        }
    }

    #[test]
    fn test_parse_init_with_path() {
        let args = Args::try_parse_from(&["demoji", "init", "."]).unwrap();
        assert!(matches!(args.command, Some(Command::Init { .. })));
        if let Some(Command::Init { path, .. }) = args.command {
            assert_eq!(path, Some(PathBuf::from(".")));
        }
    }

    #[test]
    fn test_parse_init_without_path() {
        let args = Args::try_parse_from(&["demoji", "init"]).unwrap();
        assert!(matches!(args.command, Some(Command::Init { .. })));
        if let Some(Command::Init { path, .. }) = args.command {
            assert_eq!(path, None);
        }
    }

    #[test]
    fn test_parse_no_paths_defaults_to_empty() {
        let args = Args::try_parse_from(&["demoji"]).unwrap();
        assert!(args.paths.is_empty());
    }

    #[test]
    fn test_quiet_and_verbose_can_coexist() {
        let args = Args::try_parse_from(&["demoji", "-v", "-q", "src/"]).unwrap();
        assert!(args.verbose);
        assert!(args.quiet);
    }
}
