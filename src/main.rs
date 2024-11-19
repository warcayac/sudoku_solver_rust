mod errors;
mod candidate_set;
mod board_info;
mod classic_board;

use candidate_set::CandidateSet;
use classic_board::ClassicBoard;
use wshared::types::TVoidResult;


#[allow(dead_code)]
fn testing_candidate_set() {
    println!("=== CandidateSet Example Usage ===\n");

    // Example 1: Creating a new set with all possibilities
    let mut cell = CandidateSet::new();
    println!("New cell with all candidates: {}", cell);
    println!("Binary representation: {:09b}", cell.0);
    println!("Number of candidates: {}\n", cell.len());

    // Example 2: Creating a cell with a single value
    let single = CandidateSet::single_value(5);
    println!("Cell with single value 5: {}", single);
    println!("Binary representation: {:09b}", single.0);
    println!("Is single? {}", single.is_single());
    println!("Value: {:?}\n", single.value());

    // Example 3: Removing candidates
    println!("Starting with fresh cell: {}", cell);
    // Remove candidates 2, 4, 6, 8
    for value in [2, 4, 6, 8] {
        cell.remove(value);
        println!("After removing {}: {}", value, cell);
        println!("Binary: {:09b}", cell.0);
    }
    println!();

    // Example 4: Checking candidates
    println!("Checking remaining candidates:");
    for value in 1..=9 {
        println!("Contains {}: {}", value, cell.contains(value));
    }
    println!();

    // Example 5: Iterating over candidates
    println!("Iterating over candidates:");
    for value in cell.iter() {
        println!("Found candidate: {}", value);
    }
    println!();

    // Example 6: Practical Sudoku cell elimination
    println!("Practical example - solving a cell:");
    let mut solving_cell = CandidateSet::new();
    println!("Initial cell: {}", solving_cell);
    
    println!("\nEliminating candidates based on row/column/box constraints...");
    for value in [1, 3, 5, 6, 7, 8, 9] {
        solving_cell.remove(value);
        println!("After removing {}: {} (binary: {:09b})", value, solving_cell, solving_cell.0);
    }
    
    println!("\nFinal cell state:");
    println!("Candidates: {}", solving_cell);
    println!("Is single value: {}", solving_cell.is_single());
    println!("Value: {:?}", solving_cell.value());
}

#[allow(dead_code)]
fn solve_one_board() -> TVoidResult {
    let mut board = ClassicBoard::new(
        "8...7.....753.8...6.9.1.........14.8...........7...3.224....5..9..4.76......36...", 
        false,
    )?;
    board.display();
    board.solve()?;

    Ok(())
}

fn main() -> TVoidResult {
    // testing_candidate_set();
    // solve_one_board()?;
    let boards = [
        "boards/easy50.txt",
        "boards/hardest.txt",
        "boards/hardest(2019).txt",
        "boards/latimes_expert.txt",
        "boards/top95.txt",
    ];
    ClassicBoard::solve_from_file(boards[1])?;

    Ok(())
}
