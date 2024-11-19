# Sudoku Solver

This is a simple Sudoku solver written in Rust.

## Usage

To run the solver, simply run the following command:

```bash
cargo run
```

The solver will then start solving the Sudoku puzzles from 
- a single string input, or
- a file name.

Every input string represents a Sudoku board and is valid if it satisfies the following conditions:
- a string of 81 characters,
- valid characters are: `1-9`, `.`, `_`
- minimum number of entries (`1-9`) is 17

## Reference

This code is an adaptation of [Helper & Solver for Sudoku](https://github.com/warcayac/python-sudoku-solver)

## Information

This code uses two techniques to solve the Sudoku puzzles:

- Candidate Count or Frequency Analysis, and
- Backtracking

## Example

This is the ouput of the solver when solving the board from a file:

```
🮱 Processing board [95]: 3...8.......7....51..............36...2..4....7...........6.13..452...........8..
┌─┬─┬─┬─┬─┬─┬─┬─┬─┐
|3 5 4|1 8 6|9 2 7|
|2 9 8|7 4 3|6 1 5|
|1 6 7|9 5 2|4 8 3|
├─┼─┼─┼─┼─┼─┼─┼─┼─┤
|4 8 1|5 2 7|3 6 9|
|9 3 2|6 1 4|5 7 8|
|5 7 6|3 9 8|2 4 1|
├─┼─┼─┼─┼─┼─┼─┼─┼─┤
|7 2 9|8 6 5|1 3 4|
|8 4 5|2 3 1|7 9 6|
|6 1 3|4 7 9|8 5 2|
└─┴─┴─┴─┴─┴─┴─┴─┴─┘
==> Assignments used for backtracking:
[((7, 7), 9), ((7, 6), 7), ((4, 6), 5), ((4, 0), 9), ((3, 0), 4), ((1, 0), 2), ((3, 1), 8)]
```
