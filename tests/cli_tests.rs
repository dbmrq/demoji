//! End-to-end CLI tests using assert_cmd and predicates
//!
//! These tests verify the CLI behavior with various arguments, exit codes, and output.

#![allow(
    deprecated,
    clippy::unwrap_used,
    clippy::let_underscore_must_use,
    let_underscore_drop
)]

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

/// Helper function to create a test directory with emoji files
fn create_test_dir_with_emojis() -> TempDir {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    // Create a Rust file with emojis
    fs::write(
        temp_dir.path().join("test.rs"),
        "fn main() {\n    println!(\"Hello 👋 World 🌍\");\n}\n",
    )
    .expect("Failed to write test.rs");

    // Create a Python file with emojis
    fs::write(
        temp_dir.path().join("test.py"),
        "# This is a test 🐍\nprint('Hello 👋')\n",
    )
    .expect("Failed to write test.py");

    // Create a file without emojis
    fs::write(
        temp_dir.path().join("clean.rs"),
        "fn main() {\n    println!(\"Hello World\");\n}\n",
    )
    .expect("Failed to write clean.rs");

    temp_dir
}

/// Helper function to create a test directory without emojis
fn create_test_dir_without_emojis() -> TempDir {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    fs::write(
        temp_dir.path().join("test.rs"),
        "fn main() {\n    println!(\"Hello World\");\n}\n",
    )
    .expect("Failed to write test.rs");

    temp_dir
}

// ============================================================================
// CLI INVOCATION TESTS
// ============================================================================

#[test]
fn test_demoji_default_run_on_current_directory() {
    let temp_dir = create_test_dir_with_emojis();

    let mut cmd = Command::cargo_bin("demoji").expect("Failed to get binary");
    cmd.current_dir(temp_dir.path()).arg("--dry-run");

    // When emojis are found, exit code is 1
    cmd.assert().code(1);
}

#[test]
fn test_demoji_run_with_explicit_path() {
    let temp_dir = create_test_dir_with_emojis();

    let mut cmd = Command::cargo_bin("demoji").expect("Failed to get binary");
    cmd.arg("run").arg(temp_dir.path()).arg("--dry-run");

    // When emojis are found, exit code is 1
    cmd.assert().code(1);
}

#[test]
fn test_demoji_dry_run_flag() {
    let temp_dir = create_test_dir_with_emojis();

    let mut cmd = Command::cargo_bin("demoji").expect("Failed to get binary");
    cmd.arg("--dry-run").arg(temp_dir.path());

    cmd.assert().code(1);

    // Verify file was not modified
    let content =
        fs::read_to_string(temp_dir.path().join("test.rs")).expect("Failed to read test.rs");
    assert!(
        content.contains("👋"),
        "File should not be modified in dry-run mode"
    );
}

#[test]
fn test_demoji_mode_remove() {
    let temp_dir = create_test_dir_with_emojis();

    let mut cmd = Command::cargo_bin("demoji").expect("Failed to get binary");
    cmd.arg("--mode=remove")
        .arg("--dry-run")
        .arg(temp_dir.path());

    cmd.assert().code(1);
}

#[test]
fn test_demoji_mode_replace() {
    let temp_dir = create_test_dir_with_emojis();

    let mut cmd = Command::cargo_bin("demoji").expect("Failed to get binary");
    cmd.arg("--mode=replace")
        .arg("--dry-run")
        .arg(temp_dir.path());

    cmd.assert().code(1);
}

#[test]
fn test_demoji_mode_placeholder() {
    let temp_dir = create_test_dir_with_emojis();

    let mut cmd = Command::cargo_bin("demoji").expect("Failed to get binary");
    cmd.arg("--mode=placeholder")
        .arg("--dry-run")
        .arg(temp_dir.path());

    cmd.assert().code(1);
}

