use std::fmt;

#[derive(Clone, Debug)]
pub struct CandidateSet(pub u16);

impl CandidateSet {
    pub fn new() -> Self {
        // Initialize with bits 1-9 set: 0b111111111 (511 in decimal)
        Self(0x1FF)
    }

    pub fn single_value(value: u8) -> Self {
        assert!((1..=9).contains(&value), "[SingleValue] Value must be between 1 and 9");
        // Set only one bit at position (value - 1)
        Self(1 << (value - 1))
    }

    pub fn checksum() -> u8 {
        // (1..=9).sum()
        45
    }

    pub fn remove(&mut self, value: u8) {
        assert!((1..=9).contains(&value), "[Remove] Value must be between 1 and 9");
        // Clear the bit at position (value - 1)
        self.0 &= !(1 << (value - 1));
    }

    pub fn contains(&self, value: u8) -> bool {
        assert!((1..=9).contains(&value), "[Contains] Value must be between 1 and 9");
        // Check if bit at position (value - 1) is set
        (self.0 & (1 << (value - 1))) != 0
    }

    pub fn len(&self) -> usize {
        // Count number of set bits
        self.0.count_ones() as usize
    }

    pub fn is_single(&self) -> bool {
        // Check if exactly one bit is set
        self.0.count_ones() == 1
    }

    pub fn value(&self) -> Option<u8> {
        if self.is_single() {
            // If only one bit is set, find its position and convert to Sudoku value
            Some(self.0.trailing_zeros() as u8 + 1)
        } else {
            None
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = u8> + '_ {
        // Create iterator over set bits
        (1..=9).filter(move |&i| self.contains(i))
    }

    pub fn has_values(&self) -> bool {
        self.0.count_ones() > 0
    }
}

// Pretty printing for demonstration
impl fmt::Display for CandidateSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        let values: Vec<_> = self.iter().collect();
        if !values.is_empty() {
            write!(f, "{}", values[0])?;
            for &v in &values[1..] {
                write!(f, ",{}", v)?;
            }
        }
        write!(f, "]")
    }
}
