use std::{collections::HashMap, fs::File, io::{BufRead, BufReader}};
use ndarray::{s, Array2, ArrayViewMut2, Axis, Dim, SliceInfo, SliceInfoElem};

use crate::{board_info::BoardInfo, candidate_set::CandidateSet, errors::*};

type TQuadrants = Array2<SliceInfo<[SliceInfoElem; 2], Dim<[usize; 2]>, Dim<[usize; 2]>>>;
type TBoardResult<T> = Result<T, BoardError>;
type TPosition = (usize, usize); // (row, col)
type TSinglesMap = HashMap<TPosition, u8>;
type TPositionValue<T> = (TPosition, T);
type TCountMap = HashMap<u8, u8>;


pub struct ClassicBoard {
    grid : Array2<CandidateSet>,
    quadrants : TQuadrants,
    info : BoardInfo,
    verbose: bool,
}

impl ClassicBoard {
    pub fn new(input: &str, verbose: bool) -> TBoardResult<Self> {
        let info = BoardInfo::new_9x9();

        // Validate input more efficiently
        if input.len() != info.dimmension() || !input.chars().all(|c| info.pattern.contains(c)) {
            return Err(BoardError::InvalidFormat);
        }
        
        let digit_count = input.chars().filter(|c| c.is_ascii_digit()).count();
        if digit_count < info.minimum_entries {
            return Err(BoardError::NotEnoughEntries);
        }

        // Initialize board with all candidates
        let grid = Array2::from_elem((info.side, info.side), CandidateSet::new());
        // Pre-calculate quadrant slices
        let quadrants = Array2::from_shape_fn((info.box_side, info.box_side), |(i, j)| {
            s![i*info.box_side..(i+1)*info.box_side, j*info.box_side..(j+1)*info.box_side]
        });
        // Create the board instance
        let mut board = Self { grid, quadrants, info, verbose };
                
        // Process input and assign initial values
        for (idx, ch) in input.chars().enumerate() {
            if let Some(digit) = ch.to_digit(10) {
                let (row, col) = (idx / board.info.side, idx % board.info.side);
                board.assign(row, col, digit as u8)?;
            }
        }

        Ok(board)
    }

    pub fn display(&self) {
        println!("┌─┬─┬─┬─┬─┬─┬─┬─┬─┐");
        for row in 0..self.info.side {
            for col in 0..self.info.side {
                let cell = &self.grid[(row, col)];
                let value = cell.value().map(|x| (x + b'0') as char).unwrap_or(' ');
                let sep =  if col % self.info.box_side == 0 { '|' } else { ' ' };
                print!("{}{}", sep, value);
            }
            println!("|");
            if (row + 1) % self.info.box_side == 0 && row < (self.info.side - 1) {
                println!("├─┼─┼─┼─┼─┼─┼─┼─┼─┤");
            }
        }
        println!("└─┴─┴─┴─┴─┴─┴─┴─┴─┘");
    }

    /// (row, col) is a position respect to quadrants formed in "grid" matrix,
    /// so the range for every value of position is 0..2
    #[inline]
    fn quadrant_mut(&mut self, row: usize, col: usize) -> ArrayViewMut2<CandidateSet> {
        self.grid.slice_mut(self.quadrants[(row, col)])
    }

    // #[inline]
    // fn quadrant(&self, row: usize, col: usize) -> ArrayView2<CandidateSet> {
    //     self.grid.slice(self.quadrants[(row, col)])
    // }

    #[inline]
    fn assign(&mut self, row: usize, col: usize, value: u8) -> TBoardResult<()> {
        if !self.grid[(row, col)].contains(value) {
            return Err(BoardError::InvalidAssignment { row, col, value });
        }

        self.grid[(row, col)] = CandidateSet::single_value(value);
        self.remove_candidate(row, col, value)
    }

