//! Integration tests for the guntamatic CLI

use std::process::Command;

/// Get the path to the compiled binary
fn cli_binary() -> String {
    env!("CARGO_BIN_EXE_guntamatic").to_string()
}

#[test]
fn test_cli_help() {
    let output = Command::new(cli_binary())
        .arg("--help")
        .output()
        .expect("Failed to execute binary");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("guntamatic"));
    assert!(stdout.contains("CLI tool to connect to and extract data from Guntamatic Devices"));
}

#[test]
fn test_cli_version() {
    let output = Command::new(cli_binary())
        .arg("--version")
        .output()
        .expect("Failed to execute binary");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("guntamatic"));
    assert!(stdout.contains("0.3.0"));
}

#[test]
fn test_web_command_help() {
    let output = Command::new(cli_binary())
        .args(["web", "--help"])
        .output()
        .expect("Failed to execute binary");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("web"));
}

#[test]
fn test_modbus_command_help() {
    let output = Command::new(cli_binary())
        .args(["modbus", "--help"])
        .output()
        .expect("Failed to execute binary");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("modbus"));
}

#[test]
fn test_invalid_subcommand() {
    let output = Command::new(cli_binary())
        .arg("invalid-command")
        .output()
        .expect("Failed to execute binary");

    assert!(!output.status.success());
}
