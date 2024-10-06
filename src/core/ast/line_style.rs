#[derive(Debug, Clone)]
pub struct LineStyle {
    pub name: String,
    pub width: u32,
    pub pattern: Vec<u8>,
}

impl LineStyle {
    pub fn new(name: String, width: u32, pattern: Vec<u8>) -> Self {
        LineStyle {
            name,
            width,
            pattern,
        }
    }

    pub fn from_vec(params: &Vec<String>, pattern: Vec<u8>) -> Self {
        LineStyle {
            name: params[1].clone(),
            width: params[2].parse::<u32>().unwrap(),
            pattern,
        }
    }
}