    #[inline]
    fn remove_candidate(&mut self, row: usize, col: usize, value: u8) -> TBoardResult<()> {
        let mut singles: TSinglesMap = HashMap::new();
        let mut check_and_remove = |cell: &mut CandidateSet, r: usize, c: usize, source: Sector| 
            -> TBoardResult<()> 
        {
            if cell.len() > 1 { 
                cell.remove(value);
                if !cell.has_values() {
                    return Err(BoardError::InvalidCell { row: r, col: c, from: source });
                }
                if let Some(x) = cell.value() {
                    // println!("==> Unique value {} at row {}, col {}. Source: {:?}", x, r, c, source);
                    singles.insert((r, c), x);
                }
            } else if r != row && c != col && cell.value().unwrap() == value {
                return Err(BoardError::InvalidCell { row, col, from: source });
            }

            Ok(())
        };
        
        // Remove from row
        for (c, cell) in self.grid.row_mut(row).iter_mut().enumerate() {
            check_and_remove(cell, row, c, Sector::Row)?;
        }
        // Remove from column
        for (r, cell) in self.grid.column_mut(col).iter_mut().enumerate() {
            check_and_remove(cell, r, col, Sector::Column)?;
        }
        // Remove from quadrant
        let bs = self.info.box_side;
        let (qrow, qcol) = (row/bs, col/bs);
        for (i, cell) in self.quadrant_mut(qrow, qcol).iter_mut().enumerate() {
            // this convert any (row,col) to first point (row0,col0) starting the quadrant in terms of grid
            let (row0, col0) = (qrow*bs, qcol*bs);
            let (rx, cx) = (row0 + i / bs, col0 + i % bs);
            check_and_remove(cell, rx, cx, Sector::Quadrant)?;
        }
        // Remove unique values generated by the above removals
        // if !uniques.is_empty() { println!("\n==> Remove unique values: {uniques:?}"); }
        for (r, c) in singles.keys() {
            self.println(format_args!("==> Removing unique value {} at ({}, {}) from surrounding sectors", singles[&(*r, *c)], *r, *c));
            self.remove_candidate(*r, *c, singles[&(*r, *c)])?;
            // println!("==> Current Grid: {:?}", self.grid);
        }
 
        Ok(())
    }

    fn find_solution(&mut self, assignments: &mut Vec<TPositionValue<u8>>) -> TBoardResult<()> {
        // Working with one-frequencies by sector: row, column, quadrant
        loop {
            // println!("==> Current Grid: {:?}", self.grid);
            let unit_freqs = self.find_unit_frequencies()?;
            self.println(format_args!("\n==> Found one-frequency cells: {unit_freqs:?}"));
            if unit_freqs.is_empty() { break; }

            for ((row, col), value) in unit_freqs {
                self.println(format_args!("==> Assigning {} at ({}, {})", value, row, col));
                self.assign(row, col, value)?;
                // println!("==> Current Grid: {:?}", self.grid);
            }
        }

        if self.is_solved() {
            if self.is_valid() {
                self.println(format_args!("\n"));
                // println!("==> Board is SOLVED!!!");
                self.display();
                if !assignments.is_empty() {
                    println!("==> Assignments used for backtracking:\n{assignments:?}");
                }
                return Ok(()); 
            } else {
                return Err(BoardError::InvalidBoard);
            }
        }

        let smallest_cell = self.find_min_candidates();
        let old_grid = self.grid.clone();
        let candidates = self.grid[smallest_cell.0].clone();
        let (row, col) = smallest_cell.0;
        self.println(format_args!("\n==> Found cell with the shortest length at ({}, {}) -> {}", row, col, candidates));
        self.display_if();

        for value in candidates.iter() {
            assignments.push(((row, col), value));
            self.println(format_args!("==> Starting backtracking..."));
            self.println(format_args!("==> Assigning {} at ({}, {})", value, row, col));
            self.println(format_args!("==> Current assignments: {assignments:?}\n"));
            
            if self.assign(row, col, value).is_ok() && self.find_solution(assignments).is_ok() { return Ok(()); }
            
            self.println(format_args!("\n==> Backtracking FAILED with ({},{}) = {}, restoring previous state", row, col, value));
            assignments.pop();
            self.grid = old_grid.clone();
            // self.display();
            self.println(format_args!("==> Current assignments: {assignments:?}\n"));
        }

        Err(BoardError::NoSolutionFound)
    }

