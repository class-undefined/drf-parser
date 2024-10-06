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
}
