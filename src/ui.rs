use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    prelude::{Alignment, Frame},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::{
    app::App,
    sudoku::{self, Screen},
};

const TITLE_CARD: &str = r#"
_____           _        _          
|  __ \         | |      | |         
| |__) |   _ ___| |_ ___ | | ___   _ 
|  _  / | | / __| __/ _ \| |/ / | | |
| | \ \ |_| \__ \ || (_) |   <| |_| |
|_|  \_\__,_|___/\__\___/|_|\_\\__,_|
"#;

pub fn render(app: &mut App, f: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(f.size());

    // Create a title.
    let tile_block = Block::default().borders(Borders::ALL).style(
        Style::default()
            .fg(Color::LightGreen)
            .bg(Color::Black)
            .add_modifier(Modifier::BOLD),
    );
    let title = Paragraph::new(Text::styled(
        "Rustoku",
        Style::default().fg(Color::LightGreen),
    ))
    .alignment(Alignment::Center)
    .block(tile_block);

    f.render_widget(title, chunks[0]);

    // Render app content
    match &app.screen {
        Screen::Menu(selection) => render_main_menu(f, chunks[1], &selection),
        Screen::Game => render_game(app, f),
        Screen::GameOver(_) => render_game_over(),
    }

    // Render bottom navigation bar
    let current_navigation_text = vec![
        // The first half of the text
        Span::styled("Main Menu", Style::default().fg(Color::Green)),
        // A white divider bar to separate the two sections
        Span::styled(" | ", Style::default().fg(Color::White)),
        // The section of the text, with hints on what the user is editing
        {
            match app.screen {
                Screen::Menu(_) => Span::styled(
                    "Select an option with (j) and (k) and press (enter) to select",
                    Style::default().fg(Color::Red),
                ),
                Screen::Game => Span::styled(
                    "Use (j) and (k) to move up and down, (h) and (l) to move left and right, and (1-9) to enter a number",
                    Style::default().fg(Color::Red),
                ),
                Screen::GameOver(_) => Span::styled(
                    "Press (q) to quit or (r) to restart",
                    Style::default().fg(Color::Red),
                ),
            }
        },
    ];

    let footer = Paragraph::new(Line::from(current_navigation_text))
        .block(Block::default().borders(Borders::ALL));

    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100)])
        .split(chunks[2]);
    f.render_widget(footer, footer_chunks[0]);
}

fn render_main_menu(f: &mut Frame, area: Rect, selection: &sudoku::Selection) {
    // Create a menu.
    let menu = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title("Menu")
        .title_style(Style::default().fg(Color::LightGreen))
        .style(Style::default().fg(Color::White));

    let area = centered_rect(60, 60, area);
    f.render_widget(menu, area);

    let menu_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Min(10),
                Constraint::Max(3),
                Constraint::Max(3),
                Constraint::Max(3),
            ]
            .as_ref(),
        )
        .split(area);
    // Add Title to the menu
    let title_card = Paragraph::new(Text::styled(
        TITLE_CARD,
        Style::default().fg(Color::LightGreen),
    ))
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
    );
    f.render_widget(title_card, menu_chunks[0]);

    let active_style = Style::default()
        .fg(Color::Yellow)
        .add_modifier(Modifier::BOLD);

    let mut new_game_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::default().fg(Color::Green));
    let mut continue_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::default().fg(Color::Green));
    let mut quit_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::default().fg(Color::Green));

    match *selection {
        sudoku::Selection::NewGame => {
            new_game_block = new_game_block.style(active_style);
        }
        sudoku::Selection::Continue => {
            continue_block = continue_block.style(active_style);
        }
        sudoku::Selection::Quit => {
            quit_block = quit_block.style(active_style);
        }
    }
    // Add a new game button, highlight it by default
    let new_game = Paragraph::new(Text::styled(
        "New Game",
        Style::default().fg(Color::LightGreen),
    ))
    .alignment(Alignment::Center)
    .block(new_game_block);
    f.render_widget(new_game, menu_chunks[1]);

    // Add a quit button
    let quit = Paragraph::new(Text::styled("Quit", Style::default().fg(Color::LightGreen)))
        .alignment(Alignment::Center)
        .block(quit_block);
    f.render_widget(quit, menu_chunks[2]);

    // Add Continue Button
    let continue_game = Paragraph::new(Text::styled(
        "Continue Game (Coming Soon)",
        Style::default().fg(Color::LightGreen),
    ))
    .alignment(Alignment::Center)
    .block(continue_block);
    f.render_widget(continue_game, menu_chunks[3]);
}

fn render_game_over() {}

fn render_game(app: &mut App, f: &mut Frame) {}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}
