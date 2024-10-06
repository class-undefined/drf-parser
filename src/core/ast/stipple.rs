pub struct Stipple {
    pub name: String,
    pub bit_map: Vec<Vec<u8>>,
    pub width: usize,
    pub height: usize,
}

impl Stipple {
    pub fn new(name: String, bit_map: Vec<Vec<u8>>, width: usize, height: usize) -> Self {
        Stipple {
            name,
            bit_map,
            width,
            height,
        }
    }
}
