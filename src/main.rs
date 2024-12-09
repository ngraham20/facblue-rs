use std::fs;
use anyhow::{Context, Error};
use blueprint_lib::BlueprintMeta;
use clap::{arg, Parser};

mod blueprint_lib;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file: Option<String>,
    #[arg(short, long)]
    json: Option<String>,
}

fn main() -> Result<(), Error> {
    let args =Args::parse();
    if let Some(path) = args.file {
        let blueprint = fs::read_to_string(path).context("Failed to read file")?;
        let bp = BlueprintMeta::decode(blueprint)?;
        println!("{:?}", bp);
    }
    if let Some(path) = args.json {
        let json = fs::read_to_string(path).context("Failed to read file")?;
        let bp = BlueprintMeta::parse_json(json)?;
        println!("{:?}\n", bp);
        println!("{}", BlueprintMeta::make_json(&bp)?);
    }
    Ok(())
}

