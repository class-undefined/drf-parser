pub struct Packet {
    pub name: String,
    pub stipple: String,
    pub color: String,
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
            color: String::new(),
            line_style: String::new(),
            fill: String::new(),
            outline: String::new(),
            fill_style: None,
        }
    }
}
