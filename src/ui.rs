use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Margin, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, BorderType, Borders};

use crate::game::Game;
use crate::game::model::GameMode;

mod footer;
mod map;
mod sidebar;

pub(crate) const SURFACE: Color = Color::Rgb(17, 20, 30);
pub(crate) const TEXT: Color = Color::Rgb(230, 232, 241);
pub(crate) const MUTED: Color = Color::Rgb(145, 152, 173);
const ROOT_TOP_PERCENT: u16 = 72;
const MAP_PERCENT: u16 = 68;
const RIGHT_STATS_PERCENT: u16 = 40;
const RIGHT_LOG_PERCENT: u16 = 34;
const RIGHT_CONTROLS_PERCENT: u16 = 26;

pub fn render(frame: &mut Frame, game: &Game) {
    let layout = split_layout(frame.area());

    map::render(frame, game, layout.map);
    sidebar::render_stats(frame, game, layout.hero, game.hero_scroll);
    sidebar::render_log(frame, game, layout.log, game.log_scroll);
    sidebar::render_controls(frame, game.mode, layout.controls, game.controls_scroll);
    footer::render(frame, game, layout.footer);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScrollTarget {
    Hero,
    BattleLog,
    Controls,
}

pub fn scroll_target_at(frame_area: Rect, x: u16, y: u16) -> Option<ScrollTarget> {
    let layout = split_layout(frame_area);
    if rect_contains(layout.hero, x, y) {
        return Some(ScrollTarget::Hero);
    }
    if rect_contains(layout.log, x, y) {
        return Some(ScrollTarget::BattleLog);
    }
    if rect_contains(layout.controls, x, y) {
        return Some(ScrollTarget::Controls);
    }
    None
}

#[derive(Clone, Copy)]
struct UiLayout {
    map: Rect,
    hero: Rect,
    log: Rect,
    controls: Rect,
    footer: Rect,
}

fn split_layout(frame_area: Rect) -> UiLayout {
    let area = frame_area.inner(Margin {
        horizontal: 1,
        vertical: 0,
    });
    let root = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(ROOT_TOP_PERCENT),
            Constraint::Percentage(100 - ROOT_TOP_PERCENT),
        ])
        .split(area);
    let top = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(MAP_PERCENT),
            Constraint::Percentage(100 - MAP_PERCENT),
        ])
        .split(root[0]);
    let right = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(RIGHT_STATS_PERCENT),
            Constraint::Percentage(RIGHT_LOG_PERCENT),
            Constraint::Percentage(RIGHT_CONTROLS_PERCENT),
        ])
        .split(top[1]);

    UiLayout {
        map: top[0],
        hero: right[0],
        log: right[1],
        controls: right[2],
        footer: root[1],
    }
}

fn rect_contains(rect: Rect, x: u16, y: u16) -> bool {
    x >= rect.x
        && x < rect.x.saturating_add(rect.width)
        && y >= rect.y
        && y < rect.y.saturating_add(rect.height)
}

pub(crate) fn mode_accent(mode: GameMode) -> Color {
    match mode {
        GameMode::Exploration => Color::Rgb(80, 175, 255),
        GameMode::Town => Color::Rgb(115, 212, 130),
        GameMode::Settings => Color::Rgb(240, 189, 95),
        GameMode::Battle => Color::Rgb(255, 121, 121),
        GameMode::Victory => Color::Rgb(118, 215, 141),
        GameMode::GameOver => Color::Rgb(228, 94, 84),
    }
}

pub(crate) fn panel_block<'a, T>(title: T, accent: Color) -> Block<'a>
where
    T: Into<Line<'a>>,
{
    Block::default()
        .title(title)
        .title_style(Style::default().fg(accent).add_modifier(Modifier::BOLD))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(accent))
        .style(Style::default().fg(TEXT).bg(SURFACE))
}

pub(crate) fn bar(cur: i32, max: i32, width: usize) -> String {
    let cur = cur.max(0);
    let max = max.max(1);
    let filled = ((cur as f32 / max as f32) * width as f32).round() as usize;
    let filled = filled.min(width);
    let empty = width.saturating_sub(filled);
    format!("[{}{}]", "=".repeat(filled), " ".repeat(empty))
}

#[cfg(test)]
mod tests {
    use ratatui::Terminal;
    use ratatui::backend::TestBackend;

    use crate::game::Game;
    use crate::game::model::GameMode;

    use super::render;

    #[test]
    fn render_settings_on_small_terminal_does_not_panic() {
        rust_i18n::set_locale("en");
        let mut terminal = Terminal::new(TestBackend::new(60, 16)).expect("terminal init");
        let mut game = Game::new_with_seed(2026);
        game.mode = GameMode::Settings;

        terminal
            .draw(|frame| render(frame, &game))
            .expect("settings render should succeed");
    }

    #[test]
    fn render_exploration_on_wide_short_terminal_does_not_panic() {
        rust_i18n::set_locale("en");
        let mut terminal = Terminal::new(TestBackend::new(80, 10)).expect("terminal init");
        let game = Game::new_with_seed(2026);

        terminal
            .draw(|frame| render(frame, &game))
            .expect("exploration render should succeed");
    }
}
