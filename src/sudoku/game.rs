use super::board::Board;

#[derive(Debug, Default)]
pub struct Game {
    board: Board,
    pub selected: (usize, usize),
    pub game_over: bool,
}

impl Game {
    pub fn continue_game() -> Self {
        todo!()
    }

    pub fn is_game_over(&self) -> bool {
        self.game_over
    }

    pub fn is_selected(&self, row: usize, col: usize) -> bool {
        self.selected == (row, col)
    }

    pub fn select(&mut self, row: usize, col: usize) {
        self.selected = (row, col);
    }

    pub fn set(&mut self, row: usize, col: usize, value: u8) {
        let is_solved = self.board.set(row, col, value);

        if is_solved && self.board.is_solution_valid() {
            self.game_over = true;
        }
    }

    pub fn get(&self, row: usize, col: usize) -> u8 {
        self.board.get(row, col)
    }
}

#[derive(Debug)]
pub enum Screen {
    /// Main menu.
    Menu(Selection),
    /// Game screen.
    Game,
    /// Game over screen.
    /// if true, the player won, otherwise lost.
    GameOver(bool),
}

impl Default for Screen {
    fn default() -> Self {
        Self::Menu(Selection::default())
    }
}

#[derive(Debug, Default, PartialEq, Copy, Clone, Eq, PartialOrd, Ord)]
pub enum Selection {
    #[default]
    NewGame,
    Continue,
    Quit,
}
