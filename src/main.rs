use std::{fs, io::Read};
use base64::{engine::general_purpose, Engine};
use flate2::bufread::ZlibDecoder;
use clap::{arg, Parser};
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, required = true)]
    file: String,

}

#[derive(Serialize, Deserialize, Debug)]
struct BP {
    blueprint: Blueprint,
}

#[derive(Serialize, Deserialize, Debug)]
struct Blueprint {
    item: String,
    label: Option<String>,
    label_color: Option<Color>,
    entities: Vec<Entity>,
    tiles: Option<Vec<Tile>>,
    icons: Vec<Icon>,
    schedules: Option<Vec<Schedule>>,
    description: Option<String>,
    #[serde(rename = "snap-to-grid")]
    snap_to_grid: Option<Position>,
    #[serde(rename = "absolute-snapping")]
    absolute_snapping: Option<bool>,
    #[serde(rename = "position-relative-to-grid")]
    position_relative_to_grid: Option<Position>,
    version: usize,
}

#[derive(Serialize, Deserialize, Debug)]
struct Schedule {
    schedule: Vec<ScheduleRecord>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ScheduleRecord {
    station: String,
    wait_conditions: Vec<WaitCondition>,
    temporary: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct WaitCondition {
    #[serde(rename = "type")]
    wait_condition_type: WaitConditionType,
    compare_type: WaitCompareType,
    ticks: usize,
    // TODO: implement Circuit condition
}

#[derive(Serialize, Deserialize, Debug)]
enum WaitCompareType {
    And,
    Or,
}

#[derive(Serialize, Deserialize, Debug)]
enum WaitConditionType {
    Time,
    Inactivity,
    Full,
    Empty,
    ItemCount,
    Circuit,
    RobotsInactive,
    FluidCound,
    PassengerPresent,
    PassengerNotPresent,
}

#[derive(Serialize, Deserialize, Debug)]
struct Tile {
    name: String,
    position: Position,
}

#[derive(Serialize, Deserialize, Debug)]
struct Color {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Entity {
    control_behavior: ControlBehavior,
     entity_number: usize,
     name: String,
     position: Position
}

#[derive(Serialize, Deserialize, Debug)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Icon {
    index: usize,
    signal: Signal
}

#[derive(Serialize, Deserialize, Debug)]
struct Signal {
    name: String,
    #[serde(rename = "type")]
    signal_type: Option<SignalType>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ControlBehavior {
    sections: Sections,
}

#[derive(Serialize, Deserialize, Debug)]
struct Sections {
    sections: Vec<Section>
}

#[derive(Serialize, Deserialize, Debug)]
struct Section {
    index: u8,
    group: Option<String>,
    filters: Vec<LogisticFilter>,
    multiplier: Option<usize>,
    #[serde(default = "section_active_default")]
    active: bool,
}

fn section_active_default() -> bool {
    true
}

#[derive(Serialize, Deserialize, Debug)]
struct LogisticFilter {
    index: u16,
    #[serde(rename = "type")]
    signal_type: Option<SignalType>,
    name: String,
    quality: Quality,
    comparator: Comparator,
    count: usize,
    max_count: Option<usize>,
    minimum_delivery_count: Option<usize>,
    import_from: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
enum Comparator {
    #[serde(rename = "=")]
    EqualTo,
    #[serde(rename = ">")]
    GreaterThan,
    #[serde(rename = "<")]
    LesserThan,
    #[serde(rename = ">=", alias = "≥")]
    GreaterThanOrEqualTo,
    #[serde(rename = "<=", alias = "≤")]
    LesserThanOrEqualTo,
    #[serde(rename = "!=", alias = "≠")]
    NotEqualTo,
}

#[derive(Serialize, Deserialize, Debug)]
enum Quality {
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "uncommon")]
    Uncommon,
    #[serde(rename = "rare")]
    Rare,
    #[serde(rename = "epic")]
    Epic,
    #[serde(rename = "legendary")]
    Legendary
}

#[derive(Serialize, Deserialize, Debug)]
enum SignalType {
    #[serde(rename = "item", alias = "")]
    Item,
    #[serde(rename = "fluid")]
    Fluid,
    #[serde(rename = "virtual")]
    Virtual,
    #[serde(rename = "entity")]
    Entity,
    #[serde(rename = "recipe")]
    Recipe,
    #[serde(rename = "space-location")]
    SpaceLocation,
    #[serde(rename = "asteroid-chunk")]
    AsteroidChunk,
    #[serde(rename = "quality")]
    Quality,
}

fn main() {
    let args =Args::parse();
    let blueprint = fs::read_to_string(args.file).expect("Failed to read file");
    let bytes = general_purpose::STANDARD.decode(&blueprint[1..]).unwrap();
    let mut deflater = ZlibDecoder::new(&bytes[..]);
    let mut s = String::new();
    deflater.read_to_string(&mut s).expect("failed to read string");
    // println!("{}", s);
    let bp: BP = serde_json::from_str(&s).expect("Failed to parse JSON");
    println!("{:?}", bp);
}

