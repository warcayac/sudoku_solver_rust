pub struct BoardInfo {
    pub side: usize,
    pub box_side: usize,
    pub minimum_entries: usize,
    pub pattern: String,
}

impl BoardInfo {
    pub fn new_9x9() -> Self {
        Self {
            side: 9, // size of the board
            box_side: 3, // size of a quadrant
            minimum_entries: 17, // minimum number of entries for the board to have an unique solution
            pattern: "_123456789.".to_string(),
        }
    }

    pub fn dimmension(&self) -> usize {
        self.side * self.side
    }
}