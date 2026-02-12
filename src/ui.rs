use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use rust_i18n::t;

use crate::game::Game;
use crate::game::model::{GameMode, Language, MAP_H, MAP_W, Tile};

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
            Constraint::Length(10),
            Constraint::Min(6),
            Constraint::Length(6),
        ])
        .split(top[1]);

    let map_widget = Paragraph::new(build_map_lines(game, top[0]))
        .block(
            Block::default()
                .title(t!("ui.panel.world"))
                .borders(Borders::ALL),
        )
        .wrap(Wrap { trim: false });
    frame.render_widget(map_widget, top[0]);

    let stats = Paragraph::new(vec![
        Line::from(format!("{} {}", t!("ui.stats.level"), game.player.level)),
        Line::from(format!(
            "{} {}  {}:{}  {}:{}",
            t!("ui.stats.gold"),
            game.player.gold,
            t!("ui.stats.potion_short"),
            game.player.bag.potion,
            t!("ui.stats.ether_short"),
            game.player.bag.ether
        )),
        Line::from(format!(
            "{} {}/{} {}",
            t!("ui.stats.hp"),
            game.player.hp,
            game.player.max_hp,
            bar(game.player.hp, game.player.max_hp, 10)
        )),
        Line::from(format!(
            "{} {}/{} {}",
            t!("ui.stats.mp"),
            game.player.mp,
            game.player.max_mp,
            bar(game.player.mp, game.player.max_mp, 10)
        )),
        Line::from(format!(
            "{} {}  {} {}",
            t!("ui.stats.atk"),
            game.player.total_atk(),
            t!("ui.stats.def"),
            game.player.total_def()
        )),
        Line::from(format!(
            "{} {} (+{})",
            t!("ui.stats.weapon"),
            t!(game.player.equipment.weapon.i18n_key()),
            game.player.equipment.weapon.bonus()
        )),
        Line::from(format!(
            "{} {} (+{})",
            t!("ui.stats.armor"),
            t!(game.player.equipment.armor.i18n_key()),
            game.player.equipment.armor.bonus()
        )),
        Line::from(format!(
            "{} {}/{}",
            t!("ui.stats.exp"),
            game.player.exp,
            game.player.next_exp
        )),
    ])
    .block(
        Block::default()
            .title(t!("ui.panel.hero"))
            .borders(Borders::ALL),
    );
    frame.render_widget(stats, right[0]);

    let log_lines: Vec<Line> = if game.log.is_empty() {
        vec![Line::from(t!("ui.log.no_events"))]
    } else {
        game.log
            .iter()
            .map(|line| Line::from(line.as_str()))
            .collect()
    };
    let logs = Paragraph::new(log_lines)
        .block(
            Block::default()
                .title(t!("ui.panel.battle_log"))
                .borders(Borders::ALL),
        )
        .wrap(Wrap { trim: true });
    frame.render_widget(logs, right[1]);

    let controls = Paragraph::new(control_lines(game.mode)).block(
        Block::default()
            .title(t!("ui.panel.controls"))
            .borders(Borders::ALL),
    );
    frame.render_widget(controls, right[2]);

    let bottom = match game.mode {
        GameMode::Exploration => Paragraph::new(vec![
            Line::from(t!("ui.exploration.tip_1")),
            Line::from(t!("ui.exploration.tip_2")),
            Line::from(t!("ui.exploration.tip_3")),
        ])
        .block(
            Block::default()
                .title(t!("ui.panel.adventure"))
                .borders(Borders::ALL),
        ),
        GameMode::Town => Paragraph::new(vec![
            Line::from(t!("ui.town.shop_title")),
            Line::from(t!("ui.town.shop_line_1")),
            Line::from(format!(
                "{} {}",
                t!("ui.town.weapon_upgrade"),
                weapon_upgrade_offer(game)
            )),
            Line::from(format!(
                "{} {}",
                t!("ui.town.armor_upgrade"),
                armor_upgrade_offer(game)
            )),
            Line::from(format!(
                "{} {}  {}:{}  {}:{}",
                t!("ui.stats.gold"),
                game.player.gold,
                t!("ui.stats.potion_short"),
                game.player.bag.potion,
                t!("ui.stats.ether_short"),
                game.player.bag.ether
            )),
            Line::from(t!("ui.town.leave")),
        ])
        .block(
            Block::default()
                .title(t!("ui.panel.town"))
                .borders(Borders::ALL),
        ),
        GameMode::Battle => {
            let mut lines = Vec::new();
            if let Some(battle) = &game.battle {
                lines.push(Line::from(Span::styled(
                    t!("ui.battle.encounter", enemy = battle.enemy.name.as_str()).to_string(),
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                )));
                lines.push(Line::from(format!(
                    "{}: {}/{} {}",
                    t!("ui.battle.enemy_hp"),
                    battle.enemy.hp.max(0),
                    battle.enemy.max_hp,
                    bar(battle.enemy.hp.max(0), battle.enemy.max_hp, 20)
                )));
            }
            lines.push(Line::from(t!("ui.battle.action_line_1")));
            lines.push(Line::from(t!("ui.battle.action_line_2")));
            Paragraph::new(lines).block(
                Block::default()
                    .title(t!("ui.panel.battle"))
                    .borders(Borders::ALL),
            )
        }
        GameMode::Settings => Paragraph::new(settings_lines(game)).block(
            Block::default()
                .title(t!("ui.panel.settings"))
                .borders(Borders::ALL),
        ),
        GameMode::Victory => Paragraph::new(vec![
            Line::from(Span::styled(
                t!("ui.result.victory").to_string(),
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(t!("ui.result.restart_or_quit")),
        ])
        .block(
            Block::default()
                .title(t!("ui.panel.result"))
                .borders(Borders::ALL),
        ),
        GameMode::GameOver => Paragraph::new(vec![
            Line::from(Span::styled(
                t!("ui.result.game_over").to_string(),
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            )),
            Line::from(t!("ui.result.restart_or_quit")),
        ])
        .block(
            Block::default()
                .title(t!("ui.panel.result"))
                .borders(Borders::ALL),
        ),
    };
    frame.render_widget(bottom, root[1]);
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

                if mx == game.player.x && my == game.player.y {
                    spans.push(Span::styled(
                        "@",
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD),
                    ));
                    continue;
                }

                let (ch, style) = match game.map[my][mx] {
                    Tile::Floor => (".", Style::default().fg(Color::DarkGray)),
                    Tile::Wall => ("#", Style::default().fg(Color::Gray)),
                    Tile::Town => ("H", Style::default().fg(Color::Cyan)),
                    Tile::Lair => ("X", Style::default().fg(Color::Red)),
                };
                spans.push(Span::styled(ch, style));
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

            if x == game.player.x && y == game.player.y {
                spans.push(Span::styled(
                    "@",
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ));
                continue;
            }

            let (ch, style) = match game.map[y][x] {
                Tile::Floor => (".", Style::default().fg(Color::DarkGray)),
                Tile::Wall => ("#", Style::default().fg(Color::Gray)),
                Tile::Town => ("H", Style::default().fg(Color::Cyan)),
                Tile::Lair => ("X", Style::default().fg(Color::Red)),
            };
            spans.push(Span::styled(ch, style));
        }
        lines.push(Line::from(spans));
    }
    lines
}

