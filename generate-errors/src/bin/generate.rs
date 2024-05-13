use generate_errors::*;
use std::path::PathBuf;

use clap::Parser;

/// Generate error reference pages from Bevy engine
/// for use on the Bevy website.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the directory containing the
    /// error files stored in the
    /// local Bevy GitHub repo.
    #[arg(long)]
    errors_path: PathBuf,
    /// Path to the folder which the
    /// errors section should be generated in.
    #[arg(long)]
    output_path: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    println!("Writing section index & introduction . . .");
    write_section(&args.output_path)?;
    println!("Getting error page contents . . .");
    let error_page_content = get_error_pages(&args.errors_path)?;
    println!("Writing error pages content to output path . . .");
    write_pages(&args.output_path, error_page_content)?;

    println!("All good!");
    Ok(())
}