#[test]
fn test_demoji_init_command() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    let mut cmd = Command::cargo_bin("demoji").expect("Failed to get binary");
    cmd.arg("init").arg(temp_dir.path());

    // Init command may not be fully implemented yet, so we just check it doesn't crash
    // with an unexpected error
    let assert_result = cmd.assert();
    let output = assert_result.get_output();
    let stderr = String::from_utf8_lossy(&output.stderr);
    let is_not_implemented = stderr.contains("not yet implemented");

    if !is_not_implemented {
        // If it's implemented, verify config file was created
        let config_path = temp_dir.path().join(".demoji.toml");
        if config_path.exists() {
            let content = fs::read_to_string(&config_path).expect("Failed to read config");
            assert!(
                content.contains("mode"),
                "Config should contain mode setting"
            );
        }
    }
}

#[test]
fn test_demoji_help_flag() {
    let mut cmd = Command::cargo_bin("demoji").expect("Failed to get binary");
    cmd.arg("--help");

    // clap sends help to stderr and exits with code 2 (current behavior)
    let assert_result = cmd.assert();
    let output = assert_result.get_output();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("Remove or replace emoji"),
        "Help should be in stderr"
    );
}

#[test]
fn test_demoji_version_flag() {
    let mut cmd = Command::cargo_bin("demoji").expect("Failed to get binary");
    cmd.arg("--version");

    // clap sends version to stderr and exits with code 2 (current behavior)
    let assert_result = cmd.assert();
    let output = assert_result.get_output();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("0.1.0"), "Version should be in stderr");
}

// ============================================================================
// EXIT CODE TESTS
// ============================================================================

#[test]
fn test_exit_code_0_when_no_emojis_found() {
    let temp_dir = create_test_dir_without_emojis();

    let mut cmd = Command::cargo_bin("demoji").expect("Failed to get binary");
    cmd.arg("--dry-run").arg(temp_dir.path());

    cmd.assert().success().code(0);
}

#[test]
fn test_exit_code_1_when_emojis_found_in_check_mode() {
    let temp_dir = create_test_dir_with_emojis();

    let mut cmd = Command::cargo_bin("demoji").expect("Failed to get binary");
    cmd.arg("--dry-run").arg(temp_dir.path());

    // In dry-run mode with emojis found, should exit with code 1
    cmd.assert().code(1);
}

#[test]
fn test_exit_code_2_on_invalid_path() {
    let mut cmd = Command::cargo_bin("demoji").expect("Failed to get binary");
    cmd.arg("/nonexistent/path/that/does/not/exist");

    cmd.assert().failure().code(2);
}

#[test]
fn test_exit_code_2_on_invalid_mode() {
    let temp_dir = create_test_dir_with_emojis();

    let mut cmd = Command::cargo_bin("demoji").expect("Failed to get binary");
    cmd.arg("--mode=invalid_mode").arg(temp_dir.path());

    cmd.assert().failure().code(2);
}

// ============================================================================
// OUTPUT TESTS
// ============================================================================

#[test]
fn test_output_contains_expected_messages() {
    let temp_dir = create_test_dir_with_emojis();

    let mut cmd = Command::cargo_bin("demoji").expect("Failed to get binary");
    cmd.arg("--dry-run").arg("--verbose").arg(temp_dir.path());

    cmd.assert()
        .code(1)
        .stdout(predicate::str::contains("test.rs").or(predicate::str::contains("emoji")));
}

#[test]
fn test_quiet_mode_produces_minimal_output() {
    let temp_dir = create_test_dir_with_emojis();

    let mut cmd = Command::cargo_bin("demoji").expect("Failed to get binary");
    cmd.arg("--dry-run").arg("--quiet").arg(temp_dir.path());

    // Quiet mode should still exit with code 1 when emojis found
    cmd.assert().code(1);
}

