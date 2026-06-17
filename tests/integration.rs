use std::fs;
use std::process::Command;

const BIN: &str = env!("CARGO_BIN_EXE_anchoredit");

fn create_temp_file(dir: &std::path::Path, name: &str, content: &str) -> std::path::PathBuf {
    let path = dir.join(name);
    fs::write(&path, content).expect("failed to write temp file");
    path
}

fn run_anchoredit(args: &[&str]) -> std::process::Output {
    Command::new(BIN).args(args).output().expect("failed to execute anchoredit")
}

// Library-level tests

#[test]
fn apply_success() {
    let dir = tempfile::tempdir().expect("failed to create temp dir");
    let file_path = create_temp_file(dir.path(), "test.txt", "hello world");

    let result = anchoredit::apply(
        file_path.to_str().unwrap(),
        b"hello world",
        b"goodbye world",
    );

    assert!(result.is_ok());
    let apply_result = result.unwrap();
    assert!(apply_result.bytes_written > 0);
    assert_eq!(apply_result.scope_hash.len(), 16);

    let content = fs::read_to_string(&file_path).expect("failed to read file");
    assert_eq!(content, "goodbye world");
}

#[test]
fn apply_no_match() {
    let dir = tempfile::tempdir().expect("failed to create temp dir");
    let file_path = create_temp_file(dir.path(), "test.txt", "hello world");

    let result = anchoredit::apply(
        file_path.to_str().unwrap(),
        b"not found",
        b"replacement",
    );

    assert!(result.is_err());
    match result.unwrap_err() {
        anchoredit::ApplyError::NoMatch => {} // expected
        e => panic!("expected NoMatch, got {e:?}"),
    }
}

#[test]
fn apply_multiple_matches() {
    let dir = tempfile::tempdir().expect("failed to create temp dir");
    let file_path = create_temp_file(dir.path(), "test.txt", "foo bar foo baz");

    let result = anchoredit::apply(
        file_path.to_str().unwrap(),
        b"foo",
        b"bar",
    );

    assert!(result.is_err());
    match result.unwrap_err() {
        anchoredit::ApplyError::MultipleMatches => {} // expected
        e => panic!("expected MultipleMatches, got {e:?}"),
    }
}

#[test]
fn apply_concurrent_modification_hash_mismatch() {
    // Simulate the case where the file is modified between read and write.
    // We do this by using anchorscope directly to show the scenario,
    // since apply() chains read→write atomically.
    //
    // In practice, HASH_MISMATCH in apply() can only occur if an external
    // process modifies the file between the two library calls.
    // We verify the error path exists by testing anchorscope::write with a bad hash.

    let dir = tempfile::tempdir().expect("failed to create temp dir");
    let file_path = create_temp_file(dir.path(), "test.txt", "hello world");

    let err = anchorscope::write(
        file_path.to_str().unwrap(),
        b"hello world",
        "0000000000000000",
        b"replacement",
    )
    .unwrap_err();

    match err {
        anchorscope::AnchorScopeError::HashMismatch => {} // expected
        e => panic!("expected HashMismatch, got {e:?}"),
    }
}

// CLI tests

#[test]
fn cli_apply_success() {
    let dir = tempfile::tempdir().expect("failed to create temp dir");
    let file_path = create_temp_file(dir.path(), "test.txt", "hello world");

    let output = run_anchoredit(&[
        "apply",
        "--file",
        file_path.to_str().unwrap(),
        "--anchor",
        "hello world",
        "--replacement",
        "goodbye world",
    ]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("OK: written"));

    let content = fs::read_to_string(&file_path).expect("failed to read file");
    assert_eq!(content, "goodbye world");
}

#[test]
fn cli_apply_no_match() {
    let dir = tempfile::tempdir().expect("failed to create temp dir");
    let file_path = create_temp_file(dir.path(), "test.txt", "hello world");

    let output = run_anchoredit(&[
        "apply",
        "--file",
        file_path.to_str().unwrap(),
        "--anchor",
        "not found",
        "--replacement",
        "replacement",
    ]);

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("NO_MATCH"));
}

#[test]
fn cli_apply_multiple_matches() {
    let dir = tempfile::tempdir().expect("failed to create temp dir");
    let file_path = create_temp_file(dir.path(), "test.txt", "foo bar foo baz");

    let output = run_anchoredit(&[
        "apply",
        "--file",
        file_path.to_str().unwrap(),
        "--anchor",
        "foo",
        "--replacement",
        "bar",
    ]);

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("MULTIPLE_MATCHES"));
}

#[test]
fn cli_apply_with_files() {
    let dir = tempfile::tempdir().expect("failed to create temp dir");
    let file_path = create_temp_file(dir.path(), "target.txt", "before content after");
    let anchor_path = create_temp_file(dir.path(), "anchor.txt", "content");
    let replacement_path = create_temp_file(dir.path(), "replacement.txt", "replaced");

    let output = run_anchoredit(&[
        "apply",
        "--file",
        file_path.to_str().unwrap(),
        "--anchor-file",
        anchor_path.to_str().unwrap(),
        "--replacement-file",
        replacement_path.to_str().unwrap(),
    ]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("OK: written"));

    let content = fs::read_to_string(&file_path).expect("failed to read file");
    assert_eq!(content, "before replaced after");
}
