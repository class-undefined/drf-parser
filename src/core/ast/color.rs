use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Color {
    pub rgb: [u8; 3],
    pub blink: bool,
    pub name: String,
}

impl Color {
    pub fn new(name: String, r: u8, g: u8, b: u8, blink: bool) -> Self {
        let rgb = [r, g, b];
        Color { rgb, blink, name }
    }

    pub fn from_vec(params: &Vec<String>) -> Self {
        let name = params[1].clone();
        let r = params[2].parse::<u8>().unwrap();
        let g = params[3].parse::<u8>().unwrap();
        let b = params[4].parse::<u8>().unwrap();
        if params.len() == 6 {
            let blink = params[5] == "t";
            Color::new(name, r, g, b, blink)
        } else {
            Color::new(name, r, g, b, false)
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}
