use std::collections::HashMap;

use super::{color::Color, line_style::LineStyle, packet::Packet, stipple::Stipple};

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
}
