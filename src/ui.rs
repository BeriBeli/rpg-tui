use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout};

use crate::game::Game;

mod footer;
mod map;
mod sidebar;

pub fn render(frame: &mut Frame, game: &Game) {
    let root = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(14), Constraint::Length(10)])
        .split(frame.area());
    let top = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(62), Constraint::Percentage(38)])
        .split(root[0]);
    let right = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(12),
            Constraint::Min(6),
            Constraint::Length(7),
        ])
        .split(top[1]);

    map::render(frame, game, top[0]);
    sidebar::render_stats(frame, game, right[0]);
    sidebar::render_log(frame, game, right[1]);
    sidebar::render_controls(frame, game.mode, right[2]);
    footer::render(frame, game, root[1]);
}

pub(crate) fn bar(cur: i32, max: i32, width: usize) -> String {
    let cur = cur.max(0);
    let max = max.max(1);
    let filled = ((cur as f32 / max as f32) * width as f32).round() as usize;
    let filled = filled.min(width);
    let empty = width.saturating_sub(filled);
    format!("[{}{}]", "=".repeat(filled), " ".repeat(empty))
}
