use std::fmt::{Debug, Formatter, Result};

use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct Stipple {
    pub name: String,
    pub bitmap: Vec<Vec<u8>>,
    pub row: usize,
    pub col: usize,
}

impl Stipple {
    pub fn new(name: String, bitmap: Vec<Vec<u8>>, row: usize, col: usize) -> Self {
        Stipple {
            name,
            bitmap,
            row,
            col,
        }
    }

    pub fn from_vec(params: &Vec<String>, row: usize, bitmap: Vec<Vec<u8>>) -> Self {
        let name = params[1].clone();
        let col = bitmap[0].len();
        Stipple::new(name, bitmap, row, col)
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

impl Debug for Stipple {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "Stipple(name={}, row={}, col={})",
            self.name, self.row, self.col
        )
    }
}
