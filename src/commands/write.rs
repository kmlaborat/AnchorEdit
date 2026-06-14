use super::{build_command, run_command};

/// Run `ae write` by wrapping `as write`.
pub fn run(
    file: &str,
    anchor: Option<&str>,
    anchor_file: Option<&str>,
    expected_hash: &str,
    replacement: Option<&str>,
    replacement_file: Option<&str>,
) -> i32 {
    let mut cmd = build_command("write");
    cmd.arg("--file").arg(file);

    if let Some(a) = anchor {
        cmd.arg("--anchor").arg(a);
    }
    if let Some(f) = anchor_file {
        cmd.arg("--anchor-file").arg(f);
    }

    cmd.arg("--expected-hash").arg(expected_hash);

    if let Some(r) = replacement {
        cmd.arg("--replacement").arg(r);
    }
    if let Some(f) = replacement_file {
        cmd.arg("--replacement-file").arg(f);
    }

    run_command(cmd)
}
