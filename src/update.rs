use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{app::App, sudoku::Screen};

pub fn update(app: &mut App, key_event: KeyEvent) {
    match app.screen {
        Screen::Game => update_game(app, key_event),
        Screen::Menu(_) => update_menu(app, key_event),
        Screen::GameOver(_) => update_game_over(app, key_event),
    }
    match key_event.code {
        KeyCode::Esc | KeyCode::Char('q') => app.quit(),
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit()
            }
        }
        _ => {}
    };
}

pub fn update_game(app: &mut App, key_event: KeyEvent) {}

pub fn update_menu(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Char('j') | KeyCode::Down => {
            app.update_menu_selection(crate::app::MenuAction::MoveDown)
        }
        KeyCode::Char('k') | KeyCode::Up => {
            app.update_menu_selection(crate::app::MenuAction::MoveUp)
        }
        KeyCode::Enter => app.update_menu_selection(crate::app::MenuAction::Select),
        _ => {}
    }
}

pub fn update_game_over(app: &mut App, key_event: KeyEvent) {}
