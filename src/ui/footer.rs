use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;
use rust_i18n::t;

use crate::game::Game;
use crate::game::model::{Difficulty, GameMode, Language};
use crate::ui::{MUTED, TEXT, bar, mode_accent, panel_block};

pub fn render(frame: &mut Frame, game: &Game, area: Rect) {
    let accent = mode_accent(game.mode);
    let bottom = match game.mode {
        GameMode::Exploration => Paragraph::new(with_recent_event(
            game,
            vec![
                Line::from(t!("ui.exploration.tip_1")),
                Line::from(t!("ui.exploration.tip_2")),
                Line::from(t!("ui.exploration.tip_3")),
            ],
            accent,
        ))
        .style(Style::default().fg(TEXT))
        .block(panel_block(t!("ui.panel.adventure"), accent)),
        GameMode::Town => Paragraph::new(with_recent_event(game, town_lines(game, accent), accent))
            .scroll((town_scroll(game, area), 0))
            .style(Style::default().fg(TEXT))
            .block(panel_block(t!("ui.panel.town"), accent)),
        GameMode::Battle => Paragraph::new(battle_lines(game, accent))
            .scroll((battle_scroll(game, area), 0))
            .style(Style::default().fg(TEXT))
            .block(panel_block(t!("ui.panel.battle"), accent)),
        GameMode::Settings => Paragraph::new(settings_lines(game))
            .scroll((settings_scroll(game, area), 0))
            .style(Style::default().fg(TEXT))
            .block(panel_block(t!("ui.panel.settings"), accent)),
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
            accent,
        ))
        .style(Style::default().fg(TEXT))
        .block(panel_block(t!("ui.panel.result"), accent)),
        GameMode::GameOver => Paragraph::new(with_recent_event(
            game,
            vec![
                Line::from(Span::styled(
                    t!("ui.result.game_over").to_string(),
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                )),
                Line::from(t!("ui.result.restart_or_quit")),
            ],
            accent,
        ))
        .style(Style::default().fg(TEXT))
        .block(panel_block(t!("ui.panel.result"), accent)),
    };

    frame.render_widget(bottom, area);
}

fn with_recent_event(
    game: &Game,
    mut lines: Vec<Line<'static>>,
    accent: Color,
) -> Vec<Line<'static>> {
    if let Some(recent) = &game.recent_event {
        lines.insert(
            0,
            Line::from(Span::styled(
                format!("{} {}", t!("ui.banner.recent"), recent),
                Style::default().fg(accent).add_modifier(Modifier::BOLD),
            )),
        );
    }
    lines
}

