use std::env;
use std::fs;
use std::process::Command;

const AE_BIN: &str = env!("CARGO_BIN_EXE_anchoredit");

fn anchorscope_bin() -> String {
    env::var("ANCHORSCOPE_BIN").expect("ANCHORSCOPE_BIN must be set for integration tests")
}

fn run_ae(args: &[&str]) -> std::process::Output {
    Command::new(AE_BIN)
        .env("ANCHORSCOPE_BIN", anchorscope_bin())
        .args(args)
        .output()
        .expect("failed to execute ae")
}

fn create_temp_file(dir: &std::path::Path, name: &str, content: &str) -> std::path::PathBuf {
    let path = dir.join(name);
    fs::write(&path, content).expect("failed to write temp file");
    path
}

fn create_temp_anchor_file(dir: &std::path::Path, name: &str, content: &str) -> std::path::PathBuf {
    create_temp_file(dir, name, content)
}

#[test]
fn read_success() {
    let dir = tempfile::tempdir().expect("failed to create temp dir");
    let file_path = create_temp_file(dir.path(), "test.txt", "hello world");

    let output = run_ae(&[
        "read",
        "--file",
        file_path.to_str().unwrap(),
        "--anchor",
        "hello world",
    ]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("scope_hash="));
    assert!(stdout.contains("content=hello world"));
}

#[test]
fn read_no_match() {
    let dir = tempfile::tempdir().expect("failed to create temp dir");
    let file_path = create_temp_file(dir.path(), "test.txt", "hello world");

    let output = run_ae(&[
        "read",
        "--file",
        file_path.to_str().unwrap(),
        "--anchor",
        "not found",
    ]);

    assert!(!output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stdout.contains("NO_MATCH") || stderr.contains("NO_MATCH"),
        "expected NO_MATCH in output, got stdout: {stdout} stderr: {stderr}"
    );
}

#[test]
fn read_multiple_matches() {
    let dir = tempfile::tempdir().expect("failed to create temp dir");
    let file_path = create_temp_file(dir.path(), "test.txt", "foo bar foo baz");

    let output = run_ae(&[
        "read",
        "--file",
        file_path.to_str().unwrap(),
        "--anchor",
        "foo",
    ]);

    assert!(!output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stdout.contains("MULTIPLE_MATCHES") || stderr.contains("MULTIPLE_MATCHES"),
        "expected MULTIPLE_MATCHES in output, got stdout: {stdout} stderr: {stderr}"
    );
}

#[test]
fn write_success() {
    let dir = tempfile::tempdir().expect("failed to create temp dir");
    let file_path = create_temp_file(dir.path(), "test.txt", "hello world");

    // First read to get the scope_hash
    let read_output = run_ae(&[
        "read",
        "--file",
        file_path.to_str().unwrap(),
        "--anchor",
        "hello world",
    ]);
    assert!(read_output.status.success());

    let stdout = String::from_utf8_lossy(&read_output.stdout);
    let scope_hash = stdout
        .lines()
        .find(|l| l.starts_with("scope_hash="))
        .expect("expected scope_hash in output")
        .split_once('=')
        .expect("expected scope_hash=value")
        .1
        .trim();

    // Now write with the hash
    let write_output = run_ae(&[
        "write",
        "--file",
        file_path.to_str().unwrap(),
        "--anchor",
        "hello world",
        "--expected-hash",
        scope_hash,
        "--replacement",
        "goodbye world",
    ]);

    assert!(write_output.status.success());
    let file_content = fs::read_to_string(&file_path).expect("failed to read file");
    assert_eq!(file_content, "goodbye world");
}

#[test]
fn write_hash_mismatch() {
    let dir = tempfile::tempdir().expect("failed to create temp dir");
    let file_path = create_temp_file(dir.path(), "test.txt", "hello world");

    let output = run_ae(&[
        "write",
        "--file",
        file_path.to_str().unwrap(),
        "--anchor",
        "hello world",
        "--expected-hash",
        "0000000000000000",
        "--replacement",
        "goodbye world",
    ]);

    assert!(!output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stdout.contains("HASH_MISMATCH") || stderr.contains("HASH_MISMATCH"),
        "expected HASH_MISMATCH in output, got stdout: {stdout} stderr: {stderr}"
    );
}
