use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use rust_i18n::t;

use crate::game::Game;
use crate::game::model::{MAP_H, MAP_W, Tile};

pub fn render(frame: &mut Frame, game: &Game, area: Rect) {
    let map_widget = Paragraph::new(build_map_lines(game, area))
        .block(
            Block::default()
                .title(t!("ui.panel.world"))
                .borders(Borders::ALL),
        )
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

    if view_w >= MAP_W && view_h >= MAP_H {
        let offset_x = (view_w - MAP_W) / 2;
        let offset_y = (view_h - MAP_H) / 2;

        for vy in 0..view_h {
            let mut spans = Vec::with_capacity(view_w);
            for vx in 0..view_w {
                let in_map_x = vx.checked_sub(offset_x);
                let in_map_y = vy.checked_sub(offset_y);
                let Some(mx) = in_map_x else {
                    spans.push(Span::raw(" "));
                    continue;
                };
                let Some(my) = in_map_y else {
                    spans.push(Span::raw(" "));
                    continue;
                };
                if mx >= MAP_W || my >= MAP_H {
                    spans.push(Span::raw(" "));
                    continue;
                }
                spans.push(tile_span(game, mx, my));
            }
            lines.push(Line::from(spans));
        }
        return lines;
    }

    let x0 = game
        .player
        .x
        .saturating_sub(view_w / 2)
        .min(MAP_W.saturating_sub(view_w));
    let y0 = game
        .player
        .y
        .saturating_sub(view_h / 2)
        .min(MAP_H.saturating_sub(view_h));

    for vy in 0..view_h {
        let mut spans = Vec::with_capacity(view_w);
        for vx in 0..view_w {
            let x = x0 + vx;
            let y = y0 + vy;
            spans.push(tile_span(game, x, y));
        }
        lines.push(Line::from(spans));
    }

    lines
}

fn tile_span(game: &Game, x: usize, y: usize) -> Span<'static> {
    if x == game.player.x && y == game.player.y {
        return Span::styled(
            "@",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        );
    }

    if game.world.has_unopened_chest(x, y) {
        return Span::styled("C", Style::default().fg(Color::LightYellow));
    }
    if game.world.has_active_npc(x, y) {
        return Span::styled("N", Style::default().fg(Color::LightMagenta));
    }

    match game.map[y][x] {
        Tile::Floor if game.world.tile_is_cleared(x, y) => Span::styled(
            ",",
            Style::default()
                .fg(Color::LightGreen)
                .add_modifier(Modifier::DIM),
        ),
        Tile::Floor => Span::styled(".", Style::default().fg(Color::DarkGray)),
        Tile::Wall => Span::styled("#", Style::default().fg(Color::Gray)),
        Tile::Town => Span::styled("H", Style::default().fg(Color::Cyan)),
        Tile::Lair => Span::styled("X", Style::default().fg(Color::Red)),
    }
}
