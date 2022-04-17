use anyhow::Result;
use clap::Parser;
use fs_err as fs;
use std::path::PathBuf;

mod generator;
mod json_loader;

#[derive(Debug, Parser)]
struct Args {
    input: PathBuf,
    output: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let json = fs::read_to_string(&args.input)?;
    let krate = json_loader::load_rjd(&json)?;
    let rust = generator::generate(&krate)?;
    fs::write(&args.output, rust)?;

    Ok(())
}