    fn find_unit_frequencies(&self) -> TBoardResult<TSinglesMap> {
        let mut singles: TSinglesMap = HashMap::new();
        let get_ones = |freqs: &HashMap<u8, u8>| freqs
            .iter()
            .filter(|(_, v)| **v == 1)
            .map(|(k, _)| *k)
            .collect::<Vec<_>>()
        ;

        // Search for one-frequencies by row
        for (r, row) in self.grid.axis_iter(Axis(0)).enumerate() {
            let freqs = self.count_frequencies(row.iter());
            for value in get_ones(&freqs) {
                for (c, cell) in row.iter().enumerate() {
                    if cell.contains(value) {
                        singles.entry((r, c)).or_insert(value);
                        break;
                    }
                }
            }
        }
        // Search for one-frequencies by column
        for (c, col) in self.grid.axis_iter(Axis(1)).enumerate() {
            let freqs = self.count_frequencies(col.iter());
            for value in get_ones(&freqs) {
                for (r, cell) in col.iter().enumerate() {
                    if cell.contains(value) {
                        singles.entry((r, c)).or_insert(value);
                        break;
                    }
                }
            }
        }
        // Search for one-frequencies by quadrant
        let bs = self.info.box_side;
        for ((qr,qc), quadrant) in self.quadrants.indexed_iter().map(|(p, info)| (p, self.grid.slice(info))) {
            let freqs = self.count_frequencies(quadrant.iter());
            for value in get_ones(&freqs) {
                for ((qrx, qcx), cell) in quadrant.indexed_iter() {
                    if cell.contains(value) {
                        let (row, col) = (qr*bs + qrx, qc*bs + qcx);
                        singles.entry((row, col)).or_insert(value);
                        break;
                    }
                }
            }
        }

        Ok(singles)
    }

    fn count_frequencies<'a>(&'a self, iter: impl Iterator<Item = &'a CandidateSet>) -> TCountMap {
        let mut freqs: TCountMap = HashMap::new();
        for cell in iter {
            if cell.len() == 1 { continue; }
            cell.iter().for_each(|x| { freqs.entry(x).and_modify(|e| *e += 1).or_insert(1); });
        }
        freqs
    }

    fn check_sector_integrity<'a>(&'a self, iter: impl Iterator<Item = &'a CandidateSet>) -> bool {
        iter.fold(0, |acc, x| acc + x.value().unwrap()) == CandidateSet::checksum()
    }

    fn is_solved(&self) -> bool {
        self.grid.iter().all(|cell| cell.is_single())
    }
        
    fn is_valid(&self) -> bool {
        for row in self.grid.rows() {
            if !self.check_sector_integrity(row.iter()) { return false; }
        }
        for col in self.grid.columns() {
            if !self.check_sector_integrity(col.iter()) { return false; }
        }
        for quadrant in self.quadrants.iter().map(|info| self.grid.slice(info)) {
            if !self.check_sector_integrity(quadrant.iter()) { return false; };
        }

        true
    }

    fn find_min_candidates(&self) -> TPositionValue<usize> {
        let mut min_candidates = ((0,0), self.info.side);

        for ((row, col), cell) in self.grid.indexed_iter() {
            if (2..min_candidates.1).contains(&cell.len()) {
                min_candidates = ((row, col), cell.len());
                if min_candidates.1 == 2 { break; }
            }
        }
        
        min_candidates
    }

    fn println(&self, msg: std::fmt::Arguments) {
        if self.verbose {
            println!("{}", msg);
        }
    }

    fn display_if(&self) {
        if self.verbose {
            self.display();
        }
    }

    pub fn solve(&mut self) -> TBoardResult<()> {
        self.find_solution(&mut vec![])
    }

    pub fn solve_from_file(filename: &str) -> TBoardResult<()> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);

        for (i, line) in reader.lines().enumerate() {
            match line {
                Ok(line) => {
                    println!("🮱 Processing board [{}]: {}", i+1, line);
                    match ClassicBoard::new(&line, false) {
                        Ok(mut board) => {
                            if board.solve().is_err() {
                                println!("  Board is not solvable: {:?}", line);
                            }
                        },
                        Err(e) => println!("  Error parsing line: {}", e),
                    }
                    println!();
                },
                Err(e) => println!("🯀 Error reading line: {}", e),
            }
        }

        Ok(())
    }
}
