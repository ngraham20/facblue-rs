use core::str;
use std::io::{Read, Write};

use anyhow::{Context, Error};
use base64::{engine::general_purpose, Engine};
use flate2::{read::ZlibDecoder, write::ZlibEncoder, Compression};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct BP {
    blueprint: Blueprint,
}

impl BP {
    pub fn decode(bp_string: String) -> Result<Self, Error> {
        let bytes = general_purpose::STANDARD.decode(&bp_string[1..])?;
        let mut deflater = ZlibDecoder::new(&bytes[..]);
        let mut s = String::new();
        deflater.read_to_string(&mut s).context("failed to read string")?;
        Ok(serde_json::from_str(&s).context("Failed to parse JSON")?)
    }

    pub fn encode(bp: &BP) -> Result<String, Error> {
        let json_string = serde_json::to_string(&bp)?;
        println!("{}", json_string);
        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(json_string.as_bytes())?;
        let compressed = encoder.finish()?;
        let bp_string = general_purpose::STANDARD.encode(compressed);
        Ok(format!("0{}", bp_string))
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Blueprint {
    item: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    label_color: Option<Color>,
    entities: Vec<Entity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tiles: Option<Vec<Tile>>,
    icons: Vec<Icon>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schedules: Option<Vec<Schedule>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(rename = "snap-to-grid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    snap_to_grid: Option<Position>,
    #[serde(rename = "absolute-snapping")]
    #[serde(skip_serializing_if = "Option::is_none")]
    absolute_snapping: Option<bool>,
    #[serde(rename = "position-relative-to-grid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    position_relative_to_grid: Option<Position>,
    version: usize,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Schedule {
    schedule: Vec<ScheduleRecord>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct ScheduleRecord {
    station: String,
    wait_conditions: Vec<WaitCondition>,
    temporary: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct WaitCondition {
    #[serde(rename = "type")]
    wait_condition_type: WaitConditionType,
    compare_type: WaitCompareType,
    ticks: usize,
    // TODO: implement Circuit condition
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
enum WaitCompareType {
    And,
    Or,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
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

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Tile {
    name: String,
    position: Position,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Color {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Entity {
    control_behavior: ControlBehavior,
     entity_number: usize,
     name: String,
     position: Position
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Icon {
    index: usize,
    signal: Signal
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Signal {
    name: String,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    signal_type: Option<SignalType>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct ControlBehavior {
    sections: Sections,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Sections {
    sections: Vec<Section>
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Section {
    index: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    group: Option<String>,
    filters: Vec<LogisticFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    multiplier: Option<usize>,
    #[serde(default = "section_active_default")]
    active: bool,
}

fn section_active_default() -> bool {
    true
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct LogisticFilter {
    index: u16,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    signal_type: Option<SignalType>,
    name: String,
    quality: Quality,
    comparator: Comparator,
    count: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minimum_delivery_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    import_from: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
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

#[derive(Serialize, Deserialize, Debug, PartialEq)]
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

#[derive(Serialize, Deserialize, Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_decode() -> Result<(), Error> {
        let bp_string = String::from("0eNqdksFuwjAMht/F54CgpGiNtBNvsQmhtITNUut0qVutqvLuOEUwbZdqKJfkt/39TpwJyrp3bUBiMBNg5akD8z5Bhx9k66Tx2DowMGDgXhQFZJsk3DJWSBcPUQHS2X2D2cajAkeMjO5Gmg/jifqmdEESHoDkxZZ4VfmmRLLsg9Bb30mtp2QtPL3OFYxgst06Fxep4eDrU+k+7YBSIFmdq1JB93svzveWFFywZhf+qgs324jyJQHpXkTyoZmTpNvWhrlbA6+z0KfX20b1gGeL8MPz8N0i/O15uF6Enz3/F3+UFdPHQHaNBH/+nIJB5jKPO99nhS6KXBfZy15vYrwC7F3c4g==");
        let bp: BP = BP {
            blueprint: Blueprint {
                entities: vec![
                    Entity {
                        control_behavior: ControlBehavior {
                            sections: Sections {
                                sections: vec![
                                    Section {
                                        filters: vec![
                                            LogisticFilter {
                                                comparator: Comparator::EqualTo,
                                                max_count:None,
                                                minimum_delivery_count:None,
                                                import_from:None,
                                                index:1,
                                                count:1,
                                                name:String::from("signal-0"),
                                                quality:Quality::Normal,
                                                signal_type:Some(SignalType::Virtual),
                                            },
                                            LogisticFilter {
                                                comparator: Comparator::EqualTo,
                                                max_count: None,
                                                minimum_delivery_count: None,
                                                import_from: None,
                                                index: 2,
                                                name: String::from("signal-C"),
                                                quality: Quality::Normal,
                                                signal_type: Some(SignalType::Virtual),
                                                count: 1,
                                            },
                                            LogisticFilter {
                                                comparator: Comparator::EqualTo,
                                                max_count: None,
                                                minimum_delivery_count: None,
                                                import_from: None,
                                                index: 3,
                                                count: 1,
                                                name: String::from("signal-Z"),
                                                quality: Quality::Normal,
                                                signal_type: Some(SignalType::Virtual),
                                            },
                                            LogisticFilter {
                                                comparator: Comparator::EqualTo,
                                                count: 1,
                                                index: 4,
                                                name: String::from("signal-dot"),
                                                quality: Quality::Normal,
                                                signal_type: Some(SignalType::Virtual),
                                                max_count: None,
                                                minimum_delivery_count: None,
                                                import_from: None,
                                            }
                                        ],
                                        index:1,
                                        group:None,
                                        multiplier:None,
                                        active:true,
                                    }
                                ],
                            },
                        },
                        entity_number: 1,
                        name: String::from("constant-combinator"),
                        position: Position {
                            x: 4.5,
                            y: 23.5,
                        }
                    }
                ],
                icons: vec![Icon {
                    index: 1,
                    signal: Signal {
                        name: String::from("signal-info"),
                        signal_type: Some(SignalType::Virtual),
                    }
                }],
                item: String::from("blueprint"),
                version: 562949954928640,
                label: None,
                label_color: None,
                tiles: None,
                schedules: None,
                description: None,
                snap_to_grid: None,
                absolute_snapping: None,
                position_relative_to_grid: None,
            },
        };
        assert_eq!(BP::decode(bp_string).expect("Failed to decode bp_string"), bp);
        Ok(())
    }
    
    #[test]
    fn test_encode_decode() -> Result<(), Error> {
        let bp: BP = BP {
            blueprint: Blueprint {
                entities: vec![
                    Entity {
                        control_behavior: ControlBehavior {
                            sections: Sections {
                                sections: vec![
                                    Section {
                                        filters: vec![
                                            LogisticFilter {
                                                comparator: Comparator::EqualTo,
                                                max_count:None,
                                                minimum_delivery_count:None,
                                                import_from:None,
                                                index:1,
                                                count:1,
                                                name:String::from("signal-0"),
                                                quality:Quality::Normal,
                                                signal_type:Some(SignalType::Virtual),
                                            },
                                            LogisticFilter {
                                                comparator: Comparator::EqualTo,
                                                max_count: None,
                                                minimum_delivery_count: None,
                                                import_from: None,
                                                index: 2,
                                                name: String::from("signal-C"),
                                                quality: Quality::Normal,
                                                signal_type: Some(SignalType::Virtual),
                                                count: 1,
                                            },
                                            LogisticFilter {
                                                comparator: Comparator::EqualTo,
                                                max_count: None,
                                                minimum_delivery_count: None,
                                                import_from: None,
                                                index: 3,
                                                count: 1,
                                                name: String::from("signal-Z"),
                                                quality: Quality::Normal,
                                                signal_type: Some(SignalType::Virtual),
                                            },
                                            LogisticFilter {
                                                comparator: Comparator::EqualTo,
                                                count: 1,
                                                index: 4,
                                                name: String::from("signal-dot"),
                                                quality: Quality::Normal,
                                                signal_type: Some(SignalType::Virtual),
                                                max_count: None,
                                                minimum_delivery_count: None,
                                                import_from: None,
                                            }
                                        ],
                                        index:1,
                                        group:None,
                                        multiplier:None,
                                        active:true,
                                    }
                                ],
                            },
                        },
                        entity_number: 1,
                        name: String::from("constant-combinator"),
                        position: Position {
                            x: 4.5,
                            y: 23.5,
                        }
                    }
                ],
                icons: vec![Icon {
                    index: 1,
                    signal: Signal {
                        name: String::from("signal-info"),
                        signal_type: Some(SignalType::Virtual),
                    }
                }],
                item: String::from("blueprint"),
                version: 562949954928640,
                label: None,
                label_color: None,
                tiles: None,
                schedules: None,
                description: None,
                snap_to_grid: None,
                absolute_snapping: None,
                position_relative_to_grid: None,
            },
        };
        let encoded = BP::encode(&bp)?;
        let decoded: BP = BP::decode(encoded)?;
        assert_eq!(decoded, bp);
        Ok(())
    }
}