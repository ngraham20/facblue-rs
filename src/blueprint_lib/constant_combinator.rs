use serde::{Deserialize, Serialize};

use super::{Comparator, Position, Quality, SignalType};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ConstantCombinator {
    control_behavior: ControlBehavior,
    entity_number: usize,
    name: String,
    pub position: Position
}

impl ConstantCombinator {
    pub fn new() -> Self {
        ConstantCombinator {
            control_behavior: ControlBehavior::new(),
            entity_number: 1,
            name: String::from("constant-combinator"),
            position: Position::from_xy(0f32, 0f32),
        }
    }

    pub fn push_section(&mut self, section: Section) {
        let idx = self.sections().len();
        self.sections_mut().push(section.with_index(idx as u8));
    }

    pub fn sections(&self) -> &Vec<Section> {
        &self.control_behavior.sections.sections
    }

    pub fn sections_mut(&mut self) -> &mut Vec<Section> {
        &mut self.control_behavior.sections.sections
    }

    pub fn with_sections(self, sections: Vec<Section>) -> Self {
        Self {
            control_behavior: ControlBehavior::with_sections(sections),
            ..self
        }
    }

    pub fn with_position(self, position: Position) -> Self {
        Self {
            position,
            ..self
        }
    }

    pub fn with_entity_number(self, entity_number: usize) -> Self {
        Self {
            entity_number,
            ..self
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ControlBehavior {
    pub sections: Sections,
}

impl ControlBehavior {
    fn new() -> Self {
        Self {
            sections: Sections::new(Vec::new()),
        }
    }
    fn sections(&self) -> &Vec<Section> {
        &self.sections.sections
    }
    fn push_section(&mut self, section: Section) {
        let idx = self.sections.sections.len();
        self.sections.sections.push(section.with_index(idx as u8));
    }
    pub fn with_sections(sections: Vec<Section>) -> Self {
        Self {
            sections: Sections::new(sections),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Sections {
    sections: Vec<Section>
}

impl Sections {
    fn new(sections: Vec<Section>) -> Self {
        Sections {
            sections,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Section {
    index: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    pub filters: Vec<LogisticFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiplier: Option<usize>,
    #[serde(default = "section_active_default")]
    pub active: bool,
}

impl Section {
    pub fn new(index: u8, filters: Vec<LogisticFilter>) -> Self {
        Section {
            index,
            filters,
            active: true,
            group: None,
            multiplier: None,
        }
    }

    pub fn deactivate(self) -> Self {
        Section {
            active: false,
            ..self
        }
    }

    pub fn with_index(self, index: u8) -> Self {
        Section {
            index,
            ..self
        }
    }
}

fn section_active_default() -> bool {
    true
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct LogisticFilter {
    index: u16,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signal_type: Option<SignalType>,
    pub name: String,
    pub quality: Quality,
    pub comparator: Comparator,
    pub count: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_delivery_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_from: Option<String>,
}

impl LogisticFilter {
    pub fn new(index: u16, name: String, count: usize, quality: Quality, comparator: Comparator) -> Self {
        Self {
            index,
            name,
            quality,
            comparator,
            count,
            signal_type: None,
            max_count: None,
            minimum_delivery_count: None,
            import_from: None,

        }
    }

    pub fn with_signal_type(self, signal_type: SignalType) -> Self {
        Self {
            signal_type: Some(signal_type),
            ..self
        }
    }
}