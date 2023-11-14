#[derive(Debug, Default)]
pub struct Board {
    pub board: [[u8; 9]; 9],
}

impl Board {
    pub fn new() -> Self {
        Self { board: [[0; 9]; 9] }
    }

    pub fn set(&mut self, x: usize, y: usize, value: u8) {
        self.board[x][y] = value;
    }

    pub fn get(&self, x: usize, y: usize) -> u8 {
        self.board[x][y]
    }

    pub fn is_valid(&self, x: usize, y: usize, value: u8) -> bool {
        // check row
        for i in 0..9 {
            if self.board[x][i] == value {
                return false;
            }
        }

        // check column
        for i in 0..9 {
            if self.board[i][y] == value {
                return false;
            }
        }

        // check square
        let x0 = (x / 3) * 3;
        let y0 = (y / 3) * 3;
        for i in x0..x0 + 3 {
            for j in y0..y0 + 3 {
                if self.board[i][j] == value {
                    return false;
                }
            }
        }

        true
    }

    pub fn is_solved(&self) -> bool {
        for i in 0..9 {
            for j in 0..9 {
                if self.board[i][j] == 0 {
                    return false;
                }
            }
        }

        true
    }
}
