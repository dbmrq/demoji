//! Output formatting and reporting
//!
//! Provides different reporter implementations for console and JSON output.

// Allow discarding results of writeln! in reporter implementations
#![allow(clippy::let_underscore_must_use, let_underscore_drop)]

use colored::Colorize;
use std::io::{self, Write};

/// Verbosity level for output
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerbosityLevel {
    /// Minimal output (only errors and summary)
    Quiet,
    /// Normal output (files and summary)
    Normal,
    /// Detailed output (all matches and details)
    Verbose,
}

/// Trait for reporting processing results
pub trait Reporter {
    /// Report the start of processing a file
    fn report_file(&mut self, file_path: &str, file_count: usize);

    /// Report a single emoji match found in a file
    fn report_match(&mut self, line: usize, column: usize, emoji: &str, context: &str);

    /// Report the final summary of processing
    fn report_summary(&mut self, total_files: usize, files_with_emojis: usize, total_emojis: usize);
}

/// Console reporter with colored output
pub struct ConsoleReporter {
    verbosity: VerbosityLevel,
    current_file: Option<String>,
    current_file_emojis: usize,
}

impl ConsoleReporter {
    /// Creates a new console reporter with the specified verbosity level
    #[must_use]
    pub const fn new(verbosity: VerbosityLevel) -> Self {
        Self {
            verbosity,
            current_file: None,
            current_file_emojis: 0,
        }
    }

    /// Creates a console reporter with normal verbosity
    #[must_use]
    pub const fn normal() -> Self {
        Self::new(VerbosityLevel::Normal)
    }

    /// Creates a console reporter with quiet mode
    #[must_use]
    pub const fn quiet() -> Self {
        Self::new(VerbosityLevel::Quiet)
    }

    /// Creates a console reporter with verbose mode
    #[must_use]
    pub const fn verbose() -> Self {
        Self::new(VerbosityLevel::Verbose)
    }
}

impl Reporter for ConsoleReporter {
    fn report_file(&mut self, file_path: &str, file_count: usize) {
        self.current_file = Some(file_path.to_owned());
        self.current_file_emojis = 0;

        match self.verbosity {
            VerbosityLevel::Quiet => {},
            VerbosityLevel::Normal => {
                let _ = writeln!(
                    io::stdout(),
                    "{} {} [{}]",
                    "Processing:".cyan(),
                    file_path,
                    file_count
                );
            },
            VerbosityLevel::Verbose => {
                let _ = writeln!(
                    io::stdout(),
                    "\n{} {} [File {}]",
                    "→".cyan(),
                    file_path.bold(),
                    file_count
                );
            },
        }
    }

    fn report_match(&mut self, line: usize, column: usize, emoji: &str, context: &str) {
        self.current_file_emojis += 1;

        if self.verbosity == VerbosityLevel::Verbose {
            let _ = writeln!(
                io::stdout(),
                "  {} {} at line {}, column {} {}",
                "●".yellow(),
                emoji,
                line,
                column,
                format!("({context})").dimmed()
            );
        }
    }

    fn report_summary(
        &mut self,
        total_files: usize,
        files_with_emojis: usize,
        total_emojis: usize,
    ) {
        if self.verbosity == VerbosityLevel::Quiet {
            return;
        }

        let _ = writeln!(io::stdout());

        if self.verbosity == VerbosityLevel::Verbose {
            let _ = writeln!(
                io::stdout(),
                "{}",
                "╔════════════════════════════════════╗".cyan()
            );
            let _ = writeln!(
                io::stdout(),
                "{}",
                "║         PROCESSING COMPLETE         ║".cyan()
            );
            let _ = writeln!(
                io::stdout(),
                "{}",
                "╚════════════════════════════════════╝".cyan()
            );
            let _ = writeln!(io::stdout());
        } else {
            let _ = writeln!(io::stdout(), "{}", "=== Summary ===".bold());
        }

        if total_emojis == 0 {
            let _ = writeln!(
                io::stdout(),
                "{} {} files processed, no emojis found",
                "✓".green(),
                total_files
            );
        } else {
            let _ = writeln!(
                io::stdout(),
                "{} {} files processed",
                "Files:".cyan(),
                total_files
            );
            let _ = writeln!(
                io::stdout(),
                "{} {} files with emojis",
                "Affected:".yellow(),
                files_with_emojis
            );
            let _ = writeln!(
                io::stdout(),
                "{} {} emojis found",
                "Emojis:".red(),
                total_emojis
            );
        }
    }
}

/// Output format for reporting
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OutputFormat {
    /// Human-readable colored output (default)
    #[default]
    Human,
    /// GCC/Clang-style output for IDE integration (file:line:col: warning: message)
    Gcc,
    /// JSON output for machine processing
    Json,
    /// GitHub Actions annotation format
    Github,
}

