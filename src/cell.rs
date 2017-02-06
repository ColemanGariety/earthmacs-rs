#[derive(Clone)]
pub struct Cell {
    pub ch: char,
    pub fg: i32,
}

impl Cell {
    pub fn new(ch: char, fg: i32) -> Cell {
        Cell {
            ch: ch,
            fg: fg,
        }
    }
}
