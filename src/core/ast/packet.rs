use std::fmt::{Debug, Formatter, Result};

use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct Packet {
    pub name: String,
    pub stipple: String,
    pub line_style: String,
    pub fill: String,
    pub outline: String,
    pub fill_style: Option<String>,
}

impl Packet {
    pub fn new() -> Self {
        Packet {
            name: String::new(),
            stipple: String::new(),
            line_style: String::new(),
            fill: String::new(),
            outline: String::new(),
            fill_style: None,
        }
    }

    pub fn from_vec(params: &Vec<String>) -> Self {
        if params.len() == 7 {
            return Packet {
                name: params[1].clone(),
                stipple: params[2].clone(),
                line_style: params[3].clone(),
                fill: params[4].clone(),
                outline: params[5].clone(),
                fill_style: Some(params[6].clone()),
            };
        }
        if params.len() == 6 {
            return Packet {
                name: params[1].clone(),
                stipple: params[2].clone(),
                line_style: params[3].clone(),
                fill: params[4].clone(),
                outline: params[5].clone(),
                fill_style: None,
            };
        }
        panic!("Invalid packet parameters: {:?}", params);
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

impl Debug for Packet {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "Packet(name={}, stipple={}, line_style={}, fill={}, outline={}, fill_style={:?})",
            self.name, self.stipple, self.line_style, self.fill, self.outline, self.fill_style
        )
    }
}
