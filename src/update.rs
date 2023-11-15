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

pub fn update_game(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Char('j') | KeyCode::Down => app.game.select(
            app.game.selected.0,
            app.game.selected.1.checked_sub(1).unwrap_or(8),
        ),
        KeyCode::Char('k') | KeyCode::Up => app
            .game
            .select(app.game.selected.0, (app.game.selected.1 + 1) % 9),
        KeyCode::Char('h') | KeyCode::Left => app.game.select(
            app.game.selected.0.checked_sub(1).unwrap_or(8),
            app.game.selected.1,
        ),
        KeyCode::Char('l') | KeyCode::Right => app
            .game
            .select((app.game.selected.0 + 1) % 9, app.game.selected.1),
        KeyCode::Char('1') => app.game.set(app.game.selected.0, app.game.selected.1, 1),
        KeyCode::Char('2') => app.game.set(app.game.selected.0, app.game.selected.1, 2),
        KeyCode::Char('3') => app.game.set(app.game.selected.0, app.game.selected.1, 3),
        KeyCode::Char('4') => app.game.set(app.game.selected.0, app.game.selected.1, 4),
        KeyCode::Char('5') => app.game.set(app.game.selected.0, app.game.selected.1, 5),
        KeyCode::Char('6') => app.game.set(app.game.selected.0, app.game.selected.1, 6),
        KeyCode::Char('7') => app.game.set(app.game.selected.0, app.game.selected.1, 7),
        KeyCode::Char('8') => app.game.set(app.game.selected.0, app.game.selected.1, 8),
        KeyCode::Char('9') => app.game.set(app.game.selected.0, app.game.selected.1, 9),
        KeyCode::Backspace => app.game.set(app.game.selected.0, app.game.selected.1, 0),
        _ => {}
    }
}

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