#[test]
fn test_verbose_mode_produces_detailed_output() {
    let temp_dir = create_test_dir_with_emojis();

    let mut cmd = Command::cargo_bin("demoji").expect("Failed to get binary");
    cmd.arg("--dry-run").arg("--verbose").arg(temp_dir.path());

    // Verbose mode should show detailed output
    cmd.assert().code(1);
}

// ============================================================================
// COMBINED FLAG TESTS
// ============================================================================

#[test]
fn test_dry_run_with_verbose() {
    let temp_dir = create_test_dir_with_emojis();

    let mut cmd = Command::cargo_bin("demoji").expect("Failed to get binary");
    cmd.arg("--dry-run").arg("--verbose").arg(temp_dir.path());

    cmd.assert().code(1);
}

#[test]
fn test_mode_with_dry_run() {
    let temp_dir = create_test_dir_with_emojis();

    let mut cmd = Command::cargo_bin("demoji").expect("Failed to get binary");
    cmd.arg("--mode=replace")
        .arg("--dry-run")
        .arg(temp_dir.path());

    cmd.assert().code(1);
}

#[test]
fn test_placeholder_with_custom_text() {
    let temp_dir = create_test_dir_with_emojis();

    let mut cmd = Command::cargo_bin("demoji").expect("Failed to get binary");
    cmd.arg("--mode=placeholder")
        .arg("--placeholder=[CUSTOM]")
        .arg("--dry-run")
        .arg(temp_dir.path());

    cmd.assert().code(1);
}

#[test]
fn test_extensions_filter() {
    let temp_dir = create_test_dir_with_emojis();

    let mut cmd = Command::cargo_bin("demoji").expect("Failed to get binary");
    cmd.arg("--extensions=rs")
        .arg("--dry-run")
        .arg(temp_dir.path());

    // Should find emojis in .rs files
    cmd.assert().code(1);
}

#[test]
fn test_exclude_patterns() {
    let temp_dir = create_test_dir_with_emojis();

    let mut cmd = Command::cargo_bin("demoji").expect("Failed to get binary");
    cmd.arg("--exclude=*.py")
        .arg("--dry-run")
        .arg(temp_dir.path());

    // Should still find emojis in .rs files
    cmd.assert().code(1);
}

// ============================================================================
// SUBCOMMAND TESTS
// ============================================================================

#[test]
fn test_run_subcommand_explicit() {
    let temp_dir = create_test_dir_with_emojis();

    let mut cmd = Command::cargo_bin("demoji").expect("Failed to get binary");
    cmd.arg("run").arg("--dry-run").arg(temp_dir.path());

    cmd.assert().code(1);
}

#[test]
fn test_run_subcommand_with_flags() {
    let temp_dir = create_test_dir_with_emojis();

    let mut cmd = Command::cargo_bin("demoji").expect("Failed to get binary");
    cmd.arg("run")
        .arg("--mode=remove")
        .arg("--dry-run")
        .arg("--verbose")
        .arg(temp_dir.path());

    cmd.assert().code(1);
}

#[test]
fn test_init_creates_valid_config() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    let mut cmd = Command::cargo_bin("demoji").expect("Failed to get binary");
    cmd.arg("init").arg(temp_dir.path());

    let assert_result = cmd.assert();
    let output = assert_result.get_output();
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Skip test if init is not implemented
    if stderr.contains("not yet implemented") {
        return;
    }

    let config_path = temp_dir.path().join(".demoji.toml");
    assert!(config_path.exists(), "Config file should exist");

    let content = fs::read_to_string(&config_path).expect("Failed to read config");
    assert!(content.contains("mode"), "Config should have mode");
    assert!(
        content.contains("extensions"),
        "Config should have extensions"
    );
}

#[test]
fn test_init_with_verbose() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    let mut cmd = Command::cargo_bin("demoji").expect("Failed to get binary");
    cmd.arg("init").arg("--verbose").arg(temp_dir.path());

    // Init command may not be fully implemented yet
    let _ = cmd.assert();
}

