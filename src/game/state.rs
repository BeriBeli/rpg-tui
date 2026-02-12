use std::collections::VecDeque;

use crossterm::event::KeyCode;
use rand::Rng;
use rust_i18n::t;

use crate::game::balance::RANDOM_ENCOUNTER_RATE_PERCENT;
use crate::game::battle::{self, BattleOutcome};
use crate::game::encounter;
use crate::game::event;
use crate::game::model::{
    Battle, Enemy, GameMode, LOG_CAPACITY, Language, MAP_H, MAP_W, Player, Tile,
};
use crate::game::progression;
use crate::game::town::{self, TownAction, TownOutcome};
use crate::game::world::generate_map;

pub struct Game {
    pub mode: GameMode,
    pub map: Vec<Vec<Tile>>,
    pub player: Player,
    pub battle: Option<Battle>,
    pub log: VecDeque<String>,
    pub should_quit: bool,
    pub current_language: Language,
    pub settings_cursor: usize,
    rng: rand::rngs::ThreadRng,
    settings_return_mode: GameMode,
}

impl Game {
    pub fn new() -> Self {
        let initial_language = std::env::var("RPG_LANG")
            .ok()
            .as_deref()
            .map(Language::from_locale_tag)
            .unwrap_or(Language::En);
        Self::new_with_language(initial_language)
    }

    fn new_with_language(language: Language) -> Self {
        rust_i18n::set_locale(language.locale_code());
        let mut rng = rand::rng();
        let map = generate_map(&mut rng);
        let mut game = Self {
            mode: GameMode::Exploration,
            map,
            player: Player::new(),
            battle: None,
            log: VecDeque::new(),
            should_quit: false,
            current_language: language,
            settings_cursor: language.index(),
            rng,
            settings_return_mode: GameMode::Exploration,
        };
        game.push_log(t!("log.game.welcome"));
        game.push_log(t!("log.game.town_hint"));
        game
    }

    pub fn push_log(&mut self, msg: impl Into<String>) {
        if self.log.len() >= LOG_CAPACITY {
            self.log.pop_front();
        }
        self.log.push_back(msg.into());
    }

    pub fn handle_key(&mut self, code: KeyCode) {
        if matches!(code, KeyCode::Char('q')) {
            self.should_quit = true;
            return;
        }
        if matches!(code, KeyCode::Esc) {
            if self.mode == GameMode::Settings {
                self.close_settings();
                return;
            }
            self.should_quit = true;
            return;
        }

        match self.mode {
            GameMode::Exploration => self.handle_exploration_key(code),
            GameMode::Town => self.handle_town_key(code),
            GameMode::Settings => self.handle_settings_key(code),
            GameMode::Battle => self.handle_battle_key(code),
            GameMode::Victory | GameMode::GameOver => {
                if matches!(code, KeyCode::Char('r')) {
                    *self = Self::new_with_language(self.current_language);
                }
            }
        }
    }

    fn handle_exploration_key(&mut self, code: KeyCode) {
        if matches!(code, KeyCode::Char('o')) {
            self.open_settings(GameMode::Exploration);
            return;
        }
        if matches!(code, KeyCode::Char('t')) && self.current_tile() == Tile::Town {
            self.mode = GameMode::Town;
            self.push_log(t!("log.town.menu_opened"));
            return;
        }

        let (dx, dy) = match code {
            KeyCode::Up | KeyCode::Char('w') => (0, -1),
            KeyCode::Down | KeyCode::Char('s') => (0, 1),
            KeyCode::Left | KeyCode::Char('a') => (-1, 0),
            KeyCode::Right | KeyCode::Char('d') => (1, 0),
            _ => return,
        };
        self.try_move_player(dx, dy);
    }

    fn handle_town_key(&mut self, code: KeyCode) {
        if matches!(code, KeyCode::Char('o')) {
            self.open_settings(GameMode::Town);
            return;
        }
        let action = match code {
            KeyCode::Char('1') => Some(TownAction::BuyPotion),
            KeyCode::Char('2') => Some(TownAction::BuyEther),
            KeyCode::Char('3') => Some(TownAction::UpgradeWeapon),
            KeyCode::Char('4') => Some(TownAction::UpgradeArmor),
            KeyCode::Char('5') => Some(TownAction::Leave),
            _ => None,
        };

        let Some(action) = action else {
            return;
        };
        match town::apply_action(&mut self.player, action) {
            TownOutcome::Stay(message) => self.push_log(message),
            TownOutcome::Leave(message) => {
                self.mode = GameMode::Exploration;
                self.push_log(message);
            }
        }
    }

    fn handle_settings_key(&mut self, code: KeyCode) {
        match code {
            KeyCode::Up | KeyCode::Char('w') => {
                if self.settings_cursor == 0 {
                    self.settings_cursor = Language::ALL.len() - 1;
                } else {
                    self.settings_cursor -= 1;
                }
            }
            KeyCode::Down | KeyCode::Char('s') => {
                self.settings_cursor = (self.settings_cursor + 1) % Language::ALL.len();
            }
            KeyCode::Char('1') => self.select_language(0),
            KeyCode::Char('2') => self.select_language(1),
            KeyCode::Char('3') => self.select_language(2),
            KeyCode::Char('4') => self.select_language(3),
            KeyCode::Char('5') => self.select_language(4),
            KeyCode::Enter => self.select_language(self.settings_cursor),
            KeyCode::Char('b') => self.close_settings(),
            _ => {}
        }
    }

