use core::str;
use std::io::{Read, Write};

use anyhow::{Context, Error};
use base64::{engine::general_purpose, Engine};
use flate2::{read::ZlibDecoder, write::ZlibEncoder, Compression};
use serde::{Deserialize, Serialize};

mod blueprint;
use blueprint::*;

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



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_decode() -> Result<(), Error> {
        let bp_string = String::from("0eNqdksFuwjAMht/F54CgpGiNtBNvsQmhtITNUut0qVutqvLuOEUwbZdqKJfkt/39TpwJyrp3bUBiMBNg5akD8z5Bhx9k66Tx2DowMGDgXhQFZJsk3DJWSBcPUQHS2X2D2cajAkeMjO5Gmg/jifqmdEESHoDkxZZ4VfmmRLLsg9Bb30mtp2QtPL3OFYxgst06Fxep4eDrU+k+7YBSIFmdq1JB93svzveWFFywZhf+qgs324jyJQHpXkTyoZmTpNvWhrlbA6+z0KfX20b1gGeL8MPz8N0i/O15uF6Enz3/F3+UFdPHQHaNBH/+nIJB5jKPO99nhS6KXBfZy15vYrwC7F3c4g==");
        let bp: BP = BP {
            blueprint: Blueprint::new(String::from("blueprint"), vec![Icon {
                index: 1,
                signal: Signal {
                    name: String::from("signal-info"),
                    signal_type: Some(SignalType::Virtual),
                }
            }], 562949954928640)
            .with_entities(vec![
                Entity {
                    control_behavior: ControlBehavior {
                        sections: Sections {
                            sections: vec![
                                Section::new(1, vec![
                                    LogisticFilter::new(1, String::from("signal-0"), 1, Quality::Normal, Comparator::EqualTo)
                                    .with_signal_type(SignalType::Virtual),
                                    LogisticFilter::new(2, String::from("signal-C"), 1, Quality::Normal, Comparator::EqualTo)
                                    .with_signal_type(SignalType::Virtual),
                                    LogisticFilter::new(3, String::from("signal-Z"), 1, Quality::Normal, Comparator::EqualTo)
                                    .with_signal_type(SignalType::Virtual),
                                    LogisticFilter::new(4, String::from("signal-dot"), 1, Quality::Normal, Comparator::EqualTo)
                                    .with_signal_type(SignalType::Virtual),
                                ],)
                            ]
                        },
                    },
                    entity_number: 1,
                    name: String::from("constant-combinator"),
                    position: Position::from_xy(4.5, 23.5),
                }
            ]),
        };
        assert_eq!(BP::decode(bp_string).expect("Failed to decode bp_string"), bp);
        Ok(())
    }
    
    #[test]
    fn test_encode_decode() -> Result<(), Error> {
        let bp: BP = BP {
            blueprint: Blueprint::new(String::from("blueprint"), vec![Icon {
                index: 1,
                signal: Signal {
                    name: String::from("signal-info"),
                    signal_type: Some(SignalType::Virtual),
                }
            }], 562949954928640)
            .with_entities(vec![
                Entity {
                    control_behavior: ControlBehavior {
                        sections: Sections {
                            sections: vec![
                                Section::new(1, vec![
                                    LogisticFilter::new(1, String::from("signal-0"), 1, Quality::Normal, Comparator::EqualTo)
                                    .with_signal_type(SignalType::Virtual),
                                    LogisticFilter::new(2, String::from("signal-C"), 1, Quality::Normal, Comparator::EqualTo)
                                    .with_signal_type(SignalType::Virtual),
                                    LogisticFilter::new(3, String::from("signal-Z"), 1, Quality::Normal, Comparator::EqualTo)
                                    .with_signal_type(SignalType::Virtual),
                                    LogisticFilter::new(4, String::from("signal-dot"), 1, Quality::Normal, Comparator::EqualTo)
                                    .with_signal_type(SignalType::Virtual),
                                ],)
                            ]
                        },
                    },
                    entity_number: 1,
                    name: String::from("constant-combinator"),
                    position: Position::from_xy(4.5, 23.5),
                }
            ]),
        };
        let encoded = BP::encode(&bp)?;
        let decoded: BP = BP::decode(encoded)?;
        assert_eq!(decoded, bp);
        Ok(())
    }
}