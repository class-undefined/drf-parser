use std::collections::HashMap;
use std::fmt::{Debug, Formatter, Result};

use serde::Serialize;

use super::{color::Color, line_style::LineStyle, packet::Packet, stipple::Stipple};

#[derive(Clone, Serialize)]
pub struct Drf {
    pub name: String,
    pub colors: HashMap<String, Color>,
    pub stipples: HashMap<String, Stipple>,
    pub line_styles: HashMap<String, LineStyle>,
    pub packets: HashMap<String, Packet>,
}

impl Drf {
    pub fn new(name: String) -> Self {
        Drf {
            name,
            colors: HashMap::new(),
            stipples: HashMap::new(),
            line_styles: HashMap::new(),
            packets: HashMap::new(),
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

impl Debug for Drf {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let s = format!(
            "Drf(name={:?}, colors={:?}, stipples={:?}, line_styles={:?}, packets={:?})",
            self.name, self.colors, self.stipples, self.line_styles, self.packets,
        );
        write!(f, "Drf: {}", s)
    }
}
