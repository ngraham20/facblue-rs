use std::{fs, sync::MutexGuard};
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
    let mut combinator: ConstantCombinator = ConstantCombinator::new();
    combinator.push_section(Section::new(1, Vec::new()));
    if let Some(path) = args.file {
    let recipes_string = fs::read_to_string(path).context("Failed to read file")?;
    let recipes: Vec<Recipe> = serde_json::from_str(&recipes_string)?;
    let multiple_outputs: Vec<(String, String, f64)> = recipes.into_iter()
    .filter(|x| matches!(x.desired_item(), Some(_)) && x.desired_item().unwrap().amount > 1f64)
    .map(|x| {let item = x.desired_item().unwrap(); (item.name, item.ingredient_type.to_string(), item.amount)}).collect();
    
    for recipe in multiple_outputs {
        combinator.sections_mut()[0].push_signal(recipe);
    }
        println!("{:?}", combinator);
    }

    let bp: BlueprintMeta = BlueprintMeta::Blueprint {
        index: None,
        blueprint: Blueprint::new(String::from("blueprint"), vec![Icon {
            index: 1,
            signal: Signal {
                name: String::from("signal-info"),
                signal_type: Some(SignalType::Virtual),
            }
        }], 562949954928640)
        .with_entities(vec![
            Entity::ConstantCombinator(combinator),
        ]),
    };
    let json_string = BlueprintMeta::marshal_json(&bp)?;
    let bp_string = bp.to_blueprint_string()?;
    println!("{}", bp_string);
    Ok(())
}