impl OutputFormat {
    /// Parse output format from string
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "human" => Some(Self::Human),
            "gcc" | "compiler" => Some(Self::Gcc),
            "json" => Some(Self::Json),
            "github" | "gh" => Some(Self::Github),
            _ => None,
        }
    }
}

/// GCC-style reporter for IDE integration
///
/// Outputs in the format: `file:line:column: warning: message`
/// This format is understood by Xcode, VS Code, and most IDEs.
pub struct GccReporter {
    current_file: Option<String>,
}

impl GccReporter {
    /// Creates a new GCC-style reporter
    #[must_use]
    pub const fn new() -> Self {
        Self { current_file: None }
    }
}

impl Default for GccReporter {
    fn default() -> Self {
        Self::new()
    }
}

impl Reporter for GccReporter {
    fn report_file(&mut self, file_path: &str, _file_count: usize) {
        self.current_file = Some(file_path.to_owned());
    }

    fn report_match(&mut self, line: usize, column: usize, emoji: &str, _context: &str) {
        if let Some(ref file) = self.current_file {
            // Standard GCC warning format: file:line:column: warning: message
            let _ = writeln!(
                io::stdout(),
                "{file}:{line}:{column}: warning: emoji found: {emoji}"
            );
        }
    }

    fn report_summary(
        &mut self,
        _total_files: usize,
        _files_with_emojis: usize,
        _total_emojis: usize,
    ) {
        // No summary in GCC format - IDEs parse individual warnings
    }
}

/// GitHub Actions annotation reporter
///
/// Outputs in the format: `::warning file={name},line={line},col={col}::{message}`
/// This format is understood by GitHub Actions and shown inline in PRs.
pub struct GithubReporter {
    current_file: Option<String>,
}

impl GithubReporter {
    /// Creates a new GitHub Actions reporter
    #[must_use]
    pub const fn new() -> Self {
        Self { current_file: None }
    }
}

impl Default for GithubReporter {
    fn default() -> Self {
        Self::new()
    }
}

impl Reporter for GithubReporter {
    fn report_file(&mut self, file_path: &str, _file_count: usize) {
        self.current_file = Some(file_path.to_owned());
    }

    fn report_match(&mut self, line: usize, column: usize, emoji: &str, _context: &str) {
        if let Some(ref file) = self.current_file {
            // GitHub Actions annotation format
            let _ = writeln!(
                io::stdout(),
                "::warning file={file},line={line},col={column}::Emoji found: {emoji}"
            );
        }
    }

    fn report_summary(
        &mut self,
        total_files: usize,
        files_with_emojis: usize,
        total_emojis: usize,
    ) {
        if total_emojis > 0 {
            let _ = writeln!(
                io::stdout(),
                "::notice::demoji: {total_files} files processed, {files_with_emojis} with emojis, {total_emojis} total emojis found"
            );
        }
    }
}

/// JSON reporter for machine-readable output
pub struct JsonReporter {
    current_file: Option<String>,
    matches: Vec<JsonMatch>,
}

#[derive(Debug)]
struct JsonMatch {
    file: String,
    line: usize,
    column: usize,
    emoji: String,
}

impl JsonReporter {
    /// Creates a new JSON reporter
    #[must_use]
    #[allow(clippy::missing_const_for_fn)] // Vec::new() is not const in stable Rust
    pub fn new() -> Self {
        Self {
            current_file: None,
            matches: Vec::new(),
        }
    }
}

impl Default for JsonReporter {
    fn default() -> Self {
        Self::new()
    }
}

impl Reporter for JsonReporter {
    fn report_file(&mut self, file_path: &str, _file_count: usize) {
        self.current_file = Some(file_path.to_owned());
    }

    fn report_match(&mut self, line: usize, column: usize, emoji: &str, _context: &str) {
        if let Some(ref file) = self.current_file {
            self.matches.push(JsonMatch {
                file: file.clone(),
                line,
                column,
                emoji: emoji.to_owned(),
            });
        }
    }

    fn report_summary(
        &mut self,
        total_files: usize,
        files_with_emojis: usize,
        total_emojis: usize,
    ) {
        // Output JSON at the end
        let _ = write!(io::stdout(), "{{\"summary\":{{\"total_files\":{total_files},\"files_with_emojis\":{files_with_emojis},\"total_emojis\":{total_emojis}}},\"matches\":[");

        for (i, m) in self.matches.iter().enumerate() {
            if i > 0 {
                let _ = write!(io::stdout(), ",");
            }
            // Escape the emoji for JSON
            let escaped_emoji: String = m
                .emoji
                .chars()
                .map(|c| {
                    if c == '"' {
                        "\\\"".to_owned()
                    } else if c == '\\' {
                        "\\\\".to_owned()
                    } else {
                        c.to_string()
                    }
                })
                .collect();
            let escaped_file = m.file.replace('\\', "\\\\").replace('"', "\\\"");
            let line = m.line;
            let column = m.column;
            let _ = write!(
                io::stdout(),
                "{{\"file\":\"{escaped_file}\",\"line\":{line},\"column\":{column},\"emoji\":\"{escaped_emoji}\"}}"
            );
        }

        let _ = writeln!(io::stdout(), "]}}");
    }
}