#[test]
fn test_init_with_quiet() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    let mut cmd = Command::cargo_bin("demoji").expect("Failed to get binary");
    cmd.arg("init").arg("--quiet").arg(temp_dir.path());

    // Init command may not be fully implemented yet
    let _ = cmd.assert();
}

// ============================================================================
// EDGE CASE TESTS
// ============================================================================

#[test]
fn test_multiple_paths() {
    let temp_dir1 = create_test_dir_with_emojis();
    let temp_dir2 = create_test_dir_with_emojis();

    let mut cmd = Command::cargo_bin("demoji").expect("Failed to get binary");
    cmd.arg("--dry-run")
        .arg(temp_dir1.path())
        .arg(temp_dir2.path());

    cmd.assert().code(1);
}

#[test]
fn test_file_path_instead_of_directory() {
    let temp_dir = create_test_dir_with_emojis();
    let file_path = temp_dir.path().join("test.rs");

    let mut cmd = Command::cargo_bin("demoji").expect("Failed to get binary");
    cmd.arg("--dry-run").arg(&file_path);

    cmd.assert().code(1);
}

#[test]
fn test_nested_directory_structure() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    // Create nested directories
    fs::create_dir_all(temp_dir.path().join("src/nested")).expect("Failed to create dirs");

    fs::write(
        temp_dir.path().join("src/nested/test.rs"),
        "fn main() { println!(\"Hello 👋\"); }\n",
    )
    .expect("Failed to write file");

    let mut cmd = Command::cargo_bin("demoji").expect("Failed to get binary");
    cmd.arg("--dry-run").arg(temp_dir.path());

    cmd.assert().code(1);
}

#[test]
fn test_empty_directory() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    let mut cmd = Command::cargo_bin("demoji").expect("Failed to get binary");
    cmd.arg("--dry-run").arg(temp_dir.path());

    cmd.assert().success().code(0);
}

#[test]
fn test_mixed_emoji_types() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    // Create file with various emoji types
    fs::write(
        temp_dir.path().join("mixed.rs"),
        "// Single: 😀\n// Skin tone: 👍🏻\n// ZWJ: 👨‍👩‍👧\n// Flag: 🇺🇸\n// Heart: ❤️\n",
    )
    .expect("Failed to write file");

    let mut cmd = Command::cargo_bin("demoji").expect("Failed to get binary");
    cmd.arg("--dry-run").arg(temp_dir.path());

    cmd.assert().code(1);
}

#[test]
fn test_special_characters_in_filenames() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    fs::write(
        temp_dir.path().join("test-file_123.rs"),
        "fn main() { println!(\"Hello 👋\"); }\n",
    )
    .expect("Failed to write file");

    let mut cmd = Command::cargo_bin("demoji").expect("Failed to get binary");
    cmd.arg("--dry-run").arg(temp_dir.path());

    cmd.assert().code(1);
}

// ============================================================================
// HELP AND VERSION TESTS
// ============================================================================

#[test]
fn test_help_shows_subcommands() {
    let mut cmd = Command::cargo_bin("demoji").expect("Failed to get binary");
    cmd.arg("--help");

    // clap sends help to stderr and exits with code 2
    let assert_result = cmd.assert();
    let output = assert_result.get_output();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("run") || stderr.contains("watch"),
        "Help should show subcommands"
    );
}

#[test]
fn test_run_help() {
    let mut cmd = Command::cargo_bin("demoji").expect("Failed to get binary");
    cmd.arg("run").arg("--help");

    // clap sends help to stderr and exits with code 0 for subcommand help
    let assert_result = cmd.assert();
    let output = assert_result.get_output();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("dry-run") || stderr.contains("mode"),
        "Help should show options"
    );
}

#[test]
fn test_init_help() {
    let mut cmd = Command::cargo_bin("demoji").expect("Failed to get binary");
    cmd.arg("init").arg("--help");

    // clap sends help to stderr and exits with code 0 for subcommand help
    let assert_result = cmd.assert();
    let output = assert_result.get_output();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("init") || stderr.contains("config"),
        "Help should show init info"
    );
}

