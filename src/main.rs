use std::fs;
use std::process;

use anchoredit::{apply, ApplyError};
use clap::{ArgGroup, Parser};

/// AnchorEdit v2 — Apply engine on AnchorScope
#[derive(Parser)]
#[command(name = "anchoredit", version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand)]
enum Commands {
    /// Apply a replacement at the anchored location
    #[command(groups([
        ArgGroup::new("anchor_source")
            .required(true)
            .args(["anchor", "anchor_file"]),
        ArgGroup::new("replacement_source")
            .required(true)
            .args(["replacement", "replacement_file"]),
    ]))]
    Apply {
        /// Path to the target file
        #[arg(long)]
        file: String,

        /// Anchor string to match
        #[arg(long)]
        anchor: Option<String>,

        /// Path to a file containing the anchor
        #[arg(long)]
        anchor_file: Option<String>,

        /// Replacement string
        #[arg(long)]
        replacement: Option<String>,

        /// Path to a file containing the replacement
        #[arg(long)]
        replacement_file: Option<String>,
    },
}

fn resolve_anchor(anchor: Option<String>, anchor_file: Option<String>) -> Vec<u8> {
    if let Some(a) = anchor {
        a.into_bytes()
    } else if let Some(path) = anchor_file {
        fs::read(&path).unwrap_or_else(|e| {
            eprintln!("error: failed to read anchor file: {e}");
            process::exit(1);
        })
    } else {
        unreachable!("clap ensures one of --anchor or --anchor-file is provided")
    }
}

fn resolve_replacement(replacement: Option<String>, replacement_file: Option<String>) -> Vec<u8> {
    if let Some(r) = replacement {
        r.into_bytes()
    } else if let Some(path) = replacement_file {
        fs::read(&path).unwrap_or_else(|e| {
            eprintln!("error: failed to read replacement file: {e}");
            process::exit(1);
        })
    } else {
        unreachable!("clap ensures one of --replacement or --replacement-file is provided")
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Apply {
            file,
            anchor,
            anchor_file,
            replacement,
            replacement_file,
        } => {
            let anchor_bytes = resolve_anchor(anchor, anchor_file);
            let replacement_bytes = resolve_replacement(replacement, replacement_file);

            match apply(&file, &anchor_bytes, &replacement_bytes) {
                Ok(result) => {
                    println!("OK: written {} bytes", result.bytes_written);
                }
                Err(ApplyError::NoMatch) => {
                    eprintln!("NO_MATCH");
                    process::exit(1);
                }
                Err(ApplyError::MultipleMatches) => {
                    eprintln!("MULTIPLE_MATCHES");
                    process::exit(1);
                }
                Err(ApplyError::HashMismatch) => {
                    eprintln!("HASH_MISMATCH");
                    process::exit(1);
                }
                Err(ApplyError::IoError(msg)) => {
                    eprintln!("{msg}");
                    process::exit(1);
                }
            }
        }
    }
}
