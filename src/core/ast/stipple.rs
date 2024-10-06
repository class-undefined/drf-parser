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
}
