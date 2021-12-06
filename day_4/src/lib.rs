#![allow(dead_code, unused_variables)]

use std::{collections::HashMap, vec};

/// Evaluates the bingo boards and returns the score for the winning and loosing bingo boards. This
/// accomplishes both part #1 and part #2 of this problem.
pub fn evaluate_bingo_boards(values: Vec<usize>, boards: Vec<Board>) -> (usize, usize) {
    let mut winning_board = None;
    let mut winning_board_round = 0;
    let mut winning_board_num = 0;
    let mut loosing_board = None;
    let mut loosing_board_round = 0;
    let mut loosing_board_num = 0;

    // Evaluate the Bingo values for each board in order, keeping track of which
    // one wins first. We update the variables declared above if we find a winning
    // board that beats the existing winning board.
    'boards: for mut board in boards {
        for (round, value) in values.iter().enumerate() {
            if board.visit_value(*value) {
                // Determine both the winning and the loosing boards by comparing the
                // round that the boards won on. If the round is lower than the current
                // winning boards number of rounds to win, then this board becomes the
                // new winning board. If this round is higher than the current loosing
                // board, then this becomes the new loosing board.
                if winning_board.is_none() || round <= winning_board_round {
                    winning_board_num = *value;
                    winning_board = Some(board.clone());
                    winning_board_round = round;
                } else if round > loosing_board_round {
                    loosing_board_num = *value;
                    loosing_board = Some(board.clone());
                    loosing_board_round = round;
                }
                continue 'boards;
            }
        }
    }

    // Get the sum of all un-visited cells on the board and take that times the value
    // that won the winning board's round.
    (
        winning_board_num * winning_board.unwrap().sum_unvisited(),
        loosing_board_num * loosing_board.unwrap().sum_unvisited(),
    )
}

/// Contains a value on the Bingo board and provides a wrapper around a value that can
/// maintain the state of the cell, such as whether the cell has been visited or not.
/// This state is needed to know whether there has been a bingo.
#[derive(Clone)]
struct Cell {
    visited: bool,
    value: usize,
}

impl Cell {
    /// Creates a new instance of the cell with the provided value
    fn new(v: usize) -> Self {
        Self {
            visited: false,
            value: v,
        }
    }

    /// Sets the cell as visited
    fn set_visited(&mut self) {
        self.visited = true
    }
}

/// A 5x5 Bingo board of usize values. The struct stores a 2d matrix where the first
/// dimension is the rows and the second dimension is the columns.
#[derive(Clone)]
pub struct Board {
    // Contains the actual 5x5 grid
    grid: [[Cell; 5]; 5],
    // Maps values to vectors of row/col pairs. There could be the same value in multiple
    // cells, hence the need for the vector. Having this lookup allows us to easily get the
    // board coordinates for a specific value.
    lookup: HashMap<usize, Vec<(usize, usize)>>,
    // Tracks the count of visited cells in all the rows
    visited_count_by_row: [usize; 5],
    // Tracks the count of visited cells in all the columns
    visited_count_by_col: [usize; 5],
}

impl Board {
    /// Creates a new [`Board`] from a given reader. The reader should contain a grid-like
    /// text representation of the 5x5 board like the following. Improperly formatting inputs
    /// will panic.
    ///
    /// ## Example
    /// ```
    /// # use day_4::Board;
    /// let board = Board::from_str(
    ///    "22 13 17 11  0
    ///    8  2 23  4 24
    ///   21  9 14 16  7
    ///    6 10  3 18  5
    ///    1 12 20 15 19"
    /// );
    /// ```
    pub fn from_str(input: &str) -> Self {
        let mut board = Self::default();
        input.lines().into_iter().enumerate().for_each(|(row, l)| {
            l.split_whitespace()
                .filter(|v| v.len() > 0)
                .into_iter()
                .map(|v| v.parse::<usize>().expect("expected usize value"))
                .enumerate()
                .for_each(|(col, v)| board.set_cell_idx(row, col, v));
        });
        board
    }

    /// Sets the value at the specified row and column index. Index is zero-based so row 0, col 0 is
    /// actually the first row and column. This function is useful for initializing the board within
    /// a for-loop.
    fn set_cell_idx(&mut self, row: usize, col: usize, val: usize) {
        self.grid[row][col] = Cell::new(val);

        // Update the lookup with the new value
        match self.lookup.get(&val) {
            Some(v) => {
                let mut v = v.clone();
                v.push((row, col));
                self.lookup.insert(val, v);
            }
            None => {
                self.lookup.insert(val, vec![(row, col)]);
            }
        };
    }

    /// Gets the value at the specified row and column. The rows and columns are not zero-based so row 1, col 1
    /// translates to the `self.grid[0][0]`. Thing about rows and columns as themselves, not indexes.
    fn get_cell(&self, row: usize, col: usize) -> Cell {
        self.grid[row - 1][col - 1].clone()
    }

