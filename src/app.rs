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
        match action {
            MenuAction::Select => match selection {
                sudoku::Selection::NewGame => {
                    self.screen = sudoku::Screen::Game;
                }
                sudoku::Selection::Continue => {}
                sudoku::Selection::Quit => {
                    self.should_quit = true;
                }
            },
            MenuAction::MoveUp => match selection {
                sudoku::Selection::NewGame => {
                    self.screen = sudoku::Screen::Menu(sudoku::Selection::Quit);
                }
                sudoku::Selection::Continue => {
                    self.screen = sudoku::Screen::Menu(sudoku::Selection::NewGame);
                }
                sudoku::Selection::Quit => {
                    self.screen = sudoku::Screen::Menu(sudoku::Selection::Continue);
                }
            },
            MenuAction::MoveDown => match selection {
                sudoku::Selection::NewGame => {
                    self.screen = sudoku::Screen::Menu(sudoku::Selection::Continue);
                }
                sudoku::Selection::Continue => {
                    self.screen = sudoku::Screen::Menu(sudoku::Selection::Quit);
                }
                sudoku::Selection::Quit => {
                    self.screen = sudoku::Screen::Menu(sudoku::Selection::NewGame);
                }
            },
        }
    }
}
