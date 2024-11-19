#[derive(Debug)]
pub enum Sector {
    Row,
    Column,
    Quadrant,
    // Cell,
}

#[derive(Debug, thiserror::Error)]
pub enum BoardError {
    /// The board format is invalid.
    #[error("Invalid format")]
    InvalidFormat,
    /// The board does not have enough entries.
    #[error("Not enough entries")]
    NotEnoughEntries,
    /// A cell at the specified row and column is invalid.
    #[error("Invalid cell at row {row}, col {col}, from: {from:?}")]
    InvalidCell {row:usize, col:usize, from: Sector},
    /// An invalid assignment was attempted: empty cell or with one unique value.
    #[error("Invalid assignment at row {row}, col {col}, value: {value}")]
    InvalidAssignment {row:usize, col:usize, value: u8},
    /// No solution was found for board
    #[error("No solution found")]
    NoSolutionFound,
    /// The board is not valid
    #[error("Invalid board")]
    InvalidBoard,
    #[error(transparent)]
    InvalidInputFile(#[from] std::io::Error),
}
