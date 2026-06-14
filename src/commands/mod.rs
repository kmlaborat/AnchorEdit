pub mod read;
pub mod search;
pub mod write;

use std::io::{BufRead, BufReader};
use std::process::Stdio;

use crate::get_anchorscope_bin;

/// Build a `Command` for the anchorscope binary with common arguments.
fn build_command(subcommand: &str) -> std::process::Command {
    let bin = get_anchorscope_bin();
    let mut cmd = std::process::Command::new(&bin);
    cmd.arg(subcommand);
    cmd
}

/// Spawn the command, stream stdout→stdout and stderr→stderr, and return the exit code.
pub fn run_command(mut cmd: std::process::Command) -> i32 {
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
