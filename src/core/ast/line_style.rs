pub struct LineStyle {
    pub name: String,
    pub width: f32,
    pub pattern: Vec<u8>,
}

impl LineStyle {
    pub fn new(name: String, width: f32, pattern: Vec<u8>) -> Self {
        LineStyle {
            name,
            width,
            pattern,
        }
    }
}
