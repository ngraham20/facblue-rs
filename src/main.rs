use std::fs;
use anyhow::{Context, Error};
use blueprint_lib::BP;
use clap::{arg, Parser};

mod blueprint_lib;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    decode: Option<String>,
    #[arg(short, long)]
    encode: Option<String>,
}

fn main() -> Result<(), Error> {
    let args =Args::parse();
    if let Some(path) = args.decode {
        let blueprint = fs::read_to_string(path).context("Failed to read file")?;
        let bp = BP::decode(blueprint)?;
        println!("{:?}", bp);
    }

    if let Some(path) = args.encode {
        let json = fs::read_to_string(path).context("Failed to read file")?;
    }

    Ok(())
}

