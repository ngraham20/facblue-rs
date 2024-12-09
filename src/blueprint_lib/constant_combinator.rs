use serde::{Deserialize, Serialize};

use super::{ControlBehavior, Position};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ConstantCombinator {
    pub control_behavior: ControlBehavior,
    pub entity_number: usize,
    pub name: String,
    pub position: Position
}