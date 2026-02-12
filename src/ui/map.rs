use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Paragraph, Wrap};
use rust_i18n::t;

use crate::game::Game;
use crate::game::model::{MAP_H, MAP_W, Tile};
use crate::ui::{MUTED, TEXT, mode_accent, panel_block};

pub fn render(frame: &mut Frame, game: &Game, area: Rect) {
    let map_widget = Paragraph::new(build_map_lines(game, area))
        .style(Style::default().fg(TEXT))
        .block(panel_block(t!("ui.panel.world"), mode_accent(game.mode)))
        .wrap(Wrap { trim: false });
    frame.render_widget(map_widget, area);
}

fn build_map_lines(game: &Game, area: Rect) -> Vec<Line<'static>> {
    let view_w = area.width.saturating_sub(2) as usize;
    let view_h = area.height.saturating_sub(2) as usize;
    if view_w == 0 || view_h == 0 {
        return Vec::new();
    }

    let mut lines = Vec::with_capacity(view_h);

    for vy in 0..view_h {
        let map_y = map_index_for_view(vy, view_h, MAP_H, game.player.y);
        let mut spans = Vec::with_capacity(view_w);
        for vx in 0..view_w {
            let map_x = map_index_for_view(vx, view_w, MAP_W, game.player.x);
            match (map_x, map_y) {
                (Some(x), Some(y)) => spans.push(tile_span(game, x, y)),
                _ => spans.push(margin_span(vx, vy)),
            }
        }
        lines.push(Line::from(spans));
    }

    lines
}

fn map_index_for_view(
    view_index: usize,
    view_len: usize,
    map_len: usize,
    focus: usize,
) -> Option<usize> {
    if view_len >= map_len {
        let offset = (view_len - map_len) / 2;
        let in_map = view_index.checked_sub(offset)?;
        if in_map < map_len { Some(in_map) } else { None }
    } else {
        let start = focus
            .saturating_sub(view_len / 2)
            .min(map_len.saturating_sub(view_len));
        Some(start + view_index)
    }
}

fn margin_span(x: usize, y: usize) -> Span<'static> {
    let bg = if (x + y).is_multiple_of(2) {
        Color::Rgb(20, 24, 36)
    } else {
        Color::Rgb(17, 20, 30)
    };
    Span::styled(" ", Style::default().bg(bg))
}

fn tile_span(game: &Game, x: usize, y: usize) -> Span<'static> {
    if x == game.player.x && y == game.player.y {
        return Span::styled(
            "@",
            Style::default()
                .fg(Color::Rgb(18, 20, 30))
                .bg(Color::Rgb(124, 196, 255))
                .add_modifier(Modifier::BOLD),
        );
    }

    if game.world.has_unopened_chest(x, y) {
        return Span::styled("C", Style::default().fg(Color::Rgb(255, 213, 124)));
    }
    if game.world.has_active_npc(x, y) {
        return Span::styled("N", Style::default().fg(Color::Rgb(226, 157, 255)));
    }

    match game.map[y][x] {
        Tile::Floor if game.world.tile_is_cleared(x, y) => Span::styled(
            ",",
            Style::default()
                .fg(Color::Rgb(132, 196, 136))
                .add_modifier(Modifier::DIM),
        ),
        Tile::Floor => Span::styled(".", Style::default().fg(MUTED).add_modifier(Modifier::DIM)),
        Tile::Wall => Span::styled("#", Style::default().fg(Color::Rgb(112, 120, 142))),
        Tile::Town => Span::styled("H", Style::default().fg(Color::Rgb(113, 222, 230))),
        Tile::Lair => Span::styled("X", Style::default().fg(Color::Rgb(255, 118, 118))),
    }
}