/// Helper function to create a reporter based on verbosity level and output format
pub fn create_reporter(verbosity: VerbosityLevel, format: OutputFormat) -> Box<dyn Reporter> {
    match format {
        OutputFormat::Human => Box::new(ConsoleReporter::new(verbosity)),
        OutputFormat::Gcc => Box::new(GccReporter::new()),
        OutputFormat::Github => Box::new(GithubReporter::new()),
        OutputFormat::Json => Box::new(JsonReporter::new()),
    }
}

#[cfg(test)]
#[allow(let_underscore_drop, clippy::str_to_string)]
mod tests {
    use super::*;

    #[test]
    fn test_verbosity_level_equality() {
        assert_eq!(VerbosityLevel::Quiet, VerbosityLevel::Quiet);
        assert_eq!(VerbosityLevel::Normal, VerbosityLevel::Normal);
        assert_eq!(VerbosityLevel::Verbose, VerbosityLevel::Verbose);
        assert_ne!(VerbosityLevel::Quiet, VerbosityLevel::Normal);
    }

    #[test]
    fn test_console_reporter_new() {
        let reporter = ConsoleReporter::new(VerbosityLevel::Normal);
        assert_eq!(reporter.verbosity, VerbosityLevel::Normal);
        assert_eq!(reporter.current_file, None);
        assert_eq!(reporter.current_file_emojis, 0);
    }

    #[test]
    fn test_console_reporter_normal() {
        let reporter = ConsoleReporter::normal();
        assert_eq!(reporter.verbosity, VerbosityLevel::Normal);
    }

    #[test]
    fn test_console_reporter_quiet() {
        let reporter = ConsoleReporter::quiet();
        assert_eq!(reporter.verbosity, VerbosityLevel::Quiet);
    }

    #[test]
    fn test_console_reporter_verbose() {
        let reporter = ConsoleReporter::verbose();
        assert_eq!(reporter.verbosity, VerbosityLevel::Verbose);
    }

    #[test]
    fn test_console_reporter_report_file() {
        let mut reporter = ConsoleReporter::normal();
        reporter.report_file("test.rs", 1);
        assert_eq!(reporter.current_file, Some("test.rs".to_string()));
        assert_eq!(reporter.current_file_emojis, 0);
    }

    #[test]
    fn test_console_reporter_report_match() {
        let mut reporter = ConsoleReporter::verbose();
        reporter.report_match(5, 10, "😀", "emoji");
        assert_eq!(reporter.current_file_emojis, 1);
    }

    #[test]
    fn test_console_reporter_multiple_matches() {
        let mut reporter = ConsoleReporter::verbose();
        reporter.report_match(5, 10, "😀", "emoji");
        reporter.report_match(6, 15, "🎉", "party");
        assert_eq!(reporter.current_file_emojis, 2);
    }

    #[test]
    fn test_quiet_mode_no_output() {
        let mut reporter = ConsoleReporter::quiet();
        // These should not produce any output in quiet mode
        reporter.report_file("test.rs", 1);
        reporter.report_match(5, 10, "😀", "emoji");
        reporter.report_summary(1, 1, 1);
        // Verify internal state is still updated
        assert_eq!(reporter.current_file, Some("test.rs".to_string()));
    }

    #[test]
    fn test_verbose_mode_report_file() {
        let mut reporter = ConsoleReporter::verbose();
        reporter.report_file("test.rs", 1);
        assert_eq!(reporter.current_file, Some("test.rs".to_string()));
        assert_eq!(reporter.current_file_emojis, 0);
    }

    #[test]
    fn test_verbose_mode_report_match() {
        let mut reporter = ConsoleReporter::verbose();
        reporter.report_match(5, 10, "😀", "emoji");
        assert_eq!(reporter.current_file_emojis, 1);
    }

    #[test]
    fn test_verbose_mode_multiple_matches() {
        let mut reporter = ConsoleReporter::verbose();
        reporter.report_match(5, 10, "😀", "emoji");
        reporter.report_match(6, 15, "🎉", "party");
        reporter.report_match(7, 20, "🚀", "rocket");
        assert_eq!(reporter.current_file_emojis, 3);
    }

    #[test]
    fn test_create_reporter_quiet() {
        let reporter = create_reporter(VerbosityLevel::Quiet, OutputFormat::Human);
        let _ = reporter;
    }

