//! Output formatting and reporting
//!
//! Provides different reporter implementations for console and JSON output.

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
    pub fn new(verbosity: VerbosityLevel) -> Self {
        Self {
            verbosity,
            current_file: None,
            current_file_emojis: 0,
        }
    }

    /// Creates a console reporter with normal verbosity
    pub fn normal() -> Self {
        Self::new(VerbosityLevel::Normal)
    }

    /// Creates a console reporter with quiet mode
    pub fn quiet() -> Self {
        Self::new(VerbosityLevel::Quiet)
    }

    /// Creates a console reporter with verbose mode
    pub fn verbose() -> Self {
        Self::new(VerbosityLevel::Verbose)
    }
}

impl Reporter for ConsoleReporter {
    fn report_file(&mut self, file_path: &str, file_count: usize) {
        self.current_file = Some(file_path.to_string());
        self.current_file_emojis = 0;

        if self.verbosity != VerbosityLevel::Quiet {
            let _ = writeln!(
                io::stdout(),
                "{} {} [{}]",
                "Processing:".cyan(),
                file_path,
                file_count
            );
        }
    }

    fn report_match(&mut self, line: usize, column: usize, emoji: &str, context: &str) {
        self.current_file_emojis += 1;

        if self.verbosity == VerbosityLevel::Verbose {
            let _ = writeln!(
                io::stdout(),
                "  {} Line {}, Column {}: {} {}",
                "Found:".yellow(),
                line,
                column,
                emoji,
                format!("({})", context).dimmed()
            );
        } else if self.verbosity == VerbosityLevel::Normal {
            // In normal mode, show a summary per file after all matches
            // This is handled in report_summary for the file
        }
    }

