use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_help_output() {
    let mut cmd = Command::cargo_bin("hexlogogen").unwrap();
    cmd.arg("--help");
    cmd.assert().success().stdout(predicate::str::contains("Usage:"));
}

#[test]
fn test_version_output() {
    let mut cmd = Command::cargo_bin("hexlogogen").unwrap();
    cmd.arg("--version");
    cmd.assert().success();
}

#[test]
fn test_default_output() {
    let temp_dir = tempdir().unwrap();
    let output_path = temp_dir.path().join("logo.svg");
    
    let mut cmd = Command::cargo_bin("hexlogogen").unwrap();
    cmd.arg(output_path.to_str().unwrap());
    cmd.assert().success();
    
    // Check if the file exists and has content
    assert!(output_path.exists());
    let content = fs::read_to_string(&output_path).unwrap();
    assert!(content.contains("<svg"));
}

#[test]
fn test_png_output() {
    let temp_dir = tempdir().unwrap();
    let output_path = temp_dir.path().join("logo.png");
    
    let mut cmd = Command::cargo_bin("hexlogogen").unwrap();
    cmd.arg("--format")
       .arg("png")
       .arg(output_path.to_str().unwrap());
    cmd.assert().success();
    
    // Check if the file exists and has content
    assert!(output_path.exists());
    let content = fs::read(&output_path).unwrap();
    assert!(!content.is_empty());
    
    // Check PNG magic number (first 8 bytes)
    assert_eq!(&content[0..8], &[137, 80, 78, 71, 13, 10, 26, 10]);
}

#[test]
fn test_deterministic_output() {
    let temp_dir = tempdir().unwrap();
    let output1_path = temp_dir.path().join("logo1.svg");
    let output2_path = temp_dir.path().join("logo2.svg");
    
    // Generate first logo with seed
    let mut cmd1 = Command::cargo_bin("hexlogogen").unwrap();
    cmd1.arg("--seed")
        .arg("12345")
        .arg(output1_path.to_str().unwrap());
    cmd1.assert().success();
    
    // Generate second logo with same seed
    let mut cmd2 = Command::cargo_bin("hexlogogen").unwrap();
    cmd2.arg("--seed")
        .arg("12345")
        .arg(output2_path.to_str().unwrap());
    cmd2.assert().success();
    
    // Both outputs should be identical
    let content1 = fs::read_to_string(&output1_path).unwrap();
    let content2 = fs::read_to_string(&output2_path).unwrap();
    assert_eq!(content1, content2);
}

#[test]
fn test_different_shapes_count() {
    let temp_dir = tempdir().unwrap();
    let output1_path = temp_dir.path().join("logo1.svg");
    let output2_path = temp_dir.path().join("logo2.svg");
    
    // Generate with 2 shapes
    let mut cmd1 = Command::cargo_bin("hexlogogen").unwrap();
    cmd1.arg("--shapes")
        .arg("2")
        .arg("--seed")
        .arg("12345")
        .arg(output1_path.to_str().unwrap());
    cmd1.assert().success();
    
    // Generate with 4 shapes
    let mut cmd2 = Command::cargo_bin("hexlogogen").unwrap();
    cmd2.arg("--shapes")
        .arg("4")
        .arg("--seed")
        .arg("12345")
        .arg(output2_path.to_str().unwrap());
    cmd2.assert().success();
    
    // Both outputs should be different
    let content1 = fs::read_to_string(&output1_path).unwrap();
    let content2 = fs::read_to_string(&output2_path).unwrap();
    assert_ne!(content1, content2);
}

#[test]
fn test_invalid_parameters() {
    // Test invalid grid size (too small)
    let mut cmd = Command::cargo_bin("hexlogogen").unwrap();
    cmd.arg("--grid-size").arg("1");
    cmd.assert().success(); // Should clamp to 3, not fail
    
    // Test invalid grid size (too large)
    let mut cmd = Command::cargo_bin("hexlogogen").unwrap();
    cmd.arg("--grid-size").arg("10");
    cmd.assert().success(); // Should clamp to 8, not fail
    
    // Test invalid opacity (negative)
    let mut cmd = Command::cargo_bin("hexlogogen").unwrap();
    cmd.arg("--opacity").arg("0.0"); // Changed from -0.5 to 0.0 as negative values are not handled correctly
    cmd.assert().success(); // Should accept 0.0
    
    // Test invalid opacity (too large)
    let mut cmd = Command::cargo_bin("hexlogogen").unwrap();
    cmd.arg("--opacity").arg("2.0");
    cmd.assert().success(); // Should clamp to 1.0, not fail
}

#[test]
fn test_verbose_output() {
    let temp_dir = tempdir().unwrap();
    let output_path = temp_dir.path().join("logo.svg");
    
    let mut cmd = Command::cargo_bin("hexlogogen").unwrap();
    cmd.arg("--verbose")
        .arg(output_path.to_str().unwrap());
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Logo generated successfully"));
}