    #[test]
    fn test_create_reporter_normal() {
        let reporter = create_reporter(VerbosityLevel::Normal, OutputFormat::Human);
        let _ = reporter;
    }

    #[test]
    fn test_create_reporter_verbose() {
        let reporter = create_reporter(VerbosityLevel::Verbose, OutputFormat::Human);
        let _ = reporter;
    }

    #[test]
    fn test_create_reporter_gcc_format() {
        let reporter = create_reporter(VerbosityLevel::Normal, OutputFormat::Gcc);
        let _ = reporter;
    }

    #[test]
    fn test_create_reporter_github_format() {
        let reporter = create_reporter(VerbosityLevel::Normal, OutputFormat::Github);
        let _ = reporter;
    }

    #[test]
    fn test_create_reporter_json_format() {
        let reporter = create_reporter(VerbosityLevel::Normal, OutputFormat::Json);
        let _ = reporter;
    }

    #[test]
    fn test_output_format_parse() {
        assert_eq!(OutputFormat::parse("human"), Some(OutputFormat::Human));
        assert_eq!(OutputFormat::parse("gcc"), Some(OutputFormat::Gcc));
        assert_eq!(OutputFormat::parse("compiler"), Some(OutputFormat::Gcc));
        assert_eq!(OutputFormat::parse("json"), Some(OutputFormat::Json));
        assert_eq!(OutputFormat::parse("github"), Some(OutputFormat::Github));
        assert_eq!(OutputFormat::parse("gh"), Some(OutputFormat::Github));
        assert_eq!(OutputFormat::parse("invalid"), None);
    }

    #[test]
    fn test_reporter_trait_object() {
        let mut reporter: Box<dyn Reporter> = Box::new(ConsoleReporter::normal());
        reporter.report_file("test.rs", 1);
        reporter.report_match(5, 10, "😀", "emoji");
        reporter.report_summary(1, 1, 1);
    }

    #[test]
    fn test_console_reporter_quiet_mode_no_file_output() {
        let mut reporter = ConsoleReporter::quiet();
        // In quiet mode, report_file should not output anything
        reporter.report_file("test.rs", 1);
        // Verify internal state is still updated
        assert_eq!(reporter.current_file, Some("test.rs".to_string()));
    }

    #[test]
    fn test_console_reporter_normal_mode_file_output() {
        let mut reporter = ConsoleReporter::normal();
        reporter.report_file("test.rs", 1);
        assert_eq!(reporter.current_file, Some("test.rs".to_string()));
    }

    #[test]
    fn test_console_reporter_verbose_mode_match_output() {
        let mut reporter = ConsoleReporter::verbose();
        reporter.report_file("test.rs", 1);
        reporter.report_match(5, 10, "😀", "emoji");
        assert_eq!(reporter.current_file_emojis, 1);
    }

    #[test]
    fn test_summary_with_no_emojis() {
        let mut reporter = ConsoleReporter::normal();
        reporter.report_summary(5, 0, 0);
    }

    #[test]
    fn test_summary_with_emojis() {
        let mut reporter = ConsoleReporter::normal();
        reporter.report_summary(5, 2, 7);
    }

    #[test]
    fn test_verbose_summary_with_no_emojis() {
        let mut reporter = ConsoleReporter::verbose();
        reporter.report_summary(5, 0, 0);
    }

    #[test]
    fn test_verbose_summary_with_emojis() {
        let mut reporter = ConsoleReporter::verbose();
        reporter.report_summary(5, 2, 7);
    }

    #[test]
    fn test_reporter_workflow() {
        let mut reporter = ConsoleReporter::normal();
        reporter.report_file("file1.rs", 1);
        reporter.report_match(1, 5, "😀", "emoji");
        reporter.report_file("file2.rs", 2);
        reporter.report_match(10, 15, "🎉", "party");
        reporter.report_match(20, 25, "🚀", "rocket");
        reporter.report_summary(2, 2, 3);
    }

    #[test]
    fn test_quiet_workflow() {
        let mut reporter = ConsoleReporter::quiet();
        reporter.report_file("file1.rs", 1);
        reporter.report_match(1, 5, "😀", "emoji");
        reporter.report_file("file2.rs", 2);
        reporter.report_match(10, 15, "🎉", "party");
        reporter.report_summary(2, 2, 3);
    }

    #[test]
    fn test_verbose_workflow() {
        let mut reporter = ConsoleReporter::verbose();
        reporter.report_file("file1.rs", 1);
        reporter.report_match(1, 5, "😀", "emoji");
        reporter.report_file("file2.rs", 2);
        reporter.report_match(10, 15, "🎉", "party");
        reporter.report_match(20, 25, "🚀", "rocket");
        reporter.report_summary(2, 2, 3);
    }
}
