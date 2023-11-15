use crate::sudoku;

pub const MENU_SELECTION: [sudoku::Selection; 3] = [
    sudoku::Selection::NewGame,
    sudoku::Selection::Continue,
    sudoku::Selection::Quit,
];

pub enum MenuAction {
    Select,
    MoveUp,
    MoveDown,
}

/// Application.
#[derive(Debug, Default)]
pub struct App {
    /// should the application exit?
    pub should_quit: bool,
    pub game: sudoku::Game,
    pub screen: sudoku::Screen,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn update_menu_selection(&mut self, action: MenuAction) {
        let selection = match &self.screen {
            sudoku::Screen::Menu(selection) => selection,
            _ => return,
        };
        let index = MENU_SELECTION
            .iter()
            .position(|&s| s == *selection)
            .unwrap();
        match action {
            MenuAction::MoveUp => {
                self.screen = sudoku::Screen::Menu(MENU_SELECTION[(index + 1) % 3]);
            }
            MenuAction::MoveDown => {
                self.screen = sudoku::Screen::Menu(MENU_SELECTION[(index + 2) % 3]);
            }
            MenuAction::Select => {
                self.screen = match *selection {
                    sudoku::Selection::NewGame => sudoku::Screen::Game,
                    sudoku::Selection::Continue => sudoku::Screen::Game,
                    sudoku::Selection::Quit => sudoku::Screen::GameOver(false),
                }
            }
        }
    }
}
