use std::env;
use std::fs;
use std::process::Command;

const AE_BIN: &str = env!("CARGO_BIN_EXE_anchoredit");

fn anchorscope_bin() -> String {
    env::var("ANCHORSCOPE_BIN").unwrap_or_else(|_| "anchorscope".to_string())
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

// --- read tests ---

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
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("NO_MATCH"),
        "expected NO_MATCH in stderr, got: {stderr}"
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
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("MULTIPLE_MATCHES"),
        "expected MULTIPLE_MATCHES in stderr, got: {stderr}"
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
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("HASH_MISMATCH"),
        "expected HASH_MISMATCH in stderr, got: {stderr}"
    );
}

// --- search tests ---

#[test]
fn search_initial_returns_segments() {
    let dir = tempfile::tempdir().expect("failed to create temp dir");
    // Create a file large enough to not trigger termination
    let content = "hello world\n".repeat(500);
    let file_path = create_temp_file(dir.path(), "test.txt", &content);

    let output = run_ae(&[
        "search",
        "--file",
        file_path.to_str().unwrap(),
    ]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: serde_json::Value = serde_json::from_str(&stdout).expect("expected valid JSON");

    // Should have segments, not done
    assert!(!json.get("done").unwrap_or(&serde_json::Value::Bool(false)).as_bool().unwrap());
    let segments = json.get("segments").expect("expected segments").as_array().unwrap();
    assert_eq!(segments.len(), 3);

    // Check segment IDs
    assert_eq!(segments[0]["id"], "A");
    assert_eq!(segments[1]["id"], "B");
    assert_eq!(segments[2]["id"], "C");

    // Check size_bytes is present and positive
    let size_bytes = json.get("size_bytes").expect("expected size_bytes").as_u64().unwrap();
    assert!(size_bytes > 0, "size_bytes should be positive, got {size_bytes}");
}

#[test]
fn search_with_range_returns_segments() {
    let dir = tempfile::tempdir().expect("failed to create temp dir");
    let content = "hello world\n".repeat(500);
    let file_path = create_temp_file(dir.path(), "test.txt", &content);

    let output = run_ae(&[
        "search",
        "--file",
        file_path.to_str().unwrap(),
        "--range",
        "0.3:0.7",
    ]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: serde_json::Value = serde_json::from_str(&stdout).expect("expected valid JSON");

    let segments = json.get("segments").expect("expected segments").as_array().unwrap();
    assert_eq!(segments.len(), 3);

    // Range should be set
    let range = json.get("range").expect("expected range").as_array().unwrap();
    assert_eq!(range.len(), 2);

    // size_bytes should be ~40% of file size (range 0.3-0.7 = 0.4 of file)
    let size_bytes = json.get("size_bytes").expect("expected size_bytes").as_u64().unwrap();
    assert!(size_bytes > 0, "size_bytes should be positive, got {size_bytes}");
}

#[test]
fn search_small_file_returns_done() {
    let dir = tempfile::tempdir().expect("failed to create temp dir");
    let file_path = create_temp_file(dir.path(), "test.txt", "small content");

    let output = run_ae(&[
        "search",
        "--file",
        file_path.to_str().unwrap(),
    ]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: serde_json::Value = serde_json::from_str(&stdout).expect("expected valid JSON");

    assert!(json.get("done").unwrap().as_bool().unwrap());
    let anchor = json.get("anchor").unwrap().as_str().unwrap();
    assert_eq!(anchor, "small content");

    // size_bytes should match the file size
    let size_bytes = json.get("size_bytes").expect("expected size_bytes").as_u64().unwrap();
    assert_eq!(size_bytes, 13, "expected size_bytes=13 for 'small content', got {size_bytes}");
}

#[test]
fn search_termination_bytes_returns_done() {
    let dir = tempfile::tempdir().expect("failed to create temp dir");
    let content = "hello world\n".repeat(500);
    let file_path = create_temp_file(dir.path(), "test.txt", &content);

    // Use a very small range that falls below default termination_bytes (512)
    let output = run_ae(&[
        "search",
        "--file",
        file_path.to_str().unwrap(),
        "--range",
        "0.49:0.51",
    ]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: serde_json::Value = serde_json::from_str(&stdout).expect("expected valid JSON");

    assert!(json.get("done").unwrap().as_bool().unwrap());
    assert!(json.get("anchor").unwrap().as_str().is_some());
}

#[test]
fn search_anchor_is_valid_utf8() {
    let dir = tempfile::tempdir().expect("failed to create temp dir");
    // Use UTF-8 content
    let file_path = create_temp_file(dir.path(), "test.txt", "こんにちは世界");

    let output = run_ae(&[
        "search",
        "--file",
        file_path.to_str().unwrap(),
    ]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: serde_json::Value = serde_json::from_str(&stdout).expect("expected valid JSON");

    // Should return done for small file
    assert!(json.get("done").unwrap().as_bool().unwrap());
    let anchor = json.get("anchor").unwrap().as_str().unwrap();

    // Anchor should be valid UTF-8
    assert!(std::str::from_utf8(anchor.as_bytes()).is_ok());
    assert_eq!(anchor, "こんにちは世界");
}