fn settings_lines(game: &Game) -> Vec<Line<'static>> {
    let mut lines = Vec::new();
    lines.push(Line::from(Span::styled(
        t!("ui.settings.title").to_string(),
        Style::default().fg(TEXT).add_modifier(Modifier::BOLD),
    )));
    lines.push(Line::from(Span::styled(
        format!(
            "1..{} {}  {}..{} {}",
            Language::ALL.len(),
            t!("ui.settings.title"),
            Language::ALL.len() + 1,
            Language::ALL.len() + Difficulty::ALL.len(),
            t!("ui.stats.difficulty")
        ),
        Style::default().fg(MUTED),
    )));
    lines.push(Line::from(Span::styled(
        t!("ui.settings.tip").to_string(),
        Style::default().fg(MUTED),
    )));
    lines.push(Line::from(Span::styled(
        t!("ui.settings.title").to_string(),
        Style::default().fg(Color::Rgb(240, 189, 95)),
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
        lines.push(Line::from(vec![
            Span::styled(
                format!("{marker}{}. ", idx + 1),
                Style::default()
                    .fg(if idx == game.settings_cursor {
                        Color::Rgb(240, 189, 95)
                    } else {
                        MUTED
                    })
                    .add_modifier(if idx == game.settings_cursor {
                        Modifier::BOLD
                    } else {
                        Modifier::empty()
                    }),
            ),
            Span::styled(
                t!(lang.label_key()).to_string(),
                Style::default().fg(if idx == game.settings_cursor {
                    TEXT
                } else {
                    MUTED
                }),
            ),
            Span::styled(current, Style::default().fg(Color::Rgb(113, 222, 230))),
        ]));
    }
    lines.push(Line::from(Span::styled(
        t!("ui.stats.difficulty").to_string(),
        Style::default().fg(Color::Rgb(240, 189, 95)),
    )));
    for (idx, difficulty) in Difficulty::ALL.iter().enumerate() {
        let option_index = Language::ALL.len() + idx;
        let marker = if option_index == game.settings_cursor {
            ">"
        } else {
            " "
        };
        let current = if *difficulty == game.difficulty {
            format!(" {}", t!("ui.settings.current"))
        } else {
            String::new()
        };
        lines.push(Line::from(vec![
            Span::styled(
                format!("{marker}{}. ", option_index + 1),
                Style::default()
                    .fg(if option_index == game.settings_cursor {
                        Color::Rgb(240, 189, 95)
                    } else {
                        MUTED
                    })
                    .add_modifier(if option_index == game.settings_cursor {
                        Modifier::BOLD
                    } else {
                        Modifier::empty()
                    }),
            ),
            Span::styled(
                t!(difficulty.label_key()).to_string(),
                Style::default().fg(if option_index == game.settings_cursor {
                    TEXT
                } else {
                    MUTED
                }),
            ),
            Span::styled(current, Style::default().fg(Color::Rgb(113, 222, 230))),
        ]));
    }
    lines
}

fn town_lines(game: &Game, accent: Color) -> Vec<Line<'static>> {
    let options = vec![
        t!("ui.town.action_buy_potion").to_string(),
        t!("ui.town.action_buy_ether").to_string(),
        format!(
            "{} {}",
            t!("ui.town.action_upgrade_weapon"),
            weapon_upgrade_offer(game)
        ),
        format!(
            "{} {}",
            t!("ui.town.action_upgrade_armor"),
            armor_upgrade_offer(game)
        ),
        t!("ui.town.action_healer").to_string(),
        t!("ui.town.action_inn").to_string(),
        t!("ui.town.action_quest_board").to_string(),
        t!("ui.town.action_leave").to_string(),
    ];

    let mut lines = Vec::new();
    lines.push(Line::from(Span::styled(
        t!("ui.town.shop_title").to_string(),
        Style::default().fg(TEXT).add_modifier(Modifier::BOLD),
    )));
    for (idx, text) in options.into_iter().enumerate() {
        lines.push(selectable_option_line(
            idx + 1,
            idx == game.town_cursor,
            text,
            accent,
        ));
    }
    lines
}

fn battle_lines(game: &Game, accent: Color) -> Vec<Line<'static>> {
    let mut lines = Vec::new();
    if let Some(recent) = &game.recent_event {
        lines.push(Line::from(Span::styled(
            format!("{} {}", t!("ui.banner.recent"), recent),
            Style::default().fg(accent).add_modifier(Modifier::BOLD),
        )));
    }
    if let Some(battle) = &game.battle {
        lines.push(Line::from(Span::styled(
            t!("ui.battle.encounter", enemy = battle.enemy.name.as_str()).to_string(),
            Style::default()
                .fg(Color::Rgb(255, 121, 121))
                .add_modifier(Modifier::BOLD),
        )));
        lines.push(Line::from(format!(
            "{}: {}/{} {}",
            t!("ui.battle.enemy_hp"),
            battle.enemy.hp.max(0),
            battle.enemy.max_hp,
            bar(battle.enemy.hp.max(0), battle.enemy.max_hp, 20)
        )));
    }
    lines.push(Line::from(Span::styled(
        t!("ui.panel.controls").to_string(),
        Style::default().fg(TEXT).add_modifier(Modifier::BOLD),
    )));

    let actions = vec![
        t!("ui.battle.action_attack").to_string(),
        t!("ui.battle.action_fire_slash").to_string(),
        t!("ui.battle.action_defend").to_string(),
        t!("ui.battle.action_potion").to_string(),
        t!("ui.battle.action_ether").to_string(),
        t!("ui.battle.action_run").to_string(),
    ];
    for (idx, text) in actions.into_iter().enumerate() {
        lines.push(selectable_option_line(
            idx + 1,
            idx == game.battle_cursor,
            text,
            accent,
        ));
    }
    lines
}

