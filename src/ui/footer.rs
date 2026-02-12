use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use rust_i18n::t;

use crate::game::Game;
use crate::game::model::{GameMode, Language};
use crate::ui::bar;

pub fn render(frame: &mut Frame, game: &Game, area: Rect) {
    let bottom = match game.mode {
        GameMode::Exploration => Paragraph::new(with_recent_event(
            game,
            vec![
                Line::from(t!("ui.exploration.tip_1")),
                Line::from(t!("ui.exploration.tip_2")),
                Line::from(t!("ui.exploration.tip_3")),
            ],
        ))
        .block(
            Block::default()
                .title(t!("ui.panel.adventure"))
                .borders(Borders::ALL),
        ),
        GameMode::Town => Paragraph::new(with_recent_event(
            game,
            vec![
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
                Line::from(t!("ui.town.service_line")),
                Line::from(t!("ui.town.quest_line")),
                Line::from(t!("ui.town.leave")),
            ],
        ))
        .block(
            Block::default()
                .title(t!("ui.panel.town"))
                .borders(Borders::ALL),
        ),
        GameMode::Battle => {
            let mut lines = Vec::new();
            if let Some(recent) = &game.recent_event {
                lines.push(Line::from(Span::styled(
                    format!("{} {}", t!("ui.banner.recent"), recent),
                    Style::default().fg(Color::Yellow),
                )));
            }
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
        GameMode::Victory => Paragraph::new(with_recent_event(
            game,
            vec![
                Line::from(Span::styled(
                    t!("ui.result.victory").to_string(),
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                )),
                Line::from(t!("ui.result.restart_or_quit")),
            ],
        ))
        .block(
            Block::default()
                .title(t!("ui.panel.result"))
                .borders(Borders::ALL),
        ),
        GameMode::GameOver => Paragraph::new(with_recent_event(
            game,
            vec![
                Line::from(Span::styled(
                    t!("ui.result.game_over").to_string(),
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                )),
                Line::from(t!("ui.result.restart_or_quit")),
            ],
        ))
        .block(
            Block::default()
                .title(t!("ui.panel.result"))
                .borders(Borders::ALL),
        ),
    };

    frame.render_widget(bottom, area);
}

fn with_recent_event(game: &Game, mut lines: Vec<Line<'static>>) -> Vec<Line<'static>> {
    if let Some(recent) = &game.recent_event {
        lines.insert(
            0,
            Line::from(Span::styled(
                format!("{} {}", t!("ui.banner.recent"), recent),
                Style::default().fg(Color::Yellow),
            )),
        );
    }
    lines
}

fn settings_lines(game: &Game) -> Vec<Line<'static>> {
    let mut lines = Vec::new();
    lines.push(Line::from(t!("ui.settings.title").to_string()));
    lines.push(Line::from(t!("ui.settings.tip").to_string()));
    lines.push(Line::from(format!(
        "{} {}",
        t!("ui.settings.current_difficulty"),
        t!(game.difficulty.label_key())
    )));
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
