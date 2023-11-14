use super::board::Board;

#[derive(Debug, Default)]
pub struct Game {
    pub board: Board,
    pub selected: (usize, usize),
    pub game_over: bool,
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
