#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Position {
    pub row: u64,
    pub col: u64,
}

impl Position {
    #[must_use]
    pub const fn new(row: u64, col: u64) -> Self {
        Self { row, col }
    }

    #[must_use]
    pub fn direct_neighbours(&self) -> Vec<Self> {
        let min_row = if self.row == 0 { 0 } else { self.row - 1 };
        let min_col = if self.col == 0 { 0 } else { self.col - 1 };
        let mut res: Vec<Self> = Vec::new();
        for row in min_row..=self.row + 1 {
            res.push(Self::new(row, self.col));
        }
        for col in min_col..=self.col + 1 {
            res.push(Self::new(self.row, col));
        }
        res
    }
}
