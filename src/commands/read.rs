use super::{build_command, run_command};

/// Run `ae read` by wrapping `as read`.
pub fn run(
    file: &str,
    anchor: Option<&str>,
    anchor_file: Option<&str>,
) -> i32 {
    let mut cmd = build_command("read");
    cmd.arg("--file").arg(file);

    if let Some(a) = anchor {
        cmd.arg("--anchor").arg(a);
    }
    if let Some(f) = anchor_file {
        cmd.arg("--anchor-file").arg(f);
    }

    run_command(cmd)
}
