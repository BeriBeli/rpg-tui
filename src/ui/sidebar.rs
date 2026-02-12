use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Paragraph, Wrap};
use rust_i18n::t;

use crate::game::Game;
use crate::game::model::GameMode;
use crate::ui::{MUTED, TEXT, bar, mode_accent, panel_block};

pub fn render_stats(frame: &mut Frame, game: &Game, area: Rect, requested_scroll: usize) {
    let accent = mode_accent(game.mode);
    let (quest_status, quest_color) = if !game.quest.accepted {
        (t!("ui.quest.none").to_string(), MUTED)
    } else if game.quest.completed && !game.quest.rewarded {
        (
            t!("ui.quest.ready", reward = game.quest.reward_gold).to_string(),
            Color::Rgb(255, 206, 122),
        )
    } else if game.quest.rewarded {
        (t!("ui.quest.done").to_string(), Color::Rgb(135, 210, 145))
    } else {
        (
            t!("ui.quest.progress", progress = game.quest.progress_text()).to_string(),
            Color::Rgb(140, 200, 255),
        )
    };

    let lines = vec![
        kv_line(
            t!("ui.stats.level").to_string(),
            game.player.level.to_string(),
            accent,
        ),
        Line::from(vec![
            label_span(t!("ui.stats.gold").to_string()),
            value_span(game.player.gold.to_string(), Color::Rgb(255, 213, 124)),
            Span::raw("  "),
            label_span(t!("ui.stats.bag").to_string()),
            value_span(
                format!(
                    "{}:{}  {}:{}",
                    t!("ui.stats.potion_short"),
                    game.player.bag.potion,
                    t!("ui.stats.ether_short"),
                    game.player.bag.ether
                ),
                TEXT,
            ),
        ]),
        meter_line(
            t!("ui.stats.hp").to_string(),
            game.player.hp,
            game.player.max_hp,
            12,
            Color::Rgb(255, 132, 132),
        ),
        meter_line(
            t!("ui.stats.mp").to_string(),
            game.player.mp,
            game.player.max_mp,
            12,
            Color::Rgb(122, 178, 255),
        ),
        Line::from(vec![
            label_span(t!("ui.stats.atk").to_string()),
            value_span(
                game.player.total_atk().to_string(),
                Color::Rgb(255, 170, 110),
            ),
            Span::raw("  "),
            label_span(t!("ui.stats.def").to_string()),
            value_span(
                game.player.total_def().to_string(),
                Color::Rgb(139, 215, 161),
            ),
        ]),
        Line::from(vec![
            label_span(t!("ui.stats.weapon").to_string()),
            value_span(
                t!(game.player.equipment.weapon.i18n_key()).to_string(),
                Color::Rgb(255, 170, 110),
            ),
            Span::raw("  "),
            label_span(t!("ui.stats.armor").to_string()),
            value_span(
                t!(game.player.equipment.armor.i18n_key()).to_string(),
                Color::Rgb(139, 215, 161),
            ),
        ]),
        kv_line(
            t!("ui.stats.difficulty").to_string(),
            t!(game.difficulty.label_key()).to_string(),
            Color::Rgb(240, 189, 95),
        ),
        Line::from(vec![
            label_span(t!("ui.stats.quest").to_string()),
            value_span(quest_status, quest_color),
        ]),
        meter_line(
            t!("ui.stats.exp").to_string(),
            game.player.exp,
            game.player.next_exp,
            12,
            Color::Rgb(178, 160, 255),
        ),
    ];
    let stats = Paragraph::new(lines.clone())
        .scroll((clamp_scroll(requested_scroll, area, lines.len()), 0))
        .style(Style::default().fg(TEXT))
        .block(panel_block(t!("ui.panel.hero"), accent));
    frame.render_widget(stats, area);
}

