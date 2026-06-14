mod commands;

use clap::{ArgGroup, Parser, Subcommand};

/// AnchorEdit - LLM-native code editing via AnchorScope
#[derive(Parser)]
#[command(name = "ae", version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Read content matched by an anchor
    #[command(groups([
        ArgGroup::new("anchor_source")
            .required(true)
            .args(["anchor", "anchor_file"]),
    ]))]
    Read {
        /// Path to the target file
        #[arg(long)]
        file: String,

        /// Anchor string to match
        #[arg(long)]
        anchor: Option<String>,

        /// Path to a file containing the anchor
        #[arg(long)]
        anchor_file: Option<String>,
    },
    /// Write a replacement for the anchored scope
    #[command(groups([
        ArgGroup::new("anchor_source")
            .required(true)
            .args(["anchor", "anchor_file"]),
        ArgGroup::new("replacement_source")
            .required(true)
            .args(["replacement", "replacement_file"]),
    ]))]
    Write {
        /// Path to the target file
        #[arg(long)]
        file: String,

        /// Anchor string to match
        #[arg(long)]
        anchor: Option<String>,

        /// Path to a file containing the anchor
        #[arg(long)]
        anchor_file: Option<String>,

        /// Expected scope hash from a previous read
        #[arg(long)]
        expected_hash: String,

        /// Replacement string
        #[arg(long)]
        replacement: Option<String>,

        /// Path to a file containing the replacement
        #[arg(long)]
        replacement_file: Option<String>,
    },
}

fn get_anchorscope_bin() -> String {
    std::env::var("ANCHORSCOPE_BIN").unwrap_or_else(|_| "anchorscope".to_string())
}

fn main() {
    let cli = Cli::parse();

    let code = match cli.command {
        Commands::Read {
            file,
            anchor,
            anchor_file,
        } => commands::read::run(&file, anchor.as_deref(), anchor_file.as_deref()),
        Commands::Write {
            file,
            anchor,
            anchor_file,
            expected_hash,
            replacement,
            replacement_file,
        } => {
            commands::write::run(
                &file,
                anchor.as_deref(),
                anchor_file.as_deref(),
                &expected_hash,
                replacement.as_deref(),
                replacement_file.as_deref(),
            )
        }
    };

    std::process::exit(code);
}
