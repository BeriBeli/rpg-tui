use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use rust_i18n::t;

use crate::game::Game;
use crate::game::model::GameMode;
use crate::ui::bar;

pub fn render_stats(frame: &mut Frame, game: &Game, area: Rect) {
    let quest_status = if !game.quest.accepted {
        t!("ui.quest.none").to_string()
    } else if game.quest.completed && !game.quest.rewarded {
        t!("ui.quest.ready", reward = game.quest.reward_gold).to_string()
    } else if game.quest.rewarded {
        t!("ui.quest.done").to_string()
    } else {
        t!("ui.quest.progress", progress = game.quest.progress_text()).to_string()
    };

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
            "{} {}",
            t!("ui.stats.difficulty"),
            t!(game.difficulty.label_key())
        )),
        Line::from(format!("{} {}", t!("ui.stats.quest"), quest_status)),
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
    frame.render_widget(stats, area);
}

pub fn render_log(frame: &mut Frame, game: &Game, area: Rect) {
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
    frame.render_widget(logs, area);
}

pub fn render_controls(frame: &mut Frame, mode: GameMode, area: Rect) {
    let controls = Paragraph::new(control_lines(mode)).block(
        Block::default()
            .title(t!("ui.panel.controls"))
            .borders(Borders::ALL),
    );
    frame.render_widget(controls, area);
}

fn control_lines(mode: GameMode) -> Vec<Line<'static>> {
    match mode {
        GameMode::Exploration => vec![
            Line::from(t!("ui.controls.exploration.move")),
            Line::from(t!("ui.controls.exploration.town")),
            Line::from(t!("ui.controls.save_load")),
            Line::from(t!("ui.controls.quit")),
        ],
        GameMode::Town => vec![
            Line::from(t!("ui.controls.town.buy")),
            Line::from(t!("ui.controls.town.service")),
            Line::from(t!("ui.controls.town.leave")),
            Line::from(t!("ui.controls.save_load")),
        ],
        GameMode::Settings => vec![
            Line::from(t!("ui.controls.settings.line_1")),
            Line::from(t!("ui.controls.settings.line_2")),
            Line::from(t!("ui.controls.settings.line_3")),
            Line::from(t!("ui.controls.save_load")),
        ],
        GameMode::Battle => vec![
            Line::from(t!("ui.controls.battle.line_1")),
            Line::from(t!("ui.controls.battle.line_2")),
            Line::from(t!("ui.controls.battle.line_3")),
            Line::from(t!("ui.controls.save_load")),
        ],
        GameMode::Victory | GameMode::GameOver => vec![
            Line::from(t!("ui.controls.result.restart")),
            Line::from(t!("ui.controls.save_load")),
            Line::from(t!("ui.controls.quit")),
        ],
    }
}
