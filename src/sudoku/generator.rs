use rand::{seq::SliceRandom, Rng};

const SIZE: usize = 9;

fn is_safe(board: &[[u8; SIZE]; SIZE], row: usize, col: usize, num: u8) -> bool {
    // Check if 'num' is not present in the current row and column
    for i in 0..SIZE {
        if board[row][i] == num || board[i][col] == num {
            return false;
        }
    }

    // Check if 'num' is not present in the current 3x3 subgrid
    let start_row = row - row % 3;
    let start_col = col - col % 3;
    for i in 0..3 {
        for j in 0..3 {
            if board[start_row + i][start_col + j] == num {
                return false;
            }
        }
    }

    true
}

fn find_empty_location(board: &[[u8; SIZE]; SIZE]) -> Option<(usize, usize)> {
    for i in 0..SIZE {
        for j in 0..SIZE {
            if board[i][j] == 0 {
                return Some((i, j));
            }
        }
    }
    None
}

const NUMBERS: [u8; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
fn sudoku_solver(board: &mut [[u8; SIZE]; SIZE]) -> bool {
    let shuffled_nums = {
        let mut rng = rand::thread_rng();
        let mut nums = NUMBERS.to_vec();
        nums.shuffle(&mut rng);
        nums
    };
    if let Some((row, col)) = find_empty_location(board) {
        for num in shuffled_nums {
            if is_safe(board, row, col, num) {
                board[row][col] = num;

                if sudoku_solver(board) {
                    return true;
                }

                // If placing 'num' in the current cell doesn't lead to a solution, backtrack
                board[row][col] = 0;
            }
        }
        false
    } else {
        // All cells are filled, the puzzle is solved
        true
    }
}

fn sudoku_solver_count(board: &mut [[u8; SIZE]; SIZE], count: &mut u32) {
    let shuffled_nums = {
        let mut rng = rand::thread_rng();
        let mut nums = NUMBERS.to_vec();
        nums.shuffle(&mut rng);
        nums
    };
    if let Some((row, col)) = find_empty_location(board) {
        for num in shuffled_nums {
            if is_safe(board, row, col, num) {
                board[row][col] = num;

                sudoku_solver_count(board, count);

                // If placing 'num' in the current cell doesn't lead to a solution, backtrack
                board[row][col] = 0;
            }
        }
    } else {
        // All cells are filled, the puzzle is solved
        *count += 1;
    }
}

fn print_board(board: &[[u8; SIZE]; SIZE]) {
    for row in board.iter() {
        for &num in row.iter() {
            print!("{} ", num);
        }
        println!();
    }
}

pub fn get_new_sudoku() -> [[u8; SIZE]; SIZE] {
    let mut sudoku_board = [
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
    ];

    if sudoku_solver(&mut sudoku_board) {
        println!("Sudoku Solution:");
        print_board(&sudoku_board);
    } else {
        println!("No solution exists.");
    }

    // Randomly remove digits to create the puzzle
    let mut attempts = 5;
    while attempts > 0 {
        // Select a random cell that is not already empty
        let mut row = rand::thread_rng().gen_range(0..SIZE);
        let mut col = rand::thread_rng().gen_range(0..SIZE);
        while sudoku_board[row][col] == 0 {
            row = rand::thread_rng().gen_range(0..SIZE);
            col = rand::thread_rng().gen_range(0..SIZE);
        }

        // Remember its cell value in case we need to put it back
        let backup = sudoku_board[row][col];
        sudoku_board[row][col] = 0;

        // Copy the grid so we can solve it
        let mut copy = sudoku_board.clone();

        // Count the number of solutions that this grid has
        let mut count = 0;
        sudoku_solver_count(&mut copy, &mut count);

        // If the number of solution is different from 1 then we need to
        // cancel the change by putting the value we took away back in the grid
        if count != 1 {
            sudoku_board[row][col] = backup;
            // We could stop here, but we can also have another attempt with
            // a different cell just to try to remove more numbers
            attempts -= 1;
        }
    }

    println!("Sudoku Puzzle:");
    print_board(&sudoku_board);

    return sudoku_board;
}