    /// Gets the value at the specified row and column. The rows and columns are not zero-based so row 1, col 1
    /// translates to the `self.grid[0][0]`. Thing about rows and columns as themselves, not indexes.
    fn get_cell_value(&self, row: usize, col: usize) -> usize {
        self.grid[row - 1][col - 1].value
    }

    /// Marks a given cell as 'visited' using indexes. This function uses grid indexes which are zero-based which
    /// means that row_idx 1, col_idx 1 actualls means row 2, col 2.
    fn visit_cell_idx(&mut self, row_idx: usize, col_idx: usize) {
        self.grid[row_idx][col_idx].set_visited();
        self.visited_count_by_col[col_idx] += 1;
        self.visited_count_by_row[row_idx] += 1;
    }

    /// Marks any matching values on the Bingo board as 'visited' and returns a boolean representing whether the
    /// board has a bingo.
    fn visit_value(&mut self, val: usize) -> bool {
        match self.lookup.get(&val) {
            Some(v) => {
                for (row, col) in v.clone() {
                    self.visit_cell_idx(row, col)
                }
                self.has_bingo()
            }
            None => false,
        }
    }

    /// Returns true if the board currently has a vertical or horizontal bingo. A bingo is defined by any row or
    /// column where all cells have been visited.
    fn has_bingo(&self) -> bool {
        self.visited_count_by_row.contains(&5) || self.visited_count_by_col.contains(&5)
    }

    /// Returns the sum of all unvisted rows. We could probably accomplish this with a custom iterator
    /// implementation for [`Board`] in the future but this is the most simple solution for now.
    fn sum_unvisited(&self) -> usize {
        self.grid
            .iter()
            .flatten()
            .filter(|v| !v.visited)
            .fold(0, |s, v| s + v.value)
    }
}

impl Default for Board {
    /// Creates a new zeroed instance of [`Board`]
    fn default() -> Self {
        let z = || -> Cell { Cell::new(0) };
        Self {
            visited_count_by_col: [0, 0, 0, 0, 0],
            visited_count_by_row: [0, 0, 0, 0, 0],
            lookup: HashMap::new(),
            grid: [
                [z(), z(), z(), z(), z()],
                [z(), z(), z(), z(), z()],
                [z(), z(), z(), z(), z()],
                [z(), z(), z(), z(), z()],
                [z(), z(), z(), z(), z()],
            ],
        }
    }
}

/// Parses the input.txt file and returns a vector of Bingo values and a vector of Bingo boards
/// that were parsed from the file. This function could potentially fail and should only be used
/// in controlled environments such as tests.
pub fn parse_input() -> (Vec<usize>, Vec<Board>) {
    let input = include_str!("input.txt");

    // Parse the line containing the values
    let value_input = input
        .lines()
        .next()
        .expect("expected first line to contain values");
    let values = value_input
        .split(",")
        .map(|v| v.parse::<usize>().expect("expected usize"))
        .collect::<Vec<usize>>();

    // Parse the lines containing the boards
    (
        values,
        input
            .split("\n\n")
            .skip(1)
            .into_iter()
            .map(|v| Board::from_str(v))
            .collect::<Vec<Board>>(),
    )
}

#[cfg(test)]
fn get_test_board() -> Board {
    Board::from_str(
        "22 13 17 11  0
    8  2 23  4 24
   21  9 14 16  7
    6 10  3 18  5
    1 12 20 15 19",
    )
}

#[test]
fn test_board_from_str() {
    let board = get_test_board();
    assert_eq!(18, board.get_cell_value(4, 4));
    assert_eq!(25, board.lookup.len())
}

#[test]
fn test_board_visit_value() {
    let mut board = get_test_board();
    board.visit_value(18);
    assert!(board.get_cell(4, 4).visited)
}

#[test]
fn test_board_has_bingo_horizontal() {
    let mut board = get_test_board();
    assert_eq!(false, board.has_bingo());
    board.visit_value(8);
    board.visit_value(2);
    board.visit_value(23);
    board.visit_value(4);
    board.visit_value(24);
    assert_eq!(true, board.has_bingo());
}

#[test]
fn test_board_has_bingo_vertical() {
    let mut board = get_test_board();
    assert_eq!(false, board.has_bingo());
    board.visit_value(11);
    board.visit_value(4);
    board.visit_value(16);
    board.visit_value(18);
    board.visit_value(15);
    assert_eq!(true, board.has_bingo());
}

#[test]
fn test_parse_input() {
    let input = parse_input();
    let (winning, loosing) = evaluate_bingo_boards(input.0, input.1);
    assert_eq!(8136, winning);
    assert_eq!(12738, loosing);
}
