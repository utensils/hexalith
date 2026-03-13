use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::tempdir;

fn clipboard_available() -> bool {
    std::env::var_os("DISPLAY").is_some() || std::env::var_os("WAYLAND_DISPLAY").is_some()
}

#[test]
fn test_copy_flag_prevents_file_creation() {
    // Test that --copy flag prevents file creation
    let temp_dir = tempdir().unwrap();
    let output_path = temp_dir.path().join("should_not_exist.png");

    let mut cmd = Command::cargo_bin("hexalith").unwrap();
    cmd.arg("--copy").arg(output_path.to_str().unwrap());

    let output = cmd.output().unwrap();
    if clipboard_available() {
        assert!(output.status.success());
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stderr.contains("clipboard") || stderr.contains("X11") || stderr.contains("Wayland")
        );
    }

    // But no file should be created
    assert!(
        !output_path.exists(),
        "File should not be created when using --copy flag"
    );
}

#[test]
#[cfg_attr(not(feature = "clipboard-tests"), ignore)]
fn test_copy_png_to_clipboard() {
    let mut cmd = Command::cargo_bin("hexalith").unwrap();
    cmd.arg("--copy")
        .arg("--format")
        .arg("png")
        .arg("--verbose");

    // PNG clipboard operations may fail in test environments
    // We'll just check that the command runs (success or clipboard error)
    let output = cmd.output().unwrap();

    // Either it succeeds or fails with a clipboard error
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Logo copied to clipboard as PNG image"));
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(stderr.contains("clipboard") || stderr.contains("pasteboard"));
    }
}

#[test]
fn test_copy_svg_to_clipboard() {
    let mut cmd = Command::cargo_bin("hexalith").unwrap();
    cmd.arg("--copy")
        .arg("--format")
        .arg("svg")
        .arg("--verbose");

    let output = cmd.output().unwrap();
    if clipboard_available() {
        assert!(output.status.success());
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Logo copied to clipboard as SVG text"));
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stderr.contains("clipboard") || stderr.contains("X11") || stderr.contains("Wayland")
        );
    }
}

#[test]
#[cfg_attr(not(feature = "clipboard-tests"), ignore)]
fn test_copy_with_seed() {
    let mut cmd = Command::cargo_bin("hexalith").unwrap();
    cmd.arg("--copy")
        .arg("--seed")
        .arg("12345")
        .arg("--verbose");

    // May fail in test environments
    let output = cmd.output().unwrap();

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Logo copied to clipboard"));
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(stderr.contains("clipboard") || stderr.contains("pasteboard"));
    }
}

#[test]
#[cfg_attr(not(feature = "clipboard-tests"), ignore)]
fn test_copy_respects_format_default() {
    // Default format is PNG
    let mut cmd = Command::cargo_bin("hexalith").unwrap();
    cmd.arg("--copy").arg("--verbose");

    // May fail in test environments
    let output = cmd.output().unwrap();

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Logo copied to clipboard as PNG image"));
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(stderr.contains("clipboard") || stderr.contains("pasteboard"));
    }
}

#[test]
fn test_help_shows_copy_option() {
    let mut cmd = Command::cargo_bin("hexalith").unwrap();
    cmd.arg("--help");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("--copy"))
        .stdout(predicate::str::contains("Copy to clipboard"));
}

#[test]
fn test_normal_file_creation_still_works() {
    // Ensure that normal file creation still works when --copy is not used
    let temp_dir = tempdir().unwrap();
    let output_path = temp_dir.path().join("test_logo.png");

    let mut cmd = Command::cargo_bin("hexalith").unwrap();
    cmd.arg(output_path.to_str().unwrap());

    cmd.assert().success();

    // File should be created
    assert!(
        output_path.exists(),
        "File should be created when --copy flag is not used"
    );

    // And should be a valid PNG
    let content = std::fs::read(&output_path).unwrap();
    assert!(!content.is_empty());
    assert_eq!(&content[0..8], &[137, 80, 78, 71, 13, 10, 26, 10]);
}

#[test]
fn test_copy_with_all_options() {
    // Test that --copy works with various other options
    let mut cmd = Command::cargo_bin("hexalith").unwrap();
    cmd.arg("--copy")
        .arg("--format")
        .arg("svg")
        .arg("--theme")
        .arg("rainbow")
        .arg("--grid-size")
        .arg("6")
        .arg("--shapes")
        .arg("5")
        .arg("--opacity")
        .arg("0.7")
        .arg("--overlap")
        .arg("--unique-shapes")
        .arg("--verbose");

    let output = cmd.output().unwrap();
    if clipboard_available() {
        assert!(output.status.success());
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Logo copied to clipboard as SVG text"));
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stderr.contains("clipboard") || stderr.contains("X11") || stderr.contains("Wayland")
        );
    }
}