fn selectable_option_line(
    number: usize,
    selected: bool,
    text: String,
    accent: Color,
) -> Line<'static> {
    Line::from(vec![
        Span::styled(
            format!("{}{}. ", if selected { ">" } else { " " }, number),
            Style::default()
                .fg(if selected { accent } else { MUTED })
                .add_modifier(if selected {
                    Modifier::BOLD
                } else {
                    Modifier::empty()
                }),
        ),
        Span::styled(
            text,
            Style::default()
                .fg(if selected { TEXT } else { MUTED })
                .add_modifier(if selected {
                    Modifier::BOLD
                } else {
                    Modifier::empty()
                }),
        ),
    ])
}

fn settings_scroll(game: &Game, area: Rect) -> u16 {
    let visible_rows = area.height.saturating_sub(2) as usize;
    if visible_rows == 0 {
        return 0;
    }

    let total_rows = settings_total_rows();
    if total_rows <= visible_rows {
        return 0;
    }

    let selected_row = settings_selected_row(game);
    let max_scroll = total_rows.saturating_sub(visible_rows);
    selected_row
        .saturating_sub(visible_rows / 2)
        .min(max_scroll) as u16
}

fn settings_total_rows() -> usize {
    4 + Language::ALL.len() + 1 + Difficulty::ALL.len()
}

fn settings_selected_row(game: &Game) -> usize {
    if game.settings_cursor < Language::ALL.len() {
        4 + game.settings_cursor
    } else {
        5 + game.settings_cursor
    }
}

fn town_scroll(game: &Game, area: Rect) -> u16 {
    let visible_rows = area.height.saturating_sub(2) as usize;
    if visible_rows == 0 {
        return 0;
    }
    let selected_row = town_selected_row(game);
    let total_rows = town_total_rows(game);
    if total_rows <= visible_rows {
        return 0;
    }
    let max_scroll = total_rows.saturating_sub(visible_rows);
    selected_row
        .saturating_sub(visible_rows / 2)
        .min(max_scroll) as u16
}

fn town_total_rows(game: &Game) -> usize {
    let base_rows = 1 + 8;
    if game.recent_event.is_some() {
        base_rows + 1
    } else {
        base_rows
    }
}

fn town_selected_row(game: &Game) -> usize {
    let base = if game.recent_event.is_some() { 2 } else { 1 };
    base + game.town_cursor
}

fn battle_scroll(game: &Game, area: Rect) -> u16 {
    let visible_rows = area.height.saturating_sub(2) as usize;
    if visible_rows == 0 {
        return 0;
    }
    let selected_row = battle_selected_row(game);
    let total_rows = battle_total_rows(game);
    if total_rows <= visible_rows {
        return 0;
    }
    let max_scroll = total_rows.saturating_sub(visible_rows);
    selected_row
        .saturating_sub(visible_rows / 2)
        .min(max_scroll) as u16
}

fn battle_total_rows(game: &Game) -> usize {
    let mut total = 1 + 6;
    if game.recent_event.is_some() {
        total += 1;
    }
    if game.battle.is_some() {
        total += 2;
    }
    total
}

fn battle_selected_row(game: &Game) -> usize {
    let mut base = 1;
    if game.recent_event.is_some() {
        base += 1;
    }
    if game.battle.is_some() {
        base += 2;
    }
    base + game.battle_cursor
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
