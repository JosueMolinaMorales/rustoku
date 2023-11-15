use super::generator;

#[derive(Debug)]
pub struct Board {
    pub board: [[u8; 9]; 9],
    pub original_board: [[u8; 9]; 9],
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl Board {
    pub fn new() -> Self {
        let board = generator::get_new_sudoku();
        Self {
            board,
            original_board: board.clone(),
        }
    }

    pub fn set(&mut self, x: usize, y: usize, value: u8) -> bool {
        // Check to make sure they are not changing a cell that was originally filled
        if self.original_board[x][y] != 0 {
            return false;
        }

        self.board[x][y] = value;

        // Check if the board is solved
        return self.is_board_solved();
    }

    pub fn get(&self, x: usize, y: usize) -> u8 {
        self.board[x][y]
    }

    pub fn is_safe(&self, row: usize, col: usize, num: u8) -> bool {
        // Check if 'num' is not present in the current row and column
        for i in 0..9 {
            if self.board[row][i] == num || self.board[i][col] == num {
                return false;
            }
        }

        // Check if 'num' is not present in the current 3x3 subgrid
        let start_row = row - row % 3;
        let start_col = col - col % 3;
        for i in 0..3 {
            for j in 0..3 {
                if self.board[start_row + i][start_col + j] == num {
                    return false;
                }
            }
        }

        true
    }

    pub fn is_board_solved(&self) -> bool {
        for row in 0..9 {
            for col in 0..9 {
                if self.board[row][col] == 0 {
                    return false;
                }
            }
        }
        true
    }

    pub fn is_solution_valid(&self) -> bool {
        if !self.is_board_solved() {
            return false;
        }
        for row in 0..9 {
            for col in 0..9 {
                if !self.is_safe(row, col, self.board[row][col]) {
                    return false;
                }
            }
        }
        true
    }
}