    fn report_summary(
        &mut self,
        total_files: usize,
        files_with_emojis: usize,
        total_emojis: usize,
    ) {
        let _ = writeln!(io::stdout());
        let _ = writeln!(io::stdout(), "{}", "=== Summary ===".bold());

        if total_emojis == 0 {
            let _ = writeln!(
                io::stdout(),
                "{} {} files processed, {} emojis found",
                "✓".green(),
                total_files,
                total_emojis
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

/// Quiet reporter for minimal output
pub struct QuietReporter;

impl QuietReporter {
    /// Creates a new quiet reporter
    pub fn new() -> Self {
        Self
    }
}

impl Default for QuietReporter {
    fn default() -> Self {
        Self::new()
    }
}

impl Reporter for QuietReporter {
    fn report_file(&mut self, _file_path: &str, _file_count: usize) {
        // No output in quiet mode
    }

    fn report_match(&mut self, _line: usize, _column: usize, _emoji: &str, _context: &str) {
        // No output in quiet mode
    }

    fn report_summary(
        &mut self,
        _total_files: usize,
        _files_with_emojis: usize,
        _total_emojis: usize,
    ) {
        // No output in quiet mode
    }
}

/// Verbose reporter for detailed output
pub struct VerboseReporter {
    current_file: Option<String>,
    current_file_emojis: usize,
}

impl VerboseReporter {
    /// Creates a new verbose reporter
    pub fn new() -> Self {
        Self {
            current_file: None,
            current_file_emojis: 0,
        }
    }
}

impl Default for VerboseReporter {
    fn default() -> Self {
        Self::new()
    }
}

impl Reporter for VerboseReporter {
    fn report_file(&mut self, file_path: &str, file_count: usize) {
        self.current_file = Some(file_path.to_string());
        self.current_file_emojis = 0;

        let _ = writeln!(
            io::stdout(),
            "\n{} {} [File {}/{}]",
            "→".cyan(),
            file_path.bold(),
            file_count,
            file_count
        );
    }

    fn report_match(&mut self, line: usize, column: usize, emoji: &str, context: &str) {
        self.current_file_emojis += 1;

        let _ = writeln!(
            io::stdout(),
            "  {} {} at line {}, column {} {}",
            "●".yellow(),
            emoji,
            line,
            column,
            format!("({})", context).dimmed()
        );
    }

    fn report_summary(
        &mut self,
        total_files: usize,
        files_with_emojis: usize,
        total_emojis: usize,
    ) {
        let _ = writeln!(io::stdout());
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
        let _ = writeln!(
            io::stdout(),
            "  {} Total files processed: {}",
            "•".cyan(),
            total_files.to_string().bold()
        );
        let _ = writeln!(
            io::stdout(),
            "  {} Files with emojis: {}",
            "•".yellow(),
            files_with_emojis.to_string().bold()
        );
        let _ = writeln!(
            io::stdout(),
            "  {} Total emojis found: {}",
            "•".red(),
            total_emojis.to_string().bold()
        );

        if total_emojis == 0 {
            let _ = writeln!(io::stdout());
            let _ = writeln!(io::stdout(), "{}", "✓ No emojis found!".green().bold());
        }
    }
}

/// Helper function to create a reporter based on verbosity level
pub fn create_reporter(verbosity: VerbosityLevel) -> Box<dyn Reporter> {
    match verbosity {
        VerbosityLevel::Quiet => Box::new(QuietReporter::new()),
        VerbosityLevel::Normal => Box::new(ConsoleReporter::normal()),
        VerbosityLevel::Verbose => Box::new(VerboseReporter::new()),
    }
}

#[cfg(test)]
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
    fn test_quiet_reporter_new() {
        let reporter = QuietReporter::new();
        // Should not panic
        let _ = reporter;
    }

    #[test]
    fn test_quiet_reporter_default() {
        let reporter = QuietReporter::default();
        // Should not panic
        let _ = reporter;
    }

    #[test]
    fn test_quiet_reporter_no_output() {
        let mut reporter = QuietReporter::new();
        // These should not produce any output
        reporter.report_file("test.rs", 1);
        reporter.report_match(5, 10, "😀", "emoji");
        reporter.report_summary(1, 1, 1);
    }

    #[test]
    fn test_verbose_reporter_new() {
        let reporter = VerboseReporter::new();
        assert_eq!(reporter.current_file, None);
        assert_eq!(reporter.current_file_emojis, 0);
    }

    #[test]
    fn test_verbose_reporter_default() {
        let reporter = VerboseReporter::default();
        assert_eq!(reporter.current_file, None);
        assert_eq!(reporter.current_file_emojis, 0);
    }

    #[test]
    fn test_verbose_reporter_report_file() {
        let mut reporter = VerboseReporter::new();
        reporter.report_file("test.rs", 1);
        assert_eq!(reporter.current_file, Some("test.rs".to_string()));
        assert_eq!(reporter.current_file_emojis, 0);
    }

    #[test]
    fn test_verbose_reporter_report_match() {
        let mut reporter = VerboseReporter::new();
        reporter.report_match(5, 10, "😀", "emoji");
        assert_eq!(reporter.current_file_emojis, 1);
    }

    #[test]
    fn test_verbose_reporter_multiple_matches() {
        let mut reporter = VerboseReporter::new();
        reporter.report_match(5, 10, "😀", "emoji");
        reporter.report_match(6, 15, "🎉", "party");
        reporter.report_match(7, 20, "🚀", "rocket");
        assert_eq!(reporter.current_file_emojis, 3);
    }

    #[test]
    fn test_create_reporter_quiet() {
        let reporter = create_reporter(VerbosityLevel::Quiet);
        // Should create a QuietReporter (boxed)
        // We can't directly check the type, but we can verify it doesn't panic
        let _ = reporter;
    }

    #[test]
    fn test_create_reporter_normal() {
        let reporter = create_reporter(VerbosityLevel::Normal);
        // Should create a ConsoleReporter (boxed)
        let _ = reporter;
    }

    #[test]
    fn test_create_reporter_verbose() {
        let reporter = create_reporter(VerbosityLevel::Verbose);
        // Should create a VerboseReporter (boxed)
        let _ = reporter;
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
        let mut reporter = VerboseReporter::new();
        reporter.report_summary(5, 0, 0);
    }

    #[test]
    fn test_verbose_summary_with_emojis() {
        let mut reporter = VerboseReporter::new();
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
    fn test_quiet_reporter_workflow() {
        let mut reporter = QuietReporter::new();
        reporter.report_file("file1.rs", 1);
        reporter.report_match(1, 5, "😀", "emoji");
        reporter.report_file("file2.rs", 2);
        reporter.report_match(10, 15, "🎉", "party");
        reporter.report_summary(2, 2, 3);
    }

    #[test]
    fn test_verbose_reporter_workflow() {
        let mut reporter = VerboseReporter::new();
        reporter.report_file("file1.rs", 1);
        reporter.report_match(1, 5, "😀", "emoji");
        reporter.report_file("file2.rs", 2);
        reporter.report_match(10, 15, "🎉", "party");
        reporter.report_match(20, 25, "🚀", "rocket");
        reporter.report_summary(2, 2, 3);
    }
}
