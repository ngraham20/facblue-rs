use std::fs;
use anyhow::{Context, Error};
use blueprint_lib::BlueprintMeta;
use clap::{arg, Parser};
use blueprint_lib::*;

mod blueprint_lib;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file: Option<String>,
}

fn main() -> Result<(), Error> {
    let args =Args::parse();
    if let Some(path) = args.file {
        let recipes_string = fs::read_to_string(path).context("Failed to read file")?;
        let bp: Recipes = serde_json::from_str(&recipes_string)?;
        println!("{:?}", bp);
    }
    Ok(())
}

