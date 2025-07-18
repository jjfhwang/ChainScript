// src/main.rs
/*
 * Main executable for ChainScript
 */

use clap::Parser;
use chainscript::{Result, run};

#[derive(Parser)]
#[command(version, about = "ChainScript - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
