pub mod read;
pub mod write;

use std::process::Command;

use crate::get_anchorscope_bin;

/// Build a `Command` for the anchorscope binary with common arguments.
fn build_command(subcommand: &str) -> Command {
    let bin = get_anchorscope_bin();
    let mut cmd = Command::new(&bin);
    cmd.arg(subcommand);
    cmd
}