pub fn render_log(frame: &mut Frame, game: &Game, area: Rect, requested_scroll: usize) {
    let log_lines: Vec<Line> = if game.log.is_empty() {
        vec![Line::from(Span::styled(
            t!("ui.log.no_events").to_string(),
            Style::default().fg(MUTED).add_modifier(Modifier::ITALIC),
        ))]
    } else {
        game.log
            .iter()
            .map(|line| {
                Line::from(vec![
                    Span::styled("> ", Style::default().fg(Color::Rgb(126, 149, 255))),
                    Span::raw(line.to_string()),
                ])
            })
            .collect()
    };
    let total_rows = wrapped_rows(&log_lines, area);
    let logs = Paragraph::new(log_lines)
        .style(Style::default().fg(TEXT))
        .block(panel_block(
            t!("ui.panel.battle_log"),
            Color::Rgb(126, 149, 255),
        ))
        .scroll((clamp_scroll(requested_scroll, area, total_rows), 0))
        .wrap(Wrap { trim: true });
    frame.render_widget(logs, area);
}

pub fn render_controls(frame: &mut Frame, mode: GameMode, area: Rect, requested_scroll: usize) {
    let accent = mode_accent(mode);
    let lines = control_lines(mode);
    let controls = Paragraph::new(lines.clone())
        .scroll((clamp_scroll(requested_scroll, area, lines.len()), 0))
        .style(Style::default().fg(TEXT))
        .block(panel_block(t!("ui.panel.controls"), accent));
    frame.render_widget(controls, area);
}

fn control_lines(mode: GameMode) -> Vec<Line<'static>> {
    let items: Vec<String> = match mode {
        GameMode::Exploration => vec![
            t!("ui.controls.exploration.move").to_string(),
            t!("ui.controls.exploration.town").to_string(),
            t!("ui.controls.open_settings").to_string(),
            t!("ui.controls.save_load").to_string(),
            t!("ui.controls.quit").to_string(),
        ],
        GameMode::Town => vec![
            t!("ui.controls.town.buy").to_string(),
            t!("ui.controls.town.service").to_string(),
            t!("ui.controls.town.leave").to_string(),
            t!("ui.controls.menu_select").to_string(),
            t!("ui.controls.open_settings").to_string(),
            t!("ui.controls.save_load").to_string(),
        ],
        GameMode::Settings => vec![
            t!("ui.controls.settings.line_1").to_string(),
            t!("ui.controls.settings.line_2").to_string(),
            t!("ui.controls.settings.line_3").to_string(),
            format!("6..8: {}", t!("ui.stats.difficulty")),
            t!("ui.controls.save_load").to_string(),
        ],
        GameMode::Battle => vec![
            t!("ui.controls.battle.line_1").to_string(),
            t!("ui.controls.battle.line_2").to_string(),
            t!("ui.controls.battle.line_3").to_string(),
            t!("ui.controls.menu_select").to_string(),
            t!("ui.controls.save_load").to_string(),
        ],
        GameMode::Victory | GameMode::GameOver => vec![
            t!("ui.controls.result.restart").to_string(),
            t!("ui.controls.save_load").to_string(),
            t!("ui.controls.quit").to_string(),
        ],
    };

    items
        .into_iter()
        .map(|item| {
            Line::from(vec![
                Span::styled("- ", Style::default().fg(Color::Rgb(126, 149, 255))),
                Span::raw(item),
            ])
        })
        .collect()
}

fn label_span(text: String) -> Span<'static> {
    Span::styled(
        format!("{text} "),
        Style::default().fg(MUTED).add_modifier(Modifier::BOLD),
    )
}

fn value_span(text: String, color: Color) -> Span<'static> {
    Span::styled(text, Style::default().fg(color))
}

fn kv_line(label: String, value: String, color: Color) -> Line<'static> {
    Line::from(vec![label_span(label), value_span(value, color)])
}

fn meter_line(label: String, cur: i32, max: i32, width: usize, color: Color) -> Line<'static> {
    let value = format!("{}/{} {}", cur, max, bar(cur, max, width));
    kv_line(label, value, color)
}

fn clamp_scroll(requested_scroll: usize, area: Rect, total_rows: usize) -> u16 {
    let visible_rows = area.height.saturating_sub(2) as usize;
    if visible_rows == 0 || total_rows <= visible_rows {
        return 0;
    }
    let max_scroll = total_rows.saturating_sub(visible_rows);
    requested_scroll.min(max_scroll) as u16
}

fn wrapped_rows(lines: &[Line], area: Rect) -> usize {
    let content_width = area.width.saturating_sub(2) as usize;
    if content_width == 0 {
        return 0;
    }
    lines
        .iter()
        .map(|line| {
            let width = line.width().max(1);
            ((width - 1) / content_width) + 1
        })
        .sum()
}
