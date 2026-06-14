use std::io::{BufRead, BufReader};
use std::process::Stdio;

use super::build_command;

/// Run `ae read` by wrapping `as read`.
pub fn run(
    file: &str,
    anchor: Option<&str>,
    anchor_file: Option<&str>,
) -> i32 {
    let mut cmd = build_command("read");
    cmd.arg("--file").arg(file);

    match (anchor, anchor_file) {
        (Some(a), None) => {
            cmd.arg("--anchor").arg(a);
        }
        (None, Some(f)) => {
            cmd.arg("--anchor-file").arg(f);
        }
        (Some(_), Some(_)) => {
            eprintln!("error: specify either --anchor or --anchor-file, not both");
            return 1;
        }
        (None, None) => {
            eprintln!("error: specify either --anchor or --anchor-file");
            return 1;
        }
    }

    let mut child = cmd
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap_or_else(|e| {
            eprintln!("error: failed to spawn anchorscope: {e}");
            std::process::exit(1);
        });

    // Stream stdout to our stdout
    let stdout = child.stdout.take().expect("failed to get stdout");
    let mut stdout_buf = BufReader::new(stdout);
    let mut line = String::new();
    loop {
        line.clear();
        match stdout_buf.read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => print!("{line}"),
            Err(e) => {
                eprintln!("error: failed to read stdout: {e}");
            }
        }
    }

    // Stream stderr to our stderr
    let stderr = child.stderr.take().expect("failed to get stderr");
    let mut stderr_buf = BufReader::new(stderr);
    let mut line = String::new();
    loop {
        line.clear();
        match stderr_buf.read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => eprint!("{line}"),
            Err(e) => {
                eprintln!("error: failed to read stderr: {e}");
            }
        }
    }

    child.wait().expect("failed to wait on process").code().unwrap_or(1)
}