fn control_lines(mode: GameMode) -> Vec<Line<'static>> {
    match mode {
        GameMode::Exploration => vec![
            Line::from(t!("ui.controls.exploration.move")),
            Line::from(t!("ui.controls.exploration.town")),
            Line::from(t!("ui.controls.open_settings")),
            Line::from(t!("ui.controls.quit")),
        ],
        GameMode::Town => vec![
            Line::from(t!("ui.controls.town.buy")),
            Line::from(t!("ui.controls.town.leave")),
            Line::from(t!("ui.controls.open_settings")),
            Line::from(t!("ui.controls.quit")),
        ],
        GameMode::Settings => vec![
            Line::from(t!("ui.controls.settings.line_1")),
            Line::from(t!("ui.controls.settings.line_2")),
            Line::from(t!("ui.controls.settings.line_3")),
        ],
        GameMode::Battle => vec![
            Line::from(t!("ui.controls.battle.line_1")),
            Line::from(t!("ui.controls.battle.line_2")),
            Line::from(t!("ui.controls.battle.line_3")),
        ],
        GameMode::Victory | GameMode::GameOver => {
            vec![
                Line::from(t!("ui.controls.result.restart")),
                Line::from(t!("ui.controls.quit")),
            ]
        }
    }
}

fn settings_lines(game: &Game) -> Vec<Line<'static>> {
    let mut lines = Vec::new();
    lines.push(Line::from(t!("ui.settings.title").to_string()));
    lines.push(Line::from(t!("ui.settings.tip").to_string()));
    lines.push(Line::from(String::new()));
    for (idx, lang) in Language::ALL.iter().enumerate() {
        let marker = if idx == game.settings_cursor {
            ">"
        } else {
            " "
        };
        let current = if *lang == game.current_language {
            format!(" {}", t!("ui.settings.current"))
        } else {
            String::new()
        };
        lines.push(Line::from(format!(
            "{} {}{}",
            marker,
            t!(lang.label_key()),
            current
        )));
    }
    lines
}

fn weapon_upgrade_offer(game: &Game) -> String {
    let weapon = game.player.equipment.weapon;
    match (weapon.next(), weapon.upgrade_cost()) {
        (Some(next), Some(cost)) => t!(
            "ui.town.offer_with_cost",
            name = t!(next.i18n_key()),
            cost = cost
        )
        .to_string(),
        _ => t!("ui.common.max").to_string(),
    }
}

fn armor_upgrade_offer(game: &Game) -> String {
    let armor = game.player.equipment.armor;
    match (armor.next(), armor.upgrade_cost()) {
        (Some(next), Some(cost)) => t!(
            "ui.town.offer_with_cost",
            name = t!(next.i18n_key()),
            cost = cost
        )
        .to_string(),
        _ => t!("ui.common.max").to_string(),
    }
}

fn bar(cur: i32, max: i32, width: usize) -> String {
    let cur = cur.max(0);
    let max = max.max(1);
    let filled = ((cur as f32 / max as f32) * width as f32).round() as usize;
    let filled = filled.min(width);
    let empty = width.saturating_sub(filled);
    format!("[{}{}]", "=".repeat(filled), " ".repeat(empty))
}
