// src/main.rs
/*
 * Main executable for BlockchainNFTMinterTech
 */

use clap::Parser;
use blockchainnftmintertech::{Result, run};

#[derive(Parser)]
#[command(version, about = "BlockchainNFTMinterTech - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
