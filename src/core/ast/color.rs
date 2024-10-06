#[derive(Debug, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub blink: bool,
    pub name: String,
}

impl Color {
    pub fn new(name: String, r: u8, g: u8, b: u8, blink: bool) -> Self {
        Color {
            r,
            g,
            b,
            blink,
            name,
        }
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
}