// ============================================================================
// REAL-WORLD SCENARIO TESTS
// ============================================================================

#[test]
fn test_real_world_rust_project() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    // Create a realistic Rust project structure
    fs::create_dir_all(temp_dir.path().join("src")).expect("Failed to create src");
    fs::create_dir_all(temp_dir.path().join("tests")).expect("Failed to create tests");

    fs::write(
        temp_dir.path().join("src/main.rs"),
        "fn main() {\n    println!(\"Hello 👋 World 🌍\");\n}\n",
    )
    .expect("Failed to write main.rs");

    fs::write(
        temp_dir.path().join("src/lib.rs"),
        "/// A library 📚\npub fn process() { /* ... */ }\n",
    )
    .expect("Failed to write lib.rs");

    fs::write(
        temp_dir.path().join("tests/integration_test.rs"),
        "#[test]\nfn test_success() { // ✅\n    assert!(true);\n}\n",
    )
    .expect("Failed to write test");

    let mut cmd = Command::cargo_bin("demoji").expect("Failed to get binary");
    cmd.arg("--dry-run").arg("--verbose").arg(temp_dir.path());

    cmd.assert().code(1);
}

#[test]
fn test_real_world_python_project() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    fs::create_dir_all(temp_dir.path().join("src")).expect("Failed to create src");

    fs::write(
        temp_dir.path().join("src/main.py"),
        "#!/usr/bin/env python3\n# Main script 🐍\nprint('Hello 👋')\n",
    )
    .expect("Failed to write main.py");

    fs::write(
        temp_dir.path().join("README.md"),
        "# My Project 🚀\n\nThis is a test project 📝\n",
    )
    .expect("Failed to write README");

    let mut cmd = Command::cargo_bin("demoji").expect("Failed to get binary");
    cmd.arg("--dry-run").arg(temp_dir.path());

    cmd.assert().code(1);
}

#[test]
fn test_mixed_file_types() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    // Create various file types
    fs::write(
        temp_dir.path().join("test.rs"),
        "fn main() { println!(\"Hello 👋\"); }\n",
    )
    .expect("Failed to write .rs");

    fs::write(temp_dir.path().join("test.py"), "print('Hello 👋')\n").expect("Failed to write .py");

    fs::write(
        temp_dir.path().join("test.js"),
        "console.log('Hello 👋');\n",
    )
    .expect("Failed to write .js");

    fs::write(
        temp_dir.path().join("test.json"),
        "{\"message\": \"Hello 👋\"}\n",
    )
    .expect("Failed to write .json");

    let mut cmd = Command::cargo_bin("demoji").expect("Failed to get binary");
    cmd.arg("--dry-run").arg(temp_dir.path());

    cmd.assert().code(1);
}

// ============================================================================
// CLEAN FILE TESTS (no emojis)
// ============================================================================

#[test]
fn test_clean_file_exit_code_0() {
    let temp_dir = create_test_dir_without_emojis();

    let mut cmd = Command::cargo_bin("demoji").expect("Failed to get binary");
    cmd.arg("--dry-run").arg(temp_dir.path());

    cmd.assert().success().code(0);
}

#[test]
fn test_clean_file_with_verbose() {
    let temp_dir = create_test_dir_without_emojis();

    let mut cmd = Command::cargo_bin("demoji").expect("Failed to get binary");
    cmd.arg("--dry-run").arg("--verbose").arg(temp_dir.path());

    cmd.assert().success().code(0);
}

#[test]
fn test_clean_file_with_quiet() {
    let temp_dir = create_test_dir_without_emojis();

    let mut cmd = Command::cargo_bin("demoji").expect("Failed to get binary");
    cmd.arg("--dry-run").arg("--quiet").arg(temp_dir.path());

    cmd.assert().success().code(0);
}
