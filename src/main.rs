use std::fs;
use anyhow::{Context, Error};
use blueprint_lib::BP;
use clap::{arg, Parser};

mod blueprint_lib;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, required = true)]
    file: String,

}

fn main() -> Result<(), Error> {
    let args =Args::parse();
    let blueprint = fs::read_to_string(args.file).context("Failed to read file")?;
    let bp = BP::decode(blueprint)?;
    println!("{:?}", bp);
    Ok(())
}

