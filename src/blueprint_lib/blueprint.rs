use serde::{Deserialize, Serialize};

use super::constant_combinator::ConstantCombinator;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Blueprint {
    pub item: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_color: Option<Color>,
    pub entities: Option<Vec<Entity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiles: Option<Vec<Tile>>,
    pub icons: Vec<Icon>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedules: Option<Vec<Schedule>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "snap-to-grid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snap_to_grid: Option<Position>,
    #[serde(rename = "absolute-snapping")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub absolute_snapping: Option<bool>,
    #[serde(rename = "position-relative-to-grid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_relative_to_grid: Option<Position>,
    pub version: usize,
}

impl Blueprint {
    pub fn new(item: String, icons: Vec<Icon>, version: usize) -> Self {
        Blueprint {
            item,
            entities: None,
            icons,
            version,
            label: None,
            label_color: None,
            tiles: None,
            schedules: None,
            description: None,
            snap_to_grid: None,
            absolute_snapping: None,
            position_relative_to_grid: None,
        }
    }

    pub fn with_entities(self, entities: Vec<Entity>) -> Self {
        Self {
            entities: Some(entities),
            ..self
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Schedule {
    pub schedule: Vec<ScheduleRecord>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ScheduleRecord {
    pub station: String,
    pub wait_conditions: Vec<WaitCondition>,
    pub temporary: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct WaitCondition {
    #[serde(rename = "type")]
    pub wait_condition_type: WaitConditionType,
    pub compare_type: WaitCompareType,
    pub ticks: usize,
    // TODO: implement Circuit condition
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum WaitCompareType {
    And,
    Or,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum WaitConditionType {
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

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Tile {
    pub name: String,
    pub position: Position,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum Entity {
    ConstantCombinator(ConstantCombinator)
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub fn new() -> Self {
        Position {
            x: 0f32,
            y: 0f32,
        }
    }
    
    pub fn from_xy(x: f32, y: f32) -> Self {
        Position {
            x,
            y,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Icon {
    pub index: usize,
    pub signal: Signal
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Signal {
    pub name: String,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signal_type: Option<SignalType>,
}

impl Signal {
    pub fn new(name: String) -> Self {
        Signal {
            name,
            signal_type: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Comparator {
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

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Quality {
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

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum SignalType {
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