    fn handle_battle_key(&mut self, code: KeyCode) {
        let Some(action) = battle::action_from_key(code) else {
            return;
        };

        let Some(mut battle) = self.battle.take() else {
            self.mode = GameMode::Exploration;
            return;
        };

        let result = battle::resolve_turn(action, &mut battle, &mut self.player, &mut self.rng);
        for message in result.logs {
            self.push_log(message);
        }

        match result.outcome {
            BattleOutcome::Continue => {
                self.mode = GameMode::Battle;
                self.battle = Some(battle);
            }
            BattleOutcome::Escaped => {
                self.mode = GameMode::Exploration;
                self.battle = None;
            }
            BattleOutcome::EnemyDefeated(enemy) => self.win_battle(enemy),
            BattleOutcome::PlayerDefeated => {
                self.mode = GameMode::GameOver;
                self.battle = None;
                self.push_log(t!("log.game.player_fallen_restart"));
            }
        }
    }

    fn try_move_player(&mut self, dx: i32, dy: i32) {
        let nx = self.player.x as i32 + dx;
        let ny = self.player.y as i32 + dy;
        if nx < 0 || ny < 0 || nx as usize >= MAP_W || ny as usize >= MAP_H {
            return;
        }

        let nx = nx as usize;
        let ny = ny as usize;
        if self.map[ny][nx] == Tile::Wall {
            return;
        }

        self.player.x = nx;
        self.player.y = ny;
        match self.map[ny][nx] {
            Tile::Town => {
                self.player.hp = self.player.max_hp;
                self.player.mp = self.player.max_mp;
                self.mode = GameMode::Town;
                self.push_log(t!("log.town.arrived_restore"));
            }
            Tile::Lair => self.start_boss_battle(),
            Tile::Floor => {
                if self.rng.random_range(0..100) < RANDOM_ENCOUNTER_RATE_PERCENT {
                    self.start_random_battle();
                } else if let Some(event_result) =
                    event::maybe_trigger_event(&mut self.rng, &mut self.player)
                {
                    self.push_log(event_result.message);
                    if event_result.player_dead {
                        self.mode = GameMode::GameOver;
                        self.battle = None;
                        self.push_log(t!("log.game.player_fallen_restart"));
                    }
                }
            }
            Tile::Wall => {}
        }
    }

    fn start_random_battle(&mut self) {
        let enemy = encounter::generate_enemy(&mut self.rng, self.player.level, false);
        self.push_log(t!("log.battle.wild_appears", enemy = enemy.name.as_str()));
        self.battle = Some(Battle {
            enemy,
            defending: false,
        });
        self.mode = GameMode::Battle;
    }

    fn start_boss_battle(&mut self) {
        let enemy = encounter::generate_enemy(&mut self.rng, self.player.level, true);
        self.push_log(t!(
            "log.battle.boss_blocks_path",
            enemy = enemy.name.as_str()
        ));
        self.battle = Some(Battle {
            enemy,
            defending: false,
        });
        self.mode = GameMode::Battle;
    }

    fn win_battle(&mut self, enemy: Enemy) {
        self.battle = None;
        for log in progression::apply_battle_rewards(&mut self.player, &enemy) {
            self.push_log(log);
        }

        if enemy.is_boss {
            self.mode = GameMode::Victory;
            self.push_log(t!("log.game.dragon_defeated_restart_or_quit"));
        } else {
            self.mode = GameMode::Exploration;
        }
    }

    fn current_tile(&self) -> Tile {
        self.map[self.player.y][self.player.x]
    }

    fn open_settings(&mut self, from_mode: GameMode) {
        self.settings_return_mode = from_mode;
        self.settings_cursor = self.current_language.index();
        self.mode = GameMode::Settings;
        self.push_log(t!("log.settings.opened"));
    }

    fn close_settings(&mut self) {
        let to = if self.settings_return_mode == GameMode::Settings {
            GameMode::Exploration
        } else {
            self.settings_return_mode
        };
        self.mode = to;
    }

    fn select_language(&mut self, idx: usize) {
        if idx >= Language::ALL.len() {
            return;
        }
        self.settings_cursor = idx;
        let lang = Language::ALL[idx];
        self.current_language = lang;
        rust_i18n::set_locale(lang.locale_code());
        self.push_log(t!(
            "log.settings.language_changed",
            lang = t!(lang.label_key())
        ));
    }
}

#[cfg(test)]
mod tests {
    use crossterm::event::KeyCode;

    use super::Game;
    use crate::game::model::GameMode;

    #[test]
    fn town_purchase_through_handle_key_updates_player_state() {
        rust_i18n::set_locale("en");
        let mut game = Game::new();
        game.mode = GameMode::Town;
        game.player.gold = 20;
        game.player.bag.potion = 0;

        game.handle_key(KeyCode::Char('1'));

        assert_eq!(game.player.gold, 10);
        assert_eq!(game.player.bag.potion, 1);
        assert_eq!(game.mode, GameMode::Town);
    }

    #[test]
    fn town_leave_through_handle_key_returns_to_exploration() {
        rust_i18n::set_locale("en");
        let mut game = Game::new();
        game.mode = GameMode::Town;

        game.handle_key(KeyCode::Char('5'));

        assert_eq!(game.mode, GameMode::Exploration);
    }
}
